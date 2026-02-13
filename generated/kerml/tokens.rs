//! Generated token types for SysML v2
//!
//! This file was generated from the official KEBNF grammar.
//! Do not edit manually.

/// Token types for the SysML v2 lexer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    // Keywords
    /// `about`
    About,
    /// `abstract`
    Abstract,
    /// `alias`
    Alias,
    /// `all`
    All,
    /// `and`
    And,
    /// `as`
    As,
    /// `assoc`
    Assoc,
    /// `behavior`
    Behavior,
    /// `binding`
    Binding,
    /// `bool`
    Bool,
    /// `by`
    By,
    /// `chains`
    Chains,
    /// `class`
    Class,
    /// `classifier`
    Classifier,
    /// `comment`
    Comment,
    /// `composite`
    Composite,
    /// `conjugate`
    Conjugate,
    /// `conjugates`
    Conjugates,
    /// `conjugation`
    Conjugation,
    /// `connector`
    Connector,
    /// `const`
    Const,
    /// `crosses`
    Crosses,
    /// `datatype`
    Datatype,
    /// `default`
    Default,
    /// `dependency`
    Dependency,
    /// `derived`
    Derived,
    /// `differences`
    Differences,
    /// `disjoining`
    Disjoining,
    /// `disjoint`
    Disjoint,
    /// `doc`
    Doc,
    /// `else`
    Else,
    /// `end`
    End,
    /// `expr`
    Expr,
    /// `false`
    False,
    /// `feature`
    Feature,
    /// `featured`
    Featured,
    /// `featuring`
    Featuring,
    /// `filter`
    Filter,
    /// `first`
    First,
    /// `flow`
    Flow,
    /// `for`
    For,
    /// `from`
    From,
    /// `function`
    Function,
    /// `hastype`
    Hastype,
    /// `if`
    If,
    /// `implies`
    Implies,
    /// `import`
    Import,
    /// `in`
    In,
    /// `inout`
    Inout,
    /// `interaction`
    Interaction,
    /// `intersects`
    Intersects,
    /// `inv`
    Inv,
    /// `inverse`
    Inverse,
    /// `inverting`
    Inverting,
    /// `istype`
    Istype,
    /// `language`
    Language,
    /// `library`
    Library,
    /// `locale`
    Locale,
    /// `member`
    Member,
    /// `meta`
    Meta,
    /// `metaclass`
    Metaclass,
    /// `metadata`
    Metadata,
    /// `multiplicity`
    Multiplicity,
    /// `namespace`
    Namespace,
    /// `new`
    New,
    /// `nonunique`
    Nonunique,
    /// `not`
    Not,
    /// `null`
    Null,
    /// `of`
    Of,
    /// `or`
    Or,
    /// `ordered`
    Ordered,
    /// `out`
    Out,
    /// `package`
    Package,
    /// `portion`
    Portion,
    /// `predicate`
    Predicate,
    /// `private`
    Private,
    /// `protected`
    Protected,
    /// `public`
    Public,
    /// `redefines`
    Redefines,
    /// `redefinition`
    Redefinition,
    /// `references`
    References,
    /// `rep`
    Rep,
    /// `return`
    Return,
    /// `specialization`
    Specialization,
    /// `specializes`
    Specializes,
    /// `standard`
    Standard,
    /// `step`
    Step,
    /// `struct`
    Struct,
    /// `subclassifier`
    Subclassifier,
    /// `subset`
    Subset,
    /// `subsets`
    Subsets,
    /// `subtype`
    Subtype,
    /// `succession`
    Succession,
    /// `then`
    Then,
    /// `to`
    To,
    /// `true`
    True,
    /// `type`
    Type,
    /// `typed`
    Typed,
    /// `typing`
    Typing,
    /// `unions`
    Unions,
    /// `var`
    Var,
    /// `xor`
    Xor,

    // Punctuation
    /// `"!="`
    BangEq,
    /// `"!=="`
    Punct213D3D,
    /// `"#"`
    Hash,
    /// `"$"`
    Dollar,
    /// `"%"`
    Percent,
    /// `"&"`
    Amp,
    /// `"("`
    LParen,
    /// `")"`
    RParen,
    /// `"*"`
    Star,
    /// `"**"`
    StarStar,
    /// `"*/"`
    Punct2A2F,
    /// `"+"`
    Plus,
    /// `","`
    Comma,
    /// `"-"`
    Minus,
    /// `"->"`
    Arrow,
    /// `"."`
    Dot,
    /// `".."`
    DotDot,
    /// `".?"`
    DotQuestion,
    /// `"/"`
    Slash,
    /// `"/*"`
    Punct2F2A,
    /// `"//"`
    Punct2F2F,
    /// `"//*"`
    Punct2F2F2A,
    /// `":"`
    Colon,
    /// `"::"`
    ColonColon,
    /// `"::>"`
    ColonColonGt,
    /// `":="`
    Punct3A3D,
    /// `":>"`
    ColonGt,
    /// `":>>"`
    ColonGtGt,
    /// `";"`
    Semi,
    /// `"<"`
    Lt,
    /// `"<="`
    LtEq,
    /// `"="`
    Eq,
    /// `"=="`
    EqEq,
    /// `"==="`
    Punct3D3D3D,
    /// `"=>"`
    FatArrow,
    /// `">"`
    Gt,
    /// `">="`
    GtEq,
    /// `"?"`
    Question,
    /// `"??"`
    QuestionQuestion,
    /// `"@"`
    AtSign,
    /// `"@@"`
    Punct4040,
    /// `"["`
    LBracket,
    /// `"]"`
    RBracket,
    /// `"^"`
    Caret,
    /// `"{"`
    LBrace,
    /// `"|"`
    Pipe,
    /// `"}"`
    RBrace,
    /// `"~"`
    Tilde,

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
            Self::About |
            Self::Abstract |
            Self::Alias |
            Self::All |
            Self::And |
            Self::As |
            Self::Assoc |
            Self::Behavior |
            Self::Binding |
            Self::Bool |
            Self::By |
            Self::Chains |
            Self::Class |
            Self::Classifier |
            Self::Comment |
            Self::Composite |
            Self::Conjugate |
            Self::Conjugates |
            Self::Conjugation |
            Self::Connector |
            Self::Const |
            Self::Crosses |
            Self::Datatype |
            Self::Default |
            Self::Dependency |
            Self::Derived |
            Self::Differences |
            Self::Disjoining |
            Self::Disjoint |
            Self::Doc |
            Self::Else |
            Self::End |
            Self::Expr |
            Self::False |
            Self::Feature |
            Self::Featured |
            Self::Featuring |
            Self::Filter |
            Self::First |
            Self::Flow |
            Self::For |
            Self::From |
            Self::Function |
            Self::Hastype |
            Self::If |
            Self::Implies |
            Self::Import |
            Self::In |
            Self::Inout |
            Self::Interaction |
            Self::Intersects |
            Self::Inv |
            Self::Inverse |
            Self::Inverting |
            Self::Istype |
            Self::Language |
            Self::Library |
            Self::Locale |
            Self::Member |
            Self::Meta |
            Self::Metaclass |
            Self::Metadata |
            Self::Multiplicity |
            Self::Namespace |
            Self::New |
            Self::Nonunique |
            Self::Not |
            Self::Null |
            Self::Of |
            Self::Or |
            Self::Ordered |
            Self::Out |
            Self::Package |
            Self::Portion |
            Self::Predicate |
            Self::Private |
            Self::Protected |
            Self::Public |
            Self::Redefines |
            Self::Redefinition |
            Self::References |
            Self::Rep |
            Self::Return |
            Self::Specialization |
            Self::Specializes |
            Self::Standard |
            Self::Step |
            Self::Struct |
            Self::Subclassifier |
            Self::Subset |
            Self::Subsets |
            Self::Subtype |
            Self::Succession |
            Self::Then |
            Self::To |
            Self::True |
            Self::Type |
            Self::Typed |
            Self::Typing |
            Self::Unions |
            Self::Var |
            Self::Xor
        )
    }

    /// Check if this token is punctuation
    pub fn is_punctuation(&self) -> bool {
        matches!(self,
            Self::BangEq |
            Self::Punct213D3D |
            Self::Hash |
            Self::Dollar |
            Self::Percent |
            Self::Amp |
            Self::LParen |
            Self::RParen |
            Self::Star |
            Self::StarStar |
            Self::Punct2A2F |
            Self::Plus |
            Self::Comma |
            Self::Minus |
            Self::Arrow |
            Self::Dot |
            Self::DotDot |
            Self::DotQuestion |
            Self::Slash |
            Self::Punct2F2A |
            Self::Punct2F2F |
            Self::Punct2F2F2A |
            Self::Colon |
            Self::ColonColon |
            Self::ColonColonGt |
            Self::Punct3A3D |
            Self::ColonGt |
            Self::ColonGtGt |
            Self::Semi |
            Self::Lt |
            Self::LtEq |
            Self::Eq |
            Self::EqEq |
            Self::Punct3D3D3D |
            Self::FatArrow |
            Self::Gt |
            Self::GtEq |
            Self::Question |
            Self::QuestionQuestion |
            Self::AtSign |
            Self::Punct4040 |
            Self::LBracket |
            Self::RBracket |
            Self::Caret |
            Self::LBrace |
            Self::Pipe |
            Self::RBrace |
            Self::Tilde
        )
    }

    /// Check if this token can appear in a NAME position.
    /// NAME = BASIC_NAME | UNRESTRICTED_NAME, and BASIC_NAME matches any
    /// identifier-like string [a-zA-Z_][a-zA-Z0-9_]*, which includes all keywords.
    pub fn is_name_compatible(&self) -> bool {
        matches!(self, Self::Name | Self::UnrestrictedName) || self.is_keyword()
    }

    /// Check if this token is a strict name token (Name or UnrestrictedName).
    /// Unlike `is_name_compatible`, this does NOT include keywords.
    /// Used by `parse_cross_ref` so that cross-references do not greedily
    /// consume structural keywords that belong to the enclosing grammar rule.
    pub fn is_name_token(&self) -> bool {
        matches!(self, Self::Name | Self::UnrestrictedName)
    }
}

