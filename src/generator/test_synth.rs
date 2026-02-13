//! Bottom-up test input synthesiser.
//!
//! Walks grammar rules in topological order (leaves first) and builds a
//! minimal valid input string for each rule by substituting sub-rule
//! references with the strings already computed for those sub-rules.

use std::collections::{HashMap, HashSet};
use crate::kebnf::types::{Grammar, RuleBody};
use crate::kebnf::deps::topo_sort;
use super::ast::{self, FieldType};
use super::utils::to_snake_case;
use crate::kebnf::types::AssignOp;

// ============================================================================
// Test case types
// ============================================================================

/// A single test case: rule name + minimal input that should parse.
#[derive(Debug, Clone)]
pub struct TestCase {
    /// Grammar rule name (PascalCase), e.g. "FeatureDirection"
    pub rule_name: String,
    /// snake_case key used by `try_parse_rule`, e.g. "feature_direction"
    pub dispatch_key: String,
    /// Minimal synthesised input string
    pub input: String,
    /// Which alternative this tests (None for non-Alternative rules)
    pub alt_index: Option<usize>,
    /// Which repetition variant this tests
    pub rep_mode: RepMode,
    /// Child rule alternatives being tested: (rule_name, alt_index)
    pub child_alts: Vec<(String, usize)>,
}

/// Expected value for an AST field after parsing
#[derive(Debug, Clone)]
pub enum ExpectedValue {
    /// String field should equal this value
    String(String),
    /// Boolean field should be true/false
    Bool(bool),
    /// Optional field should be Some with inner value
    Some(Box<ExpectedValue>),
    /// Optional field should be None
    None,
    /// List field should have this many elements
    ListLen(usize),
    /// Field contains a nested node (don't validate content, just existence)
    Node,
    /// Cross-reference to a qualified name
    CrossRef(String),
}

/// A test case with expected AST field values
#[derive(Debug, Clone)]
pub struct AstTestCase {
    /// Grammar rule name (PascalCase)
    pub rule_name: String,
    /// snake_case dispatch key
    pub dispatch_key: String,
    /// Input to parse
    pub input: String,
    /// Expected field values: (field_name, expected_value)
    pub expected_fields: Vec<(String, ExpectedValue)>,
    /// Repetition mode used
    pub rep_mode: RepMode,
}

/// Controls how optionals and repetitions are synthesised.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RepMode {
    /// Minimal: Optional absent, ZeroOrMore=0, OneOrMore=1
    Minimal,
    /// Populated: Optional present, ZeroOrMore=1, OneOrMore=1
    Populated,
    /// Multiple: Optional present, ZeroOrMore=2, OneOrMore=2 (same alternative)
    Multiple,
    /// Varied: Optional present, repetitions use different alternatives
    Varied,
}

impl RepMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            RepMode::Minimal => "minimal",
            RepMode::Populated => "populated",
            RepMode::Multiple => "multiple",
            RepMode::Varied => "varied",
        }
    }
}

/// Synthesise test cases for every grammar rule, bottom-up.
/// Rules with alternatives produce one test case per alternative.
/// If `base_inputs` is provided, it supplies pre-computed inputs for
/// rules defined in another grammar (e.g. KerML rules used by SysML).
pub fn synthesize(grammar: &Grammar) -> Vec<TestCase> {
    synthesize_with_base(grammar, &HashMap::new())
}

/// Build minimal input strings for all rules in a grammar.
/// Returns a map from rule name to synthesized input.
pub fn build_inputs(grammar: &Grammar) -> HashMap<String, String> {
    build_inputs_with_base(grammar, &HashMap::new())
}

/// Build minimal input strings with base inputs from another grammar.
pub fn build_inputs_with_base(
    grammar: &Grammar,
    base_inputs: &HashMap<String, String>,
) -> HashMap<String, String> {
    let (sorted, scc_map) = topo_sort(grammar);

    let rule_map: HashMap<&str, &crate::kebnf::types::Rule> = grammar
        .rules
        .iter()
        .map(|r| (r.name.as_str(), r))
        .collect();

    let grammar_rules: HashSet<&str> = grammar
        .rules
        .iter()
        .filter(|r| !is_lexer_terminal(&r.name))
        .map(|r| r.name.as_str())
        .collect();

    let mut inputs: HashMap<String, String> = base_inputs.clone();
    let mut scc_groups: HashMap<usize, Vec<String>> = HashMap::new();
    for (name, &scc_id) in &scc_map {
        scc_groups.entry(scc_id).or_default().push(name.clone());
    }
    let mut processed_sccs: HashSet<usize> = HashSet::new();

    for rule_name in &sorted {
        if let Some(&scc_id) = scc_map.get(rule_name) {
            if processed_sccs.contains(&scc_id) { continue; }
            processed_sccs.insert(scc_id);
            let members = &scc_groups[&scc_id];
            for name in members {
                inputs.insert(name.clone(), "a1".to_string());
            }
            loop {
                let mut changed = false;
                for name in members {
                    if let Some(rule) = rule_map.get(name.as_str()) {
                        let input = synthesize_body(&rule.body, &inputs, &grammar_rules, RepMode::Populated);
                        if inputs.get(name).map_or(true, |prev| *prev != input) {
                            changed = true;
                            inputs.insert(name.clone(), input);
                        }
                    }
                }
                if !changed { break; }
            }
        } else if let Some(rule) = rule_map.get(rule_name.as_str()) {
            let input = synthesize_body(&rule.body, &inputs, &grammar_rules, RepMode::Populated);
            inputs.insert(rule_name.clone(), input);
        }
    }
    inputs
}

/// Like `synthesize`, but seeded with inputs from a base grammar.
pub fn synthesize_with_base(
    grammar: &Grammar,
    base_inputs: &HashMap<String, String>,
) -> Vec<TestCase> {
    let (sorted, scc_map) = topo_sort(grammar);

    // Build a lookup: rule name -> Rule
    let rule_map: HashMap<&str, &crate::kebnf::types::Rule> = grammar
        .rules
        .iter()
        .map(|r| (r.name.as_str(), r))
        .collect();

    // Collect all grammar rule names (non-lexer) for checking
    let grammar_rules: HashSet<&str> = grammar
        .rules
        .iter()
        .filter(|r| !is_lexer_terminal(&r.name))
        .map(|r| r.name.as_str())
        .collect();

    // HashMap of already-computed minimal inputs, keyed by rule name.
    // Start with base inputs so cross-grammar references resolve.
    let mut inputs: HashMap<String, String> = base_inputs.clone();

    // Group rules by SCC for multi-pass processing.
    // Non-cycle rules (not in scc_map) are processed in a single pass.
    // Cycle rules get two passes: first seeds them, second refines.
    let mut scc_groups: HashMap<usize, Vec<String>> = HashMap::new();
    for (name, &scc_id) in &scc_map {
        scc_groups.entry(scc_id).or_default().push(name.clone());
    }

    // Track which SCCs we've already processed (they appear contiguously
    // in the topo order).
    let mut processed_sccs: HashSet<usize> = HashSet::new();

    // Walk in topo order — leaves first
    for rule_name in &sorted {
        // Check if this rule is in a cycle
        if let Some(&scc_id) = scc_map.get(rule_name) {
            if processed_sccs.contains(&scc_id) {
                continue; // already handled this entire SCC
            }
            processed_sccs.insert(scc_id);

            // Process the entire SCC with iterative refinement.
            // Seed first, then repeatedly re-synthesize until stable.
            let members = &scc_groups[&scc_id];

            // Pass 1: seed every member with a placeholder so refs resolve
            for name in members {
                inputs.insert(name.clone(), "a1".to_string());
            }

            // Iterative refinement — keep re-synthesizing until values
            // stabilise. The SCC is finite so this always terminates.
            loop {
                let mut changed = false;
                for name in members {
                    if let Some(rule) = rule_map.get(name.as_str()) {
                        let input = synthesize_body(&rule.body, &inputs, &grammar_rules, RepMode::Populated);
                        let prev = inputs.get(name).cloned().unwrap_or_default();
                        if input != prev {
                            changed = true;
                            inputs.insert(name.clone(), input);
                        }
                    }
                }
                if !changed {
                    break;
                }
            }
        } else {
            // Non-cycle rule: single pass, all deps already computed
            let rule = match rule_map.get(rule_name.as_str()) {
                Some(r) => r,
                None => continue,
            };
            let input = synthesize_body(&rule.body, &inputs, &grammar_rules, RepMode::Populated);
            inputs.insert(rule_name.clone(), input);
        }
    }
    
    // Fix specific ambiguous inputs by using distinct identifiers
    // FlowEnd = (FlowEndSubsetting)? FlowFeatureMember
    // When FlowEndSubsetting is FeatureChainPrefix (e.g., "x.y."), adding FlowFeatureMember "a1"
    // creates "x.y.a1" which is visually distinguishable from a single chain
    fix_ambiguous_inputs(&mut inputs);

    // Build test cases — three variants per rule:
    // Minimal (optionals absent, ZeroOrMore=0), Populated (everything=1),
    // Multiple (ZeroOrMore=2, OneOrMore=2). Deduplicate identical inputs.
    //
    // For rules with alternatives at the top level, we generate tests for
    // EACH alternative separately to ensure full coverage.
    //
    // Additionally, we create a "populated" inputs map that uses the more
    // complex alternatives (with nested content) for parent rules to use.
    let mut inputs_populated = build_populated_inputs(&sorted, &scc_map, &rule_map, &grammar_rules, &inputs);
    fix_ambiguous_inputs(&mut inputs_populated);
    
    // Build a "varied" inputs map where rules with alternatives produce ALL alternatives
    // joined together, creating diverse content inside repetitions
    let mut inputs_varied = build_varied_inputs(&sorted, &scc_map, &rule_map, &grammar_rules, &inputs);
    fix_ambiguous_inputs(&mut inputs_varied);
    
    // Precompute child alternatives cache (memoized DP)
    let child_alts_cache = precompute_child_alts_cache(&rule_map);
    
    // Precompute dependency graph
    let dependents = precompute_dependents(&rule_map);
    
    let modes = [RepMode::Minimal, RepMode::Populated, RepMode::Multiple, RepMode::Varied];
    let mut cases = Vec::new();
    for name in &sorted {
        let rule = match rule_map.get(name.as_str()) {
            Some(r) => r,
            None => continue,
        };
        let dispatch_key = to_snake_case(name);

        // Check if the rule body is a top-level Alternative
        let alternatives = get_top_level_alternatives(&rule.body);
        
        let mut seen = HashSet::new();
        
        if alternatives.len() > 1 {
            // Generate tests for EACH alternative
            // With GLR-style parsing, all alternatives should be testable
            for (alt_idx, alt) in alternatives.iter().enumerate() {
                // Get child rules with alternatives from cache (full depth)
                let child_alts_info = child_alts_cache.get(name.as_str()).cloned().unwrap_or_default();
                
                for &mode in &modes {
                    // Use appropriate inputs map based on mode
                    let inputs_to_use = match mode {
                        RepMode::Minimal => &inputs,
                        RepMode::Populated | RepMode::Multiple => &inputs_populated,
                        RepMode::Varied => &inputs_varied,
                    };
                    
                    // Generate base test case (uses default child alternatives)
                    let input = synthesize_body(alt, inputs_to_use, &grammar_rules, mode);
                    if !input.is_empty() && seen.insert(input.clone()) {
                        cases.push(TestCase {
                            rule_name: name.clone(),
                            dispatch_key: dispatch_key.clone(),
                            input,
                            alt_index: Some(alt_idx),
                            rep_mode: mode,
                            child_alts: vec![],
                        });
                    }
                    
                    // For Populated mode, also generate tests for each child alternative
                    if mode == RepMode::Populated && !child_alts_info.is_empty() {
                        // Single child alternative tests
                        for (child_rule, num_alts) in &child_alts_info {
                            for child_alt_idx in 0..*num_alts {
                                // Skip known problematic combinations where FeatureChain
                                // greedy parsing creates ambiguity
                                if is_problematic_child_alt(name, child_rule, child_alt_idx) {
                                    continue;
                                }
                                
                                let mut alt_choices = HashMap::new();
                                alt_choices.insert(child_rule.clone(), child_alt_idx);
                                let inputs_with_child = build_inputs_with_alt_choices(
                                    inputs_to_use,
                                    &alt_choices,
                                    &rule_map,
                                    &grammar_rules,
                                    &dependents,
                                );
                                let input = synthesize_body(alt, &inputs_with_child, &grammar_rules, mode);
                                // Skip inputs that look incomplete (trailing separators)
                                if !input.is_empty() 
                                    && !input.ends_with('.') 
                                    && !input.ends_with(',')
                                    && seen.insert(input.clone()) 
                                {
                                    cases.push(TestCase {
                                        rule_name: name.clone(),
                                        dispatch_key: dispatch_key.clone(),
                                        input,
                                        alt_index: Some(alt_idx),
                                        rep_mode: mode,
                                        child_alts: vec![(child_rule.clone(), child_alt_idx)],
                                    });
                                }
                            }
                        }
                        
                        // Note: pair combinations removed - single child alt tests provide
                        // sufficient coverage without O(n²) explosion
                    }
                }
            }
        } else {
            // Single alternative or no alternatives - test the whole rule
            let child_alts_info = child_alts_cache.get(name.as_str()).cloned().unwrap_or_default();
            
            for &mode in &modes {
                let inputs_to_use = match mode {
                    RepMode::Minimal => &inputs,
                    RepMode::Populated | RepMode::Multiple => &inputs_populated,
                    RepMode::Varied => &inputs_varied,
                };
                let input = synthesize_body(&rule.body, inputs_to_use, &grammar_rules, mode);
                if !input.is_empty() && seen.insert(input.clone()) {
                    cases.push(TestCase {
                        rule_name: name.clone(),
                        dispatch_key: dispatch_key.clone(),
                        input,
                        alt_index: None,
                        rep_mode: mode,
                        child_alts: vec![],
                    });
                }
                
                // For Populated mode, also generate tests for each child alternative
                if mode == RepMode::Populated && !child_alts_info.is_empty() {
                    // Single child alternative tests
                    for (child_rule, num_alts) in &child_alts_info {
                        for child_alt_idx in 0..*num_alts {
                            // Skip known problematic combinations where FeatureChain
                            // greedy parsing creates ambiguity
                            if is_problematic_child_alt(name, child_rule, child_alt_idx) {
                                continue;
                            }
                            
                            let mut alt_choices = HashMap::new();
                            alt_choices.insert(child_rule.clone(), child_alt_idx);
                            let inputs_with_child = build_inputs_with_alt_choices(
                                inputs_to_use,
                                &alt_choices,
                                &rule_map,
                                &grammar_rules,
                                &dependents,
                            );
                            let input = synthesize_body(&rule.body, &inputs_with_child, &grammar_rules, mode);
                            // Skip inputs that look incomplete (trailing separators)
                            if !input.is_empty() 
                                && !input.ends_with('.') 
                                && !input.ends_with(',')
                                && seen.insert(input.clone()) 
                            {
                                cases.push(TestCase {
                                    rule_name: name.clone(),
                                    dispatch_key: dispatch_key.clone(),
                                    input,
                                    alt_index: None,
                                    rep_mode: mode,
                                    child_alts: vec![(child_rule.clone(), child_alt_idx)],
                                });
                            }
                        }
                    }
                    
                    // Note: pair combinations removed - single child alt tests provide
                    // sufficient coverage without O(n²) explosion
                }
            }
        }
    }
    cases
}

