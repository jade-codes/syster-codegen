//! Parser generator that builds AST nodes
//!
//! Simple approach:
//! - Each parse method returns its own type (parse_foo() -> Foo)
//! - Pure enum rules try alternatives and wrap in enum variant
//! - Struct rules build the struct with parsed fields

use std::collections::{HashMap, HashSet};
use crate::kebnf::{AssignOp, Grammar, Rule, RuleBody};
use super::ast::{extract_ast_nodes, AstNodeDef, AstField, FieldType};
use super::utils::{to_snake_case, to_pascal_case, keyword_to_variant, punctuation_to_variant};

// ============================================================================
// Left-recursion detection
// ============================================================================

/// Find all rules that are (directly or indirectly) left-recursive,
/// and assign each to a strongly connected component (SCC) ID.
///
/// Rules in the same SCC are part of the same indirect left-recursion cycle
/// and must coordinate their seed-grow loops.
fn find_left_recursive_rules(grammar: &Grammar) -> (HashSet<String>, HashMap<String, usize>) {
    // Build first-reference graph: rule -> rules reachable at the first position
    let mut first_refs: HashMap<String, HashSet<String>> = HashMap::new();

    for rule in &grammar.rules {
        if is_lexer_rule(&rule.name) { continue; }
        let mut refs = HashSet::new();
        collect_first_parser_refs(&rule.body, &mut refs);
        first_refs.insert(rule.name.clone(), refs);
    }

    // A rule is left-recursive if it can reach itself through first-refs
    let mut lr = HashSet::new();
    for name in first_refs.keys() {
        let mut visited = HashSet::new();
        if is_reachable_via_firsts(name, name, &first_refs, &mut visited) {
            lr.insert(name.clone());
        }
    }

    // Compute SCCs among LR rules using Tarjan's algorithm
    let scc_map = compute_lr_sccs(&lr, &first_refs);

    (lr, scc_map)
}

/// Compute strongly connected components among left-recursive rules.
/// Returns a map from rule name to SCC ID.
fn compute_lr_sccs(
    lr_rules: &HashSet<String>,
    first_refs: &HashMap<String, HashSet<String>>,
) -> HashMap<String, usize> {
    // Tarjan's algorithm
    let mut index_counter = 0usize;
    let mut stack: Vec<String> = Vec::new();
    let mut on_stack: HashSet<String> = HashSet::new();
    let mut indices: HashMap<String, usize> = HashMap::new();
    let mut lowlinks: HashMap<String, usize> = HashMap::new();
    let mut scc_map: HashMap<String, usize> = HashMap::new();
    let mut scc_id = 0usize;

    fn strongconnect(
        v: &str,
        lr_rules: &HashSet<String>,
        first_refs: &HashMap<String, HashSet<String>>,
        index_counter: &mut usize,
        stack: &mut Vec<String>,
        on_stack: &mut HashSet<String>,
        indices: &mut HashMap<String, usize>,
        lowlinks: &mut HashMap<String, usize>,
        scc_map: &mut HashMap<String, usize>,
        scc_id: &mut usize,
    ) {
        indices.insert(v.to_string(), *index_counter);
        lowlinks.insert(v.to_string(), *index_counter);
        *index_counter += 1;
        stack.push(v.to_string());
        on_stack.insert(v.to_string());

        if let Some(refs) = first_refs.get(v) {
            for w in refs {
                if !lr_rules.contains(w) { continue; }
                if !indices.contains_key(w.as_str()) {
                    strongconnect(w, lr_rules, first_refs, index_counter, stack, on_stack, indices, lowlinks, scc_map, scc_id);
                    let low_w = lowlinks[w.as_str()];
                    let low_v = lowlinks[v];
                    if low_w < low_v {
                        lowlinks.insert(v.to_string(), low_w);
                    }
                } else if on_stack.contains(w.as_str()) {
                    let idx_w = indices[w.as_str()];
                    let low_v = lowlinks[v];
                    if idx_w < low_v {
                        lowlinks.insert(v.to_string(), idx_w);
                    }
                }
            }
        }

        if lowlinks[v] == indices[v] {
            // Root of an SCC
            loop {
                let w = stack.pop().unwrap();
                on_stack.remove(&w);
                scc_map.insert(w.clone(), *scc_id);
                if w == v { break; }
            }
            *scc_id += 1;
        }
    }

    for name in lr_rules {
        if !indices.contains_key(name.as_str()) {
            strongconnect(name, lr_rules, first_refs, &mut index_counter, &mut stack, &mut on_stack, &mut indices, &mut lowlinks, &mut scc_map, &mut scc_id);
        }
    }

    scc_map
}

/// Collect all parser-rule references that appear at the "first" position
/// of the given body — i.e. that can be called before any token is consumed.
fn collect_first_parser_refs(body: &RuleBody, out: &mut HashSet<String>) {
    match body {
        RuleBody::RuleRef(name) if !is_lexer_rule(name) => { out.insert(name.clone()); }
        RuleBody::RuleRef(_) => {} // lexer rules consume a token
        RuleBody::Keyword(_) => {} // consumes a token
        RuleBody::Empty | RuleBody::Action(_) | RuleBody::CrossRef(_) => {}
        RuleBody::Sequence(items) => {
            for item in items {
                collect_first_parser_refs(item, out);
                if !can_match_empty(item) { break; }
            }
        }
        RuleBody::Alternative(alts) => {
            for alt in alts { collect_first_parser_refs(alt, out); }
        }
        RuleBody::Optional(inner) | RuleBody::ZeroOrMore(inner) => {
            collect_first_parser_refs(inner, out);
            // The outer loop in Sequence will continue past us
        }
        RuleBody::OneOrMore(inner) | RuleBody::Group(inner) => {
            collect_first_parser_refs(inner, out);
        }
        RuleBody::Assignment { value, .. } | RuleBody::BoolAssign { value, .. } => {
            collect_first_parser_refs(value, out);
        }
    }
}

/// Conservative check: can this body match the empty string?
fn can_match_empty(body: &RuleBody) -> bool {
    match body {
        RuleBody::Empty | RuleBody::Action(_) => true,
        RuleBody::Optional(_) | RuleBody::ZeroOrMore(_) => true,
        RuleBody::Keyword(_) | RuleBody::RuleRef(_) | RuleBody::CrossRef(_) => false,
        RuleBody::Sequence(items) => items.iter().all(can_match_empty),
        RuleBody::Alternative(alts) => alts.iter().any(can_match_empty),
        RuleBody::Group(inner) | RuleBody::OneOrMore(inner) => can_match_empty(inner),
        RuleBody::Assignment { value, .. } | RuleBody::BoolAssign { value, .. } => can_match_empty(value),
    }
}

/// Estimate the minimum number of mandatory tokens an alternative must consume.
/// Used to sort alternatives: more constrained (higher count) should be tried first.
fn min_required_tokens(body: &RuleBody) -> usize {
    match body {
        RuleBody::Empty | RuleBody::Action(_) => 0,
        RuleBody::Optional(_) | RuleBody::ZeroOrMore(_) => 0,
        RuleBody::Keyword(_) => 1,
        RuleBody::RuleRef(_) | RuleBody::CrossRef(_) => 1,
        RuleBody::Sequence(items) => items.iter().map(min_required_tokens).sum(),
        RuleBody::Alternative(alts) => alts.iter().map(min_required_tokens).min().unwrap_or(0),
        RuleBody::Group(inner) | RuleBody::OneOrMore(inner) => min_required_tokens(inner),
        RuleBody::Assignment { value, .. } | RuleBody::BoolAssign { value, .. } => min_required_tokens(value),
    }
}

/// Count the minimum number of mandatory keywords in a body.
/// Keywords are more discriminating than rule refs, so this breaks ties
/// between alternatives with equal min_required_tokens.
fn min_required_keywords(body: &RuleBody) -> usize {
    match body {
        RuleBody::Empty | RuleBody::Action(_) => 0,
        RuleBody::Optional(_) | RuleBody::ZeroOrMore(_) => 0,
        RuleBody::Keyword(_) => 1,
        RuleBody::RuleRef(_) | RuleBody::CrossRef(_) => 0,
        RuleBody::Sequence(items) => items.iter().map(min_required_keywords).sum(),
        RuleBody::Alternative(alts) => alts.iter().map(min_required_keywords).min().unwrap_or(0),
        RuleBody::Group(inner) | RuleBody::OneOrMore(inner) => min_required_keywords(inner),
        RuleBody::Assignment { value, .. } | RuleBody::BoolAssign { value, .. } => min_required_keywords(value),
    }
}

/// Check if an alternative's primary (mandatory) content is a cross-reference.
/// CrossRef alternatives match only a bare qualified name, while RuleRef alternatives
/// (like OwnedFeatureChain) can match longer dot-separated chains.
fn is_crossref_primary(body: &RuleBody) -> bool {
    match body {
        RuleBody::CrossRef(_) => true,
        RuleBody::Assignment { value, .. } => is_crossref_primary(value),
        RuleBody::Sequence(items) => {
            // A sequence is crossref-primary if all mandatory items are crossrefs
            // and there's at least one crossref
            let mandatory: Vec<_> = items.iter().filter(|i| !can_match_empty(i)).collect();
            !mandatory.is_empty() && mandatory.iter().all(|i| is_crossref_primary(i))
        }
        RuleBody::Group(inner) => is_crossref_primary(inner),
        _ => false,
    }
}

/// Collect all rule names that are called (transitively) from a body where the
/// rule has ambiguous alternatives (CrossRef + non-CrossRef). Used by GLR retry
/// to know which rules to add exclusions for.
fn find_ambiguous_rule_refs(body: &RuleBody, grammar: &Grammar) -> Vec<String> {
    let mut result = Vec::new();
    let mut visited = HashSet::new();
    find_ambiguous_rule_refs_inner(body, grammar, &mut result, &mut visited);
    result
}

