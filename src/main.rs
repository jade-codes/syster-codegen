//! CLI for syster-codegen

use std::path::PathBuf;
use clap::{Parser, Subcommand};
use syster_codegen::kebnf::Grammar;
use syster_codegen::generator::{self, GenerateConfig};

#[derive(Parser)]
#[command(name = "syster-codegen")]
#[command(about = "Parser generator for SysML v2")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate parser from KEBNF grammar (new structure with kerml/sysml)
    Generate {
        /// Path to KerML KEBNF grammar file
        #[arg(long, required = true)]
        kerml: PathBuf,
        
        /// Path to SysML KEBNF grammar file
        #[arg(long, required = true)]
        sysml: PathBuf,
        
        /// Output directory for generated code
        #[arg(short, long)]
        output: PathBuf,
    },
    
    /// Generate parser from single grammar (legacy mode)
    GenerateLegacy {
        /// Path to KEBNF grammar file(s)
        #[arg(short, long, required = true)]
        grammar: Vec<PathBuf>,
        
        /// Output directory for generated code
        #[arg(short, long)]
        output: PathBuf,
    },
    
    /// Generate deterministic test data from grammar rules
    GenerateTests {
        /// Path to KerML KEBNF grammar file
        #[arg(long, required = true)]
        kerml: PathBuf,

        /// Path to SysML KEBNF grammar file
        #[arg(long, required = true)]
        sysml: PathBuf,

        /// Output path for parser_test_data.rs
        #[arg(short, long, default_value = "tests/parser_test_data.rs")]
        output: PathBuf,
    },

    /// Show grammar statistics
    Stats {
        /// Path to KEBNF grammar file(s)
        #[arg(short, long, required = true)]
        grammar: Vec<PathBuf>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Generate { kerml, sysml, output } => {
            println!("Loading grammar files...");
            let kerml_source = std::fs::read_to_string(&kerml)?;
            let sysml_source = std::fs::read_to_string(&sysml)?;
            
            let kerml_grammar = Grammar::parse(&kerml_source);
            let sysml_grammar = Grammar::parse(&sysml_source);
            
            println!("KerML: {} rules, {} keywords", 
                kerml_grammar.rules.len(), kerml_grammar.keywords.len());
            println!("SysML: {} rules, {} keywords",
                sysml_grammar.rules.len(), sysml_grammar.keywords.len());
            
            println!("Generating to {:?}...", output);
            
            let config = GenerateConfig {
                kerml_grammar: &kerml_grammar,
                sysml_grammar: &sysml_grammar,
                output_dir: &output,
            };
            generator::generate_all(&config)?;
            
            println!("Done!");
        }
        
        Commands::GenerateLegacy { grammar, output } => {
            println!("Loading grammar files...");
            let sources: Vec<String> = grammar.iter()
                .map(|p| std::fs::read_to_string(p))
                .collect::<Result<_, _>>()?;
            
            let source_refs: Vec<&str> = sources.iter().map(|s| s.as_str()).collect();
            let parsed = Grammar::parse_all(&source_refs);
            
            println!("Parsed {} rules, {} keywords, {} punctuation symbols",
                parsed.rules.len(),
                parsed.keywords.len(),
                parsed.punctuation.len());
            
            println!("Generating to {:?}...", output);
            generator::generate(&parsed, &output)?;
            
            println!("Done!");
        }
        
        Commands::GenerateTests { kerml, sysml, output } => {
            use syster_codegen::generator::test_synth;

            println!("Loading grammar files...");
            let kerml_source = std::fs::read_to_string(&kerml)?;
            let sysml_source = std::fs::read_to_string(&sysml)?;

            let kerml_grammar = Grammar::parse(&kerml_source);
            let sysml_grammar = Grammar::parse(&sysml_source);

            println!("KerML: {} rules", kerml_grammar.rules.len());
            println!("SysML: {} rules", sysml_grammar.rules.len());

            // Ensure output directory exists
            if let Some(parent) = output.parent() {
                std::fs::create_dir_all(parent)?;
            }

            let kerml_cases = test_synth::synthesize(&kerml_grammar);
            // Pass KerML inputs as base for SysML (SysML references KerML rules)
            let kerml_inputs = test_synth::compute_inputs(&kerml_grammar);
            let sysml_cases = test_synth::synthesize_with_base(&sysml_grammar, &kerml_inputs);

            println!("Synthesised {} KerML + {} SysML = {} test cases",
                kerml_cases.len(), sysml_cases.len(),
                kerml_cases.len() + sysml_cases.len());

            let content = test_synth::generate_test_data_rs(&sysml_cases, &kerml_cases);
            std::fs::write(&output, &content)?;
            println!("Wrote {}", output.display());

            // Generate AST expectation data alongside parser test data
            // Build merged SysML grammar (KerML + SysML overrides)
            let sysml_merged = {
                use std::collections::HashMap;
                use syster_codegen::kebnf::types::Rule;
                let mut rule_map: HashMap<String, Rule> = HashMap::new();
                for rule in &kerml_grammar.rules {
                    rule_map.insert(rule.name.clone(), Rule {
                        name: rule.name.clone(),
                        return_type: rule.return_type.clone(),
                        body: rule.body.clone(),
                    });
                }
                for rule in &sysml_grammar.rules {
                    rule_map.insert(rule.name.clone(), Rule {
                        name: rule.name.clone(),
                        return_type: rule.return_type.clone(),
                        body: rule.body.clone(),
                    });
                }
                Grammar {
                    rules: rule_map.into_values().collect(),
                    keywords: kerml_grammar.keywords.union(&sysml_grammar.keywords).cloned().collect(),
                    punctuation: kerml_grammar.punctuation.union(&sysml_grammar.punctuation).cloned().collect(),
                }
            };

            // Generate negative test cases (malformed inputs that should fail)
            let kerml_negative = test_synth::synthesize_negative(&kerml_grammar);
            let sysml_negative = test_synth::synthesize_negative(&sysml_grammar);

            println!("Negative tests: {} KerML + {} SysML cases",
                kerml_negative.len(), sysml_negative.len());

            let negative_output = output.with_file_name("negative_test_data.rs");
            let negative_content = test_synth::generate_negative_test_data_rs(&sysml_negative, &kerml_negative);
            std::fs::write(&negative_output, &negative_content)?;
            println!("Wrote {}", negative_output.display());

            // Generate lexer test cases (keywords, punctuation, literals)
            let kerml_lexer = test_synth::synthesize_lexer_tests(&kerml_grammar);
            let sysml_lexer = test_synth::synthesize_lexer_tests(&sysml_grammar);

            println!("Lexer tests: {} KerML + {} SysML cases",
                kerml_lexer.len(), sysml_lexer.len());

            let lexer_output = output.with_file_name("lexer_test_data.rs");
            let lexer_content = test_synth::generate_lexer_test_data_rs(&sysml_lexer, &kerml_lexer);
            std::fs::write(&lexer_output, &lexer_content)?;
            println!("Wrote {}", lexer_output.display());

            // Generate test runner harnesses
            let runner_output = output.with_file_name("parser_test_runner.rs");
            std::fs::write(&runner_output, test_synth::generate_parser_test_runner())?;
            println!("Wrote {}", runner_output.display());

            let ast_runner_output = output.with_file_name("parser_ast_test.rs");
            std::fs::write(&ast_runner_output, test_synth::generate_parser_ast_test())?;
            println!("Wrote {}", ast_runner_output.display());

            let ast_test_output = output.with_file_name("ast_test.rs");
            std::fs::write(&ast_test_output, test_synth::generate_ast_test())?;
            println!("Wrote {}", ast_test_output.display());

            let negative_test_output = output.with_file_name("negative_test.rs");
            std::fs::write(&negative_test_output, test_synth::generate_negative_test())?;
            println!("Wrote {}", negative_test_output.display());

            let lexer_test_output = output.with_file_name("lexer_test.rs");
            std::fs::write(&lexer_test_output, test_synth::generate_lexer_test())?;
            println!("Wrote {}", lexer_test_output.display());
        }

        Commands::Stats { grammar } => {
            let sources: Vec<String> = grammar.iter()
                .map(|p| std::fs::read_to_string(p))
                .collect::<Result<_, _>>()?;
            
            let source_refs: Vec<&str> = sources.iter().map(|s| s.as_str()).collect();
            let parsed = Grammar::parse_all(&source_refs);
            
            println!("Grammar Statistics:");
            println!("  Rules: {}", parsed.rules.len());
            println!("  Keywords: {}", parsed.keywords.len());
            println!("  Punctuation: {}", parsed.punctuation.len());
            
            println!("\nKeywords:");
            let mut keywords: Vec<_> = parsed.keywords.iter().collect();
            keywords.sort();
            for kw in keywords {
                println!("  {}", kw);
            }
            
            println!("\nPunctuation:");
            let mut puncts: Vec<_> = parsed.punctuation.iter().collect();
            puncts.sort();
            for p in puncts {
                println!("  '{}'", p);
            }
        }
    }
    
    Ok(())
}
