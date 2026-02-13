//! Generated SysML v2 parser
//!
//! This code was generated from the official KEBNF grammars.
//!
//! # Modules
//! - `common`: Shared span types and syntax kinds
//! - `kerml`: KerML AST, lexer and parser
//! - `sysml`: SysML AST, lexer and parser (superset of KerML)

pub mod common;
pub mod kerml;
pub mod sysml;

// Re-export common types
pub use common::{Span, ParseError, Result, SyntaxKind};
