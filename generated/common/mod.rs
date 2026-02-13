//! Common types shared by KerML and SysML parsers

pub mod span;
pub mod syntax_kind;

pub use span::{Span, ParseError, Result};
pub use syntax_kind::SyntaxKind;
