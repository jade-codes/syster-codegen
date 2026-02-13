//! Dependency analysis for grammar rules

use std::collections::{HashMap, HashSet};
use super::types::{Grammar, Rule, RuleBody};

/// Extract rule references from a RuleBody recursively
fn extract_refs_from_body(body: &RuleBody, refs: &mut HashSet<String>) {
    match body {
        RuleBody::RuleRef(name) => {
            refs.insert(name.clone());
        }
        RuleBody::CrossRef(name) => {
            refs.insert(name.clone());
        }
        RuleBody::Sequence(items) => {
            for item in items {
                extract_refs_from_body(item, refs);
            }
        }
        RuleBody::Alternative(alts) => {
            for alt in alts {
                extract_refs_from_body(alt, refs);
            }
        }
        RuleBody::Optional(inner)
        | RuleBody::ZeroOrMore(inner)
        | RuleBody::OneOrMore(inner)
        | RuleBody::Group(inner) => {
            extract_refs_from_body(inner, refs);
        }
        RuleBody::Assignment { value, .. }
        | RuleBody::BoolAssign { value, .. } => {
            extract_refs_from_body(value, refs);
        }
        RuleBody::Empty | RuleBody::Keyword(_) | RuleBody::Action(_) => {}
    }
}

/// Get all rule references from a single parsed Rule
pub fn get_rule_refs(rule: &Rule) -> HashSet<String> {
    let mut refs = HashSet::new();
    extract_refs_from_body(&rule.body, &mut refs);
    refs
}

/// Check if a name is a lexer terminal (ALL_CAPS)
fn is_lexer_terminal(name: &str) -> bool {
    !name.is_empty() && name.chars().all(|c| c.is_ascii_uppercase() || c == '_')
}

/// Find root rules in a Grammar (rules not referenced by any other rule)
pub fn find_roots(grammar: &Grammar) -> Vec<&Rule> {
    let rule_names: HashSet<_> = grammar.rules.iter().map(|r| r.name.as_str()).collect();
    
    // Collect all references across all rules (only to known grammar rules)
    let all_refs: HashSet<String> = grammar.rules.iter()
        .flat_map(|r| get_rule_refs(r))
        .filter(|name| rule_names.contains(name.as_str()))
        .collect();
    
    grammar.rules.iter()
        .filter(|r| !all_refs.contains(&r.name))
        .collect()
}

/// Find leaf rules in a Grammar (rules that don't reference other grammar rules)
pub fn find_leaves(grammar: &Grammar) -> Vec<&Rule> {
    let rule_names: HashSet<_> = grammar.rules.iter().map(|r| r.name.as_str()).collect();
    
    grammar.rules.iter()
        .filter(|r| {
            let refs = get_rule_refs(r);
            // A leaf has no references to other grammar rules (may reference lexer terminals)
            refs.iter().all(|ref_name| {
                !rule_names.contains(ref_name.as_str()) || is_lexer_terminal(ref_name)
            })
        })
        .collect()
}