/// Look up a keyword from its string representation
pub fn lookup_keyword(s: &str) -> Option<TokenKind> {
    match s {
        "about" => Some(TokenKind::About),
        "abstract" => Some(TokenKind::Abstract),
        "alias" => Some(TokenKind::Alias),
        "all" => Some(TokenKind::All),
        "and" => Some(TokenKind::And),
        "as" => Some(TokenKind::As),
        "assoc" => Some(TokenKind::Assoc),
        "behavior" => Some(TokenKind::Behavior),
        "binding" => Some(TokenKind::Binding),
        "bool" => Some(TokenKind::Bool),
        "by" => Some(TokenKind::By),
        "chains" => Some(TokenKind::Chains),
        "class" => Some(TokenKind::Class),
        "classifier" => Some(TokenKind::Classifier),
        "comment" => Some(TokenKind::Comment),
        "composite" => Some(TokenKind::Composite),
        "conjugate" => Some(TokenKind::Conjugate),
        "conjugates" => Some(TokenKind::Conjugates),
        "conjugation" => Some(TokenKind::Conjugation),
        "connector" => Some(TokenKind::Connector),
        "const" => Some(TokenKind::Const),
        "crosses" => Some(TokenKind::Crosses),
        "datatype" => Some(TokenKind::Datatype),
        "default" => Some(TokenKind::Default),
        "dependency" => Some(TokenKind::Dependency),
        "derived" => Some(TokenKind::Derived),
        "differences" => Some(TokenKind::Differences),
        "disjoining" => Some(TokenKind::Disjoining),
        "disjoint" => Some(TokenKind::Disjoint),
        "doc" => Some(TokenKind::Doc),
        "else" => Some(TokenKind::Else),
        "end" => Some(TokenKind::End),
        "expr" => Some(TokenKind::Expr),
        "false" => Some(TokenKind::False),
        "feature" => Some(TokenKind::Feature),
        "featured" => Some(TokenKind::Featured),
        "featuring" => Some(TokenKind::Featuring),
        "filter" => Some(TokenKind::Filter),
        "first" => Some(TokenKind::First),
        "flow" => Some(TokenKind::Flow),
        "for" => Some(TokenKind::For),
        "from" => Some(TokenKind::From),
        "function" => Some(TokenKind::Function),
        "hastype" => Some(TokenKind::Hastype),
        "if" => Some(TokenKind::If),
        "implies" => Some(TokenKind::Implies),
        "import" => Some(TokenKind::Import),
        "in" => Some(TokenKind::In),
        "inout" => Some(TokenKind::Inout),
        "interaction" => Some(TokenKind::Interaction),
        "intersects" => Some(TokenKind::Intersects),
        "inv" => Some(TokenKind::Inv),
        "inverse" => Some(TokenKind::Inverse),
        "inverting" => Some(TokenKind::Inverting),
        "istype" => Some(TokenKind::Istype),
        "language" => Some(TokenKind::Language),
        "library" => Some(TokenKind::Library),
        "locale" => Some(TokenKind::Locale),
        "member" => Some(TokenKind::Member),
        "meta" => Some(TokenKind::Meta),
        "metaclass" => Some(TokenKind::Metaclass),
        "metadata" => Some(TokenKind::Metadata),
        "multiplicity" => Some(TokenKind::Multiplicity),
        "namespace" => Some(TokenKind::Namespace),
        "new" => Some(TokenKind::New),
        "nonunique" => Some(TokenKind::Nonunique),
        "not" => Some(TokenKind::Not),
        "null" => Some(TokenKind::Null),
        "of" => Some(TokenKind::Of),
        "or" => Some(TokenKind::Or),
        "ordered" => Some(TokenKind::Ordered),
        "out" => Some(TokenKind::Out),
        "package" => Some(TokenKind::Package),
        "portion" => Some(TokenKind::Portion),
        "predicate" => Some(TokenKind::Predicate),
        "private" => Some(TokenKind::Private),
        "protected" => Some(TokenKind::Protected),
        "public" => Some(TokenKind::Public),
        "redefines" => Some(TokenKind::Redefines),
        "redefinition" => Some(TokenKind::Redefinition),
        "references" => Some(TokenKind::References),
        "rep" => Some(TokenKind::Rep),
        "return" => Some(TokenKind::Return),
        "specialization" => Some(TokenKind::Specialization),
        "specializes" => Some(TokenKind::Specializes),
        "standard" => Some(TokenKind::Standard),
        "step" => Some(TokenKind::Step),
        "struct" => Some(TokenKind::Struct),
        "subclassifier" => Some(TokenKind::Subclassifier),
        "subset" => Some(TokenKind::Subset),
        "subsets" => Some(TokenKind::Subsets),
        "subtype" => Some(TokenKind::Subtype),
        "succession" => Some(TokenKind::Succession),
        "then" => Some(TokenKind::Then),
        "to" => Some(TokenKind::To),
        "true" => Some(TokenKind::True),
        "type" => Some(TokenKind::Type),
        "typed" => Some(TokenKind::Typed),
        "typing" => Some(TokenKind::Typing),
        "unions" => Some(TokenKind::Unions),
        "var" => Some(TokenKind::Var),
        "xor" => Some(TokenKind::Xor),
        _ => None,
    }
}