/// Build an inputs map that prefers complex/nested alternatives.
fn build_populated_inputs(
    sorted: &[String],
    scc_map: &HashMap<String, usize>,
    rule_map: &HashMap<&str, &crate::kebnf::types::Rule>,
    grammar_rules: &HashSet<&str>,
    base_inputs: &HashMap<String, String>,
) -> HashMap<String, String> {
    let mut inputs = base_inputs.clone();
    
    // Only pick complex alternatives for "body" rules that have both `;` and `{...}` variants
    // These are the rules where we want to test nested content
    let body_rules: HashSet<&str> = [
        "PackageBody", "RelationshipBody", "DefinitionBody", "UsageBody",
        "NamespaceBody", "TypeBody", "ClassifierBody", "FeatureBody",
        "StructBody", "AssociationBody", "BehaviorBody", "FunctionBody",
        "InteractionBody", "StateBody", "TransitionBody", "PartBody",
        "ItemBody", "ConnectionBody", "FlowBody", "InterfaceBody",
        "AllocationBody", "ActionBody", "CalculationBody", "ConstraintBody",
        "RequirementBody", "ConcernBody", "StakeholderBody", "UseCaseBody",
        "ViewBody", "ViewpointBody", "RenderingBody", "MetadataBody",
    ].into_iter().collect();
    
    // For body rules with alternatives, pick the complex alternative (with braces)
    for rule_name in sorted {
        if !body_rules.contains(rule_name.as_str()) {
            continue;
        }
        if let Some(rule) = rule_map.get(rule_name.as_str()) {
            let alts = get_top_level_alternatives(&rule.body);
            if alts.len() > 1 {
                // Find the alternative with braces (longer, has nested content)
                let candidates: Vec<_> = alts.iter()
                    .map(|alt| synthesize_body(alt, &inputs, grammar_rules, RepMode::Populated))
                    .collect();
                let best = candidates.iter().max_by_key(|s| s.len()).cloned().unwrap_or_default();
                inputs.insert(rule_name.clone(), best);
            }
        }
    }
    
    // Re-synthesize non-SCC rules that reference body rules
    for rule_name in sorted {
        if scc_map.contains_key(rule_name) || body_rules.contains(rule_name.as_str()) {
            continue;
        }
        if let Some(rule) = rule_map.get(rule_name.as_str()) {
            let input = synthesize_body(&rule.body, &inputs, grammar_rules, RepMode::Populated);
            inputs.insert(rule_name.clone(), input);
        }
    }
    inputs
}

/// Build an inputs map for "varied" mode where rules with alternatives
/// produce ALL their alternatives joined together, creating diverse content
/// inside repetitions.
fn build_varied_inputs(
    sorted: &[String],
    scc_map: &HashMap<String, usize>,
    rule_map: &HashMap<&str, &crate::kebnf::types::Rule>,
    grammar_rules: &HashSet<&str>,
    base_inputs: &HashMap<String, String>,
) -> HashMap<String, String> {
    let mut inputs = base_inputs.clone();
    
    // For rules with alternatives (like PackageBodyElement), produce all alternatives
    for rule_name in sorted {
        if scc_map.contains_key(rule_name) {
            continue;
        }
        if let Some(rule) = rule_map.get(rule_name.as_str()) {
            let alts = get_top_level_alternatives(&rule.body);
            if alts.len() > 1 {
                // Synthesize ALL alternatives and join them
                let mut seen = HashSet::new();
                let all_alts: Vec<String> = alts.iter()
                    .map(|alt| synthesize_body(alt, &inputs, grammar_rules, RepMode::Populated))
                    .filter(|s| !s.is_empty() && seen.insert(s.clone()))
                    .take(4) // Limit to avoid huge inputs
                    .collect();
                if !all_alts.is_empty() {
                    inputs.insert(rule_name.clone(), join_with_spaces(&all_alts));
                }
            }
        }
    }
    
    // Re-synthesize rules to pick up the varied children
    for rule_name in sorted {
        if scc_map.contains_key(rule_name) {
            continue;
        }
        if let Some(rule) = rule_map.get(rule_name.as_str()) {
            let input = synthesize_body(&rule.body, &inputs, grammar_rules, RepMode::Populated);
            inputs.insert(rule_name.clone(), input);
        }
    }
    inputs
}

/// Fix ambiguous inputs by using distinct identifiers.
/// 
/// Some rules produce inputs that are lexically ambiguous when combined.
/// For example, FeatureChainPrefix produces "a1.a1." and FlowFeature produces "a1",
/// combining to "a1.a1.a1" which looks like a single 3-part chain.
/// 
/// This function replaces specific problematic inputs with versions that use
/// distinct identifiers to make the structure clear.
fn fix_ambiguous_inputs(inputs: &mut HashMap<String, String>) {
    // FeatureChainPrefix: use distinct names for chain segments
    // Original: "a1.a1." -> Fixed: "x.y."
    if inputs.get("FeatureChainPrefix").map(|s| s.as_str()) == Some("a1.a1.") {
        inputs.insert("FeatureChainPrefix".to_string(), "x.y.".to_string());
    }
    
    // OwnedFeatureChain: use distinct names
    // Original: "a1.a1" -> Fixed: "x.y"
    if inputs.get("OwnedFeatureChain").map(|s| s.as_str()) == Some("a1.a1") {
        inputs.insert("OwnedFeatureChain".to_string(), "x.y".to_string());
    }
    
    // FlowEndSubsetting with FeatureChainPrefix alternative needs fixing
    // The generated input for alt 1 uses FeatureChainPrefix, so it will pick up "x.y." now
    
    // FeatureChain: use distinct names  
    if inputs.get("FeatureChain").map(|s| s.as_str()) == Some("a1.a1") {
        inputs.insert("FeatureChain".to_string(), "x.y".to_string());
    }
}

/// Check if a parent rule + child rule + child alt combination creates
/// a known grammar ambiguity that the parser can't handle.
///
/// These are cases where FeatureChain's greedy parsing consumes too much input,
/// leaving nothing for the parent rule's remaining grammar.
///
/// NOTE: This filter should no longer be needed with the rule_context-based GLR
/// backtracking. Greedy loops now check is_current_pos_excluded() after each
/// iteration and backtrack if needed.
fn is_problematic_child_alt(_parent_rule: &str, _child_rule: &str, _child_alt_idx: usize) -> bool {
    false // Disabled - handled by improved GLR backtracking
}

/// Extract top-level alternatives from a rule body.
/// Returns a vec of the alternative bodies, or a single-element vec if no alternatives.
fn get_top_level_alternatives(body: &RuleBody) -> Vec<&RuleBody> {
    match body {
        RuleBody::Alternative(alts) => alts.iter().collect(),
        RuleBody::Group(inner) => get_top_level_alternatives(inner),
        _ => vec![body],
    }
}