/// Topological sort of grammar rules (leaves first, roots last).
///
/// Rules in cycles (SCCs with more than one member) are grouped together
/// and placed at the position of the last member to be visited.
/// Returns `(sorted_names, sccs)` where:
///  - `sorted_names` is every non-lexer rule name in bottom-up order
///  - `sccs` maps each rule that belongs to a cycle to its SCC id
pub fn topo_sort(grammar: &Grammar) -> (Vec<String>, HashMap<String, usize>) {
    let rule_names: HashSet<_> = grammar.rules.iter()
        .filter(|r| !is_lexer_terminal(&r.name))
        .map(|r| r.name.as_str())
        .collect();

    // Build adjacency: rule -> set of grammar rules it references
    let mut deps: HashMap<&str, HashSet<&str>> = HashMap::new();
    // We need owned refs so the borrows live long enough
    let mut all_refs: HashMap<String, HashSet<String>> = HashMap::new();
    for rule in &grammar.rules {
        if is_lexer_terminal(&rule.name) { continue; }
        all_refs.insert(rule.name.clone(), get_rule_refs(rule));
    }
    for rule in &grammar.rules {
        if is_lexer_terminal(&rule.name) { continue; }
        let refs = &all_refs[&rule.name];
        let filtered: HashSet<&str> = refs.iter()
            .filter_map(|r| {
                let s = r.as_str();
                if rule_names.contains(s) && s != rule.name.as_str() { Some(s) } else { None }
            })
            .collect();
        deps.insert(rule.name.as_str(), filtered);
    }

    // Tarjan's SCC — gives us both cycle detection and reverse-topo order
    let mut index_counter = 0u32;
    let mut stack: Vec<&str> = Vec::new();
    let mut on_stack: HashSet<&str> = HashSet::new();
    let mut indices: HashMap<&str, u32> = HashMap::new();
    let mut lowlinks: HashMap<&str, u32> = HashMap::new();
    let mut scc_list: Vec<Vec<String>> = Vec::new();

    fn strongconnect<'a>(
        v: &'a str,
        deps: &HashMap<&str, HashSet<&'a str>>,
        index_counter: &mut u32,
        stack: &mut Vec<&'a str>,
        on_stack: &mut HashSet<&'a str>,
        indices: &mut HashMap<&'a str, u32>,
        lowlinks: &mut HashMap<&'a str, u32>,
        scc_list: &mut Vec<Vec<String>>,
    ) {
        indices.insert(v, *index_counter);
        lowlinks.insert(v, *index_counter);
        *index_counter += 1;
        stack.push(v);
        on_stack.insert(v);

        if let Some(neighbors) = deps.get(v) {
            for &w in neighbors {
                if !indices.contains_key(w) {
                    strongconnect(w, deps, index_counter, stack, on_stack, indices, lowlinks, scc_list);
                    let wl = lowlinks[w];
                    let vl = lowlinks.get_mut(v).unwrap();
                    if wl < *vl { *vl = wl; }
                } else if on_stack.contains(w) {
                    let wi = indices[w];
                    let vl = lowlinks.get_mut(v).unwrap();
                    if wi < *vl { *vl = wi; }
                }
            }
        }

        if lowlinks[v] == indices[v] {
            let mut component = Vec::new();
            loop {
                let w = stack.pop().unwrap();
                on_stack.remove(w);
                component.push(w.to_string());
                if w == v { break; }
            }
            scc_list.push(component);
        }
    }

    // Visit all rules
    let all_names: Vec<&str> = grammar.rules.iter()
        .filter(|r| !is_lexer_terminal(&r.name))
        .map(|r| r.name.as_str())
        .collect();

    for &name in &all_names {
        if !indices.contains_key(name) {
            strongconnect(name, &deps, &mut index_counter, &mut stack, &mut on_stack,
                          &mut indices, &mut lowlinks, &mut scc_list);
        }
    }

    // Tarjan's gives SCCs in reverse topo order — scc_list[0] is a leaf SCC.
    // Flatten into a single ordered list: leaves first.
    let mut sorted = Vec::new();
    let mut scc_map: HashMap<String, usize> = HashMap::new();

    for (scc_id, component) in scc_list.iter().enumerate() {
        if component.len() > 1 {
            for name in component {
                scc_map.insert(name.clone(), scc_id);
            }
        }
        for name in component {
            sorted.push(name.clone());
        }
    }

    (sorted, scc_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_roots_and_leaves() {
        let sysml = std::fs::read_to_string("data/SysML-textual-bnf.kebnf").unwrap();
        let grammar = Grammar::parse(&sysml);
        
        let roots = find_roots(&grammar);
        let leaves = find_leaves(&grammar);
        
        assert!(!roots.is_empty(), "Should have root rules");
        assert!(!leaves.is_empty(), "Should have leaf rules");
        
        // Roots should be fewer than total rules
        assert!(roots.len() < grammar.rules.len());
    }

    #[test]
    fn test_get_rule_refs() {
        let sysml = std::fs::read_to_string("data/SysML-textual-bnf.kebnf").unwrap();
        let grammar = Grammar::parse(&sysml);
        
        // Find Package rule
        let package_rule = grammar.rules.iter().find(|r| r.name == "Package").unwrap();
        let refs = get_rule_refs(package_rule);
        
        // Package should reference other rules
        assert!(!refs.is_empty());
    }

    #[test]
    fn test_topo_sort_leaves_before_roots() {
        let sysml = std::fs::read_to_string("data/SysML-textual-bnf.kebnf").unwrap();
        let grammar = Grammar::parse(&sysml);

        let (sorted, sccs) = topo_sort(&grammar);

        // Every non-lexer rule should appear exactly once
        let rule_names: HashSet<_> = grammar.rules.iter()
            .filter(|r| !is_lexer_terminal(&r.name))
            .map(|r| r.name.as_str())
            .collect();
        assert_eq!(sorted.len(), rule_names.len(),
            "topo sort should contain all non-lexer rules");

        // For every rule, all of its non-cyclic deps should appear earlier
        let pos: HashMap<&str, usize> = sorted.iter()
            .enumerate()
            .map(|(i, n)| (n.as_str(), i))
            .collect();
        for rule in &grammar.rules {
            if is_lexer_terminal(&rule.name) { continue; }
            let my_pos = pos[rule.name.as_str()];
            let my_scc = sccs.get(&rule.name);
            for dep in get_rule_refs(rule) {
                if is_lexer_terminal(&dep) { continue; }
                if !rule_names.contains(dep.as_str()) { continue; }
                if dep == rule.name { continue; } // self-ref
                let dep_scc = sccs.get(&dep);
                // If same SCC (cycle), order doesn't matter
                if my_scc.is_some() && dep_scc.is_some() && my_scc == dep_scc {
                    continue;
                }
                let dep_pos = pos.get(dep.as_str()).expect(
                    &format!("{} references {} which is not in sorted list", rule.name, dep));
                assert!(*dep_pos < my_pos,
                    "{} (pos {}) depends on {} (pos {}) but dep comes later",
                    rule.name, my_pos, dep, dep_pos);
            }
        }

        println!("Topo sort: {} rules, {} in cycles ({} SCCs)",
            sorted.len(), sccs.len(),
            sccs.values().collect::<HashSet<_>>().len());
    }
}
