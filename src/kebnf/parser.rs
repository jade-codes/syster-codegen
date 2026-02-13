//! Parser for KEBNF rule body expressions

use super::tokens;
use super::types::{AssignOp, RuleBody};

/// Lexical token from scanning body text
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// 'keyword' or 'operator'
    Keyword(String),
    /// Identifier (rule name or field name)
    Ident(String),
    /// (
    LParen,
    /// )
    RParen,
    /// [
    LBracket,
    /// ]
    RBracket,
    /// {
    LBrace,
    /// }
    RBrace,
    /// |
    Pipe,
    /// ?
    Question,
    /// *
    Star,
    /// +
    Plus,
    /// =
    Equals,
    /// +=
    PlusEquals,
    /// ?=
    QuestionEquals,
    /// ~
    Tilde,
    /// .
    Dot,
    /// End of input
    Eof,
}

/// Tokenize rule body text into tokens
fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            // Whitespace - skip
            c if c.is_whitespace() => {
                chars.next();
            }

            // Quoted keyword/terminal
            c if c == tokens::QUOTE => {
                chars.next(); // consume opening quote
                let mut keyword = String::new();
                while let Some(&c) = chars.peek() {
                    if c == tokens::QUOTE {
                        chars.next(); // consume closing quote
                        break;
                    }
                    keyword.push(c);
                    chars.next();
                }
                tokens.push(Token::Keyword(keyword));
            }

            // Semantic action { ... } - capture content
            c if c == tokens::LBRACE => {
                chars.next();
                let mut depth = 1;
                let mut content = String::new();
                while let Some(&c) = chars.peek() {
                    if c == tokens::LBRACE {
                        depth += 1;
                    } else if c == tokens::RBRACE {
                        depth -= 1;
                        if depth == 0 {
                            chars.next();
                            break;
                        }
                    }
                    content.push(c);
                    chars.next();
                }
                // We'll represent actions as LBrace token followed by content
                tokens.push(Token::LBrace);
                if !content.trim().is_empty() {
                    tokens.push(Token::Ident(content.trim().to_string()));
                }
                tokens.push(Token::RBrace);
            }

            // Operators - check multi-char first
            '+' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::PlusEquals);
                } else {
                    tokens.push(Token::Plus);
                }
            }

            '?' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::QuestionEquals);
                } else {
                    tokens.push(Token::Question);
                }
            }

            c if c == tokens::PIPE => {
                chars.next();
                tokens.push(Token::Pipe);
            }

            c if c == tokens::LPAREN => {
                chars.next();
                tokens.push(Token::LParen);
            }

            c if c == tokens::RPAREN => {
                chars.next();
                tokens.push(Token::RParen);
            }

            c if c == tokens::LBRACKET => {
                chars.next();
                tokens.push(Token::LBracket);
            }

            c if c == tokens::RBRACKET => {
                chars.next();
                tokens.push(Token::RBracket);
            }

            c if c == tokens::EQUALS => {
                chars.next();
                tokens.push(Token::Equals);
            }

            '*' => {
                chars.next();
                tokens.push(Token::Star);
            }

            c if c == tokens::TILDE => {
                chars.next();
                tokens.push(Token::Tilde);
            }

            '.' => {
                chars.next();
                tokens.push(Token::Dot);
            }

            // Identifier
            c if c.is_ascii_alphabetic() || c == '_' => {
                let mut ident = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_alphanumeric() || c == '_' {
                        ident.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Ident(ident));
            }

            // Unknown - skip
            _ => {
                chars.next();
            }
        }
    }

    tokens.push(Token::Eof);
    tokens
}

