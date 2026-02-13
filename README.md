# syster-codegen

Code generator for SysML v2 tooling from OMG Ecore metamodels.

## Overview

This crate reads the official OMG SysML v2 metamodel (in Ecore format) and generates:

- **`syntax_kind.rs`** - `MetaKind` enum with XMI type mappings, keywords, definition/usage helpers
- **`symbol_kind.rs`** - `SymbolKind` enum for HIR symbol classification
- **`ast_nodes.rs`** - AST node structs and enumerations from metamodel classes
- **`visitor.rs`** - Visitor trait for AST traversal

## Architecture

```
data/
└── SysML.ecore        # Official OMG SysML metamodel (includes KerML)
        │
        ▼
┌─────────────────┐
│ Metamodel Parser │    Parse Ecore XML into internal representation
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│   Metamodel     │    MetaClass, MetaProperty, MetaEnumeration
│   Data Model    │    Inheritance hierarchy, documentation
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│   Generators    │    Tera template-based code generation
│  ├── syntax_kind│    → MetaKind enum, helpers
│  ├── symbol_kind│    → SymbolKind enum
│  ├── ast_nodes  │    → structs, enums from metamodel
│  └── visitor    │    → Visitor trait
└────────┬────────┘
         │
         ▼
    syster-base/src/generated/
```

## Usage

```bash
# Generate all code (default output: ../syster-base/src/generated)
cargo run -- generate --metamodel data/SysML.ecore

# Generate to custom location
cargo run -- generate --metamodel data/SysML.ecore --output /path/to/output

# Inspect metamodel
cargo run -- inspect data/SysML.ecore
cargo run -- inspect data/SysML.ecore | grep Definition
```

## Metamodel Source

The Ecore metamodel is from the official OMG SysML v2 Pilot Implementation:
https://github.com/Systems-Modeling/SysML-v2-Pilot-Implementation/tree/master/org.omg.sysml/model

To update:
```bash
curl -L "https://raw.githubusercontent.com/Systems-Modeling/SysML-v2-Pilot-Implementation/master/org.omg.sysml/model/SysML.ecore" -o data/SysML.ecore
```

## Development

```bash
# Run tests
cargo test

# Regenerate and check syster-base compiles
cargo run -- generate --metamodel data/SysML.ecore
(cd ../syster-base && cargo check)
```
