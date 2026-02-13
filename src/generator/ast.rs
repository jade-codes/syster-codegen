//! AST struct generation from grammar rules
//!
//! Simple approach:
//! - Each rule gets its own struct (fields from assignments)
//! - Pure alternative rules become enums (A = B | C -> enum A { B, C })
//! - Return type annotations are ignored for structure (just documentation)
//! - Fields that can hold multiple types get union enums

use crate::kebnf::{AssignOp, Grammar, Rule, RuleBody};
use super::utils::{to_pascal_case, to_snake_case};
use std::collections::{HashMap, HashSet};

// ============================================================================
// Types
// ============================================================================

/// A field extracted from a grammar rule
#[derive(Debug, Clone)]
pub struct AstField {
    /// Field name in snake_case
    pub name: String,
    /// Original name from grammar
    pub original_name: String,
    /// The type of the field
    pub field_type: FieldType,
    /// Whether this field is optional
    pub optional: bool,
    /// Whether this is a collection (+=)
    pub is_list: bool,
}

/// The type of a field
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FieldType {
    Bool,
    Token(String),
    Node(String),
    CrossRef(String),
    /// Union of multiple types — each entry is itself a FieldType
    Union(Vec<FieldType>),
}

/// An AST node definition
#[derive(Debug)]
pub struct AstNodeDef {
    /// Node name in PascalCase
    pub name: String,
    /// Original rule name  
    pub rule_name: String,
    /// Fields from assignments
    pub fields: Vec<AstField>,
    /// Is this a pure alternatives rule?
    pub is_enum: bool,
    /// Variants (if is_enum)
    pub variants: Vec<String>,
}

// ============================================================================
// Extraction
// ============================================================================

/// Extract AST nodes from grammar
pub fn extract_ast_nodes(grammar: &Grammar) -> Vec<AstNodeDef> {
    let defined_names: HashSet<_> = grammar.rules.iter().map(|r| r.name.as_str()).collect();
    
    // Also collect rule names that are referenced but not defined (ghost rules).
    // Treat them as valid rule names so infer_type classifies them as Node, not Token.
    let referenced = collect_body_refs(grammar);
    let ghost_rules: Vec<String> = referenced.iter()
        .filter(|name| !defined_names.contains(name.as_str()) && !is_lexer_rule(name))
        .cloned()
        .collect();
    
    let mut rule_names: HashSet<&str> = defined_names;
    for name in &ghost_rules {
        rule_names.insert(name.as_str());
    }
    
    let mut nodes: Vec<AstNodeDef> = grammar.rules
        .iter()
        .filter(|r| !is_lexer_rule(&r.name))
        .map(|rule| extract_node_def(rule, &rule_names))
        .collect();
    
    // Add stub definitions for ghost rules (empty struct with just span)
    for name in &ghost_rules {
        nodes.push(AstNodeDef {
            name: to_pascal_case(name),
            rule_name: name.clone(),
            fields: Vec::new(),
            is_enum: false,
            variants: Vec::new(),
        });
    }
    
    nodes
}

fn is_lexer_rule(name: &str) -> bool {
    name.chars().all(|c| c.is_ascii_uppercase() || c == '_')
}

/// Walk all rule bodies and collect every RuleRef name.
fn collect_body_refs(grammar: &Grammar) -> HashSet<String> {
    let mut refs = HashSet::new();
    for rule in &grammar.rules {
        collect_refs(&rule.body, &mut refs);
    }
    refs
}

fn collect_refs(body: &RuleBody, refs: &mut HashSet<String>) {
    match body {
        RuleBody::RuleRef(name) => { refs.insert(name.clone()); }
        RuleBody::CrossRef(name) => { refs.insert(name.clone()); }
        RuleBody::Sequence(items) => items.iter().for_each(|i| collect_refs(i, refs)),
        RuleBody::Alternative(alts) => alts.iter().for_each(|a| collect_refs(a, refs)),
        RuleBody::Optional(inner)
        | RuleBody::ZeroOrMore(inner)
        | RuleBody::OneOrMore(inner)
        | RuleBody::Group(inner) => collect_refs(inner, refs),
        RuleBody::Assignment { value, .. } => collect_refs(value, refs),
        RuleBody::BoolAssign { value, .. } => collect_refs(value, refs),
        RuleBody::Empty | RuleBody::Keyword(_) | RuleBody::Action(_) => {}
    }
}

