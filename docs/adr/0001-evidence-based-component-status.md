# ADR 0001: Evidence-Based Component Status Policy

- **Status**: Accepted
- **Date**: 2026-03-29
- **Deciders**: SigmaOS Core Maintainers & Architecture Review Board

## Context

In experimental operating system projects, source code presence and unit test compilation are often conflated with operational implementation status. In SigmaOS, code modules exist for many advanced kernel, storage, networking, and desktop subsystems, but unit test execution alone does not confirm runtime kernel integration, QEMU boot success, hardware compatibility, or failure recovery.

Claiming that a component is "complete" or "fully supported" without runtime verification misleads contributors, distorts project roadmaps, and compromises system reliability.

## Decision

SigmaOS adopts a strict **Evidence-Based Component Status Policy**:

> **Rule**: A component, driver, or subsystem may not be marked as `implemented`, `complete`, or `supported` in `FEATURE_STATUS.toml`, `CAPABILITY_MATRIX.toml`, `README.md`, or component documentation without explicit runtime or integration test evidence.

### Status Tiers & Required Evidence

1. **Untested / Experimental**: Code exists and compiles, unit tests may pass, but no runtime/integration evidence exists.
   - *Required Evidence*: `cargo check --lib` or standalone unit test.
2. **Partial / Simulated**: Component functions in unit test mocks or simulated run-queues, but lack end-to-end integration.
   - *Required Evidence*: Standalone integration test output log.
3. **Verified Runtime / Integration**: Component is exercised on a live kernel execution path or QEMU boot-to-shell environment.
   - *Required Evidence*: CI boot log, QEMU smoke test transcript, or hardware diagnostic report artifact.

## Consequences

- All status fields in `FEATURE_STATUS.toml` and documentation must cite explicit test IDs, log outputs, or issue references.
- Unsubstantiated claims of hardware driver or subsystem completeness will be reverted in code review.
- CI workflows will run automated status audits against `COMPONENT_RULES.toml` to enforce evidence compliance.
