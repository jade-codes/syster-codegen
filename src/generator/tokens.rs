//! Token enum generation

use crate::kebnf::Grammar;
use super::utils::{keyword_to_variant, punctuation_to_variant};

/// Generate the TokenKind enum from grammar keywords and punctuation
pub fn generate(grammar: &Grammar) -> String {
    let mut code = String::new();
    
    // Header
    code.push_str(r#"//! Generated token types for SysML v2
//!
//! This file was generated from the official KEBNF grammar.
//! Do not edit manually.

/// Token types for the SysML v2 lexer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
"#);

    // Keywords section
    code.push_str("    // Keywords\n");
    let mut keywords: Vec<_> = grammar.keywords.iter().collect();
    keywords.sort();
    
    for kw in &keywords {
        let variant = keyword_to_variant(kw);
        code.push_str(&format!("    /// `{}`\n", kw));
        code.push_str(&format!("    {},\n", variant));
    }

    // Punctuation section
    code.push_str("\n    // Punctuation\n");
    let mut puncts: Vec<_> = grammar.punctuation.iter().collect();
    puncts.sort();
    
    for punct in &puncts {
        let variant = punctuation_to_variant(punct);
        let escaped = punct.replace('\\', "\\\\").replace('"', "\\\"");
        code.push_str(&format!("    /// `\"{}\"`\n", escaped));
        code.push_str(&format!("    {},\n", variant));
    }

    // Lexer terminals (hardcoded)
    code.push_str(r#"
    // Lexer terminals
    /// Identifier (NAME = BASIC_NAME)
    Name,
    /// Unrestricted name (UNRESTRICTED_NAME = 'quoted')
    UnrestrictedName,
    /// Integer literal
    Integer,
    /// Real number literal
    Real,
    /// String literal
    String,
    /// Regular expression literal
    Regex,
    /// Block comment (/* ... */)
    BlockComment,
    /// Line comment (// ...)
    LineComment,
    /// Whitespace
    Whitespace,
    
    // Special tokens
    /// End of file
    Eof,
    /// Unknown/error token
    Error,
}

impl TokenKind {
    /// Check if this token is a keyword
    pub fn is_keyword(&self) -> bool {
        matches!(self,
"#);

    // Generate keyword match arms
    let keyword_variants: Vec<_> = keywords.iter()
        .map(|kw| format!("            Self::{}", keyword_to_variant(kw)))
        .collect();
    code.push_str(&keyword_variants.join(" |\n"));
    code.push_str("\n        )\n    }\n\n");

    // is_punctuation method
    code.push_str("    /// Check if this token is punctuation\n");
    code.push_str("    pub fn is_punctuation(&self) -> bool {\n");
    code.push_str("        matches!(self,\n");
    
    let punct_variants: Vec<_> = puncts.iter()
        .map(|p| format!("            Self::{}", punctuation_to_variant(p)))
        .collect();
    code.push_str(&punct_variants.join(" |\n"));
    code.push_str("\n        )\n    }\n\n");

    // is_name_compatible method — tokens that can appear in NAME positions
    code.push_str("    /// Check if this token can appear in a NAME position.\n");
    code.push_str("    /// NAME = BASIC_NAME | UNRESTRICTED_NAME, and BASIC_NAME matches any\n");
    code.push_str("    /// identifier-like string [a-zA-Z_][a-zA-Z0-9_]*, which includes all keywords.\n");
    code.push_str("    pub fn is_name_compatible(&self) -> bool {\n");
    code.push_str("        matches!(self, Self::Name | Self::UnrestrictedName) || self.is_keyword()\n");
    code.push_str("    }\n\n");

    // is_name_token method — strict name tokens only (no keywords)
    code.push_str("    /// Check if this token is a strict name token (Name or UnrestrictedName).\n");
    code.push_str("    /// Unlike `is_name_compatible`, this does NOT include keywords.\n");
    code.push_str("    /// Used by `parse_cross_ref` so that cross-references do not greedily\n");
    code.push_str("    /// consume structural keywords that belong to the enclosing grammar rule.\n");
    code.push_str("    pub fn is_name_token(&self) -> bool {\n");
    code.push_str("        matches!(self, Self::Name | Self::UnrestrictedName)\n");
    code.push_str("    }\n");

    code.push_str("}\n\n");

    // Generate keyword lookup table
    code.push_str("/// Look up a keyword from its string representation\n");
    code.push_str("pub fn lookup_keyword(s: &str) -> Option<TokenKind> {\n");
    code.push_str("    match s {\n");
    
    for kw in &keywords {
        let variant = keyword_to_variant(kw);
        code.push_str(&format!("        \"{}\" => Some(TokenKind::{}),\n", kw, variant));
    }
    
    code.push_str("        _ => None,\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");

    // Generate punctuation lookup table
    code.push_str("/// Look up punctuation from its string representation\n");
    code.push_str("pub fn lookup_punctuation(s: &str) -> Option<TokenKind> {\n");
    code.push_str("    match s {\n");
    
    for punct in &puncts {
        let variant = punctuation_to_variant(punct);
        let escaped = punct.replace('\\', "\\\\").replace('"', "\\\"");
        code.push_str(&format!("        \"{}\" => Some(TokenKind::{}),\n", escaped, variant));
    }
    
    code.push_str("        _ => None,\n");
    code.push_str("    }\n");
    code.push_str("}\n");

    code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_tokens() {
        let sysml = std::fs::read_to_string("data/SysML-textual-bnf.kebnf").unwrap();
        let grammar = Grammar::parse(&sysml);
        
        let code = generate(&grammar);
        
        // Should have the enum
        assert!(code.contains("pub enum TokenKind"));
        
        // Should have keywords
        assert!(code.contains("Package"));
        assert!(code.contains("Part"));
        assert!(code.contains("Import"));
        
        // Should have punctuation
        assert!(code.contains("LBrace"));
        assert!(code.contains("Semi"));
        assert!(code.contains("Colon"));
        
        // Should have lexer terminals
        assert!(code.contains("Ident"));
        assert!(code.contains("Eof"));
        
        // Should have lookup functions
        assert!(code.contains("lookup_keyword"));
        assert!(code.contains("lookup_punctuation"));
    }
}
