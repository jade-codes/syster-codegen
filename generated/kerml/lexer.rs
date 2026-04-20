//! Generated lexer
//!
//! This file was generated from the official KEBNF grammar.
//! Do not edit manually.

use super::tokens::{TokenKind, lookup_keyword};
use super::parser::Token;
use super::super::common::span::Span;

/// Lexer for source code
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

        // Try longest matches first
        if remaining.starts_with(":>>") {
            for _ in 0..3 { self.advance(); }
            return Some(Token {
                kind: TokenKind::ColonGtGt,
                text: ":>>".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with("//*") {
            for _ in 0..3 { self.advance(); }
            return Some(Token {
                kind: TokenKind::Punct2F2F2A,
                text: "//*".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with("===") {
            for _ in 0..3 { self.advance(); }
            return Some(Token {
                kind: TokenKind::Punct3D3D3D,
                text: "===".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with("::>") {
            for _ in 0..3 { self.advance(); }
            return Some(Token {
                kind: TokenKind::ColonColonGt,
                text: "::>".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with("!==") {
            for _ in 0..3 { self.advance(); }
            return Some(Token {
                kind: TokenKind::Punct213D3D,
                text: "!==".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with(":=") {
            for _ in 0..2 { self.advance(); }
            return Some(Token {
                kind: TokenKind::Punct3A3D,
                text: ":=".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with(">=") {
            for _ in 0..2 { self.advance(); }
            return Some(Token {
                kind: TokenKind::GtEq,
                text: ">=".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with("*/") {
            for _ in 0..2 { self.advance(); }
            return Some(Token {
                kind: TokenKind::Punct2A2F,
                text: "*/".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with("==") {
            for _ in 0..2 { self.advance(); }
            return Some(Token {
                kind: TokenKind::EqEq,
                text: "==".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with("/*") {
            for _ in 0..2 { self.advance(); }
            return Some(Token {
                kind: TokenKind::Punct2F2A,
                text: "/*".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with("//") {
            for _ in 0..2 { self.advance(); }
            return Some(Token {
                kind: TokenKind::Punct2F2F,
                text: "//".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with("->") {
            for _ in 0..2 { self.advance(); }
            return Some(Token {
                kind: TokenKind::Arrow,
                text: "->".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with("**") {
            for _ in 0..2 { self.advance(); }
            return Some(Token {
                kind: TokenKind::StarStar,
                text: "**".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with(".?") {
            for _ in 0..2 { self.advance(); }
            return Some(Token {
                kind: TokenKind::DotQuestion,
                text: ".?".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with("??") {
            for _ in 0..2 { self.advance(); }
            return Some(Token {
                kind: TokenKind::QuestionQuestion,
                text: "??".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with("@@") {
            for _ in 0..2 { self.advance(); }
            return Some(Token {
                kind: TokenKind::Punct4040,
                text: "@@".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with("::") {
            for _ in 0..2 { self.advance(); }
            return Some(Token {
                kind: TokenKind::ColonColon,
                text: "::".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with("<=") {
            for _ in 0..2 { self.advance(); }
            return Some(Token {
                kind: TokenKind::LtEq,
                text: "<=".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with(":>") {
            for _ in 0..2 { self.advance(); }
            return Some(Token {
                kind: TokenKind::ColonGt,
                text: ":>".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with("!=") {
            for _ in 0..2 { self.advance(); }
            return Some(Token {
                kind: TokenKind::BangEq,
                text: "!=".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with("=>") {
            for _ in 0..2 { self.advance(); }
            return Some(Token {
                kind: TokenKind::FatArrow,
                text: "=>".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with("..") {
            for _ in 0..2 { self.advance(); }
            return Some(Token {
                kind: TokenKind::DotDot,
                text: "..".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with("+") {
            for _ in 0..1 { self.advance(); }
            return Some(Token {
                kind: TokenKind::Plus,
                text: "+".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with("[") {
            for _ in 0..1 { self.advance(); }
            return Some(Token {
                kind: TokenKind::LBracket,
                text: "[".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with(".") {
            for _ in 0..1 { self.advance(); }
            return Some(Token {
                kind: TokenKind::Dot,
                text: ".".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with("]") {
            for _ in 0..1 { self.advance(); }
            return Some(Token {
                kind: TokenKind::RBracket,
                text: "]".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with("*") {
            for _ in 0..1 { self.advance(); }
            return Some(Token {
                kind: TokenKind::Star,
                text: "*".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with(")") {
            for _ in 0..1 { self.advance(); }
            return Some(Token {
                kind: TokenKind::RParen,
                text: ")".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with("#") {
            for _ in 0..1 { self.advance(); }
            return Some(Token {
                kind: TokenKind::Hash,
                text: "#".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with("&") {
            for _ in 0..1 { self.advance(); }
            return Some(Token {
                kind: TokenKind::Amp,
                text: "&".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with(">") {
            for _ in 0..1 { self.advance(); }
            return Some(Token {
                kind: TokenKind::Gt,
                text: ">".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with("<") {
            for _ in 0..1 { self.advance(); }
            return Some(Token {
                kind: TokenKind::Lt,
                text: "<".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with("~") {
            for _ in 0..1 { self.advance(); }
            return Some(Token {
                kind: TokenKind::Tilde,
                text: "~".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with("{") {
            for _ in 0..1 { self.advance(); }
            return Some(Token {
                kind: TokenKind::LBrace,
                text: "{".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with("/") {
            for _ in 0..1 { self.advance(); }
            return Some(Token {
                kind: TokenKind::Slash,
                text: "/".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with("%") {
            for _ in 0..1 { self.advance(); }
            return Some(Token {
                kind: TokenKind::Percent,
                text: "%".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with("|") {
            for _ in 0..1 { self.advance(); }
            return Some(Token {
                kind: TokenKind::Pipe,
                text: "|".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with("-") {
            for _ in 0..1 { self.advance(); }
            return Some(Token {
                kind: TokenKind::Minus,
                text: "-".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with(",") {
            for _ in 0..1 { self.advance(); }
            return Some(Token {
                kind: TokenKind::Comma,
                text: ",".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with("$") {
            for _ in 0..1 { self.advance(); }
            return Some(Token {
                kind: TokenKind::Dollar,
                text: "$".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with(";") {
            for _ in 0..1 { self.advance(); }
            return Some(Token {
                kind: TokenKind::Semi,
                text: ";".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with("(") {
            for _ in 0..1 { self.advance(); }
            return Some(Token {
                kind: TokenKind::LParen,
                text: "(".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with("}") {
            for _ in 0..1 { self.advance(); }
            return Some(Token {
                kind: TokenKind::RBrace,
                text: "}".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with("^") {
            for _ in 0..1 { self.advance(); }
            return Some(Token {
                kind: TokenKind::Caret,
                text: "^".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with("?") {
            for _ in 0..1 { self.advance(); }
            return Some(Token {
                kind: TokenKind::Question,
                text: "?".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with(":") {
            for _ in 0..1 { self.advance(); }
            return Some(Token {
                kind: TokenKind::Colon,
                text: ":".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with("=") {
            for _ in 0..1 { self.advance(); }
            return Some(Token {
                kind: TokenKind::Eq,
                text: "=".to_string(),
                span: Span { start, end: self.pos },
            });
        }
        if remaining.starts_with("@") {
            for _ in 0..1 { self.advance(); }
            return Some(Token {
                kind: TokenKind::AtSign,
                text: "@".to_string(),
                span: Span { start, end: self.pos },
            });
        }

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
