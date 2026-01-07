# LLM Agent Instructions

This project is **TDD-first**. Correctness and performance are the core goals.

## Core Rules

- **Tests drive design**: add or update tests before changing behavior.
- **Correctness is non-negotiable**: prefer clear, verifiable implementations.
- **Performance is a first-class concern**: maintain or improve speed and memory usage.

## Performance Focus Areas

- **Memory layout**: optimize data structures for cache locality.
- **SIMD**: use vectorization where it materially helps.
- **Parallelization**: leverage concurrency for large workloads.

## Contribution Expectations

- Add benchmarks for performance-sensitive changes.
- Avoid premature micro-optimizations without evidence.
- Keep APIs deterministic and well-specified.
- ALWAYS run `cargo fmt` and `cargo clippy` and make sure tests pass prior to saying work is completed.
