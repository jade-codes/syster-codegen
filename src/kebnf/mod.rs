//! KEBNF grammar parser
//!
//! Parses the official SysML v2 KEBNF grammar format.
//!
//! # Modules
//! - `tokens`: Meta-syntax tokens used in KEBNF notation
//! - `types`: Core grammar types (Grammar, Rule, RuleBody)
//! - `terminals`: Terminal/keyword extraction
//! - `rules`: Rule header parsing
//! - `deps`: Dependency analysis
//! - `parser`: Rule body expression parser

pub mod tokens;
pub mod types;
pub mod terminals;
pub mod rules;
pub mod deps;
pub mod parser;

// Re-export main types for convenience
pub use types::{Grammar, Rule, RuleBody, AssignOp};
pub use rules::{RuleHeader, extract_rule_headers};
pub use deps::{get_rule_refs, find_roots, find_leaves};
pub use parser::parse_body;