fn find_ambiguous_rule_refs_inner(
    body: &RuleBody,
    grammar: &Grammar,
    out: &mut Vec<String>,
    visited: &mut HashSet<String>,
) {
    match body {
        RuleBody::RuleRef(name) if !is_lexer_rule(name) => {
            // Avoid infinite recursion
            if !visited.insert(name.clone()) {
                return;
            }
            
            // Check if this rule has ambiguous alternatives
            if let Some(rule) = grammar.rules.iter().find(|r| r.name == *name) {
                if let RuleBody::Alternative(alts) = &rule.body {
                    let has_xref = alts.iter().any(|a| is_crossref_primary(a));
                    let has_non_xref = alts.iter().any(|a| !is_crossref_primary(a));
                    if has_xref && has_non_xref {
                        out.push(name.clone());
                    }
                }
                // Also search transitively inside the rule's body
                find_ambiguous_rule_refs_inner(&rule.body, grammar, out, visited);
            }
        }
        RuleBody::Sequence(items) => {
            for item in items {
                find_ambiguous_rule_refs_inner(item, grammar, out, visited);
            }
        }
        RuleBody::Alternative(alts) => {
            for alt in alts {
                find_ambiguous_rule_refs_inner(alt, grammar, out, visited);
            }
        }
        RuleBody::Optional(inner)
        | RuleBody::ZeroOrMore(inner)
        | RuleBody::OneOrMore(inner)
        | RuleBody::Group(inner) => {
            find_ambiguous_rule_refs_inner(inner, grammar, out, visited);
        }
        RuleBody::Assignment { value, .. } | RuleBody::BoolAssign { value, .. } => {
            find_ambiguous_rule_refs_inner(value, grammar, out, visited);
        }
        _ => {}
    }
}


/// Is `target` reachable from `start` through the first-refs graph?
fn is_reachable_via_firsts(
    start: &str,
    target: &str,
    graph: &HashMap<String, HashSet<String>>,
    visited: &mut HashSet<String>,
) -> bool {
    if let Some(refs) = graph.get(start) {
        for r in refs {
            if r == target { return true; }
            if visited.insert(r.clone()) {
                if is_reachable_via_firsts(r, target, graph, visited) {
                    return true;
                }
            }
        }
    }
    false
}

// ============================================================================
// Public API
// ============================================================================

/// Generate parser for a specific language using its own grammar.
/// Extracts AST nodes from the grammar — no merged types needed.
pub fn generate_for_language(grammar: &Grammar, root_rules: &HashSet<&str>) -> String {
    let nodes = extract_ast_nodes(grammar);
    generate_parser(grammar, &nodes, root_rules, true)
}

/// Generate parser (legacy) — single grammar mode
pub fn generate(grammar: &Grammar, root_rules: &HashSet<&str>) -> String {
    let nodes = extract_ast_nodes(grammar);
    generate_parser(grammar, &nodes, root_rules, false)
}

// ============================================================================
// Generation
// ============================================================================

fn generate_parser(grammar: &Grammar, nodes: &[AstNodeDef], root_rules: &HashSet<&str>, use_common: bool) -> String {
    let mut code = String::new();
    
    let node_map: HashMap<String, &AstNodeDef> = nodes.iter()
        .map(|n| (n.rule_name.clone(), n))
        .collect();
    
    // Detect left-recursive rules and their SCC groupings
    let (lr_rules, scc_map) = find_left_recursive_rules(grammar);
    
    // Header
    code.push_str("//! Generated parser\n//!\n");
    code.push_str("//! Do not edit manually.\n\n");
    
    if use_common {
        code.push_str("use super::ast::*;\n");
        code.push_str("use super::super::common::span::{Span, ParseError, Result};\n");
        code.push_str("use super::tokens::{TokenKind, lookup_keyword};\n\n");
    } else {
        code.push_str("use super::ast::*;\n");
        code.push_str("use super::tokens::{TokenKind, lookup_keyword};\n\n");
    }

    code.push_str(PARSER_BOILERPLATE);
    if !use_common {
        code.push_str(SPAN_BOILERPLATE);
    }

    // Generate Parser struct with dynamic lr_* fields
    code.push_str(&generate_parser_struct(&lr_rules, &node_map, &scc_map));

    // Generate methods
    for rule in &grammar.rules {
        if is_lexer_rule(&rule.name) {
            continue;
        }
        if let Some(node_def) = node_map.get(&rule.name) {
            let is_root = root_rules.contains(rule.name.as_str());
            let is_lr = lr_rules.contains(&rule.name);
            let scc_id = scc_map.get(&rule.name).copied();
            code.push_str(&generate_method(rule, node_def, grammar, is_root, is_lr, scc_id));
        }
    }

    // Generate stub methods for referenced but undefined rules
    let defined: HashSet<String> = grammar.rules.iter()
        .filter(|r| !is_lexer_rule(&r.name))
        .map(|r| r.name.clone())
        .collect();
    let referenced = collect_referenced_rules(grammar);
    for name in &referenced {
        if !defined.contains(name) && !is_lexer_rule(name) {
            let method = format!("parse_{}", to_snake_case(name));
            let type_name = to_pascal_case(name);
            code.push_str(&format!("\n    /// Stub for undefined rule `{}`\n", name));
            code.push_str(&format!("    fn {}(&mut self) -> Result<{}> {{\n", method, type_name));
            code.push_str(&format!("        Err(ParseError {{ message: \"rule {} is not defined\".into(), span: self.current_span() }})\n", name));
            code.push_str("    }\n");
        }
    }

    // Generate dispatch method: try_parse_rule(name) -> Result<usize>
    // Allows test runner to call any parse function by rule name.
    code.push_str(&generate_dispatch_method(grammar));

    // Generate AST dispatch: try_parse_rule_ast(name) -> Result<AstNodeKind>
    code.push_str(&generate_ast_dispatch_method(grammar));

    code.push_str("}\n");
    code
}

/// Generate a `try_parse_rule` method that dispatches to parse_xxx by rule name.
/// Returns Ok(consumed_position) on success, Err on failure.
fn generate_dispatch_method(grammar: &Grammar) -> String {
    let mut code = String::new();
    code.push_str("\n    /// Dispatch to a parse function by rule name (snake_case).\n");
    code.push_str("    /// Returns the parser position after successful parsing.\n");
    code.push_str("    pub fn try_parse_rule(&mut self, rule: &str) -> Result<usize> {\n");
    code.push_str("        match rule {\n");

    for rule in &grammar.rules {
        if is_lexer_rule(&rule.name) {
            continue;
        }
        let snake = to_snake_case(&rule.name);
        code.push_str(&format!(
            "            \"{}\" => self.parse_{}().map(|_| self.pos()),\n",
            snake, snake
        ));
    }

    code.push_str("            _ => Err(ParseError { message: format!(\"unknown rule: {}\", rule), span: self.current_span() }),\n");
    code.push_str("        }\n");
    code.push_str("    }\n");
    code
}

/// Generate a `try_parse_rule_ast` method that dispatches to parse_xxx
/// and wraps the result in `AstNodeKind`.
fn generate_ast_dispatch_method(grammar: &Grammar) -> String {
    let mut code = String::new();
    code.push_str("\n    /// Dispatch to a parse function by rule name, returning the AST node.\n");
    code.push_str("    pub fn try_parse_rule_ast(&mut self, rule: &str) -> Result<AstNodeKind> {\n");
    code.push_str("        match rule {\n");

    for rule in &grammar.rules {
        if is_lexer_rule(&rule.name) {
            continue;
        }
        let snake = to_snake_case(&rule.name);
        let pascal = to_pascal_case(&rule.name);
        // Check if this is an enum node (pure alternatives of rule refs).
        // Enum nodes are not boxed in AstNodeKind; struct nodes are.
        let is_enum = is_pure_alt_rule(rule, grammar);
        let wrap = if is_enum {
            format!("AstNodeKind::{pascal}(v)")
        } else {
            format!("AstNodeKind::{pascal}(Box::new(v))")
        };
        code.push_str(&format!(
            "            \"{snake}\" => self.parse_{snake}().map(|v| {wrap}),\n"
        ));
    }

    code.push_str("            _ => Err(ParseError { message: format!(\"unknown rule: {}\", rule), span: self.current_span() }),\n");
    code.push_str("        }\n");
    code.push_str("    }\n");
    code
}

/// Check whether a rule is a pure-alternatives rule (enum in AST).
fn is_pure_alt_rule(rule: &crate::kebnf::types::Rule, grammar: &Grammar) -> bool {
    let rule_names: HashSet<&str> = grammar.rules.iter().map(|r| r.name.as_str()).collect();
    if let RuleBody::Alternative(alts) = &rule.body {
        alts.iter().all(|alt| matches!(alt, RuleBody::RuleRef(name) if rule_names.contains(name.as_str())))
    } else {
        false
    }
}

/// Walk all rule bodies and collect every RuleRef name.
fn collect_referenced_rules(grammar: &Grammar) -> HashSet<String> {
    let mut refs = HashSet::new();
    for rule in &grammar.rules {
        collect_refs_from_body(&rule.body, &mut refs);
    }
    refs
}

fn collect_refs_from_body(body: &RuleBody, refs: &mut HashSet<String>) {
    match body {
        RuleBody::RuleRef(name) => { refs.insert(name.clone()); }
        RuleBody::CrossRef(name) => { refs.insert(name.clone()); }
        RuleBody::Sequence(items) => items.iter().for_each(|i| collect_refs_from_body(i, refs)),
        RuleBody::Alternative(alts) => alts.iter().for_each(|a| collect_refs_from_body(a, refs)),
        RuleBody::Optional(inner)
        | RuleBody::ZeroOrMore(inner)
        | RuleBody::OneOrMore(inner)
        | RuleBody::Group(inner) => collect_refs_from_body(inner, refs),
        RuleBody::Assignment { value, .. } => collect_refs_from_body(value, refs),
        RuleBody::BoolAssign { value, .. } => collect_refs_from_body(value, refs),
        RuleBody::Empty | RuleBody::Keyword(_) | RuleBody::Action(_) => {}
    }
}

fn is_lexer_rule(name: &str) -> bool {
    name.chars().all(|c| c.is_ascii_uppercase() || c == '_')
}

/// Map well-known lexer rule names to a single TokenKind variant.
/// Returns "Name|UnrestrictedName" for NAME to indicate it accepts both.
fn lexer_rule_to_token_kind(name: &str) -> Option<&'static str> {
    match name {
        "NAME" => Some("Name|UnrestrictedName"),
        "BASIC_NAME" => Some("Name"),
        "UNRESTRICTED_NAME" => Some("UnrestrictedName"),
        "STRING_VALUE" => Some("String"),
        "INTEGER_VALUE" | "DECIMAL_VALUE" => Some("Integer"),
        "REAL_VALUE" | "EXPONENTIAL_VALUE" => Some("Real"),
        "REGULAR_COMMENT" => Some("BlockComment"),
        _ => None,
    }
}