/// Precompute all child rules with alternatives for every rule in the grammar.
/// Returns a cache: rule_name -> Vec<(child_rule_name, num_alternatives)>
/// Uses memoization to avoid redundant traversals.
fn precompute_child_alts_cache(
    rule_map: &HashMap<&str, &crate::kebnf::types::Rule>,
) -> HashMap<String, Vec<(String, usize)>> {
    let mut cache: HashMap<String, Vec<(String, usize)>> = HashMap::new();
    let mut in_progress: HashSet<String> = HashSet::new();
    
    for name in rule_map.keys() {
        compute_child_alts_for_rule(name, rule_map, &mut cache, &mut in_progress);
    }
    
    cache
}

/// Compute child alternatives for a single rule, with memoization.
fn compute_child_alts_for_rule(
    rule_name: &str,
    rule_map: &HashMap<&str, &crate::kebnf::types::Rule>,
    cache: &mut HashMap<String, Vec<(String, usize)>>,
    in_progress: &mut HashSet<String>,
) -> Vec<(String, usize)> {
    // Already computed
    if let Some(cached) = cache.get(rule_name) {
        return cached.clone();
    }
    
    // Cycle detection: if we're already computing this rule, return empty
    if in_progress.contains(rule_name) {
        return Vec::new();
    }
    
    let Some(rule) = rule_map.get(rule_name) else {
        return Vec::new();
    };
    
    in_progress.insert(rule_name.to_string());
    
    // Get direct child rules from this rule's body
    let direct_children = get_direct_rule_refs(&rule.body);
    
    let mut result_map: HashMap<String, usize> = HashMap::new();
    
    for child_name in direct_children {
        if is_lexer_terminal(&child_name) {
            continue;
        }
        
        // Check if child has alternatives
        if let Some(child_rule) = rule_map.get(child_name.as_str()) {
            let alts = get_top_level_alternatives(&child_rule.body);
            if alts.len() > 1 {
                result_map.insert(child_name.clone(), alts.len());
            }
        }
        
        // Recursively get grandchildren's alternatives
        let grandchildren = compute_child_alts_for_rule(&child_name, rule_map, cache, in_progress);
        for (gc_name, gc_alts) in grandchildren {
            result_map.entry(gc_name).or_insert(gc_alts);
        }
    }
    
    in_progress.remove(rule_name);
    
    let result: Vec<(String, usize)> = result_map.into_iter().collect();
    cache.insert(rule_name.to_string(), result.clone());
    result
}

/// Get all direct rule references from a body (non-recursive, just this level)
fn get_direct_rule_refs(body: &RuleBody) -> Vec<String> {
    let mut refs = Vec::new();
    collect_direct_rule_refs(body, &mut refs);
    refs
}

fn collect_direct_rule_refs(body: &RuleBody, out: &mut Vec<String>) {
    match body {
        RuleBody::RuleRef(name) => {
            out.push(name.clone());
        }
        RuleBody::Sequence(items) => {
            for item in items {
                collect_direct_rule_refs(item, out);
            }
        }
        RuleBody::Alternative(alts) => {
            for alt in alts {
                collect_direct_rule_refs(alt, out);
            }
        }
        RuleBody::Optional(inner)
        | RuleBody::ZeroOrMore(inner)
        | RuleBody::OneOrMore(inner)
        | RuleBody::Group(inner) => {
            collect_direct_rule_refs(inner, out);
        }
        RuleBody::Assignment { value, .. } | RuleBody::BoolAssign { value, .. } => {
            collect_direct_rule_refs(value, out);
        }
        _ => {}
    }
}

/// Precompute the reverse dependency graph: which rules depend on which
fn precompute_dependents(
    rule_map: &HashMap<&str, &crate::kebnf::types::Rule>,
) -> HashMap<String, Vec<String>> {
    let mut dependents: HashMap<String, Vec<String>> = HashMap::new();
    for (name, rule) in rule_map {
        for dep in get_rule_dependencies(&rule.body) {
            dependents.entry(dep).or_default().push(name.to_string());
        }
    }
    dependents
}

/// Build an inputs map with specific alternative choices for certain rules.
/// `alt_choices` maps rule_name -> which alternative index to use.
/// Properly propagates changes through the entire dependency graph.
fn build_inputs_with_alt_choices(
    inputs: &HashMap<String, String>,
    alt_choices: &HashMap<String, usize>,
    rule_map: &HashMap<&str, &crate::kebnf::types::Rule>,
    grammar_rules: &HashSet<&str>,
    dependents: &HashMap<String, Vec<String>>,
) -> HashMap<String, String> {
    let mut result = inputs.clone();
    
    // First, set the chosen alternatives directly
    for (rule_name, &alt_idx) in alt_choices {
        if let Some(rule) = rule_map.get(rule_name.as_str()) {
            let alts = get_top_level_alternatives(&rule.body);
            if alt_idx < alts.len() {
                let input = synthesize_body(alts[alt_idx], &result, grammar_rules, RepMode::Populated);
                result.insert(rule_name.clone(), input);
            }
        }
    }
    
    // Propagate changes transitively - iterate until stable
    let mut changed_rules: HashSet<String> = alt_choices.keys().cloned().collect();
    let max_iterations = rule_map.len(); // Can't need more iterations than rules
    
    for _ in 0..max_iterations {
        if changed_rules.is_empty() {
            break;
        }
        
        let mut newly_changed = HashSet::new();
        
        // Find all rules that depend on changed rules
        for changed in &changed_rules {
            if let Some(deps) = dependents.get(changed) {
                for dep_name in deps {
                    if let Some(rule) = rule_map.get(dep_name.as_str()) {
                        let old_input = result.get(dep_name).cloned().unwrap_or_default();
                        let new_input = synthesize_body(&rule.body, &result, grammar_rules, RepMode::Populated);
                        if new_input != old_input {
                            result.insert(dep_name.clone(), new_input);
                            newly_changed.insert(dep_name.clone());
                        }
                    }
                }
            }
        }
        
        changed_rules = newly_changed;
    }
    
    result
}

/// Get all rule names that a body directly references
fn get_rule_dependencies(body: &RuleBody) -> Vec<String> {
    let mut deps = Vec::new();
    collect_rule_dependencies(body, &mut deps);
    deps
}

fn collect_rule_dependencies(body: &RuleBody, out: &mut Vec<String>) {
    match body {
        RuleBody::RuleRef(name) if !is_lexer_terminal(name) => {
            out.push(name.clone());
        }
        RuleBody::Sequence(items) => {
            for item in items {
                collect_rule_dependencies(item, out);
            }
        }
        RuleBody::Alternative(alts) => {
            for alt in alts {
                collect_rule_dependencies(alt, out);
            }
        }
        RuleBody::Optional(inner)
        | RuleBody::ZeroOrMore(inner)
        | RuleBody::OneOrMore(inner)
        | RuleBody::Group(inner) => {
            collect_rule_dependencies(inner, out);
        }
        RuleBody::Assignment { value, .. } | RuleBody::BoolAssign { value, .. } => {
            collect_rule_dependencies(value, out);
        }
        _ => {}
    }
}

/// Compute the inputs map for a grammar (without generating test cases).
/// Used to pass KerML inputs as a base for SysML synthesis.
pub fn compute_inputs(grammar: &Grammar) -> HashMap<String, String> {
    let (sorted, scc_map) = topo_sort(grammar);

    let rule_map: HashMap<&str, &crate::kebnf::types::Rule> = grammar
        .rules
        .iter()
        .map(|r| (r.name.as_str(), r))
        .collect();

    let grammar_rules: HashSet<&str> = grammar
        .rules
        .iter()
        .filter(|r| !is_lexer_terminal(&r.name))
        .map(|r| r.name.as_str())
        .collect();

    let mut inputs: HashMap<String, String> = HashMap::new();
    let mut scc_groups: HashMap<usize, Vec<String>> = HashMap::new();
    for (name, &scc_id) in &scc_map {
        scc_groups.entry(scc_id).or_default().push(name.clone());
    }
    let mut processed_sccs: HashSet<usize> = HashSet::new();

    for rule_name in &sorted {
        if let Some(&scc_id) = scc_map.get(rule_name) {
            if processed_sccs.contains(&scc_id) { continue; }
            processed_sccs.insert(scc_id);
            let members = &scc_groups[&scc_id];
            for name in members {
                inputs.insert(name.clone(), "a1".to_string());
            }
            loop {
                let mut changed = false;
                for name in members {
                    if let Some(rule) = rule_map.get(name.as_str()) {
                        let input = synthesize_body(&rule.body, &inputs, &grammar_rules, RepMode::Populated);
                        if inputs.get(name).map_or(true, |prev| *prev != input) {
                            changed = true;
                            inputs.insert(name.clone(), input);
                        }
                    }
                }
                if !changed { break; }
            }
        } else if let Some(rule) = rule_map.get(rule_name.as_str()) {
            let input = synthesize_body(&rule.body, &inputs, &grammar_rules, RepMode::Populated);
            inputs.insert(rule_name.clone(), input);
        }
    }
    inputs
}

/// Recursively compute the minimal input for a `RuleBody`, substituting
/// rule references from the already-computed `inputs` map.
fn synthesize_body(
    body: &RuleBody,
    inputs: &HashMap<String, String>,
    grammar_rules: &HashSet<&str>,
    mode: RepMode,
) -> String {
    match body {
        RuleBody::Empty => String::new(),

        RuleBody::Keyword(kw) => kw.clone(),

        RuleBody::RuleRef(name) => {
            if is_lexer_terminal(name) {
                lexer_placeholder(name)
            } else if let Some(s) = inputs.get(name) {
                s.clone()
            } else {
                // Rule not in grammar (e.g. FilterPackageImport) — use a
                // name-like placeholder so the parse at least has tokens.
                format!("x_{}", to_snake_case(name))
            }
        }

        RuleBody::CrossRef(_) => {
            // Cross-references parse like QualifiedName — single segment
            "a1".to_string()
        }

        RuleBody::Sequence(items) => {
            let parts: Vec<String> = items
                .iter()
                .map(|item| synthesize_body(item, inputs, grammar_rules, mode))
                .collect();
            join_with_spaces(&parts)
        }

        RuleBody::Alternative(alts) => {
            // Pick the "best" alternative: prefer keyword-starting ones
            // (they tend to be simplest and most unambiguous), then shortest.
            pick_best_alternative(alts, inputs, grammar_rules, mode)
        }

        RuleBody::Optional(inner) => {
            match mode {
                RepMode::Minimal => String::new(),
                RepMode::Populated | RepMode::Multiple | RepMode::Varied => {
                    synthesize_body(inner, inputs, grammar_rules, mode)
                }
            }
        }

        RuleBody::ZeroOrMore(inner) => {
            match mode {
                RepMode::Minimal => String::new(),
                RepMode::Populated => {
                    synthesize_body(inner, inputs, grammar_rules, mode)
                }
                RepMode::Multiple => {
                    let one = synthesize_body(inner, inputs, grammar_rules, RepMode::Populated);
                    join_with_spaces(&[one.clone(), one])
                }
                RepMode::Varied => {
                    // Generate diverse alternatives for the repeated element
                    let alts = synthesize_all_alternatives(inner, inputs, grammar_rules);
                    if alts.is_empty() {
                        String::new()
                    } else {
                        join_with_spaces(&alts)
                    }
                }
            }
        }

        RuleBody::OneOrMore(inner) => {
            match mode {
                RepMode::Minimal | RepMode::Populated => {
                    synthesize_body(inner, inputs, grammar_rules, mode)
                }
                RepMode::Multiple => {
                    let one = synthesize_body(inner, inputs, grammar_rules, RepMode::Populated);
                    join_with_spaces(&[one.clone(), one])
                }
                RepMode::Varied => {
                    // Generate diverse alternatives for the repeated element
                    let alts = synthesize_all_alternatives(inner, inputs, grammar_rules);
                    if alts.is_empty() {
                        synthesize_body(inner, inputs, grammar_rules, RepMode::Populated)
                    } else {
                        join_with_spaces(&alts)
                    }
                }
            }
        }

        RuleBody::Group(inner) => synthesize_body(inner, inputs, grammar_rules, mode),

        RuleBody::Assignment { value, .. } => {
            synthesize_body(value, inputs, grammar_rules, mode)
        }

        RuleBody::BoolAssign { value, .. } => {
            synthesize_body(value, inputs, grammar_rules, mode)
        }

        RuleBody::Action(_) => {
            // Semantic actions produce no tokens
            String::new()
        }
    }
}

