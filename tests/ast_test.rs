//! Auto-generated AST validation tests.
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