/// Generate inline code that expects a lexer rule's tokens.
/// For simple tokens (NAME, STRING_VALUE, etc.) → self.expect(TokenKind::X)
/// For operator aliases (SPECIALIZES = ':>' | 'specializes') → inline alternatives
/// Returns None if we can't generate proper code (falls back to expect_any).
fn generate_lexer_rule_expect(name: &str, grammar: &Grammar, indent: usize) -> Option<String> {
    let ind = "    ".repeat(indent);

    // Check static mapping first
    if let Some(token_kind) = lexer_rule_to_token_kind(name) {
        if token_kind.contains('|') {
            // Multi-token: NAME accepts Name or UnrestrictedName only (not keywords)
            let mut code = String::new();
            code.push_str(&format!("{}match self.current() {{\n", ind));
            code.push_str(&format!("{}    Some(t) if t.kind.is_name_token() => {{ self.pos += 1; }}\n", ind));
            code.push_str(&format!("{}    Some(t) => return Err(ParseError {{ message: format!(\"expected name, got {{:?}}\", t.kind), span: t.span }}),\n", ind));
            code.push_str(&format!("{}    None => return Err(ParseError {{ message: \"expected name, got EOF\".into(), span: Span::default() }}),\n", ind));
            code.push_str(&format!("{}}}\n", ind));
            return Some(code);
        }
        return Some(format!("{}self.expect(TokenKind::{})?;\n", ind, token_kind));
    }

    // Look up the rule definition in the grammar for operator aliases
    let rule = grammar.rules.iter().find(|r| r.name == name)?;
    match &rule.body {
        // Single keyword: SOME_RULE = 'keyword'
        RuleBody::Keyword(kw) => {
            let variant = if grammar.keywords.contains(kw) {
                keyword_to_variant(kw)
            } else {
                punctuation_to_variant(kw)
            };
            Some(format!("{}self.expect(TokenKind::{})?;\n", ind, variant))
        }
        // Alternatives of keywords/sequences: SPECIALIZES = ':>' | 'specializes'
        RuleBody::Alternative(alts) => {
            let mut code = String::new();
            code.push_str(&format!("{}let saved_lex = self.save();\n", ind));
            for (i, alt) in alts.iter().enumerate() {
                if i == 0 {
                    code.push_str(&format!("{}(|| -> std::result::Result<(), ParseError> {{\n", ind));
                    code.push_str(&generate_lexer_alt_body(alt, grammar, indent + 1));
                    code.push_str(&format!("{}    Ok(())\n", ind));
                    code.push_str(&format!("{}}})()", ind));
                } else {
                    code.push_str(&format!(".or_else(|_: ParseError| {{\n"));
                    code.push_str(&format!("{}    self.restore(saved_lex);\n", ind));
                    code.push_str(&generate_lexer_alt_body(alt, grammar, indent + 1));
                    code.push_str(&format!("{}    Ok(())\n", ind));
                    code.push_str(&format!("{}}})", ind));
                }
            }
            code.push_str("?;\n");
            Some(code)
        }
        _ => None,
    }
}

/// Generate expect code for a single alternative branch of a lexer rule.
fn generate_lexer_alt_body(body: &RuleBody, grammar: &Grammar, indent: usize) -> String {
    let ind = "    ".repeat(indent);
    match body {
        RuleBody::Keyword(kw) => {
            let variant = if grammar.keywords.contains(kw) {
                keyword_to_variant(kw)
            } else {
                punctuation_to_variant(kw)
            };
            format!("{}self.expect(TokenKind::{})?;\n", ind, variant)
        }
        RuleBody::Sequence(items) => {
            items.iter().map(|i| generate_lexer_alt_body(i, grammar, indent)).collect()
        }
        _ => String::new(),
    }
}

fn generate_method(rule: &Rule, node: &AstNodeDef, grammar: &Grammar, is_root: bool, is_lr: bool, scc_id: Option<usize>) -> String {
    let method = format!("parse_{}", to_snake_case(&rule.name));
    let return_type = &node.name;
    
    let mut code = String::new();

    if is_lr {
        let scc = scc_id.unwrap_or(0);
        let lr_head_field = format!("lr_head_{}", scc);
        // ----- Left-recursive rule: body helper + seed-grow wrapper -----
        // 1) Body helper (no visiting guard)
        code.push_str(&format!("\n    /// Parse body of `{}` (left-recursive helper)\n", rule.name));
        code.push_str(&format!("    fn {}_body(&mut self) -> Result<{}> {{\n", method, return_type));

        if node.is_enum && !node.variants.is_empty() {
            code.push_str(&generate_enum_body(node));
        } else {
            code.push_str(&generate_struct_body(rule, node, grammar));
        }

        code.push_str("    }\n");

        // 2) Public wrapper with seed-grow (only head runs grow loop)
        let lr_field = format!("lr_{}", to_snake_case(&rule.name));
        code.push_str(&format!("\n    /// Parse `{}` (left-recursive, seed-grow)\n", rule.name));
        if is_root {
            code.push_str("    /// Entry point\n");
        }
        code.push_str(&format!("    pub fn {}(&mut self) -> Result<{}> {{\n", method, return_type));
        code.push_str("        let _entry_pos = self.pos;\n");

        // Re-entry check
        code.push_str(&format!("\n        // Left-recursive re-entry: return memo if available\n"));
        code.push_str(&format!("        if self.visiting.contains(&(self.pos, \"{}\")) {{\n", rule.name));
        code.push_str(&format!("            if let Some((start, end, ref result)) = self.{} {{\n", lr_field));
        code.push_str("                if start == _entry_pos {\n");
        code.push_str("                    self.pos = end;\n");
        code.push_str("                    return Ok(result.clone());\n");
        code.push_str("                }\n");
        code.push_str("            }\n");
        code.push_str(&format!("            return Err(ParseError {{ message: \"left-recursive entry into {}\".into(), span: self.current_span() }});\n", rule.name));
        code.push_str("        }\n");

        // Head check: only the first LR rule at a position runs seed-grow
        code.push_str(&format!("\n        let is_head = self.{} != Some(_entry_pos);\n", lr_head_field));
        code.push_str(&format!("        self.visiting.insert((_entry_pos, \"{}\"));\n", rule.name));
        code.push_str(&format!("        self.push_rule_context(\"{}\", _entry_pos);\n", rule.name));

        code.push_str("\n        if is_head {\n");
        code.push_str("            // We are the LR head at this position\n");
        code.push_str(&format!("            let prev_head_pos = self.{};\n", lr_head_field));
        code.push_str(&format!("            self.{} = Some(_entry_pos);\n", lr_head_field));

        code.push_str(&format!("\n            // Seed phase\n"));
        code.push_str(&format!("            let seed = self.{}_body();\n", method));
        code.push_str("\n            if let Ok(seed_val) = seed {\n");
        code.push_str("                let mut best = seed_val;\n");
        code.push_str("                let mut best_pos = self.pos;\n");
        code.push_str("\n                // Grow loop\n");
        code.push_str("                loop {\n");
        code.push_str(&format!("                    self.{} = Some((_entry_pos, best_pos, best.clone()));\n", lr_field));
        code.push_str("                    self.pos = _entry_pos;\n");
        code.push_str(&format!("                    match self.{}_body() {{\n", method));
        code.push_str("                        Ok(grown) if self.pos > best_pos => {\n");
        code.push_str("                            best = grown;\n");
        code.push_str("                            best_pos = self.pos;\n");
        code.push_str("                        }\n");
        code.push_str("                        _ => {\n");
        code.push_str("                            self.pos = best_pos;\n");
        code.push_str("                            break;\n");
        code.push_str("                        }\n");
        code.push_str("                    }\n");
        code.push_str("                }\n");
        code.push_str(&format!("\n                self.{} = None;\n", lr_field));
        code.push_str(&format!("                self.pop_rule_context();\n"));
        code.push_str(&format!("                self.visiting.remove(&(_entry_pos, \"{}\"));\n", rule.name));
        code.push_str(&format!("                self.{} = prev_head_pos;\n", lr_head_field));
        code.push_str("                Ok(best)\n");
        code.push_str("            } else {\n");
        code.push_str(&format!("                self.pop_rule_context();\n"));
        code.push_str(&format!("                self.visiting.remove(&(_entry_pos, \"{}\"));\n", rule.name));
        code.push_str(&format!("                self.{} = prev_head_pos;\n", lr_head_field));
        code.push_str("                seed\n");
        code.push_str("            }\n");

        code.push_str("        } else {\n");
        code.push_str("            // Another LR rule is head at this position — just parse (no grow loop)\n");
        code.push_str(&format!("            let result = self.{}_body();\n", method));
        code.push_str(&format!("            self.pop_rule_context();\n"));
        code.push_str(&format!("            self.visiting.remove(&(_entry_pos, \"{}\"));\n", rule.name));
        code.push_str("            result\n");
        code.push_str("        }\n");
        code.push_str("    }\n");
    } else {
        // ----- Normal (non-left-recursive) rule -----
        code.push_str(&format!("\n    /// Parse `{}`\n", rule.name));
        if is_root {
            code.push_str("    /// Entry point\n");
        }
        code.push_str(&format!("    pub fn {}(&mut self) -> Result<{}> {{\n", method, return_type));
        code.push_str("        let _entry_pos = self.pos;\n");
        code.push_str(&format!("        if !self.enter_rule(\"{}\") {{\n", rule.name));
        code.push_str(&format!("            return Err(ParseError {{ message: \"left-recursive entry into {}\".into(), span: self.current_span() }});\n", rule.name));
        code.push_str("        }\n");
        code.push_str(&format!("        self.push_rule_context(\"{}\", _entry_pos);\n", rule.name));
        code.push_str(&format!("        let _result: Result<{}> = (|| {{\n", return_type));
        
        if node.is_enum && !node.variants.is_empty() {
            code.push_str(&generate_enum_body(node));
        } else {
            code.push_str(&generate_struct_body(rule, node, grammar));
        }
        
        code.push_str("        })();\n");
        code.push_str(&format!("        self.pop_rule_context();\n"));
        code.push_str(&format!("        self.leave_rule(_entry_pos, \"{}\");\n", rule.name));
        code.push_str("        _result\n");
        code.push_str("    }\n");
    }
    code
}

