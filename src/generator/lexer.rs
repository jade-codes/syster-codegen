//! Lexer code generation

use crate::kebnf::Grammar;
use super::utils::punctuation_to_variant;

/// Generate the lexer module (legacy mode - uses local parser::Span)
pub fn generate(grammar: &Grammar) -> String {
    generate_lexer_code(grammar, false)
}

/// Generate the lexer module (new mode - uses common::span)
pub fn generate_for_language(grammar: &Grammar) -> String {
    generate_lexer_code(grammar, true)
}

fn generate_lexer_code(grammar: &Grammar, use_common: bool) -> String {
    let mut code = String::new();

    code.push_str("//! Generated lexer\n");
    code.push_str("//!\n");
    code.push_str("//! This file was generated from the official KEBNF grammar.\n");
    code.push_str("//! Do not edit manually.\n\n");
    
    code.push_str("use super::tokens::{TokenKind, lookup_keyword};\n");
    if use_common {
        code.push_str("use super::parser::Token;\n");
        code.push_str("use super::super::common::span::Span;\n\n");
    } else {
        code.push_str("use super::parser::{Token, Span};\n\n");
    }

    code.push_str(r#"/// Lexer for source code
pub struct Lexer<'a> {
    source: &'a str,
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer for the given source
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source,
            bytes: source.as_bytes(),
            pos: 0,
        }
    }

    /// Tokenize the entire source into a Vec<Token>
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            let is_eof = tok.kind == TokenKind::Eof;
            tokens.push(tok);
            if is_eof {
                break;
            }
        }
        tokens
    }

    /// Get the next token
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let start = self.pos;

        if self.at_end() {
            return Token {
                kind: TokenKind::Eof,
                text: String::new(),
                span: Span { start, end: start },
            };
        }

        // Line comment (// but not //*)
        if self.matches("//") && !self.matches("//*") {
            while !self.at_end() && self.peek() != '\n' {
                self.advance();
            }
            return Token {
                kind: TokenKind::LineComment,
                text: self.source[start..self.pos].to_string(),
                span: Span { start, end: self.pos },
            };
        }

        // Block comment (/* but not //*)
        if self.matches("/*") && !self.matches("//*") {
            self.advance(); // /
            self.advance(); // *
            let mut depth = 1;
            while !self.at_end() && depth > 0 {
                if self.matches("/*") {
                    self.advance();
                    self.advance();
                    depth += 1;
                } else if self.matches("*/") {
                    self.advance();
                    self.advance();
                    depth -= 1;
                } else {
                    self.advance();
                }
            }
            return Token {
                kind: TokenKind::BlockComment,
                text: self.source[start..self.pos].to_string(),
                span: Span { start, end: self.pos },
            };
        }

        let ch = self.peek();

        // Unrestricted name (single-quoted identifier)
        if ch == '\'' {
            return self.scan_unrestricted_name();
        }

        // String literal
        if ch == '"' {
            return self.scan_string();
        }

        // Number literal
        if ch.is_ascii_digit() {
            return self.scan_number();
        }

        // Identifier or keyword
        if is_ident_start(ch) {
            return self.scan_identifier();
        }

        // Punctuation (try longest match first)
        if let Some(tok) = self.scan_punctuation() {
            return tok;
        }

        // Unknown character
        self.advance();
        Token {
            kind: TokenKind::Error,
            text: ch.to_string(),
            span: Span { start, end: self.pos },
        }
    }

    fn at_end(&self) -> bool {
        self.pos >= self.bytes.len()
    }

    fn peek(&self) -> char {
        if self.at_end() {
            '\0'
        } else {
            self.source[self.pos..].chars().next().unwrap_or('\0')
        }
    }

    fn peek_ahead(&self, n: usize) -> char {
        let mut pos = self.pos;
        for _ in 0..n {
            if pos >= self.source.len() {
                return '\0';
            }
            let ch = self.source[pos..].chars().next().unwrap_or('\0');
            pos += ch.len_utf8();
        }
        self.source[pos..].chars().next().unwrap_or('\0')
    }

    fn advance(&mut self) -> char {
        if self.at_end() {
            '\0'
        } else {
            let ch = self.source[self.pos..].chars().next().unwrap_or('\0');
            self.pos += ch.len_utf8();
            ch
        }
    }

    fn skip_whitespace(&mut self) {
        while !self.at_end() && self.peek().is_whitespace() {
            self.advance();
        }
    }

    fn matches(&self, s: &str) -> bool {
        self.source[self.pos..].starts_with(s)
    }

    fn scan_string(&mut self) -> Token {
        let start = self.pos;
        self.advance(); // opening "

        let mut value = String::new();
        while !self.at_end() && self.peek() != '"' {
            if self.peek() == '\\' {
                self.advance();
                match self.peek() {
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    'r' => value.push('\r'),
                    '\\' => value.push('\\'),
                    '"' => value.push('"'),
                    _ => value.push(self.peek()),
                }
                self.advance();
            } else {
                value.push(self.advance());
            }
        }

        if !self.at_end() {
            self.advance(); // closing "
        }

        Token {
            kind: TokenKind::String,
            text: value,
            span: Span { start, end: self.pos },
        }
    }

    fn scan_number(&mut self) -> Token {
        let start = self.pos;
        let mut is_real = false;

        while !self.at_end() && self.peek().is_ascii_digit() {
            self.advance();
        }

        // Check for decimal point
        if self.peek() == '.' && self.peek_ahead(1).is_ascii_digit() {
            is_real = true;
            self.advance(); // .
            while !self.at_end() && self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        // Check for exponent
        if self.peek() == 'e' || self.peek() == 'E' {
            is_real = true;
            self.advance();
            if self.peek() == '+' || self.peek() == '-' {
                self.advance();
            }
            while !self.at_end() && self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let text = self.source[start..self.pos].to_string();
        Token {
            kind: if is_real { TokenKind::Real } else { TokenKind::Integer },
            text,
            span: Span { start, end: self.pos },
        }
    }

    fn scan_identifier(&mut self) -> Token {
        let start = self.pos;

        while !self.at_end() && is_ident_char(self.peek()) {
            self.advance();
        }

        let text = self.source[start..self.pos].to_string();
        let kind = lookup_keyword(&text).unwrap_or(TokenKind::Name);

        Token {
            kind,
            text,
            span: Span { start, end: self.pos },
        }
    }

    fn scan_unrestricted_name(&mut self) -> Token {
        let start = self.pos;
        self.advance(); // opening '

        let mut value = String::new();
        while !self.at_end() && self.peek() != '\'' {
            if self.peek() == '\\' {
                self.advance();
                match self.peek() {
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    'r' => value.push('\r'),
                    '\\' => value.push('\\'),
                    '\'' => value.push('\''),
                    _ => value.push(self.peek()),
                }
                self.advance();
            } else {
                value.push(self.advance());
            }
        }

        if !self.at_end() {
            self.advance(); // closing '
        }

        Token {
            kind: TokenKind::UnrestrictedName,
            text: value,
            span: Span { start, end: self.pos },
        }
    }

    fn scan_punctuation(&mut self) -> Option<Token> {
        let start = self.pos;
        let remaining = &self.source[self.pos..];

"#);

    // Generate punctuation matching - longest match first
    let mut puncts: Vec<_> = grammar.punctuation.iter().collect();
    puncts.sort_by(|a, b| b.len().cmp(&a.len())); // Sort by length descending

    code.push_str("        // Try longest matches first\n");
    for punct in &puncts {
        let variant = punctuation_to_variant(punct);
        let escaped = punct.replace('\\', "\\\\").replace('"', "\\\"");
        code.push_str(&format!(
            "        if remaining.starts_with(\"{}\") {{\n",
            escaped
        ));
        code.push_str(&format!(
            "            for _ in 0..{} {{ self.advance(); }}\n",
            punct.len()
        ));
        code.push_str(&format!(
            "            return Some(Token {{\n                kind: TokenKind::{},\n                text: \"{}\".to_string(),\n                span: Span {{ start, end: self.pos }},\n            }});\n",
            variant, escaped
        ));
        code.push_str("        }\n");
    }

    code.push_str(
        r#"
        None
    }
}

fn is_ident_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

fn is_ident_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple() {
        let mut lexer = Lexer::new("package Foo;");
        let tokens = lexer.tokenize();
        
        assert_eq!(tokens[0].kind, TokenKind::Package);
        assert_eq!(tokens[1].kind, TokenKind::Name);
        assert_eq!(tokens[1].text, "Foo");
        assert_eq!(tokens[2].kind, TokenKind::Semi);
        assert_eq!(tokens[3].kind, TokenKind::Eof);
    }

    #[test]
    fn test_tokenize_operators() {
        let mut lexer = Lexer::new(":> ::> :>>");
        let tokens = lexer.tokenize();
        
        assert_eq!(tokens[0].kind, TokenKind::ColonGt);
        assert_eq!(tokens[1].kind, TokenKind::ColonColonGt);
        assert_eq!(tokens[2].kind, TokenKind::ColonGtGt);
    }

    #[test]
    fn test_tokenize_string() {
        let mut lexer = Lexer::new("\"hello world\"");
        let tokens = lexer.tokenize();
        
        assert_eq!(tokens[0].kind, TokenKind::String);
        assert_eq!(tokens[0].text, "hello world");
    }

    #[test]
    fn test_tokenize_numbers() {
        let mut lexer = Lexer::new("42 3.14 1e10");
        let tokens = lexer.tokenize();
        
        assert_eq!(tokens[0].kind, TokenKind::Integer);
        assert_eq!(tokens[1].kind, TokenKind::Real);
        assert_eq!(tokens[2].kind, TokenKind::Real);
    }

    #[test]
    fn test_skip_comments() {
        let mut lexer = Lexer::new("a // comment\nb /* block */ c");
        let tokens = lexer.tokenize();
        
        assert_eq!(tokens[0].text, "a");
        assert_eq!(tokens[1].kind, TokenKind::LineComment);
        assert_eq!(tokens[2].text, "b");
        assert_eq!(tokens[3].kind, TokenKind::BlockComment);
        assert_eq!(tokens[4].text, "c");
    }
}
"#,
    );

    code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_lexer() {
        let sysml = std::fs::read_to_string("data/SysML-textual-bnf.kebnf").unwrap();
        let grammar = Grammar::parse(&sysml);

        let code = generate(&grammar);

        assert!(code.contains("pub struct Lexer"));
        assert!(code.contains("fn next_token"));
        assert!(code.contains("fn scan_punctuation"));
        // Should have longest-first punctuation matching
        assert!(code.contains("::>"));
    }
}
