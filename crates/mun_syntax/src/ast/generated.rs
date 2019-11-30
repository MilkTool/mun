//! Generated file, do not edit by hand, see `crate/ra_tools/src/codegen`

// This file is automatically generated based on the file `./generated.rs.tera` when `cargo gen-syntax` is run
// Do not edit manually

//! This module contains auto-generated Rust AST. Like `SyntaxNode`s, AST nodes
//! are generic over ownership: `X<'a>` things are `Copy` references, `XNode`
//! are Arc-based. You can switch between the two variants using `.owned` and
//! `.borrowed` functions. Most of the code works with borrowed mode, and only
//! this mode has all AST accessors.

use crate::{
    ast::{self, AstNode},
    SyntaxKind::{self, *},
    SyntaxNode,
};

// ArgList

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArgList {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for ArgList {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            ARG_LIST => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(ArgList { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ArgList {
    pub fn args(&self) -> impl Iterator<Item = Expr> {
        super::children(self)
    }
}

// BinExpr

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinExpr {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for BinExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            BIN_EXPR => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(BinExpr { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl BinExpr {}

// BindPat

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BindPat {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for BindPat {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            BIND_PAT => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(BindPat { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ast::NameOwner for BindPat {}
impl BindPat {
    pub fn pat(&self) -> Option<Pat> {
        super::child_opt(self)
    }
}

// BlockExpr

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockExpr {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for BlockExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            BLOCK_EXPR => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(BlockExpr { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl BlockExpr {
    pub fn statements(&self) -> impl Iterator<Item = Stmt> {
        super::children(self)
    }

    pub fn expr(&self) -> Option<Expr> {
        super::child_opt(self)
    }
}

// BreakExpr

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BreakExpr {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for BreakExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            BREAK_EXPR => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(BreakExpr { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl BreakExpr {
    pub fn expr(&self) -> Option<Expr> {
        super::child_opt(self)
    }
}

// CallExpr

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CallExpr {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for CallExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CALL_EXPR => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(CallExpr { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ast::ArgListOwner for CallExpr {}
impl CallExpr {
    pub fn expr(&self) -> Option<Expr> {
        super::child_opt(self)
    }
}

// Condition

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Condition {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for Condition {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CONDITION => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Condition { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Condition {
    pub fn pat(&self) -> Option<Pat> {
        super::child_opt(self)
    }

    pub fn expr(&self) -> Option<Expr> {
        super::child_opt(self)
    }
}

// Expr

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Expr {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for Expr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            LITERAL | PREFIX_EXPR | PATH_EXPR | BIN_EXPR | PAREN_EXPR | CALL_EXPR | IF_EXPR
            | LOOP_EXPR | WHILE_EXPR | RETURN_EXPR | BREAK_EXPR | BLOCK_EXPR => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Expr { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExprKind {
    Literal(Literal),
    PrefixExpr(PrefixExpr),
    PathExpr(PathExpr),
    BinExpr(BinExpr),
    ParenExpr(ParenExpr),
    CallExpr(CallExpr),
    IfExpr(IfExpr),
    LoopExpr(LoopExpr),
    WhileExpr(WhileExpr),
    ReturnExpr(ReturnExpr),
    BreakExpr(BreakExpr),
    BlockExpr(BlockExpr),
}
impl From<Literal> for Expr {
    fn from(n: Literal) -> Expr {
        Expr { syntax: n.syntax }
    }
}
impl From<PrefixExpr> for Expr {
    fn from(n: PrefixExpr) -> Expr {
        Expr { syntax: n.syntax }
    }
}
impl From<PathExpr> for Expr {
    fn from(n: PathExpr) -> Expr {
        Expr { syntax: n.syntax }
    }
}
impl From<BinExpr> for Expr {
    fn from(n: BinExpr) -> Expr {
        Expr { syntax: n.syntax }
    }
}
impl From<ParenExpr> for Expr {
    fn from(n: ParenExpr) -> Expr {
        Expr { syntax: n.syntax }
    }
}
impl From<CallExpr> for Expr {
    fn from(n: CallExpr) -> Expr {
        Expr { syntax: n.syntax }
    }
}
impl From<IfExpr> for Expr {
    fn from(n: IfExpr) -> Expr {
        Expr { syntax: n.syntax }
    }
}
impl From<LoopExpr> for Expr {
    fn from(n: LoopExpr) -> Expr {
        Expr { syntax: n.syntax }
    }
}
impl From<WhileExpr> for Expr {
    fn from(n: WhileExpr) -> Expr {
        Expr { syntax: n.syntax }
    }
}
impl From<ReturnExpr> for Expr {
    fn from(n: ReturnExpr) -> Expr {
        Expr { syntax: n.syntax }
    }
}
impl From<BreakExpr> for Expr {
    fn from(n: BreakExpr) -> Expr {
        Expr { syntax: n.syntax }
    }
}
impl From<BlockExpr> for Expr {
    fn from(n: BlockExpr) -> Expr {
        Expr { syntax: n.syntax }
    }
}

impl Expr {
    pub fn kind(&self) -> ExprKind {
        match self.syntax.kind() {
            LITERAL => ExprKind::Literal(Literal::cast(self.syntax.clone()).unwrap()),
            PREFIX_EXPR => ExprKind::PrefixExpr(PrefixExpr::cast(self.syntax.clone()).unwrap()),
            PATH_EXPR => ExprKind::PathExpr(PathExpr::cast(self.syntax.clone()).unwrap()),
            BIN_EXPR => ExprKind::BinExpr(BinExpr::cast(self.syntax.clone()).unwrap()),
            PAREN_EXPR => ExprKind::ParenExpr(ParenExpr::cast(self.syntax.clone()).unwrap()),
            CALL_EXPR => ExprKind::CallExpr(CallExpr::cast(self.syntax.clone()).unwrap()),
            IF_EXPR => ExprKind::IfExpr(IfExpr::cast(self.syntax.clone()).unwrap()),
            LOOP_EXPR => ExprKind::LoopExpr(LoopExpr::cast(self.syntax.clone()).unwrap()),
            WHILE_EXPR => ExprKind::WhileExpr(WhileExpr::cast(self.syntax.clone()).unwrap()),
            RETURN_EXPR => ExprKind::ReturnExpr(ReturnExpr::cast(self.syntax.clone()).unwrap()),
            BREAK_EXPR => ExprKind::BreakExpr(BreakExpr::cast(self.syntax.clone()).unwrap()),
            BLOCK_EXPR => ExprKind::BlockExpr(BlockExpr::cast(self.syntax.clone()).unwrap()),
            _ => unreachable!(),
        }
    }
}

impl Expr {}

// ExprStmt

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprStmt {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for ExprStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            EXPR_STMT => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(ExprStmt { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ExprStmt {
    pub fn expr(&self) -> Option<Expr> {
        super::child_opt(self)
    }
}

// FunctionDef

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionDef {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for FunctionDef {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            FUNCTION_DEF => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(FunctionDef { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ast::NameOwner for FunctionDef {}
impl ast::VisibilityOwner for FunctionDef {}
impl ast::DocCommentsOwner for FunctionDef {}
impl FunctionDef {
    pub fn param_list(&self) -> Option<ParamList> {
        super::child_opt(self)
    }

    pub fn body(&self) -> Option<BlockExpr> {
        super::child_opt(self)
    }

    pub fn ret_type(&self) -> Option<RetType> {
        super::child_opt(self)
    }
}

// IfExpr

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IfExpr {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for IfExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            IF_EXPR => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(IfExpr { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl IfExpr {
    pub fn condition(&self) -> Option<Condition> {
        super::child_opt(self)
    }
}

// LetStmt

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LetStmt {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for LetStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            LET_STMT => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(LetStmt { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ast::TypeAscriptionOwner for LetStmt {}
impl LetStmt {
    pub fn pat(&self) -> Option<Pat> {
        super::child_opt(self)
    }

    pub fn initializer(&self) -> Option<Expr> {
        super::child_opt(self)
    }
}

// Literal

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Literal {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for Literal {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            LITERAL => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Literal { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Literal {}

// LoopExpr

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LoopExpr {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for LoopExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            LOOP_EXPR => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(LoopExpr { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ast::LoopBodyOwner for LoopExpr {}
impl LoopExpr {}

// ModuleItem

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleItem {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for ModuleItem {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            FUNCTION_DEF | STRUCT_DEF => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(ModuleItem { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleItemKind {
    FunctionDef(FunctionDef),
    StructDef(StructDef),
}
impl From<FunctionDef> for ModuleItem {
    fn from(n: FunctionDef) -> ModuleItem {
        ModuleItem { syntax: n.syntax }
    }
}
impl From<StructDef> for ModuleItem {
    fn from(n: StructDef) -> ModuleItem {
        ModuleItem { syntax: n.syntax }
    }
}

impl ModuleItem {
    pub fn kind(&self) -> ModuleItemKind {
        match self.syntax.kind() {
            FUNCTION_DEF => {
                ModuleItemKind::FunctionDef(FunctionDef::cast(self.syntax.clone()).unwrap())
            }
            STRUCT_DEF => ModuleItemKind::StructDef(StructDef::cast(self.syntax.clone()).unwrap()),
            _ => unreachable!(),
        }
    }
}

impl ModuleItem {}

// Name

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for Name {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            NAME => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Name { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Name {}

// NameRef

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NameRef {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for NameRef {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            NAME_REF => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(NameRef { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl NameRef {}

// NeverType

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NeverType {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for NeverType {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            NEVER_TYPE => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(NeverType { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl NeverType {}

// Param

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Param {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for Param {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            PARAM => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Param { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ast::TypeAscriptionOwner for Param {}
impl Param {
    pub fn pat(&self) -> Option<Pat> {
        super::child_opt(self)
    }
}

// ParamList

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParamList {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for ParamList {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            PARAM_LIST => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(ParamList { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ParamList {
    pub fn params(&self) -> impl Iterator<Item = Param> {
        super::children(self)
    }
}

// ParenExpr

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParenExpr {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for ParenExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            PAREN_EXPR => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(ParenExpr { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ParenExpr {
    pub fn expr(&self) -> Option<Expr> {
        super::child_opt(self)
    }
}

// Pat

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pat {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for Pat {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            BIND_PAT | PLACEHOLDER_PAT => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Pat { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatKind {
    BindPat(BindPat),
    PlaceholderPat(PlaceholderPat),
}
impl From<BindPat> for Pat {
    fn from(n: BindPat) -> Pat {
        Pat { syntax: n.syntax }
    }
}
impl From<PlaceholderPat> for Pat {
    fn from(n: PlaceholderPat) -> Pat {
        Pat { syntax: n.syntax }
    }
}

impl Pat {
    pub fn kind(&self) -> PatKind {
        match self.syntax.kind() {
            BIND_PAT => PatKind::BindPat(BindPat::cast(self.syntax.clone()).unwrap()),
            PLACEHOLDER_PAT => {
                PatKind::PlaceholderPat(PlaceholderPat::cast(self.syntax.clone()).unwrap())
            }
            _ => unreachable!(),
        }
    }
}

impl Pat {}

// Path

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Path {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for Path {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            PATH => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Path { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Path {
    pub fn segment(&self) -> Option<PathSegment> {
        super::child_opt(self)
    }

    pub fn qualifier(&self) -> Option<Path> {
        super::child_opt(self)
    }
}

// PathExpr

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathExpr {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for PathExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            PATH_EXPR => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(PathExpr { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl PathExpr {
    pub fn path(&self) -> Option<Path> {
        super::child_opt(self)
    }
}

// PathSegment

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathSegment {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for PathSegment {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            PATH_SEGMENT => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(PathSegment { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl PathSegment {
    pub fn name_ref(&self) -> Option<NameRef> {
        super::child_opt(self)
    }
}

// PathType

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathType {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for PathType {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            PATH_TYPE => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(PathType { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl PathType {
    pub fn path(&self) -> Option<Path> {
        super::child_opt(self)
    }
}

// PlaceholderPat

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlaceholderPat {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for PlaceholderPat {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            PLACEHOLDER_PAT => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(PlaceholderPat { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl PlaceholderPat {}

// PrefixExpr

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrefixExpr {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for PrefixExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            PREFIX_EXPR => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(PrefixExpr { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl PrefixExpr {
    pub fn expr(&self) -> Option<Expr> {
        super::child_opt(self)
    }
}

// RecordFieldDef

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordFieldDef {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for RecordFieldDef {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            RECORD_FIELD_DEF => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(RecordFieldDef { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ast::NameOwner for RecordFieldDef {}
impl ast::VisibilityOwner for RecordFieldDef {}
impl ast::DocCommentsOwner for RecordFieldDef {}
impl ast::TypeAscriptionOwner for RecordFieldDef {}
impl RecordFieldDef {}

// RecordFieldDefList

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordFieldDefList {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for RecordFieldDefList {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            RECORD_FIELD_DEF_LIST => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(RecordFieldDefList { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl RecordFieldDefList {
    pub fn fields(&self) -> impl Iterator<Item = RecordFieldDef> {
        super::children(self)
    }
}

// RetType

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RetType {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for RetType {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            RET_TYPE => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(RetType { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl RetType {
    pub fn type_ref(&self) -> Option<TypeRef> {
        super::child_opt(self)
    }
}

// ReturnExpr

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReturnExpr {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for ReturnExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            RETURN_EXPR => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(ReturnExpr { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ReturnExpr {
    pub fn expr(&self) -> Option<Expr> {
        super::child_opt(self)
    }
}

// SourceFile

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceFile {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for SourceFile {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            SOURCE_FILE => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(SourceFile { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ast::ModuleItemOwner for SourceFile {}
impl ast::FunctionDefOwner for SourceFile {}
impl SourceFile {}

// Stmt

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Stmt {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for Stmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            LET_STMT | EXPR_STMT => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Stmt { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StmtKind {
    LetStmt(LetStmt),
    ExprStmt(ExprStmt),
}
impl From<LetStmt> for Stmt {
    fn from(n: LetStmt) -> Stmt {
        Stmt { syntax: n.syntax }
    }
}
impl From<ExprStmt> for Stmt {
    fn from(n: ExprStmt) -> Stmt {
        Stmt { syntax: n.syntax }
    }
}

impl Stmt {
    pub fn kind(&self) -> StmtKind {
        match self.syntax.kind() {
            LET_STMT => StmtKind::LetStmt(LetStmt::cast(self.syntax.clone()).unwrap()),
            EXPR_STMT => StmtKind::ExprStmt(ExprStmt::cast(self.syntax.clone()).unwrap()),
            _ => unreachable!(),
        }
    }
}

impl Stmt {}

// StructDef

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StructDef {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for StructDef {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            STRUCT_DEF => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(StructDef { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ast::NameOwner for StructDef {}
impl ast::VisibilityOwner for StructDef {}
impl ast::DocCommentsOwner for StructDef {}
impl StructDef {}

// TupleFieldDef

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TupleFieldDef {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for TupleFieldDef {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            TUPLE_FIELD_DEF => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(TupleFieldDef { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ast::VisibilityOwner for TupleFieldDef {}
impl TupleFieldDef {
    pub fn type_ref(&self) -> Option<TypeRef> {
        super::child_opt(self)
    }
}

// TupleFieldDefList

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TupleFieldDefList {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for TupleFieldDefList {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            TUPLE_FIELD_DEF_LIST => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(TupleFieldDefList { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl TupleFieldDefList {
    pub fn fields(&self) -> impl Iterator<Item = TupleFieldDef> {
        super::children(self)
    }
}

// TypeRef

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeRef {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for TypeRef {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            PATH_TYPE | NEVER_TYPE => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(TypeRef { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeRefKind {
    PathType(PathType),
    NeverType(NeverType),
}
impl From<PathType> for TypeRef {
    fn from(n: PathType) -> TypeRef {
        TypeRef { syntax: n.syntax }
    }
}
impl From<NeverType> for TypeRef {
    fn from(n: NeverType) -> TypeRef {
        TypeRef { syntax: n.syntax }
    }
}

impl TypeRef {
    pub fn kind(&self) -> TypeRefKind {
        match self.syntax.kind() {
            PATH_TYPE => TypeRefKind::PathType(PathType::cast(self.syntax.clone()).unwrap()),
            NEVER_TYPE => TypeRefKind::NeverType(NeverType::cast(self.syntax.clone()).unwrap()),
            _ => unreachable!(),
        }
    }
}

impl TypeRef {}

// Visibility

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Visibility {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for Visibility {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            VISIBILITY => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Visibility { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Visibility {}

// WhileExpr

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WhileExpr {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for WhileExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            WHILE_EXPR => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(WhileExpr { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ast::LoopBodyOwner for WhileExpr {}
impl WhileExpr {
    pub fn condition(&self) -> Option<Condition> {
        super::child_opt(self)
    }
}