fn generate_enum_body(node: &AstNodeDef) -> String {
    let mut code = String::new();
    let type_name = &node.name;

    // Use longest-match strategy: try ALL alternatives, pick the one that
    // consumed the most input. This prevents short-prefix alternatives
    // (e.g. FeatureReferenceExpression matching just a name) from shadowing
    // longer alternatives (e.g. InvocationExpression matching name + args).
    //
    // We store each successful result wrapped in the enum type, and keep
    // the one that consumed the most tokens.
    code.push_str("        let alt_saved = self.save();\n");
    code.push_str(&format!("        let mut best: Option<({}, usize)> = None;\n", type_name));

    for variant in &node.variants {
        let variant_name = to_pascal_case(variant);
        let method = format!("parse_{}", to_snake_case(variant));
        code.push_str(&format!("\n        self.restore(alt_saved);\n"));
        code.push_str(&format!("        if let Ok(v) = self.{}() {{\n", method));
        code.push_str("            let end = self.pos;\n");
        code.push_str("            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {\n");
        code.push_str(&format!("                best = Some(({}::{}(Box::new(v)), end));\n", type_name, variant_name));
        code.push_str("            }\n");
        code.push_str("        }\n");
    }

    // Restore to the winning position and return
    code.push_str("\n        match best {\n");
    code.push_str("            Some((result, end_pos)) => {\n");
    code.push_str("                self.pos = end_pos;\n");
    code.push_str("                Ok(result)\n");
    code.push_str("            }\n");
    code.push_str(&format!("            None => Err(ParseError {{ message: \"expected {}\".into(), span: self.current_span() }})\n", node.rule_name));
    code.push_str("        }\n");
    code
}

fn generate_struct_body(rule: &Rule, node: &AstNodeDef, grammar: &Grammar) -> String {
    let mut code = String::new();
    
    code.push_str("        let start = self.current_span();\n");
    
    // Init fields - use Option for required node types
    for field in &node.fields {
        let (init, use_option) = if field.is_list {
            ("Vec::new()".to_string(), false)
        } else if matches!(field.field_type, FieldType::Bool) {
            ("false".to_string(), false)
        } else if field.optional {
            ("None".to_string(), false)
        } else if matches!(field.field_type, FieldType::Token(_)) {
            ("String::new()".to_string(), false)
        } else {
            // Required non-primitive field - use Option and unwrap later
            ("None".to_string(), true)
        };
        
        if use_option {
            code.push_str(&format!("        let mut {}_opt: Option<_> = {};\n", field.name, init));
        } else {
            code.push_str(&format!("        let mut {} = {};\n", field.name, init));
        }
    }
    
    // Parse body
    code.push_str(&parse_body(&rule.body, node, grammar, 2));
    
    // Build struct - unwrap Option fields
    code.push_str("\n        let end = self.current_span();\n");
    code.push_str(&format!("        Ok({} {{\n", node.name));
    code.push_str("            span: start.merge(end),\n");
    for field in &node.fields {
        let needs_unwrap = !field.is_list && !field.optional && 
            !matches!(field.field_type, FieldType::Bool | FieldType::Token(_));
        if needs_unwrap {
            code.push_str(&format!("            {}: {}_opt.ok_or_else(|| ParseError {{ message: \"missing {}\".into(), span: start }})?,\n", 
                field.name, field.name, field.name));
        } else {
            code.push_str(&format!("            {},\n", field.name));
        }
    }
    code.push_str("        })\n");
    
    code
}

/// Generate code for a Sequence, with lookahead guards on greedy repetitions.
///
/// When a `OneOrMore` or `ZeroOrMore` item is followed by remaining items
/// that could match the same pattern as the loop body, the loop must check
/// whether the remainder can parse before consuming another iteration.
/// Without this, `(A)+ A` would have the greedy `+` eat everything, leaving
/// nothing for the trailing `A`.
fn generate_sequence(items: &[RuleBody], node: &AstNodeDef, grammar: &Grammar, indent: usize) -> String {
    let mut code = String::new();
    let ind = "    ".repeat(indent);

    for (i, item) in items.iter().enumerate() {
        let remainder = &items[i + 1..];

        match item {
            RuleBody::OneOrMore(inner) if !remainder.is_empty() && could_match_same(inner, remainder) => {
                let collect_field = match inner.as_ref() {
                    RuleBody::RuleRef(name) if !is_lexer_rule(name) => Some(to_snake_case(name)),
                    _ => None,
                };
                // First iteration required
                if let (Some(ref field_name), RuleBody::RuleRef(name)) = (&collect_field, inner.as_ref()) {
                    let method = format!("parse_{}", to_snake_case(name));
                    code.push_str(&format!("{}let v = self.{}()?;\n", ind, method));
                    code.push_str(&format!("{}{}.push(v);\n", ind, field_name));
                } else {
                    code.push_str(&parse_body(inner, node, grammar, indent));
                }
                // Guarded loop: only consume another iteration if remainder can still parse afterward.
                // This prevents greedy over-consumption when loop body and remainder overlap.
                code.push_str(&format!("{}loop {{\n", ind));
                code.push_str(&format!("{}    let saved = self.save();\n", ind));
                // Try another iteration of the loop body
                code.push_str(&format!("{}    let body_ok: std::result::Result<(), ParseError> = (|| {{\n", ind));
                if let (Some(ref field_name), RuleBody::RuleRef(name)) = (&collect_field, inner.as_ref()) {
                    let method = format!("parse_{}", to_snake_case(name));
                    code.push_str(&format!("{}        let v = self.{}()?;\n", ind, method));
                    code.push_str(&format!("{}        {}.push(v);\n", ind, field_name));
                } else {
                    code.push_str(&parse_body(inner, node, grammar, indent + 2));
                }
                code.push_str(&format!("{}        Ok(())\n", ind));
                code.push_str(&format!("{}    }})();\n", ind));
                code.push_str(&format!("{}    if body_ok.is_err() {{\n", ind));
                code.push_str(&format!("{}        self.restore(saved);\n", ind));
                code.push_str(&format!("{}        break; // Loop body failed, exit\n", ind));
                code.push_str(&format!("{}    }}\n", ind));
                code.push_str(&format!("{}    if self.save() == saved {{ break; }} // No progress, exit\n", ind));
                code.push_str(&format!("{}    // Loop body succeeded - check if remainder can still parse\n", ind));
                code.push_str(&format!("{}    let pos_after_body = self.save();\n", ind));
                code.push_str(&format!("{}    let remainder_ok = (|| -> std::result::Result<(), ParseError> {{\n", ind));
                code.push_str(&generate_probe(remainder, grammar, indent + 2));
                code.push_str(&format!("{}        Ok(())\n", ind));
                code.push_str(&format!("{}    }})().is_ok();\n", ind));
                code.push_str(&format!("{}    self.restore(pos_after_body);\n", ind));
                code.push_str(&format!("{}    if !remainder_ok {{\n", ind));
                code.push_str(&format!("{}        // Remainder can't parse after consuming this iteration\n", ind));
                code.push_str(&format!("{}        // Backtrack and leave input for remainder\n", ind));
                // Need to undo the field push if we collected
                if collect_field.is_some() {
                    code.push_str(&format!("{}        {}.pop();\n", ind, collect_field.as_ref().unwrap()));
                }
                code.push_str(&format!("{}        self.restore(saved);\n", ind));
                code.push_str(&format!("{}        break;\n", ind));
                code.push_str(&format!("{}    }}\n", ind));
                code.push_str(&format!("{}    // Remainder can parse, keep going\n", ind));
                code.push_str(&format!("{}}}\n", ind));
            }
            RuleBody::ZeroOrMore(inner) if !remainder.is_empty() && could_match_same(inner, remainder) => {
                let collect_field = match inner.as_ref() {
                    RuleBody::RuleRef(name) if !is_lexer_rule(name) => Some(to_snake_case(name)),
                    _ => None,
                };
                // Guarded loop: only consume another iteration if remainder can still parse afterward.
                // This prevents greedy over-consumption when loop body and remainder overlap.
                code.push_str(&format!("{}loop {{\n", ind));
                code.push_str(&format!("{}    let saved = self.save();\n", ind));
                // Try another iteration of the loop body
                code.push_str(&format!("{}    let body_ok: std::result::Result<(), ParseError> = (|| {{\n", ind));
                if let (Some(ref field_name), RuleBody::RuleRef(name)) = (&collect_field, inner.as_ref()) {
                    let method = format!("parse_{}", to_snake_case(name));
                    code.push_str(&format!("{}        let v = self.{}()?;\n", ind, method));
                    code.push_str(&format!("{}        {}.push(v);\n", ind, field_name));
                } else {
                    code.push_str(&parse_body(inner, node, grammar, indent + 2));
                }
                code.push_str(&format!("{}        Ok(())\n", ind));
                code.push_str(&format!("{}    }})();\n", ind));
                code.push_str(&format!("{}    if body_ok.is_err() {{\n", ind));
                code.push_str(&format!("{}        self.restore(saved);\n", ind));
                code.push_str(&format!("{}        break; // Loop body failed, exit\n", ind));
                code.push_str(&format!("{}    }}\n", ind));
                code.push_str(&format!("{}    if self.save() == saved {{ break; }} // No progress, exit\n", ind));
                code.push_str(&format!("{}    // Loop body succeeded - check if remainder can still parse\n", ind));
                code.push_str(&format!("{}    let pos_after_body = self.save();\n", ind));
                code.push_str(&format!("{}    let remainder_ok = (|| -> std::result::Result<(), ParseError> {{\n", ind));
                code.push_str(&generate_probe(remainder, grammar, indent + 2));
                code.push_str(&format!("{}        Ok(())\n", ind));
                code.push_str(&format!("{}    }})().is_ok();\n", ind));
                code.push_str(&format!("{}    self.restore(pos_after_body);\n", ind));
                code.push_str(&format!("{}    if !remainder_ok {{\n", ind));
                code.push_str(&format!("{}        // Remainder can't parse after consuming this iteration\n", ind));
                code.push_str(&format!("{}        // Backtrack and leave input for remainder\n", ind));
                // Need to undo the field push if we collected
                if collect_field.is_some() {
                    code.push_str(&format!("{}        {}.pop();\n", ind, collect_field.as_ref().unwrap()));
                }
                code.push_str(&format!("{}        self.restore(saved);\n", ind));
                code.push_str(&format!("{}        break;\n", ind));
                code.push_str(&format!("{}    }}\n", ind));
                code.push_str(&format!("{}    // Remainder can parse, keep going\n", ind));
                code.push_str(&format!("{}}}\n", ind));
            }
            RuleBody::Optional(inner) if !remainder.is_empty() && could_match_same_with_grammar(inner, remainder, Some(grammar)) => {
                // The optional could consume tokens the remainder needs.
                // GLR-style: try optional, if it fails OR remainder fails, exclude
                // the longest ambiguous parse and retry with shorter alternatives.
                
                // Find ambiguous rules inside this optional for exclusion tracking
                let ambiguous_rules = find_ambiguous_rule_refs(inner, grammar);
                
                code.push_str(&format!("{}let saved_opt = self.save();\n", ind));
                code.push_str(&format!("{}let mut opt_succeeded = false;\n", ind));
                code.push_str(&format!("{}let mut glr_attempts = 0;\n", ind));
                code.push_str(&format!("{}const MAX_GLR_ATTEMPTS: usize = 10;\n", ind));
                code.push_str(&format!("{}let mut last_pos_before_opt = saved_opt;\n", ind));
                code.push_str(&format!("{}loop {{\n", ind));
                code.push_str(&format!("{}    if glr_attempts >= MAX_GLR_ATTEMPTS {{ break; }}\n", ind));
                code.push_str(&format!("{}    glr_attempts += 1;\n", ind));
                code.push_str(&format!("{}    self.restore(saved_opt);\n", ind));
                code.push_str(&format!("{}    let pre_opt_pos = self.pos;\n", ind));
                code.push_str(&format!("{}    let opt_ok: std::result::Result<(), ParseError> = (|| {{\n", ind));
                code.push_str(&parse_body(inner, node, grammar, indent + 2));
                code.push_str(&format!("{}        Ok(())\n", ind));
                code.push_str(&format!("{}    }})();\n", ind));
                code.push_str(&format!("{}    let post_opt_pos = self.pos;\n", ind));
                code.push_str(&format!("{}    if opt_ok.is_err() {{\n", ind));
                code.push_str(&format!("{}        // Optional parsing failed — if we progressed, exclude and retry\n", ind));
                code.push_str(&format!("{}        if post_opt_pos > pre_opt_pos && post_opt_pos != last_pos_before_opt {{\n", ind));
                code.push_str(&format!("{}            last_pos_before_opt = post_opt_pos;\n", ind));
                // Add exclusions for ambiguous rules — they must have stopped somewhere
                for rule_name in &ambiguous_rules {
                    code.push_str(&format!("{}            self.exclude_parse(saved_opt, \"{}\", post_opt_pos);\n", ind, rule_name));
                }
                code.push_str(&format!("{}            continue; // Retry with shorter alternative\n", ind));
                code.push_str(&format!("{}        }}\n", ind));
                code.push_str(&format!("{}        self.restore(saved_opt);\n", ind));
                code.push_str(&format!("{}        break; // No more alternatives\n", ind));
                code.push_str(&format!("{}    }}\n", ind));
                code.push_str(&format!("{}    let opt_end = self.pos;\n", ind));
                code.push_str(&format!("{}    // Probe if remainder can parse from here\n", ind));
                code.push_str(&format!("{}    let rem_ok: std::result::Result<(), ParseError> = (|| {{\n", ind));
                code.push_str(&generate_probe(remainder, grammar, indent + 2));
                code.push_str(&format!("{}        Ok(())\n", ind));
                code.push_str(&format!("{}    }})();\n", ind));
                code.push_str(&format!("{}    self.pos = opt_end; // Restore after probe\n", ind));
                code.push_str(&format!("{}    if rem_ok.is_ok() {{\n", ind));
                code.push_str(&format!("{}        opt_succeeded = true;\n", ind));
                code.push_str(&format!("{}        break;\n", ind));
                code.push_str(&format!("{}    }}\n", ind));
                code.push_str(&format!("{}    // Remainder failed — exclude this parse result and retry\n", ind));
                // Add exclusions for all ambiguous rules at their respective positions
                for rule_name in &ambiguous_rules {
                    code.push_str(&format!("{}    self.exclude_parse(saved_opt, \"{}\", opt_end);\n", ind, rule_name));
                }
                code.push_str(&format!("{}}}\n", ind));
                code.push_str(&format!("{}if !opt_succeeded {{\n", ind));
                code.push_str(&format!("{}    self.restore(saved_opt);\n", ind));
                code.push_str(&format!("{}}}\n", ind));
            }
            _ => {
                code.push_str(&parse_body(item, node, grammar, indent));
            }
        }
    }
    code
}