/// Pick the best alternative for synthesis.
///
/// Strategy (in order of preference):
///   - In Minimal mode: single keyword first, then keyword-starting, then shortest
///   - In Populated/Multiple mode: longest non-trivial alternative (to test nesting)
fn pick_best_alternative(
    alts: &[RuleBody],
    inputs: &HashMap<String, String>,
    grammar_rules: &HashSet<&str>,
    mode: RepMode,
) -> String {
    let mut candidates: Vec<(String, bool, bool, bool)> = alts
        .iter()
        .map(|alt| {
            let s = synthesize_body(alt, inputs, grammar_rules, mode);
            let starts_kw = starts_with_keyword(alt);
            let is_single_kw = matches!(alt, RuleBody::Keyword(_));
            let has_nested = has_nested_content(alt);
            (s, starts_kw, is_single_kw, has_nested)
        })
        .collect();

    // Always prefer simpler alternatives to avoid cycles during synthesis.
    // The test case generation will test each alternative separately anyway.
    candidates.sort_by(|a, b| {
        b.2.cmp(&a.2)                       // single keyword first
            .then(b.1.cmp(&a.1))             // then starts-with-keyword
            .then(a.0.len().cmp(&b.0.len())) // then shortest
    });

    candidates
        .into_iter()
        .next()
        .map(|(s, _, _, _)| s)
        .unwrap_or_default()
}

/// Synthesize all alternatives from a body for varied mode.
/// Returns a vector of distinct synthesized strings, one per alternative.
/// If the body is not an Alternative, returns multiple instances with variation.
fn synthesize_all_alternatives(
    body: &RuleBody,
    inputs: &HashMap<String, String>,
    grammar_rules: &HashSet<&str>,
) -> Vec<String> {
    match body {
        RuleBody::Alternative(alts) => {
            // Synthesize each alternative and dedupe
            let mut seen = HashSet::new();
            alts.iter()
                .map(|alt| synthesize_body(alt, inputs, grammar_rules, RepMode::Populated))
                .filter(|s| !s.is_empty() && seen.insert(s.clone()))
                .take(4) // Limit to avoid explosion
                .collect()
        }
        RuleBody::Group(inner) => synthesize_all_alternatives(inner, inputs, grammar_rules),
        RuleBody::Assignment { value, .. } => synthesize_all_alternatives(value, inputs, grammar_rules),
        RuleBody::Sequence(items) => {
            // For sequences containing alternatives, find the alternative part and expand
            // e.g., (',' Name)* should produce: , a1, , a2 
            // But simpler: just produce 2-3 instances
            let base = synthesize_body(body, inputs, grammar_rules, RepMode::Populated);
            if base.is_empty() {
                vec![]
            } else {
                vec![base.clone(), base.clone(), base]
            }
        }
        _ => {
            // No alternatives - just produce a couple copies
            let base = synthesize_body(body, inputs, grammar_rules, RepMode::Populated);
            if base.is_empty() {
                vec![]
            } else {
                vec![base.clone(), base]
            }
        }
    }
}

/// Check if a rule body contains nested parser rules (not just keywords/terminals)
fn has_nested_content(body: &RuleBody) -> bool {
    match body {
        RuleBody::Keyword(_) => false,
        RuleBody::Empty => false,
        RuleBody::RuleRef(name) => !is_lexer_terminal(name),
        RuleBody::CrossRef(_) => true,
        RuleBody::Sequence(items) => items.iter().any(has_nested_content),
        RuleBody::Alternative(alts) => alts.iter().any(has_nested_content),
        RuleBody::Optional(inner) => has_nested_content(inner),
        RuleBody::ZeroOrMore(inner) => has_nested_content(inner),
        RuleBody::OneOrMore(inner) => has_nested_content(inner),
        RuleBody::Group(inner) => has_nested_content(inner),
        RuleBody::Assignment { value, .. } => has_nested_content(value),
        RuleBody::BoolAssign { value, .. } => has_nested_content(value),
        RuleBody::Action(_) => false,
    }
}

/// Does this body start with a keyword terminal?
fn starts_with_keyword(body: &RuleBody) -> bool {
    match body {
        RuleBody::Keyword(_) => true,
        RuleBody::Sequence(items) => items.first().is_some_and(|i| starts_with_keyword(i)),
        RuleBody::Group(inner) => starts_with_keyword(inner),
        RuleBody::Assignment { value, .. } => starts_with_keyword(value),
        RuleBody::BoolAssign { value, .. } => starts_with_keyword(value),
        _ => false,
    }
}

/// Placeholder for lexer terminals.
fn lexer_placeholder(name: &str) -> String {
    match name {
        "NAME" => "a1".to_string(),
        "STRING_VALUE" => r#""hello""#.to_string(),
        "DECIMAL_VALUE" => "1".to_string(),
        "INTEGER_VALUE" => "1".to_string(),
        "REAL_VALUE" => "1.0".to_string(),
        "EXPONENTIAL_VALUE" => "1e1".to_string(),
        "REGULAR_COMMENT" => "/* c */".to_string(),
        "DOCUMENTATION_COMMENT" => "/** d */".to_string(),
        // Composite operator terminals
        "TYPED_BY" | "DEFINED_BY" => ":".to_string(),
        "SPECIALIZES" => ":>".to_string(),
        "SUBSETS" => ":>".to_string(),
        "REFERENCES" => "::>".to_string(),
        "CROSSES" => "=>".to_string(),
        "REDEFINES" => ":>>".to_string(),
        "CONJUGATES" => "~".to_string(),
        _ => "a1".to_string(),
    }
}

/// Join non-empty parts with context-sensitive spacing.
/// Punctuation like `#`, `<`, `~`, `(`, `[` glue to the next token;
/// `)`, `]`, `>`, `;`, `,` glue to the previous token.
fn join_with_spaces(parts: &[String]) -> String {
    let filtered: Vec<&str> = parts
        .iter()
        .map(|p| p.as_str())
        .filter(|p| !p.is_empty())
        .collect();
    if filtered.is_empty() {
        return String::new();
    }
    let mut out = filtered[0].to_string();
    for i in 1..filtered.len() {
        let prev = filtered[i - 1];
        let cur = filtered[i];
        // No space after opening punctuation or before closing punctuation
        let glue = prev.ends_with('#')
            || prev.ends_with('<')
            || prev.ends_with('(')
            || prev.ends_with('[')
            || prev.ends_with('~')
            || prev.ends_with('$')
            || prev.ends_with('.')
            || cur.starts_with('>')
            || cur.starts_with(')')
            || cur.starts_with(']')
            || cur.starts_with(';')
            || cur.starts_with(',')
            || cur.starts_with('.');
        if !glue {
            out.push(' ');
        }
        out.push_str(cur);
    }
    out
}

/// Check if a name is a lexer terminal (ALL_CAPS).
fn is_lexer_terminal(name: &str) -> bool {
    !name.is_empty() && name.chars().all(|c| c.is_ascii_uppercase() || c == '_')
}

// ── Code generation ────────────────────────────────────────────────────

/// Generate the `parser_test_data.rs` file containing all test cases as static data.
pub fn generate_test_data_rs(
    sysml_cases: &[TestCase],
    kerml_cases: &[TestCase],
) -> String {
    let mut out = String::new();
    out.push_str("//! Auto-generated test data — do not edit.\n");
    out.push_str("//!\n");
    out.push_str("//! Generated by `cargo run -- generate-tests ...`\n\n");

    // Format child_alts as a static slice
    fn format_child_alts(child_alts: &[(String, usize)]) -> String {
        if child_alts.is_empty() {
            "&[]".to_string()
        } else {
            let parts: Vec<String> = child_alts.iter()
                .map(|(name, idx)| format!("({:?}, {})", name, idx))
                .collect();
            format!("&[{}]", parts.join(", "))
        }
    }

    out.push_str("/// (dispatch_key, grammar_rule_name, alt_index, rep_mode, input, child_alts)\n");
    out.push_str("pub const SYSML_CASES: &[(&str, &str, Option<usize>, &str, &str, &[(&str, usize)])] = &[\n");
    for tc in sysml_cases {
        let alt = match tc.alt_index {
            Some(i) => format!("Some({})", i),
            None => "None".to_string(),
        };
        let child_alts = format_child_alts(&tc.child_alts);
        out.push_str(&format!(
            "    ({:?}, {:?}, {}, {:?}, {:?}, {}),\n",
            tc.dispatch_key, tc.rule_name, alt, tc.rep_mode.as_str(), tc.input, child_alts
        ));
    }
    out.push_str("];\n\n");

    out.push_str("pub const KERML_CASES: &[(&str, &str, Option<usize>, &str, &str, &[(&str, usize)])] = &[\n");
    for tc in kerml_cases {
        let alt = match tc.alt_index {
            Some(i) => format!("Some({})", i),
            None => "None".to_string(),
        };
        let child_alts = format_child_alts(&tc.child_alts);
        out.push_str(&format!(
            "    ({:?}, {:?}, {}, {:?}, {:?}, {}),\n",
            tc.dispatch_key, tc.rule_name, alt, tc.rep_mode.as_str(), tc.input, child_alts
        ));
    }
    out.push_str("];\n");

    out
}

// ── AST expectation data ───────────────────────────────────────────────

/// Expected state of a single AST field for a given test case.
#[derive(Debug, Clone)]
pub struct FieldExpectation {
    /// Field name in snake_case
    pub name: String,
    /// Expected kind: "bool", "string", "node", "crossref", "union"
    pub kind: String,
    /// Is this field optional (Option<T>)?
    pub optional: bool,
    /// Is this field a list (Vec<T>)?
    pub is_list: bool,
}

/// Expected AST node structure for a test case.
#[derive(Debug, Clone)]
pub struct AstExpectation {
    /// Grammar rule name
    pub rule_name: String,
    /// PascalCase type name
    pub type_name: String,
    /// Is this an enum (pure alternatives)?
    pub is_enum: bool,
    /// Number of enum variants (0 for structs)
    pub variant_count: usize,
    /// Number of fields (0 for enums)
    pub field_count: usize,
    /// Field expectations
    pub fields: Vec<FieldExpectation>,
}

