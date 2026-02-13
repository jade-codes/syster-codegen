//! Name conversion utilities

/// Convert a grammar keyword to a Rust enum variant name
/// e.g., "package" -> "Package", "abstract" -> "Abstract"
pub fn keyword_to_variant(keyword: &str) -> String {
    // Just use PascalCase - Rust keywords are lowercase so capitalized versions are fine
    to_pascal_case(keyword)
}

/// Convert a string to PascalCase
/// e.g., "package_body" -> "PackageBody", "import" -> "Import"
pub fn to_pascal_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;
    
    for ch in s.chars() {
        if ch == '_' || ch == '-' || ch.is_whitespace() {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(ch.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(ch);
        }
    }
    
    result
}

/// Convert a string to snake_case
/// e.g., "PackageBody" -> "package_body"
pub fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    
    for (i, ch) in s.chars().enumerate() {
        if ch.is_ascii_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(ch.to_ascii_lowercase());
        } else {
            result.push(ch);
        }
    }
    
    escape_rust_keyword(&result)
}

/// Escape Rust reserved keywords by appending an underscore
/// e.g., "type" -> "type_", "self" -> "self_"
pub fn escape_rust_keyword(name: &str) -> String {
    const RUST_KEYWORDS: &[&str] = &[
        "as", "async", "await", "break", "const", "continue", "crate", "dyn",
        "else", "enum", "extern", "false", "fn", "for", "if", "impl", "in",
        "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
        "self", "Self", "static", "struct", "super", "trait", "true", "type",
        "unsafe", "use", "where", "while", "abstract", "become", "box", "do",
        "final", "macro", "override", "priv", "typeof", "unsized", "virtual",
        "yield", "try",
    ];
    
    if RUST_KEYWORDS.contains(&name) {
        format!("{}_", name)
    } else {
        name.to_string()
    }
}

/// Convert punctuation to a Rust enum variant name
/// e.g., "{" -> "LBrace", "::" -> "ColonColon"
pub fn punctuation_to_variant(punct: &str) -> String {
    match punct {
        "{" => "LBrace".to_string(),
        "}" => "RBrace".to_string(),
        "(" => "LParen".to_string(),
        ")" => "RParen".to_string(),
        "[" => "LBracket".to_string(),
        "]" => "RBracket".to_string(),
        "<" => "Lt".to_string(),
        ">" => "Gt".to_string(),
        "<<" => "LtLt".to_string(),
        ">>" => "GtGt".to_string(),
        "<=" => "LtEq".to_string(),
        ">=" => "GtEq".to_string(),
        "=" => "Eq".to_string(),
        "==" => "EqEq".to_string(),
        "!=" => "BangEq".to_string(),
        "!" => "Bang".to_string(),
        ":" => "Colon".to_string(),
        "::" => "ColonColon".to_string(),
        "::>" => "ColonColonGt".to_string(),
        ":>" => "ColonGt".to_string(),
        ":>>" => "ColonGtGt".to_string(),
        ";" => "Semi".to_string(),
        "," => "Comma".to_string(),
        "." => "Dot".to_string(),
        ".." => "DotDot".to_string(),
        ".?" => "DotQuestion".to_string(),
        "..." => "DotDotDot".to_string(),
        "->" => "Arrow".to_string(),
        "=>" => "FatArrow".to_string(),
        "+" => "Plus".to_string(),
        "-" => "Minus".to_string(),
        "*" => "Star".to_string(),
        "/" => "Slash".to_string(),
        "**" => "StarStar".to_string(),
        "%" => "Percent".to_string(),
        "&" => "Amp".to_string(),
        "&&" => "AmpAmp".to_string(),
        "|" => "Pipe".to_string(),
        "||" => "PipePipe".to_string(),
        "^" => "Caret".to_string(),
        "~" => "Tilde".to_string(),
        "?" => "Question".to_string(),
        "??" => "QuestionQuestion".to_string(),
        "@" => "AtSign".to_string(),
        "#" => "Hash".to_string(),
        "$" => "Dollar".to_string(),
        "\\" => "Backslash".to_string(),
        "'" => "Quote".to_string(),
        "\"" => "DoubleQuote".to_string(),
        "`" => "Backtick".to_string(),
        // Compound operators
        "+=" => "PlusEq".to_string(),
        "-=" => "MinusEq".to_string(),
        "*=" => "StarEq".to_string(),
        "/=" => "SlashEq".to_string(),
        // Default: generate from chars
        other => {
            let mut name = String::from("Punct");
            for ch in other.chars() {
                name.push_str(&format!("{:02X}", ch as u32));
            }
            name
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_to_variant() {
        assert_eq!(keyword_to_variant("package"), "Package");
        assert_eq!(keyword_to_variant("abstract"), "Abstract");
        assert_eq!(keyword_to_variant("for"), "For");
        assert_eq!(keyword_to_variant("part"), "Part");
    }

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(to_pascal_case("package"), "Package");
        assert_eq!(to_pascal_case("package_body"), "PackageBody");
        assert_eq!(to_pascal_case("packageBody"), "PackageBody");
    }

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("Package"), "package");
        assert_eq!(to_snake_case("PackageBody"), "package_body");
        assert_eq!(to_snake_case("XMLParser"), "x_m_l_parser");
    }

    #[test]
    fn test_punctuation_to_variant() {
        assert_eq!(punctuation_to_variant("{"), "LBrace");
        assert_eq!(punctuation_to_variant("::"), "ColonColon");
        assert_eq!(punctuation_to_variant(":>"), "ColonGt");
        assert_eq!(punctuation_to_variant(";"), "Semi");
    }
}
