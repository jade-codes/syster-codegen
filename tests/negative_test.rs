//! Auto-generated negative test runner (malformed inputs).
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
