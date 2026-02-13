//! Auto-generated lexer test runner.
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
