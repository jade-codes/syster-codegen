# Code Generation Plan

## Status: Structure Complete ✅

The new generation structure is complete:
- ✅ Separate parsing of KerML and SysML grammars  
- ✅ `MergedTypes` struct for unified type information
- ✅ New directory structure (common/, kerml/, sysml/)
- ✅ Shared AST in common/ast.rs (525 nodes)
- ✅ Shared SyntaxKind in common/syntax_kind.rs
- ✅ Language-specific lexers (working!)
- ✅ Language-specific parsers (structure complete, needs fixes)
- ✅ New CLI with `--kerml` and `--sysml` flags

## Remaining Issues

The parser generator (`parser_ast.rs`) has issues:
1. Enum vs struct confusion - some rules are enums but parser tries struct construction
2. Type mismatches in Vec assignments (different member types pushed to same vec)
3. Missing `Default` implementations for some enum types

These are pre-existing issues in the parser generator that need fixing.

## Architecture

### Input Structure
```
data/
  KerML-textual-bnf.kebnf   # Base language
  SysML-textual-bnf.kebnf   # Extension (references KerML types)
```

### Output Structure
```
generated/
  mod.rs                     # Re-exports everything
  
  common/
    mod.rs
    ast.rs                   # All AST types (merged, deduplicated)
    syntax_kind.rs           # All SyntaxKind variants (merged)
    span.rs                  # Span, ParseError, Result types
  
  kerml/
    mod.rs
    tokens.rs                # KerML keywords + punctuation
    lexer.rs                 # KerML lexer
    parser.rs                # KerML parser (only KerML rules)
  
  sysml/
    mod.rs  
    tokens.rs                # SysML keywords + punctuation
    lexer.rs                 # SysML lexer
    parser.rs                # SysML parser (KerML + SysML rules)
```

## Generation Steps

### Step 1: Parse Both Grammars Separately
- Parse KerML grammar → `kerml_grammar: Grammar`
- Parse SysML grammar → `sysml_grammar: Grammar`
- Keep them separate, don't merge yet

### Step 2: Build Merged Type Information
- Collect all unique rules from both grammars
- For duplicate rule names: SysML overrides KerML
- Build unified AST node definitions
- Build unified SyntaxKind enum

### Step 3: Generate Common Module
- `ast.rs`: All struct/enum definitions from merged rules
- `syntax_kind.rs`: All token and node kinds
- `span.rs`: Shared types (Span, ParseError, Result)

### Step 4: Generate KerML Module
- `tokens.rs`: Only KerML keywords/punctuation → `TokenKind` enum
- `lexer.rs`: Lexer using KerML tokens
- `parser.rs`: Parser methods only for rules defined in KerML
  - Returns types from `common::ast`
  - Uses `common::syntax_kind::SyntaxKind`

### Step 5: Generate SysML Module  
- `tokens.rs`: All SysML keywords/punctuation (superset of KerML)
- `lexer.rs`: Lexer using SysML tokens
- `parser.rs`: Parser methods for all rules (KerML base + SysML extensions)
  - Returns types from `common::ast`

## Key Design Decisions

### 1. AST Ownership
- All AST types live in `common::ast`
- Both parsers return the same types
- No duplication

### 2. Token Handling
- Each language has its own `TokenKind` enum
- Different keyword sets but same structural tokens
- Lexers are language-specific

### 3. CrossRef Resolution
- `[QualifiedName]` in grammar becomes field type `QualifiedNameRef`
- `QualifiedName` rule exists in KerML, generates `parse_qualified_name()`
- CrossRef fields store the parsed reference, resolution happens later

### 4. Rule Inheritance
- SysML can override KerML rules (same name, different body)
- SysML can add new rules
- SysML parser has all rules; KerML parser has only KerML rules

## Implementation Order

1. [ ] Refactor `Grammar` to not auto-merge, keep grammars separate
2. [ ] Create `MergedTypes` struct to hold unified type information
3. [ ] Update `generator/mod.rs` to generate new directory structure
4. [ ] Update `generator/ast.rs` to use merged types
5. [ ] Update `generator/syntax_kind.rs` to use merged types
6. [ ] Split `generator/tokens.rs` to generate per-language tokens
7. [ ] Split `generator/lexer.rs` to generate per-language lexers
8. [ ] Split `generator/parser_ast.rs` to generate per-language parsers
9. [ ] Update CLI to take `--kerml` and `--sysml` paths explicitly
10. [ ] Add integration tests for both languages

## CLI Interface

```bash
# Generate all code
syster-codegen generate \
  --kerml data/KerML-textual-bnf.kebnf \
  --sysml data/SysML-textual-bnf.kebnf \
  --output generated/

# Stats for a single grammar
syster-codegen stats --grammar data/KerML-textual-bnf.kebnf
```

## Validation

After generation:
1. `cargo check` on generated code should pass
2. Lexer tests: tokenize sample KerML/SysML files
3. Parser tests: parse sample files, check AST structure
4. Round-trip: parse → print → parse should be stable