/// Parser for rule body expressions
struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) -> Token {
        let tok = self.tokens.get(self.pos).cloned().unwrap_or(Token::Eof);
        self.pos += 1;
        tok
    }

    fn check(&self, expected: &Token) -> bool {
        std::mem::discriminant(self.peek()) == std::mem::discriminant(expected)
    }

    /// Top level: alternatives (lowest precedence)
    /// alternatives = sequence ('|' sequence)*
    fn parse_alternatives(&mut self) -> Result<RuleBody, String> {
        let mut alts = vec![self.parse_sequence()?];

        while self.check(&Token::Pipe) {
            self.advance(); // consume |
            alts.push(self.parse_sequence()?);
        }

        if alts.len() == 1 {
            Ok(alts.pop().unwrap())
        } else {
            Ok(RuleBody::Alternative(alts))
        }
    }

    /// sequence = postfix+
    fn parse_sequence(&mut self) -> Result<RuleBody, String> {
        let mut items = Vec::new();

        while !self.is_sequence_end() {
            items.push(self.parse_postfix()?);
        }

        if items.is_empty() {
            Ok(RuleBody::Empty)
        } else if items.len() == 1 {
            Ok(items.pop().unwrap())
        } else {
            Ok(RuleBody::Sequence(items))
        }
    }

    fn is_sequence_end(&self) -> bool {
        matches!(
            self.peek(),
            Token::Pipe | Token::RParen | Token::Eof | Token::RBrace
        )
    }

    /// postfix = atom ('?' | '*' | '+')?
    fn parse_postfix(&mut self) -> Result<RuleBody, String> {
        let atom = self.parse_atom()?;

        match self.peek() {
            Token::Question => {
                self.advance();
                Ok(RuleBody::Optional(Box::new(atom)))
            }
            Token::Star => {
                self.advance();
                Ok(RuleBody::ZeroOrMore(Box::new(atom)))
            }
            Token::Plus => {
                self.advance();
                Ok(RuleBody::OneOrMore(Box::new(atom)))
            }
            _ => Ok(atom),
        }
    }

    /// atom = keyword | group | cross_ref | assignment | rule_ref | action
    fn parse_atom(&mut self) -> Result<RuleBody, String> {
        match self.peek().clone() {
            // 'keyword'
            Token::Keyword(kw) => {
                self.advance();
                Ok(RuleBody::Keyword(kw))
            }

            // ( ... )
            Token::LParen => {
                self.advance(); // consume (
                let inner = self.parse_alternatives()?;
                if !self.check(&Token::RParen) {
                    return Err("Expected ')'".to_string());
                }
                self.advance(); // consume )
                Ok(RuleBody::Group(Box::new(inner)))
            }

            // [ CrossRef ]
            Token::LBracket => {
                self.advance(); // consume [
                let name = match self.advance() {
                    Token::Ident(s) => s,
                    _ => return Err("Expected identifier in cross-ref".to_string()),
                };
                if !self.check(&Token::RBracket) {
                    return Err("Expected ']'".to_string());
                }
                self.advance(); // consume ]
                Ok(RuleBody::CrossRef(name))
            }

            // { action }
            Token::LBrace => {
                self.advance(); // consume {
                let content = match self.peek() {
                    Token::Ident(s) => {
                        let s = s.clone();
                        self.advance();
                        s
                    }
                    _ => String::new(),
                };
                if self.check(&Token::RBrace) {
                    self.advance(); // consume }
                }
                Ok(RuleBody::Action(content))
            }

            // ~ negation (rare, treat as skip for now)
            Token::Tilde => {
                self.advance();
                let inner = self.parse_atom()?;
                // We don't have Negation in RuleBody, wrap as Group
                Ok(RuleBody::Group(Box::new(inner)))
            }

            // Identifier - could be assignment or rule ref
            Token::Ident(name) => {
                self.advance();

                // Handle qualified names like s.isSufficient
                let final_name = if self.check(&Token::Dot) {
                    self.advance(); // consume .
                    match self.peek() {
                        Token::Ident(suffix) => {
                            let suffix = suffix.clone();
                            self.advance();
                            // Use the final part as the name (s.isSufficient -> isSufficient)
                            suffix
                        }
                        _ => name.clone(),
                    }
                } else {
                    name.clone()
                };

                // Check for assignment: name = value, name += value, name ?= value
                match self.peek() {
                    Token::Equals => {
                        self.advance();
                        let value = self.parse_postfix()?;
                        Ok(RuleBody::Assignment {
                            name: final_name,
                            operator: AssignOp::Assign,
                            value: Box::new(value),
                        })
                    }
                    Token::PlusEquals => {
                        self.advance();
                        let value = self.parse_postfix()?;
                        Ok(RuleBody::Assignment {
                            name: final_name,
                            operator: AssignOp::AddAssign,
                            value: Box::new(value),
                        })
                    }
                    Token::QuestionEquals => {
                        self.advance();
                        let value = self.parse_postfix()?;
                        Ok(RuleBody::BoolAssign {
                            name: final_name,
                            value: Box::new(value),
                        })
                    }
                    // Plain rule reference - use original name for rule refs
                    _ => Ok(RuleBody::RuleRef(name)),
                }
            }

            Token::Eof => Err("Unexpected end of input".to_string()),

            other => Err(format!("Unexpected token: {:?}", other)),
        }
    }
}

