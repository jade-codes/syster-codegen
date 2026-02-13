//! Core grammar types

use std::collections::HashSet;
use crate::kebnf::terminals::extract_terminals;
use crate::kebnf::rules::extract_rule_headers;
use crate::kebnf::parser::parse_body;

/// A parsed KEBNF grammar
#[derive(Debug, Default)]
pub struct Grammar {
    pub rules: Vec<Rule>,
    /// Reserved keywords (alphabetic identifiers like 'package', 'part')
    pub keywords: HashSet<String>,
    /// Punctuation/operators (like ':', ':>', '{', '}')
    pub punctuation: HashSet<String>,
}

/// A grammar rule
#[derive(Debug)]
pub struct Rule {
    pub name: String,
    pub return_type: Option<String>,
    pub body: RuleBody,
}

/// The body/production of a rule
#[derive(Debug, Clone)]
pub enum RuleBody {
    /// Empty production
    Empty,
    /// Sequence: A B C
    Sequence(Vec<RuleBody>),
    /// Alternatives: A | B | C
    Alternative(Vec<RuleBody>),
    /// Optional: A?
    Optional(Box<RuleBody>),
    /// Zero or more: A*
    ZeroOrMore(Box<RuleBody>),
    /// One or more: A+
    OneOrMore(Box<RuleBody>),
    /// Grouped: ( A B )
    Group(Box<RuleBody>),
    /// Terminal keyword: 'keyword'
    Keyword(String),
    /// Rule reference: RuleName
    RuleRef(String),
    /// Assignment: name = Value or name += Value
    Assignment {
        name: String,
        operator: AssignOp,
        value: Box<RuleBody>,
    },
    /// Cross-reference: [QualifiedName]
    CrossRef(String),
    /// Boolean assignment: flag ?= 'keyword'
    BoolAssign {
        name: String,
        value: Box<RuleBody>,
    },
    /// Semantic action: { code }
    Action(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignOp {
    /// =
    Assign,
    /// +=
    AddAssign,
}

impl Grammar {
    /// Parse a KEBNF grammar from source text
    pub fn parse(source: &str) -> Self {
        let (keywords, punctuation) = extract_terminals(source);
        let headers = extract_rule_headers(source);
        
        let rules = headers
            .iter()
            .map(|h| Rule {
                name: h.name.clone(),
                return_type: h.return_type.clone(),
                body: parse_body(&h.body_text).unwrap_or(RuleBody::Empty),
            })
            .collect();
        
        Grammar {
            rules,
            keywords,
            punctuation,
        }
    }
    
    /// Parse multiple grammar files and merge them
    pub fn parse_all(sources: &[&str]) -> Self {
        let mut grammar = Grammar::default();
        let mut rule_map = std::collections::HashMap::new();
        
        for source in sources {
            let (keywords, punctuation) = extract_terminals(source);
            grammar.keywords.extend(keywords);
            grammar.punctuation.extend(punctuation);
            
            let headers = extract_rule_headers(source);
            for h in headers {
                // Later definitions override earlier ones (SysML overrides KerML)
                rule_map.insert(h.name.clone(), Rule {
                    name: h.name,
                    return_type: h.return_type,
                    body: parse_body(&h.body_text).unwrap_or(RuleBody::Empty),
                });
            }
        }
        
        // Collect rules preserving insertion order
        grammar.rules = rule_map.into_values().collect();
        grammar
    }
}