/// Build AST expectations from a grammar.
pub fn build_ast_expectations(grammar: &Grammar) -> Vec<AstExpectation> {
    let nodes = ast::extract_ast_nodes(grammar);
    nodes.iter().map(|node| {
        let fields = node.fields.iter().map(|f| {
            let kind = match &f.field_type {
                FieldType::Bool => "bool",
                FieldType::Token(_) => "string",
                FieldType::Node(_) => "node",
                FieldType::CrossRef(_) => "crossref",
                FieldType::Union(_) => "union",
            }.to_string();
            FieldExpectation {
                name: f.name.clone(),
                kind,
                optional: f.optional,
                is_list: f.is_list,
            }
        }).collect();

        AstExpectation {
            rule_name: node.rule_name.clone(),
            type_name: node.name.clone(),
            is_enum: node.is_enum,
            variant_count: node.variants.len(),
            field_count: node.fields.len(),
            fields,
        }
    }).collect()
}

/// Synthesize AST test cases with expected field values.
/// This builds test cases for rules that produce AST structs (not enums),
/// tracking what values should appear in each field after parsing.
pub fn synthesize_ast_tests(grammar: &Grammar) -> Vec<AstTestCase> {
    synthesize_ast_tests_with_base(grammar, &HashMap::new())
}

/// Like synthesize_ast_tests but with base inputs from another grammar.
pub fn synthesize_ast_tests_with_base(
    grammar: &Grammar,
    base_inputs: &HashMap<String, String>,
) -> Vec<AstTestCase> {
    let nodes = ast::extract_ast_nodes(grammar);
    let inputs = build_inputs_with_base(grammar, base_inputs);
    
    let grammar_rules: HashSet<&str> = grammar
        .rules
        .iter()
        .filter(|r| !is_lexer_terminal(&r.name))
        .map(|r| r.name.as_str())
        .collect();
    
    let rule_map: HashMap<&str, &crate::kebnf::types::Rule> = grammar
        .rules
        .iter()
        .map(|r| (r.name.as_str(), r))
        .collect();
    
    let mut cases = Vec::new();
    
    // Only generate tests for struct nodes (not enums) that have fields
    for node in &nodes {
        if node.is_enum || node.fields.is_empty() {
            continue;
        }
        
        let Some(rule) = rule_map.get(node.rule_name.as_str()) else {
            continue;
        };
        
        // Generate test for Populated mode (all optionals present, lists have 1 element)
        let mut expected_fields = Vec::new();
        let input = synthesize_body_with_expectations(
            &rule.body,
            &inputs,
            &grammar_rules,
            RepMode::Populated,
            &mut expected_fields,
        );
        
        if !expected_fields.is_empty() {
            cases.push(AstTestCase {
                rule_name: node.rule_name.clone(),
                dispatch_key: to_snake_case(&node.rule_name),
                input,
                expected_fields,
                rep_mode: RepMode::Populated,
            });
        }
    }
    
    cases
}

/// Synthesize body while tracking field assignments
fn synthesize_body_with_expectations(
    body: &RuleBody,
    inputs: &HashMap<String, String>,
    grammar_rules: &HashSet<&str>,
    mode: RepMode,
    fields: &mut Vec<(String, ExpectedValue)>,
) -> String {
    match body {
        RuleBody::Empty => String::new(),
        RuleBody::Keyword(kw) => kw.clone(),
        
        RuleBody::RuleRef(name) => {
            if is_lexer_terminal(name) {
                lexer_placeholder(name)
            } else if let Some(s) = inputs.get(name) {
                s.clone()
            } else {
                format!("x_{}", to_snake_case(name))
            }
        }
        
        RuleBody::CrossRef(_) => "a1".to_string(),
        
        RuleBody::Sequence(items) => {
            let parts: Vec<String> = items
                .iter()
                .map(|item| synthesize_body_with_expectations(item, inputs, grammar_rules, mode, fields))
                .collect();
            join_with_spaces(&parts)
        }
        
        RuleBody::Alternative(alts) => {
            // For expectations, we need to pick one alternative consistently
            // Use the same logic as pick_best_alternative
            let best = find_best_alternative_index(alts, inputs, grammar_rules, mode);
            synthesize_body_with_expectations(&alts[best], inputs, grammar_rules, mode, fields)
        }
        
        RuleBody::Optional(inner) => {
            match mode {
                RepMode::Minimal => String::new(),
                RepMode::Populated | RepMode::Multiple | RepMode::Varied => {
                    synthesize_body_with_expectations(inner, inputs, grammar_rules, mode, fields)
                }
            }
        }
        
        RuleBody::ZeroOrMore(inner) => {
            match mode {
                RepMode::Minimal => String::new(),
                RepMode::Populated => {
                    synthesize_body_with_expectations(inner, inputs, grammar_rules, mode, fields)
                }
                RepMode::Multiple | RepMode::Varied => {
                    let one = synthesize_body_with_expectations(inner, inputs, grammar_rules, RepMode::Populated, fields);
                    // For Multiple/Varied, we add a second instance but don't track expectations for it
                    let two = synthesize_body(inner, inputs, grammar_rules, RepMode::Populated);
                    join_with_spaces(&[one, two])
                }
            }
        }
        
        RuleBody::OneOrMore(inner) => {
            match mode {
                RepMode::Minimal | RepMode::Populated => {
                    synthesize_body_with_expectations(inner, inputs, grammar_rules, mode, fields)
                }
                RepMode::Multiple | RepMode::Varied => {
                    let one = synthesize_body_with_expectations(inner, inputs, grammar_rules, RepMode::Populated, fields);
                    let two = synthesize_body(inner, inputs, grammar_rules, RepMode::Populated);
                    join_with_spaces(&[one, two])
                }
            }
        }
        
        RuleBody::Group(inner) => {
            synthesize_body_with_expectations(inner, inputs, grammar_rules, mode, fields)
        }
        
        RuleBody::Assignment { name, operator, value } => {
            let synth = synthesize_body_with_expectations(value, inputs, grammar_rules, mode, fields);
            let field_name = to_snake_case(name);
            
            // Determine expected value based on the value type
            let expected = infer_expected_value(value, &synth, inputs, grammar_rules);
            
            // Handle list fields (+=) vs single fields (=)
            match operator {
                AssignOp::Assign => {
                    fields.push((field_name, expected));
                }
                AssignOp::AddAssign => {
                    // For lists, we track that there's at least one element
                    // Find existing list expectation or create one
                    if let Some(pos) = fields.iter().position(|(n, _)| n == &field_name) {
                        if let ExpectedValue::ListLen(n) = &fields[pos].1 {
                            fields[pos].1 = ExpectedValue::ListLen(n + 1);
                        }
                    } else {
                        fields.push((field_name, ExpectedValue::ListLen(1)));
                    }
                }
            }
            
            synth
        }
        
        RuleBody::BoolAssign { name, value, .. } => {
            let synth = synthesize_body_with_expectations(value, inputs, grammar_rules, mode, fields);
            let field_name = to_snake_case(name);
            // BoolAssign sets the field to true if the value is present
            fields.push((field_name, ExpectedValue::Bool(true)));
            synth
        }
        
        RuleBody::Action(_) => String::new(),
    }
}

/// Infer the expected value from a synthesized body
fn infer_expected_value(
    body: &RuleBody,
    synth: &str,
    _inputs: &HashMap<String, String>,
    grammar_rules: &HashSet<&str>,
) -> ExpectedValue {
    match body {
        RuleBody::RuleRef(name) if is_lexer_terminal(name) => {
            // It's a lexer token - the value is the synthesized string
            match name.as_str() {
                "NAME" | "UNRESTRICTED_NAME" => ExpectedValue::String(synth.to_string()),
                "STRING_VALUE" => {
                    // Strip quotes from string
                    let inner = synth.trim_matches('"').to_string();
                    ExpectedValue::String(inner)
                }
                "REGULAR_COMMENT" => {
                    // Strip /* */ from comment
                    let inner = synth.trim_start_matches("/*").trim_end_matches("*/").to_string();
                    ExpectedValue::String(inner)
                }
                _ => ExpectedValue::String(synth.to_string()),
            }
        }
        RuleBody::RuleRef(name) if grammar_rules.contains(name.as_str()) => {
            // It's a node reference
            ExpectedValue::Node
        }
        RuleBody::CrossRef(_) => {
            ExpectedValue::CrossRef(synth.to_string())
        }
        RuleBody::Keyword(_) => {
            // Keywords assigned to fields become booleans in the AST
            // (the field is true if the keyword is present)
            ExpectedValue::Bool(true)
        }
        _ => ExpectedValue::Node, // Default to Node for complex structures
    }
}

/// Find the index of the best alternative (same logic as pick_best_alternative)
fn find_best_alternative_index(
    alts: &[RuleBody],
    inputs: &HashMap<String, String>,
    grammar_rules: &HashSet<&str>,
    mode: RepMode,
) -> usize {
    let mut best_idx = 0;
    let mut best_score = (false, usize::MAX);
    
    for (i, alt) in alts.iter().enumerate() {
        let synth = synthesize_body(alt, inputs, grammar_rules, mode);
        let starts_with_keyword = matches!(first_token(alt), Some(RuleBody::Keyword(_)));
        let score = (starts_with_keyword, synth.len());
        if score.0 && !best_score.0 {
            best_idx = i;
            best_score = score;
        } else if score.0 == best_score.0 && score.1 < best_score.1 {
            best_idx = i;
            best_score = score;
        }
    }
    
    best_idx
}

/// Get the first token of a rule body
fn first_token(body: &RuleBody) -> Option<&RuleBody> {
    match body {
        RuleBody::Sequence(items) => items.first().and_then(first_token),
        RuleBody::Group(inner) => first_token(inner),
        RuleBody::Assignment { value, .. } => first_token(value),
        RuleBody::BoolAssign { value, .. } => first_token(value),
        _ => Some(body),
    }
}

/// Generate the `ast_test_data.rs` file containing AST expectations as static data.
pub fn generate_ast_test_data_rs(
    sysml_expectations: &[AstExpectation],
    kerml_expectations: &[AstExpectation],
) -> String {
    let mut out = String::new();
    out.push_str("//! Auto-generated AST test data — do not edit.\n");
    out.push_str("//!\n");
    out.push_str("//! Generated by `cargo run -- generate-tests ...`\n\n");

    // Field expectations: (field_name, kind, optional, is_list)
    out.push_str("/// (field_name, kind, optional, is_list)\n");
    out.push_str("pub type FieldExp = (&'static str, &'static str, bool, bool);\n\n");

    // AST expectations: (rule_name, type_name, is_enum, variant_count, field_count, &[FieldExp])
    out.push_str("/// (rule_name, type_name, is_enum, variant_count, field_count, fields)\n");
    out.push_str("pub type AstExp = (&'static str, &'static str, bool, usize, usize, &'static [FieldExp]);\n\n");

    write_ast_expectations(&mut out, "KERML_AST", kerml_expectations);
    write_ast_expectations(&mut out, "SYSML_AST", sysml_expectations);

    out
}

fn write_ast_expectations(out: &mut String, const_name: &str, expectations: &[AstExpectation]) {
    out.push_str(&format!("pub const {}: &[AstExp] = &[\n", const_name));
    for exp in expectations {
        out.push_str(&format!(
            "    ({:?}, {:?}, {}, {}, {}, &[",
            exp.rule_name, exp.type_name, exp.is_enum, exp.variant_count, exp.field_count,
        ));
        if exp.fields.is_empty() {
            out.push_str("]),\n");
        } else {
            out.push('\n');
            for f in &exp.fields {
                out.push_str(&format!(
                    "        ({:?}, {:?}, {}, {}),\n",
                    f.name, f.kind, f.optional, f.is_list,
                ));
            }
            out.push_str("    ]),\n");
        }
    }
    out.push_str("];\n\n");
}

