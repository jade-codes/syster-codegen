//! Auto-generated integration test runner for parser test cases.
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
