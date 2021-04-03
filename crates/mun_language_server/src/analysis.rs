use crate::{
    cancelation::Canceled, change::AnalysisChange, db::AnalysisDatabase, diagnostics,
    diagnostics::Diagnostic, file_structure,
};
use hir::{line_index::LineIndex, AstDatabase, SourceDatabase};
use salsa::{ParallelDatabase, Snapshot};
use std::sync::Arc;

/// Result of an operation that can be canceled.
pub type Cancelable<T> = Result<T, Canceled>;

/// The `Analysis` struct is the basis of all language server operations. It maintains the current
/// state of the source.
pub struct Analysis {
    db: AnalysisDatabase,
}

impl Analysis {
    pub fn new() -> Self {
        Analysis {
            db: AnalysisDatabase::new(),
        }
    }

    /// Applies the given changes to the state. If there are outstanding `AnalysisSnapshot`s they
    /// will be canceled.
    pub fn apply_change(&mut self, change: AnalysisChange) {
        self.db.apply_change(change)
    }

    /// Creates a snapshot of the current `Analysis`. You can query the resulting `AnalysisSnapshot`
    /// to get analysis and diagnostics.
    pub fn snapshot(&self) -> AnalysisSnapshot {
        AnalysisSnapshot {
            db: self.db.snapshot(),
        }
    }

    /// Requests any outstanding snapshot to cancel computations.
    pub fn request_cancelation(&mut self) {
        self.db.request_cancelation();
    }
}

/// The `AnalysisSnapshot` is a snapshot of the state of the source, it enables querying for
/// the snapshot in a consistent state.
///
/// A `AnalysisSnapshot` is created by calling `Analysis::snapshot`. When applying changes to the
/// `Analysis` struct through the use of `Analysis::apply_changes` all snapshots are cancelled (most
/// methods return `Err(Canceled)`).
pub struct AnalysisSnapshot {
    db: Snapshot<AnalysisDatabase>,
}

impl AnalysisSnapshot {
    /// Computes the set of diagnostics for the given file.
    pub fn diagnostics(&self, file_id: hir::FileId) -> Cancelable<Vec<Diagnostic>> {
        self.with_db(|db| diagnostics::diagnostics(db, file_id))
    }

    /// Returns all the source files of the given package
    pub fn package_source_files(&self, package_id: hir::PackageId) -> Cancelable<Vec<hir::FileId>> {
        self.with_db(|db| {
            let packages = db.packages();
            let source_root = db.source_root(packages[package_id].source_root);
            source_root.files().collect()
        })
    }

    /// Returns the line index for the specified file
    pub fn file_line_index(&self, file_id: hir::FileId) -> Cancelable<Arc<LineIndex>> {
        self.with_db(|db| db.line_index(file_id))
    }

    /// Returns a tree structure of the symbols of a file.
    pub fn file_structure(
        &self,
        file_id: hir::FileId,
    ) -> Cancelable<Vec<file_structure::StructureNode>> {
        self.with_db(|db| file_structure::file_structure(&db.parse(file_id).tree()))
    }

    /// Performs an operation on that may be Canceled.
    fn with_db<F: FnOnce(&AnalysisDatabase) -> T + std::panic::UnwindSafe, T>(
        &self,
        f: F,
    ) -> Cancelable<T> {
        self.db.catch_canceled(f)
    }
}
