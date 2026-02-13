//! Rule header parsing

use super::tokens;

/// A raw rule header (before body parsing)
#[derive(Debug)]
pub struct RuleHeader {
    pub name: String,
    pub return_type: Option<String>,
    pub body_text: String,
}

/// Extract rule headers from grammar source.
/// 
/// Rule headers have the format:
/// - `RuleName = ...` (no return type)
/// - `RuleName : ReturnType = ...` (with return type)
pub fn extract_rule_headers(source: &str) -> Vec<RuleHeader> {
    let mut headers = Vec::new();
    let mut lines = source.lines().peekable();
    
    while let Some(line) = lines.next() {
        let trimmed = line.trim();
        
        // Skip comments and empty lines
        if trimmed.is_empty() || trimmed.starts_with(tokens::COMMENT_START) {
            continue;
        }
        
        // Try to parse as rule header
        if let Some((name, return_type, rest)) = parse_rule_header_line(trimmed) {
            // Collect the body text (rest of this line + continuation lines)
            let mut body_text = rest.to_string();
            
            // Continue collecting until we hit another rule header or EOF
            while let Some(&next_line) = lines.peek() {
                let next_trimmed = next_line.trim();
                
                // Stop if it's a new rule header
                if is_rule_header_start(next_trimmed) {
                    break;
                }
                
                // Skip comment-only lines but include others
                if !next_trimmed.starts_with(tokens::COMMENT_START) {
                    if !body_text.is_empty() {
                        body_text.push(' ');
                    }
                    body_text.push_str(next_trimmed);
                }
                
                lines.next();
            }
            
            headers.push(RuleHeader {
                name,
                return_type,
                body_text: body_text.trim().to_string(),
            });
        }
    }
    
    headers
}

/// Try to parse a line as a rule header.
/// Returns (name, return_type, rest_of_line) if successful.
fn parse_rule_header_line(line: &str) -> Option<(String, Option<String>, &str)> {
    // Must start with an uppercase letter (rule names are PascalCase)
    let first_char = line.chars().next()?;
    if !first_char.is_ascii_uppercase() {
        return None;
    }
    
    // Find the '=' that marks the rule definition
    let eq_pos = line.find(tokens::EQUALS)?;
    
    // Everything before '=' is the header part
    let header_part = line[..eq_pos].trim();
    let rest = line[eq_pos + 1..].trim();
    
    // Check if there's a return type: "Name : Type"
    if let Some(colon_pos) = header_part.find(tokens::COLON) {
        let name = header_part[..colon_pos].trim().to_string();
        let return_type = header_part[colon_pos + 1..].trim().to_string();
        
        // Validate name and return type are identifiers
        if is_identifier(&name) && is_identifier(&return_type) {
            return Some((name, Some(return_type), rest));
        }
    } else {
        // No return type: just "Name"
        let name = header_part.to_string();
        if is_identifier(&name) {
            return Some((name, None, rest));
        }
    }
    
    None
}

/// Check if a line looks like the start of a rule header
fn is_rule_header_start(line: &str) -> bool {
    let first_char = match line.chars().next() {
        Some(c) => c,
        None => return false,
    };
    
    if !first_char.is_ascii_uppercase() {
        return false;
    }
    
    // Check for pattern: Identifier (: Identifier)? =
    if let Some(eq_pos) = line.find(tokens::EQUALS) {
        let before_eq = line[..eq_pos].trim();
        if let Some(colon_pos) = before_eq.find(':') {
            let name = before_eq[..colon_pos].trim();
            let ret_type = before_eq[colon_pos + 1..].trim();
            return is_identifier(name) && is_identifier(ret_type);
        } else {
            return is_identifier(before_eq);
        }
    }
    
    false
}

/// Check if a string is a valid identifier (PascalCase or UPPER_CASE)
fn is_identifier(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_rule_headers() {
        let source = r#"
// Comment
Package =
    'package' Identification PackageBody

PackageBody : Package =
    ';' | '{' PackageBodyElement* '}'

PartDefinition =
    OccurrenceDefinitionPrefix 'part' 'def' Definition
"#;
        let headers = extract_rule_headers(source);
        
        assert_eq!(headers.len(), 3);
        
        // Package (no return type)
        assert_eq!(headers[0].name, "Package");
        assert_eq!(headers[0].return_type, None);
        assert!(headers[0].body_text.contains("'package'"));
        
        // PackageBody : Package
        assert_eq!(headers[1].name, "PackageBody");
        assert_eq!(headers[1].return_type, Some("Package".to_string()));
        assert!(headers[1].body_text.contains("';'"));
        
        // PartDefinition
        assert_eq!(headers[2].name, "PartDefinition");
        assert_eq!(headers[2].return_type, None);
        assert!(headers[2].body_text.contains("'part'"));
    }

    #[test]
    fn test_extract_rule_headers_from_real_grammar() {
        let sysml = std::fs::read_to_string("data/SysML-textual-bnf.kebnf").unwrap();
        let headers = extract_rule_headers(&sysml);
        
        // Should have many rules
        assert!(headers.len() > 100, "Expected many rules, got {}", headers.len());
        
        // Check specific rules exist
        let names: Vec<_> = headers.iter().map(|h| h.name.as_str()).collect();
        assert!(names.contains(&"Package"));
        assert!(names.contains(&"PackageBody"));
        assert!(names.contains(&"PartDefinition"));
        assert!(names.contains(&"PartUsage"));
    }
}
