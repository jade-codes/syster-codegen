//! Auto-generated integration test runner for parser AST output.
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
