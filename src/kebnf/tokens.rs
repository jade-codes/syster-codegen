//! Meta-syntax tokens used in KEBNF grammar notation

// Rule definition
pub const EQUALS: char = '=';
pub const COLON: char = ':';

// Alternatives
pub const PIPE: char = '|';

// Grouping
pub const LPAREN: char = '(';
pub const RPAREN: char = ')';

// Quantifiers (postfix)
pub const OPTIONAL: char = '?';
pub const ZERO_OR_MORE: char = '*';
pub const ONE_OR_MORE: char = '+';

// Terminals
pub const QUOTE: char = '\'';

// Cross-references
pub const LBRACKET: char = '[';
pub const RBRACKET: char = ']';
pub const TILDE: char = '~';

// Semantic actions (we skip these)
pub const LBRACE: char = '{';
pub const RBRACE: char = '}';

// Assignments
pub const PLUS_EQUALS: &str = "+=";
pub const QUESTION_EQUALS: &str = "?=";

// Comments
pub const COMMENT_START: &str = "//";