fn extract_node_def(rule: &Rule, rule_names: &HashSet<&str>) -> AstNodeDef {
    let mut fields = Vec::new();
    let mut variants = Vec::new();
    let mut is_enum = false;
    
    // Check if pure alternatives rule
    if let RuleBody::Alternative(alts) = &rule.body {
        let all_refs: Vec<_> = alts.iter()
            .filter_map(|alt| match alt {
                RuleBody::RuleRef(name) if rule_names.contains(name.as_str()) => Some(name.clone()),
                _ => None,
            })
            .collect();
        
        if all_refs.len() == alts.len() && !all_refs.is_empty() {
            is_enum = true;
            variants = all_refs;
        }
    }
    
    // Extract fields
    extract_fields(&rule.body, &mut fields, rule_names, false);
    let fields = deduplicate_fields(fields);
    
    AstNodeDef {
        name: to_pascal_case(&rule.name),
        rule_name: rule.name.clone(),
        fields,
        is_enum,
        variants,
    }
}

fn extract_fields(body: &RuleBody, fields: &mut Vec<AstField>, rule_names: &HashSet<&str>, in_optional: bool) {
    match body {
        RuleBody::Assignment { name, operator, value } => {
            let field_type = infer_type(value, rule_names);
            let is_list = *operator == AssignOp::AddAssign;
            fields.push(AstField {
                name: to_snake_case(name),
                original_name: name.clone(),
                field_type,
                optional: in_optional && !is_list,
                is_list,
            });
        }
        RuleBody::BoolAssign { name, .. } => {
            fields.push(AstField {
                name: to_snake_case(name),
                original_name: name.clone(),
                field_type: FieldType::Bool,
                optional: false,
                is_list: false,
            });
        }
        RuleBody::Sequence(items) => {
            for item in items {
                extract_fields(item, fields, rule_names, in_optional);
            }
        }
        RuleBody::Alternative(alts) => {
            for alt in alts {
                extract_fields(alt, fields, rule_names, true);
            }
        }
        RuleBody::Optional(inner) => {
            extract_fields(inner, fields, rule_names, true);
        }
        RuleBody::ZeroOrMore(inner) | RuleBody::OneOrMore(inner) => {
            // If inner is a bare rule ref to a parser rule without an assignment
            // wrapper, auto-create a list field so the AST captures the children.
            if let RuleBody::RuleRef(name) = inner.as_ref() {
                if !is_lexer_rule(name) && rule_names.contains(name.as_str()) {
                    fields.push(AstField {
                        name: to_snake_case(name),
                        original_name: name.clone(),
                        field_type: FieldType::Node(name.clone()),
                        optional: false,
                        is_list: true,
                    });
                }
            }
            extract_fields(inner, fields, rule_names, in_optional);
        }
        RuleBody::Group(inner) => {
            extract_fields(inner, fields, rule_names, in_optional);
        }
        _ => {}
    }
}

fn infer_type(value: &RuleBody, rule_names: &HashSet<&str>) -> FieldType {
    match value {
        RuleBody::RuleRef(name) => {
            if is_lexer_rule(name) {
                FieldType::Token(name.clone())
            } else if rule_names.contains(name.as_str()) {
                FieldType::Node(name.clone())
            } else {
                FieldType::Token(name.clone())
            }
        }
        RuleBody::CrossRef(name) => FieldType::CrossRef(name.clone()),
        RuleBody::Keyword(_) => FieldType::Bool,
        RuleBody::Group(inner) | RuleBody::Optional(inner) => infer_type(inner, rule_names),
        RuleBody::Alternative(alts) if !alts.is_empty() => {
            let types: Vec<_> = alts.iter().map(|a| infer_type(a, rule_names)).collect();
            if types.iter().all(|t| t == &types[0]) {
                return types[0].clone();
            }
            let mut members: Vec<FieldType> = types.into_iter()
                .filter(|t| matches!(t, FieldType::Node(_) | FieldType::CrossRef(_)))
                .collect();
            members.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));
            members.dedup();
            if members.len() > 1 {
                FieldType::Union(members)
            } else if members.len() == 1 {
                members.remove(0)
            } else {
                FieldType::Token("Unknown".to_string())
            }
        }
        _ => FieldType::Token("Unknown".to_string()),
    }
}

