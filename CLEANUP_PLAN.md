# syster-codegen Architecture Cleanup Plan

## Executive Summary

This plan addresses the architectural mess in the codebase by:
1. Removing all duplicates
2. Establishing clear separation between input models and generators
3. Consolidating generated outputs
4. Streamlining CLI commands

## Current State Analysis

### DUPLICATES IDENTIFIED

#### 1. **DUPLICATE: generated_parser.rs in TWO locations**
- ❌ `/syster-codegen/src/generated_parser.rs` (25,729 lines)
- ❌ `/syster-base/src/generated/parser.rs` (36,556 lines)
- **Problem**: Same file, different stats, unclear which is canonical
- **Decision**: Delete syster-codegen copy, only keep in syster-base/src/generated/

#### 2. **DUPLICATE: rust_parser.rs deleted but parser_generator.rs exists**
- ✅ Correctly renamed to parser_generator.rs
- **No action needed**

#### 3. **DUPLICATE: SyntaxKind in THREE places**
- ❌ `/syster-base/src/parser/syntax_kind.rs` - Hand-written (547 lines)
- ❌ `/syster-base/src/generated/parser.rs` - Generated from Xtext (includes SyntaxKind enum)
- ❌ `/syster-base/src/generated/syntax_kind.rs` - Generated from metamodel (MetaKind, not SyntaxKind)
- **Problem**: Confusion about which one to use
- **Decision**: 
  - Keep ONLY parser.rs SyntaxKind (generated from Xtext grammar)
  - Delete hand-written parser/syntax_kind.rs
  - Rename generated/syntax_kind.rs → generated/meta_kind.rs (it contains MetaKind, not SyntaxKind)

#### 4. **DUPLICATE/CONFUSING: AST nodes in TWO places**
- ❌ `/syster-base/src/parser/ast.rs` - Hand-written (2,673 lines)
- ❌ `/syster-base/src/generated/ast_nodes.rs` - Generated (167k lines)
- **Problem**: Hand-written wrappers vs generated nodes
- **Decision**: Keep both BUT clarify roles:
  - `generated/ast_nodes.rs` - Low-level metamodel nodes
  - `parser/ast.rs` - High-level API wrappers around generated nodes
  - Rename: `parser/ast.rs` → `parser/ast_wrappers.rs` for clarity

#### 5. **HAND-WRITTEN LEXER - Should be generated**
- ❌ `/syster-codegen/src/runtime/lexer.rs` - 559 lines, ALL KEYWORDS HARDCODED
- **Problem**: 
  - Grammar defines all keywords in Xtext rules
  - parser_generator.rs extracts keywords and generates `lookup_keyword()`
  - BUT lexer Token enum is completely hand-written with hardcoded keywords
  - Keywords can get out of sync between grammar and lexer
- **Decision**: 
  - parser_generator.rs should generate BOTH parser.rs AND lexer.rs
  - lexer.rs Token enum generated from grammar keywords
  - Delete hand-written runtime/lexer.rs entirely

### STRUCTURAL ISSUES

#### 6. **CONFUSING: Three different "kinds" enums**
- `SyntaxKind` - CST tokens and nodes (from Xtext grammar)
- `MetaKind` - Metamodel element types (from .ecore)
- `SymbolKind` - Semantic symbol types (from .ecore)
- **Status**: These are actually different! No duplication.
- **Action**: Better documentation only

#### 7. **MISSING: syntax_kind.rs generator exists but generates wrong thing**
- `/syster-codegen/src/generators/syntax_kind.rs` - Generates MetaKind
- **Problem**: Name mismatch - generates MetaKind but called syntax_kind
- **Decision**: Rename generator to meta_kind.rs

## Target Architecture

```
syster-codegen/
├── src/
│   ├── models/              # Input format parsers
│   │   ├── xtext/          # Xtext grammar parser (.xtext → XtextGrammar)
│   │   └── ecore/          # Metamodel parser (.ecore → Metamodel)
│   ├── generators/          # Code generators
│   │   ├── parser_generator.rs    # Grammar → parser.rs + lexer.rs
│   │   ├── ast_nodes.rs           # Metamodel → ast_nodes.rs
│   │   ├── meta_kind.rs           # Metamodel → meta_kind.rs
│   │   ├── symbol_kind.rs         # Metamodel → symbol_kind.rs
│   │   ├── visitor.rs             # Metamodel → visitor.rs
│   │   └── constraints.rs         # Metamodel → constraints.rs
│   ├── runtime/             # Runtime support (EMPTY after cleanup)
│   └── main.rs              # CLI

syster-base/
├── src/
│   ├── generated/           # ALL generated code
│   │   ├── parser.rs       # From Xtext (includes SyntaxKind)
│   │   ├── lexer.rs        # From Xtext (NEW - generated)
│   │   ├── ast_nodes.rs    # From metamodel
│   │   ├── meta_kind.rs    # From metamodel (renamed)
│   │   ├── symbol_kind.rs  # From metamodel
│   │   ├── visitor.rs      # From metamodel
│   │   └── constraints.rs  # From metamodel
│   ├── parser/              # Hand-written parser infrastructure
│   │   ├── mod.rs          # Public API
│   │   ├── ast_wrappers.rs # High-level API (renamed from ast.rs)
│   │   └── rule_parser.rs  # Rule-by-rule parsing helpers
│   └── ...
```

