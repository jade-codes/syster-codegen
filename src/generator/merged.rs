//! Merged type information from KerML and SysML grammars
//!
//! Used only for shared token/keyword/punctuation sets and rule classification.
//! Each language has its own AST — no merged AST needed.

use std::collections::HashSet;
use crate::kebnf::Grammar;

/// Merged info from both grammars (tokens, keywords, rule sets)
#[derive(Debug)]
pub struct MergedTypes {
    /// All rule names
    pub all_rule_names: HashSet<String>,
    /// KerML-only rules
    pub kerml_only_rules: HashSet<String>,
    /// SysML-only rules  
    pub sysml_only_rules: HashSet<String>,
    /// Shared rules
    pub shared_rules: HashSet<String>,
    /// All keywords (union of both)
    pub all_keywords: HashSet<String>,
    /// All punctuation (union of both)
    pub all_punctuation: HashSet<String>,
    /// KerML keywords
    pub kerml_keywords: HashSet<String>,
    /// SysML keywords
    pub sysml_keywords: HashSet<String>,
}

impl MergedTypes {
    /// Build from grammars
    pub fn from_grammars(kerml: &Grammar, sysml: &Grammar) -> Self {
        let kerml_names: HashSet<_> = kerml.rules.iter().map(|r| r.name.clone()).collect();
        let sysml_names: HashSet<_> = sysml.rules.iter().map(|r| r.name.clone()).collect();
        
        let kerml_only_rules = kerml_names.difference(&sysml_names).cloned().collect();
        let sysml_only_rules = sysml_names.difference(&kerml_names).cloned().collect();
        let shared_rules = kerml_names.intersection(&sysml_names).cloned().collect();
        let all_rule_names = kerml_names.union(&sysml_names).cloned().collect();
        
        let all_keywords = kerml.keywords.union(&sysml.keywords).cloned().collect();
        let all_punctuation = kerml.punctuation.union(&sysml.punctuation).cloned().collect();
        
        MergedTypes {
            all_rule_names,
            kerml_only_rules,
            sysml_only_rules,
            shared_rules,
            all_keywords,
            all_punctuation,
            kerml_keywords: kerml.keywords.clone(),
            sysml_keywords: sysml.keywords.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_merged_types() {
        let kerml = Grammar::parse(include_str!("../../data/KerML-textual-bnf.kebnf"));
        let sysml = Grammar::parse(include_str!("../../data/SysML-textual-bnf.kebnf"));
        
        let merged = MergedTypes::from_grammars(&kerml, &sysml);
        
        println!("KerML: {}", kerml.rules.len());
        println!("SysML: {}", sysml.rules.len());
        println!("KerML-only: {}", merged.kerml_only_rules.len());
        println!("SysML-only: {}", merged.sysml_only_rules.len());
        println!("Shared: {}", merged.shared_rules.len());
    }
}