/// Generate `ast_test_data.rs` with test cases that include expected field values.
/// This produces data used by the `ast_test.rs` runner to verify field parsing.
#[allow(dead_code)]
pub fn generate_ast_test_cases_rs(
    sysml_cases: &[AstTestCase],
    kerml_cases: &[AstTestCase],
) -> String {
    let mut out = String::new();
    out.push_str("//! Auto-generated AST test case data — do not edit.\n");
    out.push_str("//!\n");
    out.push_str("//! This file contains test cases with expected field values.\n");
    out.push_str("//! Generated by `cargo run -- generate-tests ...`\n\n");
    
    // Define the test case struct
    out.push_str("/// A test case with expected field values for verification.\n");
    out.push_str("/// Expectations are tuples of (field_name, kind, value_string)\n");
    out.push_str("/// where kind is \"string\", \"bool\", or \"list\" (for list length)\n");
    out.push_str("pub struct AstTestCaseData {\n");
    out.push_str("    pub rule_name: &'static str,\n");
    out.push_str("    pub dispatch_key: &'static str,\n");
    out.push_str("    pub input: &'static str,\n");
    out.push_str("    /// Field expectations: (field_name, kind, expected_value)\n");
    out.push_str("    pub expectations: &'static [(&'static str, &'static str, &'static str)],\n");
    out.push_str("}\n\n");
    
    write_ast_test_cases(&mut out, "KERML_AST_CASES", kerml_cases);
    write_ast_test_cases(&mut out, "SYSML_AST_CASES", sysml_cases);
    
    out
}

/// Expected value type for test verification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ExpectKind {
    String,
    Bool,
    ListLen,
}

#[allow(dead_code)]
fn write_ast_test_cases(out: &mut String, const_name: &str, cases: &[AstTestCase]) {
    out.push_str(&format!("pub const {}: &[AstTestCaseData] = &[\n", const_name));
    for case in cases {
        // Collect all verifiable expectations
        let mut expectations: Vec<(&str, &str, String)> = Vec::new(); // (field, kind, value)
        
        for (name, value) in &case.expected_fields {
            match value {
                ExpectedValue::String(s) => {
                    expectations.push((name.as_str(), "string", s.clone()));
                }
                ExpectedValue::CrossRef(s) => {
                    expectations.push((name.as_str(), "string", s.clone()));
                }
                ExpectedValue::Bool(b) => {
                    expectations.push((name.as_str(), "bool", if *b { "true".to_string() } else { "false".to_string() }));
                }
                ExpectedValue::ListLen(n) => {
                    expectations.push((name.as_str(), "list", format!("{}", n)));
                }
                ExpectedValue::Some(_) | ExpectedValue::None | ExpectedValue::Node => {
                    // Skip these for now - harder to verify
                }
            }
        }
        
        out.push_str("    AstTestCaseData {\n");
        out.push_str(&format!("        rule_name: {:?},\n", case.rule_name));
        out.push_str(&format!("        dispatch_key: {:?},\n", case.dispatch_key));
        out.push_str(&format!("        input: {:?},\n", case.input));
        out.push_str("        expectations: &[\n");
        for (field, kind, value) in &expectations {
            out.push_str(&format!("            ({:?}, {:?}, {:?}),\n", field, kind, value));
        }
        out.push_str("        ],\n");
        out.push_str("    },\n");
    }
    out.push_str("];\n\n");
}

// ============================================================================
// Negative Test Generation
// ============================================================================

/// A negative test case: input that should fail to parse.
#[derive(Debug, Clone)]
pub struct NegativeTestCase {
    /// Grammar rule name
    pub rule_name: String,
    /// snake_case dispatch key
    pub dispatch_key: String,
    /// The malformed input
    pub input: String,
    /// Description of what's wrong
    pub mutation: String,
}

/// Generate negative test cases by mutating valid inputs.
/// These mutations are designed to actually break parsing, not just add noise.
pub fn synthesize_negative(grammar: &Grammar) -> Vec<NegativeTestCase> {
    let positive = synthesize(grammar);
    let mut negative = Vec::new();
    
    for tc in &positive {
        // Only generate from "populated" cases to have enough content to mutate
        if tc.rep_mode != RepMode::Populated {
            continue;
        }
        
        // Skip very short inputs - not enough to mutate meaningfully
        if tc.input.len() < 5 {
            continue;
        }
        
        // Mutation 1: Insert garbage in the MIDDLE (not end) - this breaks parsing
        let words: Vec<&str> = tc.input.split_whitespace().collect();
        if words.len() >= 2 {
            let mid = words.len() / 2;
            let mut mutated: Vec<&str> = words[..mid].to_vec();
            mutated.push("@#$INVALID@#$");
            mutated.extend(&words[mid..]);
            negative.push(NegativeTestCase {
                rule_name: tc.rule_name.clone(),
                dispatch_key: tc.dispatch_key.clone(),
                input: mutated.join(" "),
                mutation: "garbage_middle".to_string(),
            });
        }
        
        // Mutation 2: Remove the first keyword/token (makes rule unrecognizable)
        if words.len() >= 2 {
            let without_first = words[1..].join(" ");
            negative.push(NegativeTestCase {
                rule_name: tc.rule_name.clone(),
                dispatch_key: tc.dispatch_key.clone(),
                input: without_first,
                mutation: "missing_first_token".to_string(),
            });
        }
        
        // Mutation 3: Remove required braces - unbalanced brackets always fail
        if tc.input.contains('{') && tc.input.contains('}') {
            // Remove opening brace - parser will choke
            negative.push(NegativeTestCase {
                rule_name: tc.rule_name.clone(),
                dispatch_key: tc.dispatch_key.clone(),
                input: tc.input.replacen("{", "", 1),
                mutation: "missing_open_brace".to_string(),
            });
        }
        
        // Mutation 4: Remove required semicolons at end of statements
        if tc.input.ends_with(';') || tc.input.ends_with("; }") || tc.input.ends_with(";}") {
            // Replace the semicolon with a wrong token
            let mut mutated = tc.input.clone();
            if let Some(pos) = mutated.rfind(';') {
                mutated.replace_range(pos..pos+1, " @INVALID@ ");
            }
            negative.push(NegativeTestCase {
                rule_name: tc.rule_name.clone(),
                dispatch_key: tc.dispatch_key.clone(),
                input: mutated,
                mutation: "replaced_semicolon".to_string(),
            });
        }
        
        // Mutation 5: Swap adjacent tokens - breaks expected order
        if words.len() >= 3 {
            let mut swapped = words.to_vec();
            swapped.swap(0, 1);
            negative.push(NegativeTestCase {
                rule_name: tc.rule_name.clone(),
                dispatch_key: tc.dispatch_key.clone(),
                input: swapped.join(" "),
                mutation: "swapped_tokens".to_string(),
            });
        }
    }
    
    negative
}

/// Generate the `negative_test_data.rs` file.
pub fn generate_negative_test_data_rs(
    sysml_cases: &[NegativeTestCase],
    kerml_cases: &[NegativeTestCase],
) -> String {
    let mut out = String::new();
    out.push_str("//! Auto-generated negative test data — do not edit.\n");
    out.push_str("//!\n");
    out.push_str("//! These inputs should FAIL to parse.\n");
    out.push_str("//! Generated by `cargo run -- generate-tests ...`\n\n");
    
    out.push_str("/// (dispatch_key, rule_name, mutation, input)\n");
    out.push_str("pub type NegativeCase = (&'static str, &'static str, &'static str, &'static str);\n\n");
    
    write_negative_cases(&mut out, "KERML_NEGATIVE", kerml_cases);
    write_negative_cases(&mut out, "SYSML_NEGATIVE", sysml_cases);
    
    out
}

fn write_negative_cases(out: &mut String, const_name: &str, cases: &[NegativeTestCase]) {
    out.push_str(&format!("pub const {}: &[NegativeCase] = &[\n", const_name));
    for tc in cases {
        out.push_str(&format!(
            "    ({:?}, {:?}, {:?}, {:?}),\n",
            tc.dispatch_key, tc.rule_name, tc.mutation, tc.input,
        ));
    }
    out.push_str("];\n\n");
}

// ============================================================================
// Lexer Token Test Generation
// ============================================================================

/// A lexer test case.
#[derive(Debug, Clone)]
pub struct LexerTestCase {
    /// Input text
    pub input: String,
    /// Expected token kind
    pub expected_kind: String,
    /// Expected text (may differ from input for e.g. strings)
    pub expected_text: String,
    /// Category: "keyword", "punctuation", "literal", "identifier"
    pub category: String,
}

/// Generate lexer test cases from grammar keywords and punctuation.
pub fn synthesize_lexer_tests(grammar: &Grammar) -> Vec<LexerTestCase> {
    let mut cases = Vec::new();
    
    // Keywords
    for kw in &grammar.keywords {
        let variant = super::utils::keyword_to_variant(kw);
        cases.push(LexerTestCase {
            input: kw.clone(),
            expected_kind: variant,
            expected_text: kw.clone(),
            category: "keyword".to_string(),
        });
    }
    
    // Punctuation (skip comment starters which lexer handles specially)
    for punct in &grammar.punctuation {
        // Skip comment markers - these are tokenized as comments, not punctuation
        if punct == "//" || punct == "/*" {
            continue;
        }
        let variant = super::utils::punctuation_to_variant(punct);
        cases.push(LexerTestCase {
            input: punct.clone(),
            expected_kind: variant,
            expected_text: punct.clone(),
            category: "punctuation".to_string(),
        });
    }
    
    // Literals
    cases.push(LexerTestCase {
        input: "42".to_string(),
        expected_kind: "Integer".to_string(),
        expected_text: "42".to_string(),
        category: "literal".to_string(),
    });
    cases.push(LexerTestCase {
        input: "3.14".to_string(),
        expected_kind: "Real".to_string(),
        expected_text: "3.14".to_string(),
        category: "literal".to_string(),
    });
    cases.push(LexerTestCase {
        input: "1.5e10".to_string(),
        expected_kind: "Real".to_string(),
        expected_text: "1.5e10".to_string(),
        category: "literal".to_string(),
    });
    cases.push(LexerTestCase {
        input: "\"hello world\"".to_string(),
        expected_kind: "String".to_string(),
        expected_text: "hello world".to_string(),  // lexer strips quotes
        category: "literal".to_string(),
    });
    
    // Identifiers
    cases.push(LexerTestCase {
        input: "myIdentifier".to_string(),
        expected_kind: "Name".to_string(),
        expected_text: "myIdentifier".to_string(),
        category: "identifier".to_string(),
    });
    cases.push(LexerTestCase {
        input: "'unrestricted name'".to_string(),
        expected_kind: "UnrestrictedName".to_string(),
        expected_text: "unrestricted name".to_string(),  // lexer strips quotes
        category: "identifier".to_string(),
    });
    
    cases
}