/// Check if a repetition body could match the same first tokens as the remainder.
/// Used to decide whether a greedy loop needs a lookahead guard.
///
/// Only returns `true` when there is concrete evidence of overlap — either
/// matching keyword/terminal tokens or matching parser rule references.
/// This avoids false positives that would add unnecessary guards and break parsing.
fn could_match_same(loop_body: &RuleBody, remainder: &[RuleBody]) -> bool {
    could_match_same_with_grammar(loop_body, remainder, None)
}

fn could_match_same_with_grammar(loop_body: &RuleBody, remainder: &[RuleBody], grammar: Option<&Grammar>) -> bool {
    // Check terminal (keyword/punctuation) overlap
    let loop_terms = first_terminals(loop_body);
    let rem_terms = first_terminals_of_seq(remainder);
    if !loop_terms.is_empty() && !rem_terms.is_empty() {
        if loop_terms.iter().any(|t| rem_terms.contains(t)) {
            return true;
        }
    }

    // Check parser rule ref overlap — only flag when the SAME rule name
    // can appear at the start of both the loop body and the remainder
    let loop_refs = first_rule_refs(loop_body);
    let rem_refs = first_rule_refs_of_seq(remainder);
    if !loop_refs.is_empty() && !rem_refs.is_empty() {
        if loop_refs.iter().any(|r| rem_refs.contains(r)) {
            return true;
        }
    }

    // If we have the grammar, transitively expand rule refs to find
    // overlap (e.g. both rules eventually start with [QualifiedName])
    if let Some(grammar) = grammar {
        let rule_map: HashMap<&str, &RuleBody> = grammar.rules.iter()
            .map(|r| (r.name.as_str(), &r.body))
            .collect();

        let expand = |initial: &HashSet<String>| -> HashSet<String> {
            let mut all = initial.clone();
            let mut frontier: Vec<String> = initial.iter().cloned().collect();
            while let Some(name) = frontier.pop() {
                if let Some(body) = rule_map.get(name.as_str()) {
                    for r in first_rule_refs(body) {
                        if all.insert(r.clone()) {
                            frontier.push(r);
                        }
                    }
                }
            }
            all
        };

        let expanded_loop = expand(&loop_refs);
        let expanded_rem = expand(&rem_refs);
        if expanded_loop.iter().any(|r| expanded_rem.contains(r)) {
            return true;
        }
    }

    false
}

/// Collect the set of parser rule names that could appear at the start of a body.
/// Only includes non-lexer rule refs (parser rules).
fn first_rule_refs(body: &RuleBody) -> HashSet<String> {
    let mut out = HashSet::new();
    match body {
        RuleBody::RuleRef(name) if !is_lexer_rule(name) => { out.insert(name.clone()); }
        RuleBody::CrossRef(name) => { out.insert(name.clone()); }
        RuleBody::Sequence(items) => {
            for item in items {
                out.extend(first_rule_refs(item));
                if !can_match_empty(item) { break; }
            }
        }
        RuleBody::Alternative(alts) => {
            for alt in alts {
                out.extend(first_rule_refs(alt));
            }
        }
        RuleBody::Group(inner) | RuleBody::OneOrMore(inner) => {
            out.extend(first_rule_refs(inner));
        }
        RuleBody::Optional(inner) | RuleBody::ZeroOrMore(inner) => {
            out.extend(first_rule_refs(inner));
        }
        RuleBody::Assignment { value, .. } | RuleBody::BoolAssign { value, .. } => {
            out.extend(first_rule_refs(value));
        }
        _ => {}
    }
    out
}

/// First parser rule refs of a sequence of items.
fn first_rule_refs_of_seq(items: &[RuleBody]) -> HashSet<String> {
    let mut out = HashSet::new();
    for item in items {
        out.extend(first_rule_refs(item));
        if !can_match_empty(item) { break; }
    }
    out
}

/// Collect the set of possible first terminal tokens (keywords/punctuation)
/// that a body can start with. Returns empty if it starts with a parser rule ref
/// (which could match anything).
fn first_terminals(body: &RuleBody) -> HashSet<String> {
    let mut out = HashSet::new();
    match body {
        RuleBody::Keyword(kw) => { out.insert(kw.clone()); }
        RuleBody::Sequence(items) => {
            for item in items {
                let t = first_terminals(item);
                out.extend(t.iter().cloned());
                if !can_match_empty(item) { break; }
            }
        }
        RuleBody::Alternative(alts) => {
            for alt in alts {
                out.extend(first_terminals(alt));
            }
        }
        RuleBody::Group(inner) | RuleBody::OneOrMore(inner) => {
            out.extend(first_terminals(inner));
        }
        RuleBody::Optional(inner) | RuleBody::ZeroOrMore(inner) => {
            out.extend(first_terminals(inner));
        }
        RuleBody::Assignment { value, .. } | RuleBody::BoolAssign { value, .. } => {
            out.extend(first_terminals(value));
        }
        RuleBody::RuleRef(name) if is_lexer_rule(name) => { out.insert(name.clone()); }
        RuleBody::RuleRef(_) | RuleBody::CrossRef(_) => {
            // Parser rule or cross-ref — can't determine first terminal
        }
        RuleBody::Empty | RuleBody::Action(_) => {}
    }
    out
}

/// First terminals of a sequence of items.
fn first_terminals_of_seq(items: &[RuleBody]) -> HashSet<String> {
    let mut out = HashSet::new();
    for item in items {
        out.extend(first_terminals(item));
        if !can_match_empty(item) { break; }
    }
    out
}

/// Check if a body can start with a CrossRef (directly or transitively
/// through assignments, groups, alternatives, optionals).
#[allow(dead_code)]
fn has_first_crossref(body: &RuleBody) -> bool {
    match body {
        RuleBody::CrossRef(_) => true,
        RuleBody::Sequence(items) => {
            for item in items {
                if has_first_crossref(item) { return true; }
                if !can_match_empty(item) { break; }
            }
            false
        }
        RuleBody::Alternative(alts) => alts.iter().any(|a| has_first_crossref(a)),
        RuleBody::Group(inner) | RuleBody::OneOrMore(inner) => has_first_crossref(inner),
        RuleBody::Optional(inner) | RuleBody::ZeroOrMore(inner) => has_first_crossref(inner),
        RuleBody::Assignment { value, .. } | RuleBody::BoolAssign { value, .. } => has_first_crossref(value),
        _ => false,
    }
}

/// Generate a position-only probe for a sequence of body items.
/// Unlike `generate_sequence`/`parse_body`, this does NOT assign to AST fields.
/// It only advances the parser position to check if the sequence can parse.
/// Used as a lookahead guard inside greedy-loop code.
fn generate_probe(items: &[RuleBody], grammar: &Grammar, indent: usize) -> String {
    let ind = "    ".repeat(indent);
    let mut code = String::new();
    for item in items {
        code.push_str(&generate_probe_body(item, grammar, indent));
    }
    let _ = &ind; // suppress unused
    code
}

