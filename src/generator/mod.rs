//! Parser code generator
//!
//! Generates Rust parser code from a parsed KEBNF grammar.
//!
//! # Modules
//! - `utils`: Name conversion utilities
//! - `tokens`: Token enum generation
//! - `lexer`: Lexer generation
//! - `parser`: Parser function generation (builds AST)
//! - `ast`: AST struct generation
//! - `merged`: Merged type information from KerML + SysML

pub mod ast;
pub mod lexer;
pub mod merged;
pub mod parser;
pub mod syntax_kind;
pub mod test_synth;
pub mod tokens;
pub mod utils;

use crate::kebnf::{Grammar, find_roots, find_leaves};
use merged::MergedTypes;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// Configuration for code generation
pub struct GenerateConfig<'a> {
    pub kerml_grammar: &'a Grammar,
    pub sysml_grammar: &'a Grammar,
    pub output_dir: &'a Path,
}

/// Generate parser code from KerML and SysML grammars
pub fn generate_all(config: &GenerateConfig) -> anyhow::Result<()> {
    let output_dir = config.output_dir;
    
    // Create directory structure
    fs::create_dir_all(output_dir)?;
    fs::create_dir_all(output_dir.join("common"))?;
    fs::create_dir_all(output_dir.join("kerml"))?;
    fs::create_dir_all(output_dir.join("sysml"))?;

    // Build merged info (for tokens/keywords/syntax_kind only)
    let merged = MergedTypes::from_grammars(config.kerml_grammar, config.sysml_grammar);
    
    // Build merged grammar for SysML (all rules, SysML overrides KerML)
    let sysml_merged_grammar = build_merged_grammar(config.kerml_grammar, config.sysml_grammar);
    
    println!("  KerML rules: {}", config.kerml_grammar.rules.len());
    println!("  SysML rules: {}", config.sysml_grammar.rules.len());
    println!("  KerML-only: {}", merged.kerml_only_rules.len());
    println!("  SysML-only: {}", merged.sysml_only_rules.len());
    println!("  Shared: {}", merged.shared_rules.len());

    // === Common module (span + syntax_kind only, no AST) ===
    
    let syntax_kind_code = syntax_kind::generate_from_merged(&merged);
    fs::write(output_dir.join("common/syntax_kind.rs"), syntax_kind_code)?;
    
    let span_code = generate_span_module();
    fs::write(output_dir.join("common/span.rs"), span_code)?;
    
    let common_mod = generate_common_mod();
    fs::write(output_dir.join("common/mod.rs"), common_mod)?;

    // === KerML module (own AST + parser) ===
    
    let kerml_ast_nodes = ast::extract_ast_nodes(config.kerml_grammar);
    let kerml_ast_code = ast::generate_from_nodes(&kerml_ast_nodes, "super::super::common::span::Span");
    fs::write(output_dir.join("kerml/ast.rs"), kerml_ast_code)?;
    println!("  KerML AST nodes: {}", kerml_ast_nodes.len());
    
    let kerml_tokens = tokens::generate(config.kerml_grammar);
    fs::write(output_dir.join("kerml/tokens.rs"), kerml_tokens)?;
    
    let kerml_lexer = lexer::generate_for_language(config.kerml_grammar);
    fs::write(output_dir.join("kerml/lexer.rs"), kerml_lexer)?;
    
    let kerml_roots = find_parser_roots(config.kerml_grammar);
    let kerml_parser = parser::generate_for_language(config.kerml_grammar, &kerml_roots);
    fs::write(output_dir.join("kerml/parser.rs"), kerml_parser)?;
    
    let kerml_mod = generate_language_mod("kerml");
    fs::write(output_dir.join("kerml/mod.rs"), kerml_mod)?;

    // === SysML module (own AST + parser, using merged grammar) ===
    
    let sysml_ast_nodes = ast::extract_ast_nodes(&sysml_merged_grammar);
    let sysml_ast_code = ast::generate_from_nodes(&sysml_ast_nodes, "super::super::common::span::Span");
    fs::write(output_dir.join("sysml/ast.rs"), sysml_ast_code)?;
    println!("  SysML AST nodes: {}", sysml_ast_nodes.len());
    
    let sysml_tokens = tokens::generate(&sysml_merged_grammar);
    fs::write(output_dir.join("sysml/tokens.rs"), sysml_tokens)?;
    
    let sysml_lexer = lexer::generate_for_language(&sysml_merged_grammar);
    fs::write(output_dir.join("sysml/lexer.rs"), sysml_lexer)?;
    
    let sysml_roots = find_parser_roots(&sysml_merged_grammar);
    let sysml_parser = parser::generate_for_language(&sysml_merged_grammar, &sysml_roots);
    fs::write(output_dir.join("sysml/parser.rs"), sysml_parser)?;
    
    let sysml_mod = generate_language_mod("sysml");
    fs::write(output_dir.join("sysml/mod.rs"), sysml_mod)?;

    // === Root mod.rs ===
    let root_mod = generate_root_mod();
    fs::write(output_dir.join("mod.rs"), root_mod)?;

    Ok(())
}