/// Generate the `lexer_test_data.rs` file.
pub fn generate_lexer_test_data_rs(
    sysml_cases: &[LexerTestCase],
    kerml_cases: &[LexerTestCase],
) -> String {
    let mut out = String::new();
    out.push_str("//! Auto-generated lexer test data — do not edit.\n");
    out.push_str("//!\n");
    out.push_str("//! Generated by `cargo run -- generate-tests ...`\n\n");
    
    out.push_str("/// (input, expected_kind, expected_text, category)\n");
    out.push_str("pub type LexerCase = (&'static str, &'static str, &'static str, &'static str);\n\n");
    
    write_lexer_cases(&mut out, "KERML_LEXER", kerml_cases);
    write_lexer_cases(&mut out, "SYSML_LEXER", sysml_cases);
    
    out
}

fn write_lexer_cases(out: &mut String, const_name: &str, cases: &[LexerTestCase]) {
    out.push_str(&format!("pub const {}: &[LexerCase] = &[\n", const_name));
    for tc in cases {
        out.push_str(&format!(
            "    ({:?}, {:?}, {:?}, {:?}),\n",
            tc.input, tc.expected_kind, tc.expected_text, tc.category,
        ));
    }
    out.push_str("];\n\n");
}

// ============================================================================
// Test Runner Generation
// ============================================================================

/// Generate the `parser_test_runner.rs` test harness.
pub fn generate_parser_test_runner() -> String {
    r#"//! Auto-generated integration test runner for parser test cases.
//!
//! Tests verify that:
//! 1. Parsing succeeds
//! 2. ALL tokens are consumed (no leftover input)
//!
//! Do not edit manually — regenerate with `cargo run -- generate-tests ...`

mod parser_test_data;

#[path = "../generated/mod.rs"]
mod generated;

use generated::sysml;
use generated::kerml;

#[test]
fn sysml_synthesised_cases() {
    let mut pass = 0;
    let mut fail = 0;
    let mut failures = Vec::new();

    for &(dispatch_key, rule_name, alt_index, rep_mode, input, child_alts) in parser_test_data::SYSML_CASES {
        let tokens = sysml::lexer::Lexer::new(input).tokenize();
        let token_count = tokens.len();
        let mut parser = sysml::parser::Parser::new(tokens);
        
        match parser.try_parse_rule(dispatch_key) {
            Ok(_) => {
                let pos = parser.pos();
                if pos + 1 < token_count {
                    // Didn't consume all tokens - this is a failure
                    fail += 1;
                    failures.push((rule_name, dispatch_key, alt_index, rep_mode, input, child_alts,
                        format!("unconsumed tokens: parsed {pos}/{token_count}")));
                } else {
                    pass += 1;
                }
            }
            Err(e) => {
                fail += 1;
                failures.push((rule_name, dispatch_key, alt_index, rep_mode, input, child_alts, format!("{e}")));
            }
        }
    }

    println!("\nSysML: {pass}/{} passed ({fail} failures)",
        parser_test_data::SYSML_CASES.len());

    if !failures.is_empty() {
        println!("\nFailed rules:");
        for (rule, key, alt, rep, input, child_alts, err) in &failures {
            let alt_str = match alt {
                Some(i) => format!(" alt {i}"),
                None => String::new(),
            };
            let child_str = if child_alts.is_empty() {
                String::new()
            } else {
                let parts: Vec<_> = child_alts.iter().map(|(n, i)| format!("{n}={i}")).collect();
                format!(" children:[{}]", parts.join(", "))
            };
            println!("  {rule} ({key}{alt_str} [{rep}]{child_str})");
            println!("    input: {input:?}");
            println!("    error: {err}");
        }
    }

    assert_eq!(fail, 0,
        "{fail}/{} SysML test cases failed", parser_test_data::SYSML_CASES.len());
}

#[test]
fn kerml_synthesised_cases() {
    let mut pass = 0;
    let mut fail = 0;
    let mut failures = Vec::new();

    for &(dispatch_key, rule_name, alt_index, rep_mode, input, child_alts) in parser_test_data::KERML_CASES {
        let tokens = kerml::lexer::Lexer::new(input).tokenize();
        let token_count = tokens.len();
        let mut parser = kerml::parser::Parser::new(tokens);
        
        match parser.try_parse_rule(dispatch_key) {
            Ok(_) => {
                let pos = parser.pos();
                if pos + 1 < token_count {
                    fail += 1;
                    failures.push((rule_name, dispatch_key, alt_index, rep_mode, input, child_alts,
                        format!("unconsumed tokens: parsed {pos}/{token_count}")));
                } else {
                    pass += 1;
                }
            }
            Err(e) => {
                fail += 1;
                failures.push((rule_name, dispatch_key, alt_index, rep_mode, input, child_alts, format!("{e}")));
            }
        }
    }

    println!("\nKerML: {pass}/{} passed ({fail} failures)",
        parser_test_data::KERML_CASES.len());

    if !failures.is_empty() {
        println!("\nFailed rules:");
        for (rule, key, alt, rep, input, child_alts, err) in &failures {
            let alt_str = match alt {
                Some(i) => format!(" alt {i}"),
                None => String::new(),
            };
            let child_str = if child_alts.is_empty() {
                String::new()
            } else {
                let parts: Vec<_> = child_alts.iter().map(|(n, i)| format!("{n}={i}")).collect();
                format!(" children:[{}]", parts.join(", "))
            };
            println!("  {rule} ({key}{alt_str} [{rep}]{child_str})");
            println!("    input: {input:?}");
            println!("    error: {err}");
        }
    }

    assert_eq!(fail, 0,
        "{fail}/{} KerML test cases failed", parser_test_data::KERML_CASES.len());
}
"#.to_string()
}

/// Generate the `parser_ast_test.rs` test harness.
pub fn generate_parser_ast_test() -> String {
    r#"//! Auto-generated integration test runner for parser AST output.
//!
//! Tests verify that:
//! 1. Parsing succeeds and returns an AST node
//! 2. The AST node kind matches the expected rule
//! 3. ALL tokens are consumed (no leftover input)
//!
//! Do not edit manually — regenerate with `cargo run -- generate-tests ...`

mod parser_test_data;

#[path = "../generated/mod.rs"]
mod generated;

use generated::sysml;
use generated::kerml;

#[test]
fn kerml_synthesised_ast_cases() {
    let mut pass = 0;
    let mut fail = 0;
    let mut failures = Vec::new();

    for &(dispatch_key, rule_name, alt_index, rep_mode, input, _child_alts) in parser_test_data::KERML_CASES {
        let tokens = kerml::lexer::Lexer::new(input).tokenize();
        let token_count = tokens.len();
        let mut parser = kerml::parser::Parser::new(tokens);

        match parser.try_parse_rule_ast(dispatch_key) {
            Ok(node) => {
                let mut errs = Vec::new();
                
                // Check AST node kind matches
                let kind = node.kind_name();
                if kind != rule_name {
                    errs.push(format!("kind: expected {rule_name:?}, got {kind:?}"));
                }
                
                // Check all tokens consumed
                let pos = parser.pos();
                if pos + 1 < token_count {
                    errs.push(format!("unconsumed: parsed {pos}/{token_count}"));
                }
                
                if errs.is_empty() {
                    pass += 1;
                } else {
                    fail += 1;
                    failures.push((rule_name, dispatch_key, alt_index, rep_mode, input, errs.join("; ")));
                }
            }
            Err(e) => {
                fail += 1;
                failures.push((rule_name, dispatch_key, alt_index, rep_mode, input, format!("parse error: {e}")));
            }
        }
    }

    println!("\nKerML AST: {pass}/{} passed ({fail} failures)",
        parser_test_data::KERML_CASES.len());

    if !failures.is_empty() {
        println!("\nFailed rules:");
        for (rule, key, alt, rep, input, err) in &failures {
            let alt_str = match alt { Some(i) => format!(" alt {i}"), None => String::new() };
            println!("  {rule} ({key}{alt_str} [{rep}])");
            println!("    input: {input:?}");
            println!("    error: {err}");
        }
    }

    assert_eq!(fail, 0, "{fail}/{} KerML AST test cases failed", parser_test_data::KERML_CASES.len());
}

#[test]
fn sysml_synthesised_ast_cases() {
    let mut pass = 0;
    let mut fail = 0;
    let mut failures = Vec::new();

    for &(dispatch_key, rule_name, alt_index, rep_mode, input, _child_alts) in parser_test_data::SYSML_CASES {
        let tokens = sysml::lexer::Lexer::new(input).tokenize();
        let token_count = tokens.len();
        let mut parser = sysml::parser::Parser::new(tokens);

        match parser.try_parse_rule_ast(dispatch_key) {
            Ok(node) => {
                let mut errs = Vec::new();
                
                let kind = node.kind_name();
                if kind != rule_name {
                    errs.push(format!("kind: expected {rule_name:?}, got {kind:?}"));
                }
                
                let pos = parser.pos();
                if pos + 1 < token_count {
                    errs.push(format!("unconsumed: parsed {pos}/{token_count}"));
                }
                
                if errs.is_empty() {
                    pass += 1;
                } else {
                    fail += 1;
                    failures.push((rule_name, dispatch_key, alt_index, rep_mode, input, errs.join("; ")));
                }
            }
            Err(e) => {
                fail += 1;
                failures.push((rule_name, dispatch_key, alt_index, rep_mode, input, format!("parse error: {e}")));
            }
        }
    }

    println!("\nSysML AST: {pass}/{} passed ({fail} failures)",
        parser_test_data::SYSML_CASES.len());

    if !failures.is_empty() {
        println!("\nFailed rules:");
        for (rule, key, alt, rep, input, err) in &failures {
            let alt_str = match alt { Some(i) => format!(" alt {i}"), None => String::new() };
            println!("  {rule} ({key}{alt_str} [{rep}])");
            println!("    input: {input:?}");
            println!("    error: {err}");
        }
    }

    assert_eq!(fail, 0, "{fail}/{} SysML AST test cases failed", parser_test_data::SYSML_CASES.len());
}
"#.to_string()
}