## CLI Commands (Final)

### 1. `generate-models` - Generate from Ecore metamodel
```bash
syster-codegen generate-models \
  --metamodel data/SysML.ecore \
  --output ../syster-base/src/generated
```

**Generates:**
- ast_nodes.rs
- meta_kind.rs
- symbol_kind.rs
- visitor.rs
- constraints.rs

### 2. `generate-parser` - Generate from Xtext grammar
```bash
syster-codegen generate-parser \
  --grammar data/SysML.xtext \
  --output ../syster-base/src/generated
```

**Generates:**
- parser.rs (includes SyntaxKind enum)
- lexer.rs (includes Token enum with keywords from grammar)

### 3. `generate` - Generate everything
```bash
syster-codegen generate \
  --metamodel data/SysML.ecore \
  --grammar data/SysML.xtext \
  --output ../syster-base/src/generated
```

**Calls both generate-models and generate-parser**

## Implementation Steps

### Phase 1: Clean Duplicates (1-2 hours)

1. **Delete syster-codegen/src/generated_parser.rs**
   - It's a stale copy, real one is in syster-base/src/generated/

2. **Delete syster-base/src/parser/syntax_kind.rs**
   - Replaced by generated parser.rs SyntaxKind

3. **Rename syster-base/src/generated/syntax_kind.rs → meta_kind.rs**
   - Clarify it contains MetaKind, not SyntaxKind

4. **Rename syster-codegen/src/generators/syntax_kind.rs → meta_kind.rs**
   - Match the output it generates

5. **Rename syster-base/src/parser/ast.rs → ast_wrappers.rs**
   - Clarify it's high-level wrappers, not low-level nodes

### Phase 2: Generate Lexer (2-3 hours)

6. **Add lexer generation to parser_generator.rs**
   - Extract keywords from XtextGrammar
   - Generate Token enum with all keywords
   - Generate lexer with logos annotations
   - Include tokenize() function

7. **Delete syster-codegen/src/runtime/lexer.rs**
   - Replaced by generated lexer

8. **Empty syster-codegen/src/runtime/ directory**
   - test_harness.rs can be deleted or moved to tests/

### Phase 3: Restructure CLI (1 hour)

9. **Update main.rs commands**
   - Rename `generate` → `generate-models`
   - Keep `generate-parser` as is
   - Add new `generate` that calls both

10. **Update help text and documentation**

### Phase 4: Fix Imports (1 hour)

11. **Update all imports in syster-base**
    - parser/mod.rs: remove syntax_kind imports
    - Use generated::parser::SyntaxKind everywhere
    - Use generated::lexer instead of syster_codegen::runtime::lexer

12. **Update Makefiles**
    - Update generation commands to use new CLI

### Phase 5: Documentation (1 hour)

13. **Add architecture documentation**
    - Document the two generation paths clearly
    - Explain SyntaxKind vs MetaKind vs SymbolKind

14. **Update README files**

## Lexer Clarification

### Current (Wrong) Understanding:
- "We don't need lexer because parser includes it"
- "Lexer is covered by grammar"

### Correct Understanding:
The parser generation has TWO phases:

1. **Lexing**: Text → Tokens
   - Input: Raw source text
   - Process: Regex matching via logos
   - Output: `Vec<(Token, &str)>`
   - **Keywords are defined in grammar, Token enum currently hand-written**

2. **Parsing**: Tokens → CST
   - Input: `&[(SyntaxKind, &str)]` (converted from Token)
   - Process: Recursive descent based on grammar rules
   - Output: Rowan GreenNode (CST)

**The grammar DOES define keywords**, but currently:
- ❌ Token enum is hand-written in runtime/lexer.rs
- ❌ Manual Token → SyntaxKind conversion in parser/mod.rs
- ✅ Grammar keywords are used in parser.rs

**After fix**:
- ✅ Token enum generated from grammar keywords
- ✅ SyntaxKind enum generated from grammar (already done)
- ✅ Automatic Token → SyntaxKind alignment

## Risk Assessment

**Low Risk:**
- Deleting duplicates (they're unused)
- Renaming files for clarity
- Restructuring CLI commands

**Medium Risk:**
- Generating lexer (need to test thoroughly)
- Updating imports across codebase

**Mitigation:**
- Make changes incrementally
- Test after each phase
- Keep git commits small and atomic

## Success Criteria

✅ No duplicate files
✅ Clear separation: models/ vs generators/
✅ All generated code in syster-base/src/generated/
✅ Lexer fully generated from grammar
✅ Three clear CLI commands
✅ All tests passing
✅ Clean architecture documentation

## Estimated Time: 6-8 hours

Ready to proceed?
