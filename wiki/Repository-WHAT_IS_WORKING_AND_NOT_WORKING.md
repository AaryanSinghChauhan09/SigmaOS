> Imported repository document from [`WHAT_IS_WORKING_AND_NOT_WORKING.md`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/WHAT_IS_WORKING_AND_NOT_WORKING.md). For current component status and roadmap, use the canonical component page linked from [Home](Home.md).

---

# What Works and What Still Needs Validation

This page is a short status entry point, not a claim of release readiness. SigmaOS is an experimental OS development project; module presence and unit tests do not prove that a feature is integrated into a booted system.

## Verified development checks

As recorded in [Project Status](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/PROJECT_STATUS.md), `cargo check --lib`, the library test suite, and `./run_sigma_tests.sh` passed in the documented environment on 2026-10-04. The library run reported 6,349 passed and 66 ignored tests. QEMU boot was not checked because QEMU was unavailable.

## Current limitations

- General installation and boot-to-user-session are not supported claims.
- Hardware, package update, recovery, desktop, and security enforcement claims need end-to-end validation against named configurations.
- The capability inventory can include prototype or model-level implementations; check each component's status and linked evidence.

## Where to get current detail

- [Project status and release blockers](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/PROJECT_STATUS.md)
- [Machine-readable feature inventory](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/FEATURE_STATUS.toml)
- [Testing and release-readiness process](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/wiki/Testing.md)
- [Canonical component pages and roadmaps](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/wiki/Home.md)
- [Contributor workflow](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/CONTRIBUTING.md)

Update this page only when a new verification run or project milestone changes the summary above. Component details belong on their owning wiki page and should include exact test evidence and limitations.
