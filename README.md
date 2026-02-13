# syster-codegen

Parser generator for SysML v2 / KerML from KEBNF grammar files.

## Overview

This crate reads KEBNF grammar files (derived from the OMG SysML v2 spec) and generates:

- **Lexer** - Token definitions and lexer implementation
- **Parser** - Recursive descent parser with GLR-style backtracking
- **AST** - Typed AST node structs and enums
- **Test Suite** - Synthesized test cases from grammar rules

## Architecture

```
data/
├── KerML-textual-bnf.kebnf   # KerML grammar (core language)
└── SysML-textual-bnf.kebnf   # SysML grammar (extends KerML)
        │
        ▼
┌─────────────────┐
│  KEBNF Parser   │    Parse grammar into rule definitions
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│   Generators    │
│  ├── lexer      │    → Token enum, Lexer impl
│  ├── parser     │    → Recursive descent with backtracking
│  ├── ast        │    → Typed AST nodes
│  └── test_synth │    → Synthesized test cases
└────────┬────────┘
         │
         ▼
    generated/
    ├── common/           # Shared types (Span, SyntaxKind)
    ├── kerml/            # KerML lexer, parser, AST
    └── sysml/            # SysML lexer, parser, AST
```

## Commands

### Generate Parser

Generate lexer, parser, and AST from both grammar files:

```bash
cargo run -- generate \
  --kerml data/KerML-textual-bnf.kebnf \
  --sysml data/SysML-textual-bnf.kebnf \
  --output generated
```

### Generate Tests

Synthesize test cases from grammar rules:

```bash
cargo run -- generate-tests \
  --kerml data/KerML-textual-bnf.kebnf \
  --sysml data/SysML-textual-bnf.kebnf \
  --output tests/parser_test_data.rs
```

This generates:
- `parser_test_data.rs` - Positive parse test inputs
- `negative_test_data.rs` - Invalid inputs that should fail
- `lexer_test_data.rs` - Lexer token tests
- `parser_test_runner.rs` - Test harness for parser
- `parser_ast_test.rs` - AST construction tests
- `ast_test.rs` - AST node tests
- `negative_test.rs` - Negative test runner
- `lexer_test.rs` - Lexer test runner

### Show Grammar Stats

Display statistics about grammar files:

```bash
cargo run -- stats \
  --grammar data/KerML-textual-bnf.kebnf \
  --grammar data/SysML-textual-bnf.kebnf
```

### Legacy Generate (Single Grammar)

Generate from a single merged grammar file:

```bash
cargo run -- generate-legacy \
  --grammar data/merged.kebnf \
  --output generated
```

## Development

```bash
# Run all tests
cargo test

# Run tests in release mode (faster)
cargo test --release

# Regenerate parser and tests
cargo run -- generate \
  --kerml data/KerML-textual-bnf.kebnf \
  --sysml data/SysML-textual-bnf.kebnf \
  --output generated

cargo run -- generate-tests \
  --kerml data/KerML-textual-bnf.kebnf \
  --sysml data/SysML-textual-bnf.kebnf
```

## Grammar Source

The KEBNF grammar files are derived from the official OMG SysML v2 specification:
https://github.com/Systems-Modeling/SysML-v2-Release

Grammar files in `data/` are licensed under LGPL v3.0 (see `data/LICENSE`).

## License

- Code: MIT License (see `LICENSE`)
- Grammar files (`data/`): LGPL v3.0 (see `data/LICENSE`)
