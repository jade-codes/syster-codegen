//! SyntaxKind enum generator
//!
//! Generates a flat enum suitable for use with rowan-style CST parsers.

use crate::kebnf::Grammar;
use super::ast::extract_ast_nodes;
use super::merged::MergedTypes;


/// Generate SyntaxKind from merged types (for common/ module)
pub fn generate_from_merged(merged: &MergedTypes) -> String {
    let rule_names: Vec<_> = merged.all_rule_names.iter().map(|n| n.as_str()).collect();
    generate_syntax_kind_code(
        &merged.all_keywords,
        &merged.all_punctuation,
        &rule_names,
    )
}

/// Generate the SyntaxKind enum from grammar
pub fn generate(grammar: &Grammar) -> String {
    let nodes = extract_ast_nodes(grammar);
    let node_names: Vec<_> = nodes.iter().map(|n| n.rule_name.as_str()).collect();
    generate_syntax_kind_code(&grammar.keywords, &grammar.punctuation, &node_names)
}

fn generate_syntax_kind_code(
    keywords: &std::collections::HashSet<String>,
    punctuation: &std::collections::HashSet<String>,
    node_names: &[&str],
) -> String {
    let mut code = String::new();
    
    code.push_str(r#"//! Generated SyntaxKind enum for SysML v2
//!
//! This file was generated from the official KEBNF grammar.
//! Do not edit manually.

/// All syntax kinds (tokens and nodes) in SysML v2
///
/// Tokens are leaf nodes (identifiers, keywords, punctuation).
/// Nodes are composite (packages, definitions, usages).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
#[allow(non_camel_case_types)]
pub enum SyntaxKind {
    // =========================================================================
    // TRIVIA (whitespace and comments)
    // =========================================================================
    WHITESPACE = 0,
    LINE_COMMENT,
    BLOCK_COMMENT,

    // =========================================================================
    // LITERALS
    // =========================================================================
    IDENT,
    UNRESTRICTED_NAME,
    INTEGER,
    DECIMAL,
    STRING,
    REGEX,

    // =========================================================================
    // PUNCTUATION
    // =========================================================================
"#);

    // Generate punctuation kinds
    let mut puncts: Vec<_> = punctuation.iter().collect();
    puncts.sort_by_key(|p| (std::cmp::Reverse(p.len()), p.as_str()));
    
    for punct in &puncts {
        let name = punct_to_syntax_kind(punct);
        code.push_str(&format!("    {},\n", name));
    }

    code.push_str("\n    // =========================================================================\n");
    code.push_str("    // KEYWORDS\n");
    code.push_str("    // =========================================================================\n");

    // Generate keyword kinds
    let mut sorted_keywords: Vec<_> = keywords.iter().collect();
    sorted_keywords.sort();
    
    for kw in &sorted_keywords {
        let name = keyword_to_syntax_kind(kw);
        code.push_str(&format!("    {},\n", name));
    }

    code.push_str("\n    // =========================================================================\n");
    code.push_str("    // COMPOSITE NODES (non-terminals)\n");
    code.push_str("    // =========================================================================\n");
    code.push_str("    SOURCE_FILE,\n\n");

    // Generate node kinds
    let mut sorted_nodes: Vec<_> = node_names.to_vec();
    sorted_nodes.sort();
    
    for name in &sorted_nodes {
        let syntax_name = name_to_syntax_kind(name);
        code.push_str(&format!("    {},\n", syntax_name));
    }

    code.push_str("\n    // =========================================================================\n");
    code.push_str("    // SPECIAL\n");
    code.push_str("    // =========================================================================\n");
    code.push_str("    ERROR,\n");
    code.push_str("    TOMBSTONE,\n\n");
    code.push_str("    #[doc(hidden)]\n");
    code.push_str("    __LAST,\n");
    code.push_str("}\n\n");

    // Generate helper methods
    code.push_str(r#"impl SyntaxKind {
    /// Check if this is a trivia token (whitespace or comment)
    pub fn is_trivia(self) -> bool {
        matches!(self, Self::WHITESPACE | Self::LINE_COMMENT | Self::BLOCK_COMMENT)
    }

    /// Check if this is a keyword
    pub fn is_keyword(self) -> bool {
"#);

    // Generate keyword range check
    if let (Some(first), Some(last)) = (sorted_keywords.first(), sorted_keywords.last()) {
        code.push_str(&format!(
            "        (self as u16) >= (Self::{} as u16) && (self as u16) <= (Self::{} as u16)\n",
            keyword_to_syntax_kind(first),
            keyword_to_syntax_kind(last)
        ));
    } else {
        code.push_str("        false\n");
    }
    code.push_str("    }\n\n");

    code.push_str(r#"    /// Check if this is a punctuation token
    pub fn is_punct(self) -> bool {
"#);
    if let (Some(first), Some(last)) = (puncts.first(), puncts.last()) {
        code.push_str(&format!(
            "        (self as u16) >= (Self::{} as u16) && (self as u16) <= (Self::{} as u16)\n",
            punct_to_syntax_kind(first),
            punct_to_syntax_kind(last)
        ));
    } else {
        code.push_str("        false\n");
    }
    code.push_str("    }\n");
    code.push_str("}\n\n");

    // Generate lookup functions
    code.push_str("/// Look up a keyword's SyntaxKind\n");
    code.push_str("pub fn lookup_keyword(s: &str) -> Option<SyntaxKind> {\n");
    code.push_str("    match s {\n");
    for kw in keywords {
        code.push_str(&format!(
            "        {:?} => Some(SyntaxKind::{}),\n",
            kw,
            keyword_to_syntax_kind(kw)
        ));
    }
    code.push_str("        _ => None,\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");

    code.push_str("/// Look up a punctuation's SyntaxKind\n");
    code.push_str("pub fn lookup_punct(s: &str) -> Option<SyntaxKind> {\n");
    code.push_str("    match s {\n");
    for punct in &puncts {
        code.push_str(&format!(
            "        {:?} => Some(SyntaxKind::{}),\n",
            punct,
            punct_to_syntax_kind(punct)
        ));
    }
    code.push_str("        _ => None,\n");
    code.push_str("    }\n");
    code.push_str("}\n");

    code
}

/// Convert a keyword to SCREAMING_SNAKE_CASE with _KW suffix
fn keyword_to_syntax_kind(kw: &str) -> String {
    let upper = kw.to_uppercase();
    format!("{}_KW", upper.replace('-', "_"))
}

/// Convert punctuation to a SyntaxKind name
fn punct_to_syntax_kind(punct: &str) -> String {
    match punct {
        "{" => "L_BRACE",
        "}" => "R_BRACE",
        "(" => "L_PAREN",
        ")" => "R_PAREN",
        "[" => "L_BRACKET",
        "]" => "R_BRACKET",
        "<" => "LT",
        ">" => "GT",
        "<<" => "LT_LT",
        ">>" => "GT_GT",
        "<=" => "LT_EQ",
        ">=" => "GT_EQ",
        "=" => "EQ",
        "==" => "EQ_EQ",
        "===" => "EQ_EQ_EQ",
        "!=" => "BANG_EQ",
        "!==" => "BANG_EQ_EQ",
        "!" => "BANG",
        ":" => "COLON",
        "::" => "COLON_COLON",
        "::>" => "COLON_COLON_GT",
        ":>" => "COLON_GT",
        ":>>" => "COLON_GT_GT",
        ":=" => "COLON_EQ",
        ";" => "SEMICOLON",
        "," => "COMMA",
        "." => "DOT",
        ".." => "DOT_DOT",
        "..." => "DOT_DOT_DOT",
        "->" => "ARROW",
        "=>" => "FAT_ARROW",
        "+" => "PLUS",
        "-" => "MINUS",
        "*" => "STAR",
        "/" => "SLASH",
        "**" => "STAR_STAR",
        "%" => "PERCENT",
        "&" => "AMP",
        "&&" => "AMP_AMP",
        "|" => "PIPE",
        "||" => "PIPE_PIPE",
        "^" => "CARET",
        "~" => "TILDE",
        "?" => "QUESTION",
        "??" => "QUESTION_QUESTION",
        "@" => "AT",
        "@@" => "AT_AT",
        "#" => "HASH",
        "$" => "DOLLAR",
        "\\" => "BACKSLASH",
        "'" => "QUOTE",
        "\"" => "DOUBLE_QUOTE",
        "`" => "BACKTICK",
        other => {
            // Generate from chars as fallback
            let mut name = String::from("PUNCT_");
            for ch in other.chars() {
                name.push_str(&format!("{:02X}", ch as u32));
            }
            return name;
        }
    }.to_string()
}

/// Convert a PascalCase rule name to SCREAMING_SNAKE_CASE
fn name_to_syntax_kind(name: &str) -> String {
    let mut result = String::new();
    for (i, ch) in name.chars().enumerate() {
        if ch.is_ascii_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(ch.to_ascii_uppercase());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_syntax_kind() {
        let sysml = std::fs::read_to_string("data/SysML-textual-bnf.kebnf").unwrap();
        let grammar = Grammar::parse(&sysml);
        
        let code = generate(&grammar);
        
        // Should have the enum
        assert!(code.contains("pub enum SyntaxKind"));
        
        // Should have trivia
        assert!(code.contains("WHITESPACE"));
        assert!(code.contains("LINE_COMMENT"));
        
        // Should have keywords
        assert!(code.contains("PACKAGE_KW"));
        assert!(code.contains("PART_KW"));
        
        // Should have punctuation
        assert!(code.contains("L_BRACE"));
        assert!(code.contains("SEMICOLON"));
        
        // Should have nodes
        assert!(code.contains("PACKAGE,"));
        assert!(code.contains("PART_DEFINITION,"));
        
        println!("Generated {} bytes", code.len());
    }
}