fn deduplicate_fields(fields: Vec<AstField>) -> Vec<AstField> {
    let mut seen: HashMap<String, AstField> = HashMap::new();
    let mut types_by_name: HashMap<String, Vec<FieldType>> = HashMap::new();
    
    for field in fields {
        types_by_name.entry(field.name.clone()).or_default().push(field.field_type.clone());
        if let Some(existing) = seen.get_mut(&field.name) {
            existing.optional = existing.optional || field.optional;
            existing.is_list = existing.is_list || field.is_list;
        } else {
            seen.insert(field.name.clone(), field);
        }
    }
    
    // Create unions for fields with multiple types
    for (name, types) in &types_by_name {
        let unique: HashSet<_> = types.iter().cloned().collect();
        if unique.len() > 1 {
            if let Some(field) = seen.get_mut(name) {
                // Keep only Node and CrossRef types for unions
                let mut members: Vec<FieldType> = unique.into_iter()
                    .filter(|t| matches!(t, FieldType::Node(_) | FieldType::CrossRef(_)))
                    .collect();
                members.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));
                if members.len() > 1 {
                    field.field_type = FieldType::Union(members);
                } else if members.len() == 1 {
                    field.field_type = members.remove(0);
                }
            }
        }
    }
    
    let mut result: Vec<_> = seen.into_values().collect();
    result.sort_by(|a, b| a.name.cmp(&b.name));
    result
}

// ============================================================================
// Code Generation
// ============================================================================

/// Generate AST code from nodes, using the given span import path
pub fn generate_from_nodes(nodes: &[AstNodeDef], span_import: &str) -> String {
    let mut code = String::new();
    
    code.push_str("//! Generated AST types\n//!\n");
    code.push_str("//! Do not edit manually.\n\n");
    code.push_str(&format!("use {};\n\n", span_import));
    
    code.push_str("/// Trait for AST nodes\n");
    code.push_str("pub trait AstNode {\n");
    code.push_str("    fn span(&self) -> Span;\n");
    code.push_str("}\n\n");
    
    // CrossRef types
    let crossrefs = collect_crossrefs(nodes);
    if !crossrefs.is_empty() {
        code.push_str("// Cross-references\n\n");
        for name in &crossrefs {
            code.push_str("#[derive(Debug, Clone, Default)]\n");
            code.push_str(&format!("pub struct {}Ref {{\n", to_pascal_case(name)));
            code.push_str("    pub path: Vec<String>,\n");
            code.push_str("    pub span: Span,\n");
            code.push_str("}\n\n");
        }
    }
    
    // Nodes
    code.push_str("// AST nodes\n\n");
    for node in nodes {
        code.push_str(&generate_node(node));
        code.push('\n');
    }
    
    // Top-level wrapping enum
    code.push_str(&generate_node_kind_enum(nodes));
    
    code
}

/// Generate AST for a per-language module (imports span from common)
pub fn generate_for_language(grammar: &Grammar) -> String {
    let nodes = extract_ast_nodes(grammar);
    generate_from_nodes(&nodes, "super::super::common::span::{Span, ParseError, Result}")
}

/// Generate from grammar (legacy single-grammar mode)
pub fn generate(grammar: &Grammar) -> String {
    let nodes = extract_ast_nodes(grammar);
    generate_from_nodes(&nodes, "super::span::Span")
}

/// Generate the `AstNodeKind` wrapping enum — one variant per AST node.
///
/// This allows `try_parse_rule_ast` to return a single type that carries
/// the concrete AST node for any grammar rule.
fn generate_node_kind_enum(nodes: &[AstNodeDef]) -> String {
    let mut code = String::new();
    
    code.push_str("// Top-level node wrapper\n\n");
    code.push_str("/// Wraps every AST node type so a single parse dispatch can\n");
    code.push_str("/// return the concrete result without erasing it.\n");
    code.push_str("#[derive(Debug, Clone)]\n");
    code.push_str("pub enum AstNodeKind {\n");
    for node in nodes {
        code.push_str(&format!("    {}({}),\n", node.name, if node.is_enum {
            node.name.clone()
        } else {
            format!("Box<{}>", node.name)
        }));
    }
    code.push_str("}\n\n");
    
    // impl AstNodeKind { fn span(), fn kind_name() }
    code.push_str("impl AstNodeKind {\n");
    
    // span()
    code.push_str("    /// Return the span of the contained node.\n");
    code.push_str("    pub fn span(&self) -> Span {\n");
    code.push_str("        match self {\n");
    for node in nodes {
        if node.is_enum {
            // Enum nodes don't directly have a span; delegate to variants
            // For now, just match and return a default — we'll handle below
            code.push_str(&format!("            AstNodeKind::{}(_v) => {{", node.name));
            // Enum variants each hold a Box<SubType>; pick the first variant's span
            code.push_str(" Span::default() },\n");
        } else {
            code.push_str(&format!("            AstNodeKind::{}(v) => v.span,\n", node.name));
        }
    }
    code.push_str("        }\n");
    code.push_str("    }\n\n");
    
    // kind_name()
    code.push_str("    /// Return the variant name as a string.\n");
    code.push_str("    pub fn kind_name(&self) -> &'static str {\n");
    code.push_str("        match self {\n");
    for node in nodes {
        code.push_str(&format!(
            "            AstNodeKind::{}(_) => {:?},\n",
            node.name, node.name
        ));
    }
    code.push_str("        }\n");
    code.push_str("    }\n");
    
    code.push_str("}\n\n");
    code
}

