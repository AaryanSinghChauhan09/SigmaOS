# What Works and What Still Needs Validation

This page is a short status entry point, not a claim of release readiness. SigmaOS is an experimental OS development project; module presence and unit tests do not prove that a feature is integrated into a booted system.

## Verified development checks

As recorded in [Project Status](docs/PROJECT_STATUS.md), `cargo check --lib`, the library test suite, and `./run_sigma_tests.sh` passed in the documented environment on 2026-10-04. The library run reported 6,349 passed and 66 ignored tests. QEMU boot was not checked because QEMU was unavailable.

## Current limitations

- General installation and boot-to-user-session are not supported claims.
- Hardware, package update, recovery, desktop, and security enforcement claims need end-to-end validation against named configurations.
- The capability inventory can include prototype or model-level implementations; check each component's status and linked evidence.

## Where to get current detail

- [Project status and release blockers](docs/PROJECT_STATUS.md)
- [Machine-readable feature inventory](FEATURE_STATUS.toml)
- [Testing and release-readiness process](wiki/Testing.md)
- [Canonical component pages and roadmaps](wiki/Home.md)
- [Contributor workflow](CONTRIBUTING.md)

Update this page only when a new verification run or project milestone changes the summary above. Component details belong on their owning wiki page and should include exact test evidence and limitations.