/// Legacy single-grammar generate (for backwards compatibility)
pub fn generate(grammar: &Grammar, output_dir: &Path) -> anyhow::Result<()> {
    fs::create_dir_all(output_dir)?;

    let roots = find_roots(grammar);
    let leaves = find_leaves(grammar);
    
    let parser_roots: Vec<_> = roots.iter()
        .filter(|r| !r.name.chars().all(|c| c.is_ascii_uppercase() || c == '_'))
        .copied()
        .collect();

    println!("  Entry point rules: {}", parser_roots.len());
    for r in parser_roots.iter().take(10) {
        println!("    - {}", r.name);
    }

    println!("  Leaf rules (terminals): {}", leaves.len());

    let tokens_code = tokens::generate(grammar);
    fs::write(output_dir.join("tokens.rs"), tokens_code)?;

    let lexer_code = lexer::generate(grammar);
    fs::write(output_dir.join("lexer.rs"), lexer_code)?;

    let ast_code = ast::generate(grammar);
    fs::write(output_dir.join("ast.rs"), ast_code)?;

    let root_names: HashSet<_> = parser_roots.iter().map(|r| r.name.as_str()).collect();
    let parser_code = parser::generate(grammar, &root_names);
    fs::write(output_dir.join("parser.rs"), parser_code)?;
    
    let ast_nodes = ast::extract_ast_nodes(grammar);
    let with_fields = ast_nodes.iter().filter(|n| !n.fields.is_empty()).count();
    let enums = ast_nodes.iter().filter(|n| n.is_enum).count();
    println!("  AST nodes: {} ({} with fields, {} enums)", ast_nodes.len(), with_fields, enums);

    let syntax_kind_code = syntax_kind::generate(grammar);
    fs::write(output_dir.join("syntax_kind.rs"), syntax_kind_code)?;

    let mod_code = generate_legacy_mod();
    fs::write(output_dir.join("mod.rs"), mod_code)?;

    Ok(())
}

fn find_parser_roots(grammar: &Grammar) -> HashSet<&str> {
    let roots = find_roots(grammar);
    roots.iter()
        .filter(|r| !r.name.chars().all(|c| c.is_ascii_uppercase() || c == '_'))
        .map(|r| r.name.as_str())
        .collect()
}

fn build_merged_grammar(kerml: &Grammar, sysml: &Grammar) -> Grammar {
    use std::collections::HashMap;
    use crate::kebnf::Rule;
    
    let mut rule_map: HashMap<String, Rule> = HashMap::new();
    
    // Add KerML rules first
    for rule in &kerml.rules {
        rule_map.insert(rule.name.clone(), Rule {
            name: rule.name.clone(),
            return_type: rule.return_type.clone(),
            body: rule.body.clone(),
        });
    }
    
    // SysML overrides
    for rule in &sysml.rules {
        rule_map.insert(rule.name.clone(), Rule {
            name: rule.name.clone(),
            return_type: rule.return_type.clone(),
            body: rule.body.clone(),
        });
    }
    
    Grammar {
        rules: rule_map.into_values().collect(),
        keywords: kerml.keywords.union(&sysml.keywords).cloned().collect(),
        punctuation: kerml.punctuation.union(&sysml.punctuation).cloned().collect(),
    }
}

fn generate_span_module() -> String {
    r#"//! Common span and error types

/// Source span (byte offsets)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Span { start, end }
    }
    
    pub fn merge(self, other: Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}

/// Parse error
#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub span: Span,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {}..{}", self.message, self.span.start, self.span.end)
    }
}

impl std::error::Error for ParseError {}

pub type Result<T> = std::result::Result<T, ParseError>;
"#.to_string()
}

fn generate_common_mod() -> String {
    r#"//! Common types shared by KerML and SysML parsers

pub mod span;
pub mod syntax_kind;

pub use span::{Span, ParseError, Result};
pub use syntax_kind::SyntaxKind;
"#.to_string()
}

fn generate_language_mod(lang: &str) -> String {
    format!(r#"//! {lang} parser
//!
//! Generated from the official KEBNF grammar.

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod tokens;

pub use ast::*;
pub use lexer::Lexer;
pub use parser::Parser;
pub use tokens::TokenKind;
"#, lang = lang.to_uppercase())
}

fn generate_root_mod() -> String {
    r#"//! Generated SysML v2 parser
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
"#.to_string()
}

fn generate_legacy_mod() -> String {
    r#"//! Generated SysML v2 parser
//!
//! This code was generated from the official KEBNF grammar.

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod syntax_kind;
pub mod tokens;

pub use ast::AstNode;
pub use lexer::Lexer;
pub use parser::Parser;
pub use syntax_kind::SyntaxKind;
pub use tokens::TokenKind;
"#.to_string()
}