/// Generate position-advancing-only code for a single RuleBody.
/// Assignments/BoolAssigns just parse their value without storing.
fn generate_probe_body(body: &RuleBody, grammar: &Grammar, indent: usize) -> String {
    let ind = "    ".repeat(indent);
    match body {
        RuleBody::Empty | RuleBody::Action(_) => String::new(),
        RuleBody::Keyword(kw) => {
            let variant = if grammar.keywords.contains(kw) {
                keyword_to_variant(kw)
            } else {
                punctuation_to_variant(kw)
            };
            format!("{}self.expect(TokenKind::{})?;\n", ind, variant)
        }
        RuleBody::RuleRef(name) => {
            if is_lexer_rule(name) {
                if let Some(code) = generate_lexer_rule_expect(name, grammar, indent) {
                    code
                } else {
                    format!("{}self.expect_any()?;\n", ind)
                }
            } else {
                format!("{}self.parse_{}()?;\n", ind, to_snake_case(name))
            }
        }
        RuleBody::CrossRef(_) => {
            format!("{}self.parse_cross_ref()?;\n", ind)
        }
        RuleBody::Sequence(items) => {
            items.iter().map(|i| generate_probe_body(i, grammar, indent)).collect()
        }
        RuleBody::Assignment { value, .. } | RuleBody::BoolAssign { value, .. } => {
            generate_probe_body(value, grammar, indent)
        }
        RuleBody::Optional(inner) => {
            let mut code = String::new();
            code.push_str(&format!("{}let saved = self.save();\n", ind));
            code.push_str(&format!("{}let _: std::result::Result<(), ParseError> = (|| {{\n", ind));
            code.push_str(&generate_probe_body(inner, grammar, indent + 1));
            code.push_str(&format!("{}    Ok(())\n", ind));
            code.push_str(&format!("{}}})()", ind));
            code.push_str(".map_err(|e| { self.restore(saved); e });\n");
            code
        }
        RuleBody::ZeroOrMore(inner) => {
            let mut code = String::new();
            code.push_str(&format!("{}loop {{\n", ind));
            code.push_str(&format!("{}    let saved = self.save();\n", ind));
            code.push_str(&format!("{}    let ok: std::result::Result<(), ParseError> = (|| {{\n", ind));
            code.push_str(&generate_probe_body(inner, grammar, indent + 2));
            code.push_str(&format!("{}        Ok(())\n", ind));
            code.push_str(&format!("{}    }})();\n", ind));
            code.push_str(&format!("{}    if ok.is_err() {{ self.restore(saved); break; }}\n", ind));
            code.push_str(&format!("{}    if self.save() == saved {{ break; }}\n", ind));
            code.push_str(&format!("{}}}\n", ind));
            code
        }
        RuleBody::OneOrMore(inner) => {
            let mut code = String::new();
            code.push_str(&generate_probe_body(inner, grammar, indent));
            code.push_str(&format!("{}loop {{\n", ind));
            code.push_str(&format!("{}    let saved = self.save();\n", ind));
            code.push_str(&format!("{}    let ok: std::result::Result<(), ParseError> = (|| {{\n", ind));
            code.push_str(&generate_probe_body(inner, grammar, indent + 2));
            code.push_str(&format!("{}        Ok(())\n", ind));
            code.push_str(&format!("{}    }})();\n", ind));
            code.push_str(&format!("{}    if ok.is_err() {{ self.restore(saved); break; }}\n", ind));
            code.push_str(&format!("{}    if self.save() == saved {{ break; }}\n", ind));
            code.push_str(&format!("{}}}\n", ind));
            code
        }
        RuleBody::Group(inner) => generate_probe_body(inner, grammar, indent),
        RuleBody::Alternative(alts) => {
            // Try alternatives in sequence, pick the first that succeeds
            let mut code = String::new();
            code.push_str(&format!("{}let saved_probe = self.save();\n", ind));
            for (i, alt) in alts.iter().enumerate() {
                if i == 0 {
                    code.push_str(&format!("{}(|| -> std::result::Result<(), ParseError> {{\n", ind));
                } else {
                    code.push_str(&format!(".or_else(|_: ParseError| {{\n"));
                    code.push_str(&format!("{}    self.restore(saved_probe);\n", ind));
                }
                code.push_str(&generate_probe_body(alt, grammar, indent + 1));
                code.push_str(&format!("{}    Ok(())\n", ind));
                if i == 0 {
                    code.push_str(&format!("{}}})()", ind));
                } else {
                    code.push_str(&format!("{}}})", ind));
                }
            }
            code.push_str("?;\n");
            code
        }
    }
}

fn parse_body(body: &RuleBody, node: &AstNodeDef, grammar: &Grammar, indent: usize) -> String {
    let ind = "    ".repeat(indent);
    
    match body {
        RuleBody::Empty => String::new(),
        
        RuleBody::Keyword(kw) => {
            let variant = if grammar.keywords.contains(kw) {
                keyword_to_variant(kw)
            } else {
                punctuation_to_variant(kw)
            };
            format!("{}self.expect(TokenKind::{})?;\n", ind, variant)
        }
        
        RuleBody::RuleRef(name) => {
            if is_lexer_rule(name) {
                // Generate proper token expectations for lexer rules
                if let Some(code) = generate_lexer_rule_expect(name, grammar, indent) {
                    code
                } else {
                    format!("{}self.expect_any()?; // TODO: unmapped lexer rule {}\n", ind, name)
                }
            } else {
                format!("{}self.parse_{}()?;\n", ind, to_snake_case(name))
            }
        }
        
        RuleBody::Sequence(items) => {
            generate_sequence(items, node, grammar, indent)
        }
        
        RuleBody::Assignment { name, operator, value } => {
            let field_name = to_snake_case(name);
            let is_list = *operator == AssignOp::AddAssign;
            let field = node.fields.iter().find(|f| f.name == field_name);
            generate_assignment(&field_name, value, field, node, grammar, indent, is_list)
        }
        
        RuleBody::BoolAssign { name, value } => {
            let field_name = to_snake_case(name);
            let mut code = parse_body(value, node, grammar, indent);
            code.push_str(&format!("{}{} = true;\n", "    ".repeat(indent), field_name));
            code
        }
        
        RuleBody::Optional(inner) => {
            let mut code = String::new();
            code.push_str(&format!("{}let saved = self.save();\n", ind));
            code.push_str(&format!("{}let _: std::result::Result<(), ParseError> = (|| {{\n", ind));
            code.push_str(&parse_body(inner, node, grammar, indent + 1));
            code.push_str(&format!("{}    Ok(())\n", ind));
            code.push_str(&format!("{}}})()", ind));
            code.push_str(".map_err(|e| { self.restore(saved); e });\n");
            code
        }
        
        RuleBody::ZeroOrMore(inner) => {
            let mut code = String::new();
            // If inner is a bare rule ref, capture results into the auto-field
            let collect_field = match inner.as_ref() {
                RuleBody::RuleRef(name) if !is_lexer_rule(name) => Some(to_snake_case(name)),
                _ => None,
            };
            
            // Track valid stopping positions for GLR backtracking
            code.push_str(&format!("{}let mut _glr_stop_positions: Vec<usize> = vec![self.pos];\n", ind));
            
            code.push_str(&format!("{}loop {{\n", ind));
            code.push_str(&format!("{}    let saved = self.save();\n", ind));
            code.push_str(&format!("{}    let ok: std::result::Result<(), ParseError> = (|| {{\n", ind));
            if let (Some(ref field_name), RuleBody::RuleRef(name)) = (&collect_field, inner.as_ref()) {
                let method = format!("parse_{}", to_snake_case(name));
                code.push_str(&format!("{}        let v = self.{}()?;\n", ind, method));
                code.push_str(&format!("{}        {}.push(v);\n", ind, field_name));
            } else {
                code.push_str(&parse_body(inner, node, grammar, indent + 2));
            }
            code.push_str(&format!("{}        Ok(())\n", ind));
            code.push_str(&format!("{}    }})();\n", ind));
            code.push_str(&format!("{}    if ok.is_err() {{ self.restore(saved); break; }}\n", ind));
            code.push_str(&format!("{}    if self.save() == saved {{ break; }} // no progress\n", ind));
            code.push_str(&format!("{}    _glr_stop_positions.push(self.pos);\n", ind));
            code.push_str(&format!("{}}}\n", ind));
            
            // After loop: check if current position is excluded; backtrack if needed
            code.push_str(&format!("{}while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {{\n", ind));
            code.push_str(&format!("{}    _glr_stop_positions.pop();\n", ind));
            if let Some(ref field_name) = collect_field {
                code.push_str(&format!("{}    {}.pop();\n", ind, field_name));
            }
            code.push_str(&format!("{}    self.pos = *_glr_stop_positions.last().unwrap();\n", ind));
            code.push_str(&format!("{}}}\n", ind));
            
            code
        }
        
        RuleBody::OneOrMore(inner) => {
            let mut code = String::new();
            let collect_field = match inner.as_ref() {
                RuleBody::RuleRef(name) if !is_lexer_rule(name) => Some(to_snake_case(name)),
                _ => None,
            };
            
            // Track valid stopping positions for GLR backtracking
            code.push_str(&format!("{}let mut _glr_stop_positions: Vec<usize> = Vec::new();\n", ind));
            
            // First iteration required
            if let (Some(ref field_name), RuleBody::RuleRef(name)) = (&collect_field, inner.as_ref()) {
                let method = format!("parse_{}", to_snake_case(name));
                code.push_str(&format!("{}let v = self.{}()?;\n", ind, method));
                code.push_str(&format!("{}{}.push(v);\n", ind, field_name));
            } else {
                code.push_str(&parse_body(inner, node, grammar, indent));
            }
            code.push_str(&format!("{}_glr_stop_positions.push(self.pos);\n", ind));
            
            // Rest optional (same as ZeroOrMore)
            code.push_str(&format!("{}loop {{\n", ind));
            code.push_str(&format!("{}    let saved = self.save();\n", ind));
            code.push_str(&format!("{}    let ok: std::result::Result<(), ParseError> = (|| {{\n", ind));
            if let (Some(ref field_name), RuleBody::RuleRef(name)) = (&collect_field, inner.as_ref()) {
                let method = format!("parse_{}", to_snake_case(name));
                code.push_str(&format!("{}        let v = self.{}()?;\n", ind, method));
                code.push_str(&format!("{}        {}.push(v);\n", ind, field_name));
            } else {
                code.push_str(&parse_body(inner, node, grammar, indent + 2));
            }
            code.push_str(&format!("{}        Ok(())\n", ind));
            code.push_str(&format!("{}    }})();\n", ind));
            code.push_str(&format!("{}    if ok.is_err() {{ self.restore(saved); break; }}\n", ind));
            code.push_str(&format!("{}    if self.save() == saved {{ break; }} // no progress\n", ind));
            code.push_str(&format!("{}    _glr_stop_positions.push(self.pos);\n", ind));
            code.push_str(&format!("{}}}\n", ind));
            
            // After loop: check if current position is excluded; backtrack if needed
            code.push_str(&format!("{}while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {{\n", ind));
            code.push_str(&format!("{}    _glr_stop_positions.pop();\n", ind));
            if let Some(ref field_name) = collect_field {
                code.push_str(&format!("{}    {}.pop();\n", ind, field_name));
            }
            code.push_str(&format!("{}    self.pos = *_glr_stop_positions.last().unwrap();\n", ind));
            code.push_str(&format!("{}}}\n", ind));
            
            code
        }
        
        RuleBody::Group(inner) => parse_body(inner, node, grammar, indent),
        
        RuleBody::Alternative(alts) => {
            if alts.len() == 1 {
                return parse_body(&alts[0], node, grammar, indent);
            }
            
            // Detect whether alternatives mix CrossRef-primary and non-CrossRef
            // alternatives (e.g. GeneralType = [QualifiedName] | OwnedFeatureChain).
            let has_xref = alts.iter().any(|a| is_crossref_primary(a));
            let has_non_xref = alts.iter().any(|a| !is_crossref_primary(a));
            let is_ambiguous = has_xref && has_non_xref;
            
            // Reorder alternatives: put more specific (higher min-required-tokens)
            // first, and alternatives that can match empty last.
            let mut indexed: Vec<(usize, &RuleBody)> = alts.iter().enumerate().collect();
            indexed.sort_by(|(_, a), (_, b)| {
                let a_empty = can_match_empty(a);
                let b_empty = can_match_empty(b);
                match (a_empty, b_empty) {
                    (false, true) => std::cmp::Ordering::Less,
                    (true, false) => std::cmp::Ordering::Greater,
                    _ => {
                        let a_req = min_required_tokens(a);
                        let b_req = min_required_tokens(b);
                        b_req.cmp(&a_req).then_with(|| {
                            let a_kw = min_required_keywords(a);
                            let b_kw = min_required_keywords(b);
                            b_kw.cmp(&a_kw)
                        })
                    }
                }
            });
            let sorted_alts: Vec<&RuleBody> = indexed.into_iter().map(|(_, alt)| alt).collect();
            
            if is_ambiguous {
                // GLR-style: try all alternatives, pick the longest non-excluded one.
                // If outer parsing fails, the caller will add an exclusion and re-parse.
                let mut code = String::new();
                code.push_str(&format!("{}let saved_alt = self.save();\n", ind));
                code.push_str(&format!("{}let mut alt_results: Vec<usize> = Vec::new();\n", ind));
                
                for alt in sorted_alts.iter() {
                    code.push_str(&format!("{}self.restore(saved_alt);\n", ind));
                    code.push_str(&format!("{}if (|| -> std::result::Result<(), ParseError> {{\n", ind));
                    code.push_str(&parse_body(alt, node, grammar, indent + 1));
                    code.push_str(&format!("{}    Ok(())\n", ind));
                    code.push_str(&format!("{}}})()", ind));
                    code.push_str(".is_ok() {\n");
                    code.push_str(&format!("{}    let end_pos = self.pos;\n", ind));
                    code.push_str(&format!("{}    if !self.is_parse_excluded(saved_alt, \"{}\", end_pos) {{\n", ind, node.rule_name));
                    code.push_str(&format!("{}        alt_results.push(end_pos);\n", ind));
                    code.push_str(&format!("{}    }}\n", ind));
                    code.push_str(&format!("{}}}\n", ind));
                }
                
                // Sort by end position descending (longest first), pick first non-excluded
                code.push_str(&format!("{}alt_results.sort_by(|a, b| b.cmp(a));\n", ind));
                code.push_str(&format!("{}alt_results.dedup();\n", ind));
                
                // Use the longest non-excluded result
                code.push_str(&format!("{}match alt_results.first() {{\n", ind));
                code.push_str(&format!("{}    Some(&pos) => self.pos = pos,\n", ind));
                code.push_str(&format!("{}    None => return Err(ParseError {{ message: \"no alternative matched\".into(), span: self.current_span() }}),\n", ind));
                code.push_str(&format!("{}}}\n", ind));
                code
            } else {
                // Use longest-match for nested alternatives: try all alternatives
                // and pick the one that consumed the most tokens. This prevents
                // greedy first-match from blocking longer alternatives (e.g.,
                // PositionalArgumentList blocking NamedArgumentList in ArgumentList).
                let mut code = String::new();
                code.push_str(&format!("{}let saved_alt = self.save();\n", ind));
                code.push_str(&format!("{}let mut best_alt_pos: Option<usize> = None;\n", ind));
                
                for (_i, alt) in sorted_alts.iter().enumerate() {
                    code.push_str(&format!("{}self.restore(saved_alt);\n", ind));
                    code.push_str(&format!("{}if (|| -> std::result::Result<(), ParseError> {{\n", ind));
                    code.push_str(&parse_body(alt, node, grammar, indent + 1));
                    code.push_str(&format!("{}    Ok(())\n", ind));
                    code.push_str(&format!("{}}})()" , ind));
                    code.push_str(".is_ok() {\n");
                    code.push_str(&format!("{}    let end = self.save();\n", ind));
                    code.push_str(&format!("{}    if best_alt_pos.map_or(true, |b| end > b) {{ best_alt_pos = Some(end); }}\n", ind));
                    code.push_str(&format!("{}}}\n", ind));
                }
                
                code.push_str(&format!("{}match best_alt_pos {{\n", ind));
                code.push_str(&format!("{}    Some(pos) => self.pos = pos,\n", ind));
                code.push_str(&format!("{}    None => return Err(ParseError {{ message: \"no alternative matched\".into(), span: self.current_span() }}),\n", ind));
                code.push_str(&format!("{}}}\n", ind));
                code
            }
        }
        
        RuleBody::CrossRef(_) => {
            // Bare cross-reference: parse a qualified name (consume tokens)
            format!("{}self.parse_cross_ref()?;\n", ind)
        }
        
        RuleBody::Action(_) => String::new(),
    }
}