fn collect_crossrefs(nodes: &[AstNodeDef]) -> Vec<String> {
    let mut refs: HashSet<String> = HashSet::new();
    for node in nodes {
        for field in &node.fields {
            match &field.field_type {
                FieldType::CrossRef(name) => { refs.insert(name.clone()); }
                FieldType::Union(members) => {
                    for m in members {
                        if let FieldType::CrossRef(name) = m {
                            refs.insert(name.clone());
                        }
                    }
                }
                _ => {}
            }
        }
    }
    let mut result: Vec<_> = refs.into_iter().collect();
    result.sort();
    result
}

fn generate_node(node: &AstNodeDef) -> String {
    let mut code = String::new();
    
    code.push_str(&format!("/// `{}`\n", node.rule_name));
    
    if node.is_enum && !node.variants.is_empty() {
        // Enum
        code.push_str("#[derive(Debug, Clone)]\n");
        code.push_str(&format!("pub enum {} {{\n", node.name));
        for variant in &node.variants {
            let v = to_pascal_case(variant);
            code.push_str(&format!("    {}(Box<{}>),\n", v, v));
        }
        code.push_str("}\n");
    } else {
        // Union enums for fields
        for field in &node.fields {
            if let FieldType::Union(variants) = &field.field_type {
                let union_name = format!("{}{}Member", node.name, to_pascal_case(&field.original_name));
                code.push_str("#[derive(Debug, Clone)]\n");
                code.push_str(&format!("pub enum {} {{\n", union_name));
                for v in variants {
                    match v {
                        FieldType::Node(n) => {
                            let vn = to_pascal_case(n);
                            code.push_str(&format!("    {}(Box<{}>),\n", vn, vn));
                        }
                        FieldType::CrossRef(n) => {
                            let vn = to_pascal_case(n);
                            code.push_str(&format!("    {}Ref({}Ref),\n", vn, vn));
                        }
                        _ => {}
                    }
                }
                code.push_str("}\n\n");
            }
        }
        
        // Struct
        code.push_str("#[derive(Debug, Clone)]\n");
        code.push_str(&format!("pub struct {} {{\n", node.name));
        code.push_str("    pub span: Span,\n");
        for field in &node.fields {
            let rust_type = field_to_type(field, &node.name);
            code.push_str(&format!("    pub {}: {},\n", field.name, rust_type));
        }
        code.push_str("}\n");
        
        code.push_str(&format!("\nimpl AstNode for {} {{\n", node.name));
        code.push_str("    fn span(&self) -> Span { self.span }\n");
        code.push_str("}\n");
    }
    
    code
}

fn field_to_type(field: &AstField, node_name: &str) -> String {
    let base = match &field.field_type {
        FieldType::Bool => "bool".to_string(),
        FieldType::Token(name) => match name.as_str() {
            "NAME" | "UNRESTRICTED_NAME" | "STRING_VALUE" | "STRING" => "String".to_string(),
            "INTEGER_VALUE" | "INTEGER" => "i64".to_string(),
            "REAL_VALUE" | "REAL" => "f64".to_string(),
            _ => "String".to_string(),
        },
        FieldType::Node(name) => format!("Box<{}>", to_pascal_case(name)),
        FieldType::CrossRef(name) => format!("{}Ref", to_pascal_case(name)),
        FieldType::Union(_) => format!("{}{}Member", node_name, to_pascal_case(&field.original_name)),
    };
    
    if field.is_list {
        format!("Vec<{}>", base.trim_start_matches("Box<").trim_end_matches(">"))
    } else if field.optional && !matches!(field.field_type, FieldType::Bool) {
        format!("Option<{}>", base)
    } else {
        base
    }
}
