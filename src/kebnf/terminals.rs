//! Terminal/keyword extraction from KEBNF grammars

use std::collections::HashSet;
use super::tokens;

/// Extract all terminals (quoted strings) from the grammar source and categorize them.
/// Returns (keywords, punctuation) where:
/// - keywords are alphabetic identifiers like 'package', 'part'
/// - punctuation are operators/symbols like ':', ':>', '{'
pub fn extract_terminals(source: &str) -> (HashSet<String>, HashSet<String>) {
    let mut keywords = HashSet::new();
    let mut punctuation = HashSet::new();
    let mut chars = source.chars().peekable();
    
    while let Some(ch) = chars.next() {
        // Skip line comments
        if ch == '/' && chars.peek() == Some(&'/') {
            while let Some(c) = chars.next() {
                if c == '\n' { break; }
            }
            continue;
        }
        
        // Found a quoted string
        if ch == tokens::QUOTE {
            let mut terminal = String::new();
            while let Some(c) = chars.next() {
                if c == tokens::QUOTE { break; }
                terminal.push(c);
            }
            
            if terminal.is_empty() || is_lexer_pattern(&terminal) {
                continue;
            }
            
            if is_keyword(&terminal) {
                keywords.insert(terminal);
            } else {
                punctuation.insert(terminal);
            }
        }
    }
    
    (keywords, punctuation)
}

/// Check if a terminal looks like a lexer pattern (not a real token)
fn is_lexer_pattern(s: &str) -> bool {
    if s.starts_with('\\') { return true; }
    // Check for single-char tokens first - these are valid even if they're regex chars
    if s.len() == 1 {
        let c = s.chars().next().unwrap();
        // Keep meaningful single-char punctuation  
        return !matches!(c, '{' | '}' | '(' | ')' | '[' | ']' | ';' | ',' | '.' | ':' | '<' | '>' | '=' | '@' | '#' | '~' | '*' | '+' | '-' | '/' | '&' | '|' | '!' | '?' | '^' | '$' | '%');
    }
    // ".?" is a valid operator, not a regex pattern
    if s == ".?" { return false; }
    // Multi-char strings with regex metacharacters are lexer patterns
    if s.contains('[') || s.contains(']') { return true; }
    if s.contains("any printable") || s.starts_with("#x") { return true; }
    if s.contains(".*") { return true; }
    false
}

/// Check if a terminal is a keyword (alphabetic identifier)
fn is_keyword(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_alphabetic())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_terminals() {
        let source = r#"
// Comment with 'ignored'
Package = 'package' Identification ';'
PartDef = 'part' 'def' ':>' Definition
"#;
        let (keywords, punctuation) = extract_terminals(source);
        
        assert!(keywords.contains("package"));
        assert!(keywords.contains("part"));
        assert!(keywords.contains("def"));
        assert!(!keywords.contains("ignored"));
        
        assert!(punctuation.contains(";"));
        assert!(punctuation.contains(":>"));
    }

    #[test]
    fn test_parse_both_grammars() {
        let kerml = std::fs::read_to_string("data/KerML-textual-bnf.kebnf").unwrap();
        let sysml = std::fs::read_to_string("data/SysML-textual-bnf.kebnf").unwrap();
        
        let (mut keywords, mut punctuation) = extract_terminals(&kerml);
        let (kw2, p2) = extract_terminals(&sysml);
        keywords.extend(kw2);
        punctuation.extend(p2);
        
        // Basic sanity checks
        assert!(keywords.contains("package"));
        assert!(keywords.contains("part"));
        assert!(keywords.contains("feature")); // KerML only
        assert!(punctuation.contains(":>"));
        assert!(punctuation.contains("::>"));
    }
}
