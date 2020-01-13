//! The Mun ABI
//!
//! The Mun ABI defines the binary format used to communicate between the Mun Compiler and Mun
//! Runtime.
#![warn(missing_docs)]

// Bindings are automatically generated from C on `cargo build`
mod autogen;

mod autogen_impl;
mod macros;
mod reflection;

pub use autogen::*;
pub use reflection::Reflection;

/// The Mun ABI prelude
///
/// The *prelude* contains imports that are used almost every time.
pub mod prelude {
    pub use crate::autogen::*;
    pub use crate::reflection::Reflection;
    pub use crate::{Privacy, TypeGroup};
}

/// Represents the privacy level of modules, functions, or variables.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Privacy {
    /// Publicly (and privately) accessible
    Public = 0,
    /// Privately accessible
    Private = 1,
}

/// Represents a group of types that illicit the same characteristics.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum TypeGroup {
    /// Fundamental types (i.e. `()`, `bool`, `float`, `int`, etc.)
    FundamentalTypes = 0,
    /// Struct types (i.e. record, tuple, or unit structs)
    StructTypes = 1,
}

impl TypeGroup {
    /// Returns whether this is a fundamental type.
    pub fn is_fundamental(self) -> bool {
        match self {
            TypeGroup::FundamentalTypes => true,
            _ => false,
        }
    }

    /// Returns whether this is a struct type.
    pub fn is_struct(self) -> bool {
        match self {
            TypeGroup::StructTypes => true,
            _ => false,
        }
    }
}