/// Get the variable name used for a field in the generated parse method.
#[allow(dead_code)]
fn field_var_name(field: &AstField) -> String {
    let uses_opt = !field.is_list && !field.optional &&
        !matches!(field.field_type, FieldType::Bool | FieldType::Token(_));
    if uses_opt { format!("{}_opt", field.name) } else { field.name.clone() }
}

/// Generate code to save all field values into `_save_<name>` variables.
#[allow(dead_code)]
fn generate_field_saves(node: &AstNodeDef, indent: usize) -> String {
    let ind = "    ".repeat(indent);
    let mut code = String::new();
    for field in &node.fields {
        let var = field_var_name(field);
        code.push_str(&format!("{}let _save_{} = {}.clone();\n", ind, field.name, var));
    }
    code
}

/// Generate code to restore all field values from `_save_<name>` variables.
#[allow(dead_code)]
fn generate_field_restores(node: &AstNodeDef, indent: usize) -> String {
    let ind = "    ".repeat(indent);
    let mut code = String::new();
    for field in &node.fields {
        let var = field_var_name(field);
        code.push_str(&format!("{}{} = _save_{};\n", ind, var, field.name));
    }
    code
}

fn generate_assignment(field_name: &str, value: &RuleBody, field: Option<&AstField>, node: &AstNodeDef, grammar: &Grammar, indent: usize, is_list: bool) -> String {
    let ind = "    ".repeat(indent);
    let mut code = String::new();
    
    // Use the field definition's is_list if available (grammar may use = in one 
    // alternative and += in another for the same field)
    let is_list = field.map(|f| f.is_list).unwrap_or(is_list);
    let is_optional = field.map(|f| f.optional).unwrap_or(false);
    let needs_box = field.map(|f| matches!(&f.field_type, FieldType::Node(_))).unwrap_or(false);
    let is_union = field.map(|f| matches!(&f.field_type, FieldType::Union(_))).unwrap_or(false);
    
    // Check if this field uses _opt pattern (required non-primitive)
    let uses_opt = field.map(|f| !f.is_list && !f.optional && 
        !matches!(f.field_type, FieldType::Bool | FieldType::Token(_))).unwrap_or(false);
    let var_name = if uses_opt { format!("{}_opt", field_name) } else { field_name.to_string() };
    
    match value {
        RuleBody::RuleRef(ref_name) if !is_lexer_rule(ref_name) => {
            let method = format!("parse_{}", to_snake_case(ref_name));
            code.push_str(&format!("{}let v = self.{}()?;\n", ind, method));
            
            // Determine wrapping
            let wrapped = if is_union {
                // Wrap in union variant
                let variant_name = to_pascal_case(ref_name);
                let union_type = format!("{}{}Member", node.name, to_pascal_case(field.map(|f| f.original_name.as_str()).unwrap_or("")));
                format!("{}::{}(Box::new(v))", union_type, variant_name)
            } else if needs_box && !is_list {
                "Box::new(v)".to_string()
            } else {
                "v".to_string()
            };
            
            if is_list {
                code.push_str(&format!("{}{}.push({});\n", ind, var_name, wrapped));
            } else if is_optional || uses_opt {
                code.push_str(&format!("{}{} = Some({});\n", ind, var_name, wrapped));
            } else {
                code.push_str(&format!("{}{} = {};\n", ind, var_name, wrapped));
            }
        }
        RuleBody::RuleRef(ref_name) if is_lexer_rule(ref_name) => {
            // Lexer token — use proper token kind when possible
            if let Some(token_kind) = lexer_rule_to_token_kind(ref_name) {
                if token_kind.contains('|') {
                    // Multi-token: NAME accepts Name or UnrestrictedName only (not keywords)
                    code.push_str(&format!("{}let v = match self.current() {{\n", ind));
                    code.push_str(&format!("{}    Some(t) if t.kind.is_name_token() => {{ let text = t.text.clone(); self.pos += 1; text }}\n", ind));
                    code.push_str(&format!("{}    Some(t) => return Err(ParseError {{ message: format!(\"expected name, got {{:?}}\", t.kind), span: t.span }}),\n", ind));
                    code.push_str(&format!("{}    None => return Err(ParseError {{ message: \"expected name, got EOF\".into(), span: Span::default() }}),\n", ind));
                    code.push_str(&format!("{}}};\n", ind));
                } else {
                    code.push_str(&format!("{}let v = self.expect(TokenKind::{})?.text.clone();\n", ind, token_kind));
                }
            } else {
                code.push_str(&format!("{}let v = self.expect_any()?.text.clone(); // TODO: unmapped lexer rule {}\n", ind, ref_name));
            }
            if is_list {
                code.push_str(&format!("{}{}.push(v);\n", ind, var_name));
            } else if is_optional {
                code.push_str(&format!("{}{} = Some(v);\n", ind, var_name));
            } else {
                code.push_str(&format!("{}{} = v;\n", ind, var_name));
            }
        }
        RuleBody::CrossRef(ref_name) => {
            code.push_str(&format!("{}let v = self.parse_cross_ref()?;\n", ind));
            let wrapped = if is_union {
                let variant_name = format!("{}Ref", to_pascal_case(ref_name));
                let union_type = format!("{}{}Member", node.name, to_pascal_case(field.map(|f| f.original_name.as_str()).unwrap_or("")));
                format!("{}::{}(v)", union_type, variant_name)
            } else {
                "v".to_string()
            };
            if is_list {
                code.push_str(&format!("{}{}.push({});\n", ind, var_name, wrapped));
            } else if is_optional || uses_opt {
                code.push_str(&format!("{}{} = Some({});\n", ind, var_name, wrapped));
            } else {
                code.push_str(&format!("{}{} = {};\n", ind, var_name, wrapped));
            }
        }
        // Handle Group(CrossRef(...)) — produced by KEBNF parser for ~[QualifiedName]
        // The Group wrapper is semantically insignificant; treat the inner CrossRef
        // the same as a direct CrossRef for field assignment purposes.
        RuleBody::Group(inner) if matches!(inner.as_ref(), RuleBody::CrossRef(_)) => {
            if let RuleBody::CrossRef(ref_name) = inner.as_ref() {
                code.push_str(&format!("{}let v = self.parse_cross_ref()?;\n", ind));
                let wrapped = if is_union {
                    let variant_name = format!("{}Ref", to_pascal_case(ref_name));
                    let union_type = format!("{}{}Member", node.name, to_pascal_case(field.map(|f| f.original_name.as_str()).unwrap_or("")));
                    format!("{}::{}(v)", union_type, variant_name)
                } else {
                    "v".to_string()
                };
                if is_list {
                    code.push_str(&format!("{}{}.push({});\n", ind, var_name, wrapped));
                } else if is_optional || uses_opt {
                    code.push_str(&format!("{}{} = Some({});\n", ind, var_name, wrapped));
                } else {
                    code.push_str(&format!("{}{} = {};\n", ind, var_name, wrapped));
                }
            }
        }
        _ => {
            code.push_str(&parse_body(value, node, grammar, indent));
        }
    }
    
    code
}