/// Look up punctuation from its string representation
pub fn lookup_punctuation(s: &str) -> Option<TokenKind> {
    match s {
        "!=" => Some(TokenKind::BangEq),
        "!==" => Some(TokenKind::Punct213D3D),
        "#" => Some(TokenKind::Hash),
        "$" => Some(TokenKind::Dollar),
        "%" => Some(TokenKind::Percent),
        "&" => Some(TokenKind::Amp),
        "(" => Some(TokenKind::LParen),
        ")" => Some(TokenKind::RParen),
        "*" => Some(TokenKind::Star),
        "**" => Some(TokenKind::StarStar),
        "*/" => Some(TokenKind::Punct2A2F),
        "+" => Some(TokenKind::Plus),
        "," => Some(TokenKind::Comma),
        "-" => Some(TokenKind::Minus),
        "->" => Some(TokenKind::Arrow),
        "." => Some(TokenKind::Dot),
        ".." => Some(TokenKind::DotDot),
        ".?" => Some(TokenKind::DotQuestion),
        "/" => Some(TokenKind::Slash),
        "/*" => Some(TokenKind::Punct2F2A),
        "//" => Some(TokenKind::Punct2F2F),
        "//*" => Some(TokenKind::Punct2F2F2A),
        ":" => Some(TokenKind::Colon),
        "::" => Some(TokenKind::ColonColon),
        "::>" => Some(TokenKind::ColonColonGt),
        ":=" => Some(TokenKind::Punct3A3D),
        ":>" => Some(TokenKind::ColonGt),
        ":>>" => Some(TokenKind::ColonGtGt),
        ";" => Some(TokenKind::Semi),
        "<" => Some(TokenKind::Lt),
        "<=" => Some(TokenKind::LtEq),
        "=" => Some(TokenKind::Eq),
        "==" => Some(TokenKind::EqEq),
        "===" => Some(TokenKind::Punct3D3D3D),
        "=>" => Some(TokenKind::FatArrow),
        ">" => Some(TokenKind::Gt),
        ">=" => Some(TokenKind::GtEq),
        "?" => Some(TokenKind::Question),
        "??" => Some(TokenKind::QuestionQuestion),
        "@" => Some(TokenKind::AtSign),
        "@@" => Some(TokenKind::Punct4040),
        "[" => Some(TokenKind::LBracket),
        "]" => Some(TokenKind::RBracket),
        "^" => Some(TokenKind::Caret),
        "{" => Some(TokenKind::LBrace),
        "|" => Some(TokenKind::Pipe),
        "}" => Some(TokenKind::RBrace),
        "~" => Some(TokenKind::Tilde),
        _ => None,
    }
}
