# Overview

This repository uses a Cargo workspace to collect all crates that make up the project. Below is a recommended layout for organizing the existing code and upcoming components such as an interactive REPL, scripting language, terminal UI, utilities, product definitions, random number generator and financial models.

## Proposed directory structure

```
ak/
├── Cargo.toml             # Workspace definition
├── README.md              # Project overview and workspace layout (this file)
├── ak-core/               # Low level utilities shared across crates
├── ak-contract/           # Domain specific language for contracts
├── ak-cashflow/           # Cash‑flow calculations and primitives
├── ak-repl/               # Interactive REPL (binary crate)
├── ak-scripting/          # Scripting language runtime & compiler
├── ak-tui/                # Text user interface (binary crate)
├── ak-products/           # Product definitions
├── ak-random/             # Random number generator utilities
├── ak-models/             # Pricing and risk models
└── ak-cli/                # Main command line application
```

Each directory represents a separate crate. Libraries that are depended upon by multiple crates (e.g. `ak-core`, `ak-random`) should be library crates. Components such as the REPL and TUI are binary crates so they produce executables.

### Workspace configuration

The root `Cargo.toml` lists each crate under `[workspace.members]`. Shared dependencies can be specified under `[workspace.dependencies]` so that every crate in the workspace uses the same version:

```toml
[workspace]
members = [
    "ak-core",
    "ak-contract",
    "ak-cashflow",
    "ak-random",
    "ak-products",
    "ak-models",
    "ak-scripting",
    "ak-repl",
    "ak-tui",
    "ak-cli",
]
resolver = "3"

[workspace.dependencies]
jiff = "0.2.14"
```

This structure keeps the workspace modular and makes it clear which crates provide reusable utilities versus binary applications.

