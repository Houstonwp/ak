# Project Constitution

This document defines binding rules for all work in this repository.

## 1. Scope

- These rules apply to all files and directories in this repository.
- No exceptions.

## 2. Mission

- The project exists to provide the most useful actuarial modeling software solution.
- Correctness and speed are the primary guarantees.

## 3. Supported Platforms

- Linux, macOS, and Windows are first-class platforms.
- Changes MUST NOT knowingly break any of these platforms.

## 4. Quality Gates (Required Before PR Creation)

- `cargo fmt` MUST pass with no diffs.
- `cargo clippy` MUST pass with no warnings (use `-D warnings`).
- `cargo test` MUST pass.
- `cargo doc` MUST pass.
- `cargo llvm-cov` MUST report >= 90% line coverage.
- Docs MUST be updated when behavior or public API changes (README, rustdoc, or other relevant docs).

## 5. Tests Drive Design (TDD-First)

- For every bug fix or new feature, add tests that would fail on the prior code.
- Do not change production code before adding the failing test.
- Tests MUST be deterministic and specify expected behavior clearly.

## 6. Performance Policy

- Run `cargo bench` for performance-sensitive changes.
- Performance-sensitive changes include algorithm changes, data structure changes,
  hot-path refactors, and benchmark modifications.
- Any regression >= 10% in benchmarks MUST be documented in the PR with:
  - The measured results.
  - Rationale for accepting the regression.
  - Alternatives evaluated and why they were rejected.

## 7. API Stability and Versioning

- Semantic Versioning is enforced.
- The project is pre-alpha and versions are 0.x.
- No backward-compatibility guarantees are provided.

## 8. Review and Approval

- SENTINEL MUST review the local diff before creating a PR.
- The PR description MUST note that SENTINEL reviewed the diff.

## 9. Dependencies

- Keep dependencies minimal and mission-aligned.
- New crates MUST be justified in the PR description.
- Crates that materially support correctness, speed, or ubiquitous use (e.g. `serde`)
  are acceptable when justified.

## 10. Security and Safety

- `unsafe` code is forbidden in this repository.
- Secrets, API keys, and credentials MUST NOT be committed.

## 11. Release Notes

- Changelog entries are derived from merged PRs.
- No fixed release cadence is required.

## 12. Constitution Changes

- Changes to this document MUST be made via PR.
- Use `gh` for all GitHub operations.
- Merging constitution changes requires explicit user approval.