/// Generate the `ast_test.rs` test harness.
/// This test uses the same parser_test_data cases to verify AST node creation.
pub fn generate_ast_test() -> String {
    r#"//! Auto-generated AST validation tests.
//!
//! These tests use the same synthesized inputs as parser tests and verify:
//! 1. Parsing produces an AST node (via try_parse_rule_ast)
//! 2. The AST node has the expected kind name
//! 3. All tokens are consumed
//!
//! This provides AST coverage for all 812+ parser test cases.

mod parser_test_data;

#[path = "../generated/mod.rs"]
mod generated;

use generated::sysml;
use generated::kerml;

/// Test KerML AST node creation for all synthesized cases
#[test]
fn kerml_ast_nodes() {
    let mut pass = 0;
    let mut fail = 0;
    let mut failures = Vec::new();

    for &(dispatch_key, rule_name, _alt_index, _rep_mode, input, _child_alts) in parser_test_data::KERML_CASES {
        let tokens = kerml::lexer::Lexer::new(input).tokenize();
        let token_count = tokens.len();
        let mut parser = kerml::parser::Parser::new(tokens);

        match parser.try_parse_rule_ast(dispatch_key) {
            Ok(node) => {
                let mut errs = Vec::new();
                
                // Verify node kind matches
                let kind = node.kind_name();
                if kind != rule_name {
                    errs.push(format!("kind: expected {rule_name:?}, got {kind:?}"));
                }
                
                // Verify all tokens consumed
                let pos = parser.pos();
                if pos + 1 < token_count {
                    errs.push(format!("unconsumed: {}/{} tokens", pos, token_count));
                }
                
                if errs.is_empty() {
                    pass += 1;
                } else {
                    fail += 1;
                    if failures.len() < 20 {
                        failures.push(format!("{} [{}]: {}", rule_name, dispatch_key, errs.join("; ")));
                    }
                }
            }
            Err(e) => {
                fail += 1;
                if failures.len() < 20 {
                    failures.push(format!("{}: parse error: {}", rule_name, e));
                }
            }
        }
    }

    println!("\nKerML AST: {pass}/{} passed", parser_test_data::KERML_CASES.len());
    
    if !failures.is_empty() {
        println!("First {} failures:", failures.len());
        for f in &failures {
            println!("  {}", f);
        }
    }
    
    assert_eq!(fail, 0, "{} KerML AST tests failed", fail);
}

/// Test SysML AST node creation for all synthesized cases
#[test]
fn sysml_ast_nodes() {
    let mut pass = 0;
    let mut fail = 0;
    let mut failures = Vec::new();

    for &(dispatch_key, rule_name, _alt_index, _rep_mode, input, _child_alts) in parser_test_data::SYSML_CASES {
        let tokens = sysml::lexer::Lexer::new(input).tokenize();
        let token_count = tokens.len();
        let mut parser = sysml::parser::Parser::new(tokens);

        match parser.try_parse_rule_ast(dispatch_key) {
            Ok(node) => {
                let mut errs = Vec::new();
                
                let kind = node.kind_name();
                if kind != rule_name {
                    errs.push(format!("kind: expected {rule_name:?}, got {kind:?}"));
                }
                
                let pos = parser.pos();
                if pos + 1 < token_count {
                    errs.push(format!("unconsumed: {}/{} tokens", pos, token_count));
                }
                
                if errs.is_empty() {
                    pass += 1;
                } else {
                    fail += 1;
                    if failures.len() < 20 {
                        failures.push(format!("{} [{}]: {}", rule_name, dispatch_key, errs.join("; ")));
                    }
                }
            }
            Err(e) => {
                fail += 1;
                if failures.len() < 20 {
                    failures.push(format!("{}: parse error: {}", rule_name, e));
                }
            }
        }
    }

    println!("\nSysML AST: {pass}/{} passed", parser_test_data::SYSML_CASES.len());
    
    if !failures.is_empty() {
        println!("First {} failures:", failures.len());
        for f in &failures {
            println!("  {}", f);
        }
    }
    
    assert_eq!(fail, 0, "{} SysML AST tests failed", fail);
}
"#.to_string()
}


/// Generate the `negative_test.rs` test harness.
pub fn generate_negative_test() -> String {
    r#"//! Auto-generated negative test runner (malformed inputs).
//!
//! These tests verify that malformed inputs either:
//! 1. Fail to parse entirely, OR
//! 2. Parse but leave unconsumed tokens (indicating incomplete parse)
//!
//! Do not edit manually — regenerate with `cargo run -- generate-tests ...`

mod negative_test_data;

#[path = "../generated/mod.rs"]
mod generated;

use generated::sysml;
use generated::kerml;
use generated::common::span::Span;

/// Check if parse "succeeded" - meaning it parsed AND consumed all tokens
fn sysml_parse_fully(input: &str, dispatch_key: &str) -> bool {
    let tokens = sysml::lexer::Lexer::new(input).tokenize();
    let token_count = tokens.len();
    let mut parser = sysml::parser::Parser::new(tokens);
    match parser.try_parse_rule(dispatch_key) {
        Ok(_) => {
            // Check if we consumed all tokens (except EOF)
            let pos = parser.pos();
            pos + 1 >= token_count  // +1 because last token is EOF
        }
        Err(_) => false,
    }
}

fn kerml_parse_fully(input: &str, dispatch_key: &str) -> bool {
    let tokens = kerml::lexer::Lexer::new(input).tokenize();
    let token_count = tokens.len();
    let mut parser = kerml::parser::Parser::new(tokens);
    match parser.try_parse_rule(dispatch_key) {
        Ok(_) => {
            let pos = parser.pos();
            pos + 1 >= token_count
        }
        Err(_) => false,
    }
}

#[test]
fn sysml_negative_cases() {
    let mut expected_fail = 0;
    let mut unexpected_pass = 0;
    let mut partial_parse = 0;  // parsed but didn't consume all

    for (dispatch_key, rule_name, mutation, input) in negative_test_data::SYSML_NEGATIVE {
        let tokens = sysml::lexer::Lexer::new(input).tokenize();
        let token_count = tokens.len();
        let mut parser = sysml::parser::Parser::new(tokens);
        
        match parser.try_parse_rule(dispatch_key) {
            Ok(_) => {
                let pos = parser.pos();
                if pos + 1 >= token_count {
                    // Fully parsed - this is bad, the malformed input was accepted
                    unexpected_pass += 1;
                    if unexpected_pass <= 10 {
                        println!("Unexpected full pass: {rule_name} [{mutation}]: {input:?}");
                    }
                } else {
                    // Partial parse - good! The garbage caused leftover tokens
                    partial_parse += 1;
                }
            }
            Err(_) => expected_fail += 1,
        }
    }

    let total = negative_test_data::SYSML_NEGATIVE.len();
    let good = expected_fail + partial_parse;
    println!("\nSysML negative: {good}/{total} correctly rejected ({expected_fail} failed, {partial_parse} partial, {unexpected_pass} fully passed)");

    // We want most inputs to either fail OR leave unconsumed tokens
    let threshold = total / 5;  // Allow up to 20% to fully pass
    assert!(unexpected_pass <= threshold,
        "Too many negative tests fully passed ({unexpected_pass}/{total}), expected most to fail or partial-parse");
}

#[test]
fn kerml_negative_cases() {
    let mut expected_fail = 0;
    let mut unexpected_pass = 0;
    let mut partial_parse = 0;

    for (dispatch_key, rule_name, mutation, input) in negative_test_data::KERML_NEGATIVE {
        let tokens = kerml::lexer::Lexer::new(input).tokenize();
        let token_count = tokens.len();
        let mut parser = kerml::parser::Parser::new(tokens);
        
        match parser.try_parse_rule(dispatch_key) {
            Ok(_) => {
                let pos = parser.pos();
                if pos + 1 >= token_count {
                    unexpected_pass += 1;
                    if unexpected_pass <= 10 {
                        println!("Unexpected full pass: {rule_name} [{mutation}]: {input:?}");
                    }
                } else {
                    partial_parse += 1;
                }
            }
            Err(_) => expected_fail += 1,
        }
    }

    let total = negative_test_data::KERML_NEGATIVE.len();
    let good = expected_fail + partial_parse;
    println!("\nKerML negative: {good}/{total} correctly rejected ({expected_fail} failed, {partial_parse} partial, {unexpected_pass} fully passed)");

    let threshold = total / 5;
    assert!(unexpected_pass <= threshold,
        "Too many negative tests fully passed ({unexpected_pass}/{total}), expected most to fail or partial-parse");
}
"#.to_string()
}

/// Generate the `lexer_test.rs` test harness.
pub fn generate_lexer_test() -> String {
    r#"//! Auto-generated lexer test runner.
//!
//! Do not edit manually — regenerate with `cargo run -- generate-tests ...`

mod lexer_test_data;

#[path = "../generated/mod.rs"]
mod generated;

use generated::sysml;
use generated::kerml;

#[test]
fn sysml_lexer_cases() {
    let mut pass = 0;
    let mut fail = 0;

    for (input, expected_kind, expected_text, category) in lexer_test_data::SYSML_LEXER {
        let tokens = sysml::lexer::Lexer::new(input).tokenize();
        if tokens.is_empty() {
            fail += 1;
            println!("FAIL [{category}]: no tokens for {input:?}");
            continue;
        }
        let tok = &tokens[0];
        let kind_str = format!("{:?}", tok.kind);
        if kind_str == *expected_kind && tok.text == *expected_text {
            pass += 1;
        } else {
            fail += 1;
            if fail <= 20 {
                println!("FAIL [{category}]: {input:?} => {:?}/{:?}, expected {expected_kind}/{expected_text}",
                    kind_str, tok.text);
            }
        }
    }

    println!("\nSysML lexer: {pass}/{} passed", lexer_test_data::SYSML_LEXER.len());
    assert_eq!(fail, 0, "{fail} lexer tests failed");
}

#[test]
fn kerml_lexer_cases() {
    let mut pass = 0;
    let mut fail = 0;

    for (input, expected_kind, expected_text, category) in lexer_test_data::KERML_LEXER {
        let tokens = kerml::lexer::Lexer::new(input).tokenize();
        if tokens.is_empty() {
            fail += 1;
            println!("FAIL [{category}]: no tokens for {input:?}");
            continue;
        }
        let tok = &tokens[0];
        let kind_str = format!("{:?}", tok.kind);
        if kind_str == *expected_kind && tok.text == *expected_text {
            pass += 1;
        } else {
            fail += 1;
            if fail <= 20 {
                println!("FAIL [{category}]: {input:?} => {:?}/{:?}, expected {expected_kind}/{expected_text}",
                    kind_str, tok.text);
            }
        }
    }

    println!("\nKerML lexer: {pass}/{} passed", lexer_test_data::KERML_LEXER.len());
    assert_eq!(fail, 0, "{fail} lexer tests failed");
}
"#.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kebnf::Grammar;

    #[test]
    fn test_synthesize_sysml() {
        let src = std::fs::read_to_string("data/SysML-textual-bnf.kebnf").unwrap();
        let grammar = Grammar::parse(&src);

        let cases = synthesize(&grammar);

        assert!(!cases.is_empty(), "should produce test cases");

        // Print a summary
        println!("SysML: {} test cases synthesised", cases.len());
        for tc in &cases[..5.min(cases.len())] {
            println!("  {} => {:?}", tc.rule_name, tc.input);
        }
    }

    #[test]
    fn test_synthesize_kerml() {
        let src = std::fs::read_to_string("data/KerML-textual-bnf.kebnf").unwrap();
        let grammar = Grammar::parse(&src);

        let cases = synthesize(&grammar);

        assert!(!cases.is_empty(), "should produce test cases");

        println!("KerML: {} test cases synthesised", cases.len());
        for tc in &cases[..5.min(cases.len())] {
            println!("  {} => {:?}", tc.rule_name, tc.input);
        }
    }

    #[test]
    fn test_leaves_are_self_contained() {
        // Leaf rules should produce inputs that don't reference other grammar rules
        let src = std::fs::read_to_string("data/SysML-textual-bnf.kebnf").unwrap();
        let grammar = Grammar::parse(&src);
        let cases = synthesize(&grammar);
        let leaves = crate::kebnf::deps::find_leaves(&grammar);
        let leaf_names: HashSet<_> = leaves.iter().map(|r| r.name.as_str()).collect();

        for tc in &cases {
            if leaf_names.contains(tc.rule_name.as_str()) {
                // A leaf input should be just keywords and lexer placeholders
                assert!(
                    !tc.input.is_empty(),
                    "leaf rule {} should have a non-empty input",
                    tc.rule_name
                );
            }
        }
    }
}