/// Parse a rule body string into a RuleBody AST
pub fn parse_body(input: &str) -> Result<RuleBody, String> {
    let tokens = tokenize(input);
    let mut parser = Parser::new(tokens);
    parser.parse_alternatives()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple() {
        let tokens = tokenize("'package' Name Body");
        assert_eq!(
            tokens,
            vec![
                Token::Keyword("package".to_string()),
                Token::Ident("Name".to_string()),
                Token::Ident("Body".to_string()),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_tokenize_operators() {
        let tokens = tokenize("A | B? C* D+");
        assert_eq!(
            tokens,
            vec![
                Token::Ident("A".to_string()),
                Token::Pipe,
                Token::Ident("B".to_string()),
                Token::Question,
                Token::Ident("C".to_string()),
                Token::Star,
                Token::Ident("D".to_string()),
                Token::Plus,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_tokenize_assignment() {
        let tokens = tokenize("name=NAME items+=Item flag?='true'");
        assert_eq!(
            tokens,
            vec![
                Token::Ident("name".to_string()),
                Token::Equals,
                Token::Ident("NAME".to_string()),
                Token::Ident("items".to_string()),
                Token::PlusEquals,
                Token::Ident("Item".to_string()),
                Token::Ident("flag".to_string()),
                Token::QuestionEquals,
                Token::Keyword("true".to_string()),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_parse_keyword() {
        let body = parse_body("'package'").unwrap();
        assert!(matches!(body, RuleBody::Keyword(k) if k == "package"));
    }

    #[test]
    fn test_parse_sequence() {
        let body = parse_body("'package' Identification PackageBody").unwrap();
        match body {
            RuleBody::Sequence(items) => {
                assert_eq!(items.len(), 3);
                assert!(matches!(&items[0], RuleBody::Keyword(k) if k == "package"));
                assert!(matches!(&items[1], RuleBody::RuleRef(r) if r == "Identification"));
                assert!(matches!(&items[2], RuleBody::RuleRef(r) if r == "PackageBody"));
            }
            _ => panic!("Expected Sequence, got {:?}", body),
        }
    }

    #[test]
    fn test_parse_alternatives() {
        let body = parse_body("';' | '{' '}'").unwrap();
        match body {
            RuleBody::Alternative(alts) => {
                assert_eq!(alts.len(), 2);
                assert!(matches!(&alts[0], RuleBody::Keyword(k) if k == ";"));
            }
            _ => panic!("Expected Alternative, got {:?}", body),
        }
    }

    #[test]
    fn test_parse_optional() {
        let body = parse_body("Name?").unwrap();
        match body {
            RuleBody::Optional(inner) => {
                assert!(matches!(*inner, RuleBody::RuleRef(r) if r == "Name"));
            }
            _ => panic!("Expected Optional, got {:?}", body),
        }
    }

    #[test]
    fn test_parse_zero_or_more() {
        let body = parse_body("Element*").unwrap();
        match body {
            RuleBody::ZeroOrMore(inner) => {
                assert!(matches!(*inner, RuleBody::RuleRef(r) if r == "Element"));
            }
            _ => panic!("Expected ZeroOrMore, got {:?}", body),
        }
    }

    #[test]
    fn test_parse_group_with_alternatives() {
        let body = parse_body("( 'from' | 'to' )?").unwrap();
        match body {
            RuleBody::Optional(inner) => match *inner {
                RuleBody::Group(g) => match *g {
                    RuleBody::Alternative(alts) => {
                        assert_eq!(alts.len(), 2);
                    }
                    _ => panic!("Expected Alternative inside Group"),
                },
                _ => panic!("Expected Group"),
            },
            _ => panic!("Expected Optional, got {:?}", body),
        }
    }

    #[test]
    fn test_parse_assignment() {
        let body = parse_body("name=NAME").unwrap();
        match body {
            RuleBody::Assignment { name, operator, value } => {
                assert_eq!(name, "name");
                assert_eq!(operator, AssignOp::Assign);
                assert!(matches!(*value, RuleBody::RuleRef(r) if r == "NAME"));
            }
            _ => panic!("Expected Assignment, got {:?}", body),
        }
    }

    #[test]
    fn test_parse_add_assignment() {
        let body = parse_body("items+=Item").unwrap();
        match body {
            RuleBody::Assignment { name, operator, value } => {
                assert_eq!(name, "items");
                assert_eq!(operator, AssignOp::AddAssign);
                assert!(matches!(*value, RuleBody::RuleRef(r) if r == "Item"));
            }
            _ => panic!("Expected Assignment, got {:?}", body),
        }
    }

    #[test]
    fn test_parse_bool_assignment() {
        let body = parse_body("isAbstract?='abstract'").unwrap();
        match body {
            RuleBody::BoolAssign { name, value } => {
                assert_eq!(name, "isAbstract");
                assert!(matches!(*value, RuleBody::Keyword(k) if k == "abstract"));
            }
            _ => panic!("Expected BoolAssign, got {:?}", body),
        }
    }

    #[test]
    fn test_parse_cross_ref() {
        let body = parse_body("[Element]").unwrap();
        match body {
            RuleBody::CrossRef(name) => {
                assert_eq!(name, "Element");
            }
            _ => panic!("Expected CrossRef, got {:?}", body),
        }
    }

    #[test]
    fn test_parse_complex_rule() {
        // PackageBody from SysML grammar
        let body = parse_body("';' | '{' PackageBodyElement* '}'").unwrap();
        match body {
            RuleBody::Alternative(alts) => {
                assert_eq!(alts.len(), 2);
                // First alt is just ';'
                assert!(matches!(&alts[0], RuleBody::Keyword(k) if k == ";"));
                // Second alt is sequence
                match &alts[1] {
                    RuleBody::Sequence(items) => {
                        assert_eq!(items.len(), 3);
                        assert!(matches!(&items[0], RuleBody::Keyword(k) if k == "{"));
                        assert!(matches!(&items[1], RuleBody::ZeroOrMore(_)));
                        assert!(matches!(&items[2], RuleBody::Keyword(k) if k == "}"));
                    }
                    _ => panic!("Expected Sequence"),
                }
            }
            _ => panic!("Expected Alternative, got {:?}", body),
        }
    }

    #[test]
    fn test_parse_real_grammar_rules() {
        // Test a few rules from the actual SysML grammar
        let rules = vec![
            "'package' Identification PackageBody",
            "( '<' name=NAME '>' )?",
            "visibility=VisibilityIndicator? 'import' isAll?='all'? importedReference=[QualifiedName]",
            "relatedElement+=[QualifiedName] ( ',' relatedElement+=[QualifiedName] )*",
        ];

        for rule in rules {
            let result = parse_body(rule);
            assert!(result.is_ok(), "Failed to parse: {}\nError: {:?}", rule, result);
        }
    }

    #[test]
    fn test_parse_full_sysml_grammar() {
        use crate::kebnf::Grammar;
        
        let sysml = std::fs::read_to_string("data/SysML-textual-bnf.kebnf").unwrap();
        let grammar = Grammar::parse(&sysml);
        
        assert_eq!(grammar.rules.len(), 350);
        assert!(!grammar.keywords.is_empty());
        assert!(!grammar.punctuation.is_empty());
        
        // Check that all rules have parsed bodies (not Empty)
        let empty_count = grammar.rules.iter()
            .filter(|r| matches!(r.body, RuleBody::Empty))
            .count();
        
        // Some rules may legitimately be empty, but most should have content
        assert!(empty_count < 10, "Too many empty rules: {}", empty_count);
    }
}
