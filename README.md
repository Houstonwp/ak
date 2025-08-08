# Overview

The `ak` project is a collection of Rust crates for modelling and simulating financial contracts. Each crate in the workspace focuses on a specific concern, ranging from low level date handling to Monte Carlo simulation and command line tools.

## Workspace structure

```
ak/
├── Cargo.toml            # Workspace configuration
├── README.md             # Project overview (this file)
├── ak-core/              # Shared date and schedule utilities
├── ak-contract/          # DSL and simulator for contracts
├── ak-cashflow/          # Cash-flow calculations and primitives
├── ak-random/            # Random number generation utilities
├── ak-sim/               # Monte Carlo simulation framework
├── ak-repl/              # Interactive REPL for experimenting with contracts
└── ak-cli/               # Command line application
```

## Crate summaries

- **ak-core** – foundational types such as calendars, day-count conventions, compounding and scheduling helpers.
- **ak-contract** – parser and runtime for a domain specific language describing financial contracts.
- **ak-cashflow** – helpers for building and aggregating cash-flows.
- **ak-random** – utilities for deterministic and parallel-friendly random number generation.
- **ak-sim** – Monte Carlo engine built on `ak-random` for running simulations.
- **ak-repl** – interactive Read-Eval-Print loop powered by `ak-contract`.
- **ak-cli** – entry point binary that ties the crates together.

### Workspace configuration

The root `Cargo.toml` registers all crates as members of a single workspace and defines shared dependencies:

```toml
[workspace]
members = [
    "ak-core",
    "ak-contract",
    "ak-cashflow",
    "ak-repl",
    "ak-cli",
    "ak-random",
    "ak-sim",
]
resolver = "3"

[workspace.dependencies]
jiff = "0.2.15"
anyhow = "1.0.98"
rayon = "1.10.0"
thiserror = "2.0"
```

This layout keeps the workspace modular and makes it clear which crates provide reusable libraries versus binaries.