/// Generate the Parser struct definition and impl block.
/// Uses per-SCC LR head tracking for indirect left recursion.
fn generate_parser_struct(
    lr_rules: &HashSet<String>,
    node_map: &HashMap<String, &AstNodeDef>,
    scc_map: &HashMap<String, usize>,
) -> String {
    let mut code = String::new();

    // Collect unique SCC IDs
    let mut scc_ids: Vec<usize> = scc_map.values().copied().collect();
    scc_ids.sort();
    scc_ids.dedup();

    // ---- struct Parser ----
    code.push_str("\npub struct Parser {\n");
    code.push_str("    tokens: Vec<Token>,\n");
    code.push_str("    pos: usize,\n");
    code.push_str("    visiting: std::collections::HashSet<(usize, &'static str)>,\n");
    // GLR-style exclusion set: prevents certain parse endpoints from being used
    // Tuple: (start_pos, rule_name, end_pos) - skip this specific parse result
    code.push_str("    /// GLR-style exclusion set for ambiguous parses.\n");
    code.push_str("    /// Contains (start_pos, rule_name, end_pos) tuples to skip.\n");
    code.push_str("    excluded_parses: std::collections::HashSet<(usize, &'static str, usize)>,\n");
    // Stack of (rule_name, entry_pos) for GLR-aware greedy loops to check exclusions
    code.push_str("    /// Stack of (rule_name, entry_pos) for current parse context.\n");
    code.push_str("    /// Used by greedy loops to check if their endpoint is excluded.\n");
    code.push_str("    rule_context: Vec<(&'static str, usize)>,\n");

    // Per-SCC LR head position tracking
    for scc_id in &scc_ids {
        code.push_str(&format!("    /// LR head position for SCC group {}\n", scc_id));
        code.push_str(&format!("    lr_head_{}: Option<usize>,\n", scc_id));
    }

    // Per-rule memoisation fields for left-recursive rules
    let mut lr_fields: Vec<(String, String)> = Vec::new();
    for name in lr_rules {
        if let Some(node) = node_map.get(name) {
            let field = format!("lr_{}", to_snake_case(name));
            let ty = node.name.clone();
            lr_fields.push((field, ty));
        }
    }
    lr_fields.sort_by(|a, b| a.0.cmp(&b.0));
    for (field, ty) in &lr_fields {
        code.push_str(&format!("    {}: Option<(usize, usize, {})>,\n", field, ty));
    }

    code.push_str("}\n\n");

    // ---- impl Parser ----
    code.push_str("impl Parser {\n");
    code.push_str("    pub fn new(tokens: Vec<Token>) -> Self {\n");
    code.push_str("        Parser {\n");
    code.push_str("            tokens,\n");
    code.push_str("            pos: 0,\n");
    code.push_str("            visiting: std::collections::HashSet::new(),\n");
    code.push_str("            excluded_parses: std::collections::HashSet::new(),\n");
    code.push_str("            rule_context: Vec::new(),\n");
    for scc_id in &scc_ids {
        code.push_str(&format!("            lr_head_{}: None,\n", scc_id));
    }
    for (field, _) in &lr_fields {
        code.push_str(&format!("            {}: None,\n", field));
    }
    code.push_str("        }\n");
    code.push_str("    }\n\n");

    // Helper methods
    code.push_str(PARSER_METHODS);

    code
}

// ============================================================================
// Boilerplate
// ============================================================================

const PARSER_BOILERPLATE: &str = r#"#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
    pub span: Span,
}
"#;

const SPAN_BOILERPLATE: &str = r#"
#[derive(Debug, Clone, Copy, Default)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn merge(self, other: Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}

pub struct ParseError {
    pub message: String,
    pub span: Span,
}

pub type Result<T> = std::result::Result<T, ParseError>;
"#;

/// Methods inside `impl Parser { … }`.
/// The struct definition and `new()` are generated dynamically.
const PARSER_METHODS: &str = r#"
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Returns true if this (pos, rule) pair is new (not already being visited).
    /// Inserts it into the visiting set.
    #[inline]
    fn enter_rule(&mut self, rule: &'static str) -> bool {
        self.visiting.insert((self.pos, rule))
    }

    #[inline]
    fn leave_rule(&mut self, pos: usize, rule: &'static str) {
        self.visiting.remove(&(pos, rule));
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn current_span(&self) -> Span {
        self.current().map(|t| t.span).unwrap_or_default()
    }

    fn advance(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.pos);
        self.pos += 1;
        tok
    }

    fn expect(&mut self, kind: TokenKind) -> Result<&Token> {
        match self.current() {
            Some(t) if t.kind == kind => {
                self.pos += 1;
                Ok(&self.tokens[self.pos - 1])
            }
            Some(t) => Err(ParseError {
                message: format!("expected {:?}, got {:?}", kind, t.kind),
                span: t.span,
            }),
            None => Err(ParseError {
                message: format!("expected {:?}, got EOF", kind),
                span: Span::default(),
            }),
        }
    }

    fn expect_any(&mut self) -> Result<&Token> {
        self.advance().ok_or_else(|| ParseError {
            message: "unexpected EOF".into(),
            span: Span::default(),
        })
    }

    fn save(&self) -> usize {
        self.pos
    }

    fn restore(&mut self, pos: usize) {
        self.pos = pos;
    }

    /// GLR-style: Check if a specific parse result is excluded.
    fn is_parse_excluded(&self, start_pos: usize, rule: &'static str, end_pos: usize) -> bool {
        self.excluded_parses.contains(&(start_pos, rule, end_pos))
    }

    /// GLR-style: Exclude a specific parse result so it won't be used on retry.
    fn exclude_parse(&mut self, start_pos: usize, rule: &'static str, end_pos: usize) {
        self.excluded_parses.insert((start_pos, rule, end_pos));
    }

    /// GLR-style: Clear all exclusions (called when starting a new top-level parse).
    #[allow(dead_code)]
    fn clear_exclusions(&mut self) {
        self.excluded_parses.clear();
    }

    /// Push a rule onto the context stack (called at rule entry).
    #[inline]
    fn push_rule_context(&mut self, rule: &'static str, entry_pos: usize) {
        self.rule_context.push((rule, entry_pos));
    }

    /// Pop a rule from the context stack (called at rule exit).
    #[inline]
    fn pop_rule_context(&mut self) {
        self.rule_context.pop();
    }

    /// Check if the current position is excluded for ANY rule in the context stack.
    /// Used by greedy loops to stop early when continuing would produce an excluded result.
    fn is_current_pos_excluded(&self) -> bool {
        let cur_pos = self.pos;
        for &(rule, entry_pos) in &self.rule_context {
            if self.excluded_parses.contains(&(entry_pos, rule, cur_pos)) {
                return true;
            }
        }
        false
    }

    fn parse_cross_ref(&mut self) -> Result<QualifiedNameRef> {
        // Parse qualified name as cross-reference: Name ('::' Name)*
        // Only consume '::' if the next token after it is a strict name token,
        // so that '::' '*' and '::' '**' remain available for the caller.
        // Uses is_name_token() (not is_name_compatible()) to avoid greedily
        // consuming keywords that serve as structural delimiters in the
        // enclosing grammar rule (e.g. 'then', 'to', 'accept', 'if', 'do').
        let start = self.current_span();
        let mut path = Vec::new();
        
        loop {
            if let Some(t) = self.current() {
                if t.kind.is_name_token() {
                    path.push(t.text.clone());
                    self.advance();
                    // Peek: only consume '::' if followed by another name segment
                    if self.current().map(|t| t.kind == TokenKind::ColonColon).unwrap_or(false) {
                        if self.tokens.get(self.pos + 1).map(|t| t.kind.is_name_token()).unwrap_or(false) {
                            self.advance(); // consume '::'
                            continue;
                        }
                    }
                }
            }
            break;
        }
        
        if path.is_empty() {
            return Err(ParseError {
                message: "expected qualified name".into(),
                span: start,
            });
        }

        let end = self.current_span();
        Ok(QualifiedNameRef { path, span: start.merge(end) })
    }
"#;
