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
    /// `accept`
    Accept,
    /// `action`
    Action,
    /// `actor`
    Actor,
    /// `after`
    After,
    /// `alias`
    Alias,
    /// `all`
    All,
    /// `allocate`
    Allocate,
    /// `allocation`
    Allocation,
    /// `analysis`
    Analysis,
    /// `and`
    And,
    /// `as`
    As,
    /// `assert`
    Assert,
    /// `assign`
    Assign,
    /// `assoc`
    Assoc,
    /// `assume`
    Assume,
    /// `assumption`
    Assumption,
    /// `at`
    At,
    /// `attribute`
    Attribute,
    /// `behavior`
    Behavior,
    /// `bind`
    Bind,
    /// `binding`
    Binding,
    /// `bool`
    Bool,
    /// `by`
    By,
    /// `calc`
    Calc,
    /// `case`
    Case,
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
    /// `concern`
    Concern,
    /// `conjugate`
    Conjugate,
    /// `conjugates`
    Conjugates,
    /// `conjugation`
    Conjugation,
    /// `connect`
    Connect,
    /// `connection`
    Connection,
    /// `connector`
    Connector,
    /// `const`
    Const,
    /// `constant`
    Constant,
    /// `constraint`
    Constraint,
    /// `crosses`
    Crosses,
    /// `datatype`
    Datatype,
    /// `decide`
    Decide,
    /// `def`
    Def,
    /// `default`
    Default,
    /// `defined`
    Defined,
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
    /// `do`
    Do,
    /// `doc`
    Doc,
    /// `effect`
    Effect,
    /// `else`
    Else,
    /// `end`
    End,
    /// `entry`
    Entry,
    /// `enum`
    Enum,
    /// `event`
    Event,
    /// `exhibit`
    Exhibit,
    /// `exit`
    Exit,
    /// `expose`
    Expose,
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
    /// `fork`
    Fork,
    /// `frame`
    Frame,
    /// `from`
    From,
    /// `function`
    Function,
    /// `guard`
    Guard,
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
    /// `include`
    Include,
    /// `individual`
    Individual,
    /// `inout`
    Inout,
    /// `interaction`
    Interaction,
    /// `interface`
    Interface,
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
    /// `item`
    Item,
    /// `join`
    Join,
    /// `language`
    Language,
    /// `library`
    Library,
    /// `locale`
    Locale,
    /// `loop`
    Loop,
    /// `member`
    Member,
    /// `merge`
    Merge,
    /// `message`
    Message,
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
    /// `objective`
    Objective,
    /// `occurrence`
    Occurrence,
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
    /// `parallel`
    Parallel,
    /// `part`
    Part,
    /// `perform`
    Perform,
    /// `port`
    Port,
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
    /// `ref`
    Ref,
    /// `references`
    References,
    /// `render`
    Render,
    /// `rendering`
    Rendering,
    /// `rep`
    Rep,
    /// `require`
    Require,
    /// `requirement`
    Requirement,
    /// `return`
    Return,
    /// `satisfy`
    Satisfy,
    /// `send`
    Send,
    /// `snapshot`
    Snapshot,
    /// `specialization`
    Specialization,
    /// `specializes`
    Specializes,
    /// `stakeholder`
    Stakeholder,
    /// `standard`
    Standard,
    /// `state`
    State,
    /// `step`
    Step,
    /// `struct`
    Struct,
    /// `subclassifier`
    Subclassifier,
    /// `subject`
    Subject,
    /// `subset`
    Subset,
    /// `subsets`
    Subsets,
    /// `subtype`
    Subtype,
    /// `succession`
    Succession,
    /// `terminate`
    Terminate,
    /// `then`
    Then,
    /// `timeslice`
    Timeslice,
    /// `to`
    To,
    /// `transition`
    Transition,
    /// `trigger`
    Trigger,
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
    /// `until`
    Until,
    /// `use`
    Use,
    /// `var`
    Var,
    /// `variant`
    Variant,
    /// `variation`
    Variation,
    /// `verification`
    Verification,
    /// `verify`
    Verify,
    /// `via`
    Via,
    /// `view`
    View,
    /// `viewpoint`
    Viewpoint,
    /// `when`
    When,
    /// `while`
    While,
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
            Self::Accept |
            Self::Action |
            Self::Actor |
            Self::After |
            Self::Alias |
            Self::All |
            Self::Allocate |
            Self::Allocation |
            Self::Analysis |
            Self::And |
            Self::As |
            Self::Assert |
            Self::Assign |
            Self::Assoc |
            Self::Assume |
            Self::Assumption |
            Self::At |
            Self::Attribute |
            Self::Behavior |
            Self::Bind |
            Self::Binding |
            Self::Bool |
            Self::By |
            Self::Calc |
            Self::Case |
            Self::Chains |
            Self::Class |
            Self::Classifier |
            Self::Comment |
            Self::Composite |
            Self::Concern |
            Self::Conjugate |
            Self::Conjugates |
            Self::Conjugation |
            Self::Connect |
            Self::Connection |
            Self::Connector |
            Self::Const |
            Self::Constant |
            Self::Constraint |
            Self::Crosses |
            Self::Datatype |
            Self::Decide |
            Self::Def |
            Self::Default |
            Self::Defined |
            Self::Dependency |
            Self::Derived |
            Self::Differences |
            Self::Disjoining |
            Self::Disjoint |
            Self::Do |
            Self::Doc |
            Self::Effect |
            Self::Else |
            Self::End |
            Self::Entry |
            Self::Enum |
            Self::Event |
            Self::Exhibit |
            Self::Exit |
            Self::Expose |
            Self::Expr |
            Self::False |
            Self::Feature |
            Self::Featured |
            Self::Featuring |
            Self::Filter |
            Self::First |
            Self::Flow |
            Self::For |
            Self::Fork |
            Self::Frame |
            Self::From |
            Self::Function |
            Self::Guard |
            Self::Hastype |
            Self::If |
            Self::Implies |
            Self::Import |
            Self::In |
            Self::Include |
            Self::Individual |
            Self::Inout |
            Self::Interaction |
            Self::Interface |
            Self::Intersects |
            Self::Inv |
            Self::Inverse |
            Self::Inverting |
            Self::Istype |
            Self::Item |
            Self::Join |
            Self::Language |
            Self::Library |
            Self::Locale |
            Self::Loop |
            Self::Member |
            Self::Merge |
            Self::Message |
            Self::Meta |
            Self::Metaclass |
            Self::Metadata |
            Self::Multiplicity |
            Self::Namespace |
            Self::New |
            Self::Nonunique |
            Self::Not |
            Self::Null |
            Self::Objective |
            Self::Occurrence |
            Self::Of |
            Self::Or |
            Self::Ordered |
            Self::Out |
            Self::Package |
            Self::Parallel |
            Self::Part |
            Self::Perform |
            Self::Port |
            Self::Portion |
            Self::Predicate |
            Self::Private |
            Self::Protected |
            Self::Public |
            Self::Redefines |
            Self::Redefinition |
            Self::Ref |
            Self::References |
            Self::Render |
            Self::Rendering |
            Self::Rep |
            Self::Require |
            Self::Requirement |
            Self::Return |
            Self::Satisfy |
            Self::Send |
            Self::Snapshot |
            Self::Specialization |
            Self::Specializes |
            Self::Stakeholder |
            Self::Standard |
            Self::State |
            Self::Step |
            Self::Struct |
            Self::Subclassifier |
            Self::Subject |
            Self::Subset |
            Self::Subsets |
            Self::Subtype |
            Self::Succession |
            Self::Terminate |
            Self::Then |
            Self::Timeslice |
            Self::To |
            Self::Transition |
            Self::Trigger |
            Self::True |
            Self::Type |
            Self::Typed |
            Self::Typing |
            Self::Unions |
            Self::Until |
            Self::Use |
            Self::Var |
            Self::Variant |
            Self::Variation |
            Self::Verification |
            Self::Verify |
            Self::Via |
            Self::View |
            Self::Viewpoint |
            Self::When |
            Self::While |
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
        "accept" => Some(TokenKind::Accept),
        "action" => Some(TokenKind::Action),
        "actor" => Some(TokenKind::Actor),
        "after" => Some(TokenKind::After),
        "alias" => Some(TokenKind::Alias),
        "all" => Some(TokenKind::All),
        "allocate" => Some(TokenKind::Allocate),
        "allocation" => Some(TokenKind::Allocation),
        "analysis" => Some(TokenKind::Analysis),
        "and" => Some(TokenKind::And),
        "as" => Some(TokenKind::As),
        "assert" => Some(TokenKind::Assert),
        "assign" => Some(TokenKind::Assign),
        "assoc" => Some(TokenKind::Assoc),
        "assume" => Some(TokenKind::Assume),
        "assumption" => Some(TokenKind::Assumption),
        "at" => Some(TokenKind::At),
        "attribute" => Some(TokenKind::Attribute),
        "behavior" => Some(TokenKind::Behavior),
        "bind" => Some(TokenKind::Bind),
        "binding" => Some(TokenKind::Binding),
        "bool" => Some(TokenKind::Bool),
        "by" => Some(TokenKind::By),
        "calc" => Some(TokenKind::Calc),
        "case" => Some(TokenKind::Case),
        "chains" => Some(TokenKind::Chains),
        "class" => Some(TokenKind::Class),
        "classifier" => Some(TokenKind::Classifier),
        "comment" => Some(TokenKind::Comment),
        "composite" => Some(TokenKind::Composite),
        "concern" => Some(TokenKind::Concern),
        "conjugate" => Some(TokenKind::Conjugate),
        "conjugates" => Some(TokenKind::Conjugates),
        "conjugation" => Some(TokenKind::Conjugation),
        "connect" => Some(TokenKind::Connect),
        "connection" => Some(TokenKind::Connection),
        "connector" => Some(TokenKind::Connector),
        "const" => Some(TokenKind::Const),
        "constant" => Some(TokenKind::Constant),
        "constraint" => Some(TokenKind::Constraint),
        "crosses" => Some(TokenKind::Crosses),
        "datatype" => Some(TokenKind::Datatype),
        "decide" => Some(TokenKind::Decide),
        "def" => Some(TokenKind::Def),
        "default" => Some(TokenKind::Default),
        "defined" => Some(TokenKind::Defined),
        "dependency" => Some(TokenKind::Dependency),
        "derived" => Some(TokenKind::Derived),
        "differences" => Some(TokenKind::Differences),
        "disjoining" => Some(TokenKind::Disjoining),
        "disjoint" => Some(TokenKind::Disjoint),
        "do" => Some(TokenKind::Do),
        "doc" => Some(TokenKind::Doc),
        "effect" => Some(TokenKind::Effect),
        "else" => Some(TokenKind::Else),
        "end" => Some(TokenKind::End),
        "entry" => Some(TokenKind::Entry),
        "enum" => Some(TokenKind::Enum),
        "event" => Some(TokenKind::Event),
        "exhibit" => Some(TokenKind::Exhibit),
        "exit" => Some(TokenKind::Exit),
        "expose" => Some(TokenKind::Expose),
        "expr" => Some(TokenKind::Expr),
        "false" => Some(TokenKind::False),
        "feature" => Some(TokenKind::Feature),
        "featured" => Some(TokenKind::Featured),
        "featuring" => Some(TokenKind::Featuring),
        "filter" => Some(TokenKind::Filter),
        "first" => Some(TokenKind::First),
        "flow" => Some(TokenKind::Flow),
        "for" => Some(TokenKind::For),
        "fork" => Some(TokenKind::Fork),
        "frame" => Some(TokenKind::Frame),
        "from" => Some(TokenKind::From),
        "function" => Some(TokenKind::Function),
        "guard" => Some(TokenKind::Guard),
        "hastype" => Some(TokenKind::Hastype),
        "if" => Some(TokenKind::If),
        "implies" => Some(TokenKind::Implies),
        "import" => Some(TokenKind::Import),
        "in" => Some(TokenKind::In),
        "include" => Some(TokenKind::Include),
        "individual" => Some(TokenKind::Individual),
        "inout" => Some(TokenKind::Inout),
        "interaction" => Some(TokenKind::Interaction),
        "interface" => Some(TokenKind::Interface),
        "intersects" => Some(TokenKind::Intersects),
        "inv" => Some(TokenKind::Inv),
        "inverse" => Some(TokenKind::Inverse),
        "inverting" => Some(TokenKind::Inverting),
        "istype" => Some(TokenKind::Istype),
        "item" => Some(TokenKind::Item),
        "join" => Some(TokenKind::Join),
        "language" => Some(TokenKind::Language),
        "library" => Some(TokenKind::Library),
        "locale" => Some(TokenKind::Locale),
        "loop" => Some(TokenKind::Loop),
        "member" => Some(TokenKind::Member),
        "merge" => Some(TokenKind::Merge),
        "message" => Some(TokenKind::Message),
        "meta" => Some(TokenKind::Meta),
        "metaclass" => Some(TokenKind::Metaclass),
        "metadata" => Some(TokenKind::Metadata),
        "multiplicity" => Some(TokenKind::Multiplicity),
        "namespace" => Some(TokenKind::Namespace),
        "new" => Some(TokenKind::New),
        "nonunique" => Some(TokenKind::Nonunique),
        "not" => Some(TokenKind::Not),
        "null" => Some(TokenKind::Null),
        "objective" => Some(TokenKind::Objective),
        "occurrence" => Some(TokenKind::Occurrence),
        "of" => Some(TokenKind::Of),
        "or" => Some(TokenKind::Or),
        "ordered" => Some(TokenKind::Ordered),
        "out" => Some(TokenKind::Out),
        "package" => Some(TokenKind::Package),
        "parallel" => Some(TokenKind::Parallel),
        "part" => Some(TokenKind::Part),
        "perform" => Some(TokenKind::Perform),
        "port" => Some(TokenKind::Port),
        "portion" => Some(TokenKind::Portion),
        "predicate" => Some(TokenKind::Predicate),
        "private" => Some(TokenKind::Private),
        "protected" => Some(TokenKind::Protected),
        "public" => Some(TokenKind::Public),
        "redefines" => Some(TokenKind::Redefines),
        "redefinition" => Some(TokenKind::Redefinition),
        "ref" => Some(TokenKind::Ref),
        "references" => Some(TokenKind::References),
        "render" => Some(TokenKind::Render),
        "rendering" => Some(TokenKind::Rendering),
        "rep" => Some(TokenKind::Rep),
        "require" => Some(TokenKind::Require),
        "requirement" => Some(TokenKind::Requirement),
        "return" => Some(TokenKind::Return),
        "satisfy" => Some(TokenKind::Satisfy),
        "send" => Some(TokenKind::Send),
        "snapshot" => Some(TokenKind::Snapshot),
        "specialization" => Some(TokenKind::Specialization),
        "specializes" => Some(TokenKind::Specializes),
        "stakeholder" => Some(TokenKind::Stakeholder),
        "standard" => Some(TokenKind::Standard),
        "state" => Some(TokenKind::State),
        "step" => Some(TokenKind::Step),
        "struct" => Some(TokenKind::Struct),
        "subclassifier" => Some(TokenKind::Subclassifier),
        "subject" => Some(TokenKind::Subject),
        "subset" => Some(TokenKind::Subset),
        "subsets" => Some(TokenKind::Subsets),
        "subtype" => Some(TokenKind::Subtype),
        "succession" => Some(TokenKind::Succession),
        "terminate" => Some(TokenKind::Terminate),
        "then" => Some(TokenKind::Then),
        "timeslice" => Some(TokenKind::Timeslice),
        "to" => Some(TokenKind::To),
        "transition" => Some(TokenKind::Transition),
        "trigger" => Some(TokenKind::Trigger),
        "true" => Some(TokenKind::True),
        "type" => Some(TokenKind::Type),
        "typed" => Some(TokenKind::Typed),
        "typing" => Some(TokenKind::Typing),
        "unions" => Some(TokenKind::Unions),
        "until" => Some(TokenKind::Until),
        "use" => Some(TokenKind::Use),
        "var" => Some(TokenKind::Var),
        "variant" => Some(TokenKind::Variant),
        "variation" => Some(TokenKind::Variation),
        "verification" => Some(TokenKind::Verification),
        "verify" => Some(TokenKind::Verify),
        "via" => Some(TokenKind::Via),
        "view" => Some(TokenKind::View),
        "viewpoint" => Some(TokenKind::Viewpoint),
        "when" => Some(TokenKind::When),
        "while" => Some(TokenKind::While),
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
