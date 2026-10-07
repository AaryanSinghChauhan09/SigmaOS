---
name: Open Source Idea / Feature Adoption Proposal
about: Propose adapting a feature, architectural pattern, or capability from an open-source OS (Redox, seL4, Tock, Fuchsia, Linux, etc.) into SigmaOS.
title: '[OSS Adoption] <Short title of feature/idea>'
labels: ['enhancement', 'oss-idea-adoption']
assignees: ''
---

## Source OS / Reference Project
<!--- Specify the source open-source project (e.g. Redox OS, seL4, Tock OS, Fuchsia, Linux, smoltcp, Wasmtime, etc.) and link to relevant repository, RFC, or documentation. -->
- **Source OS / Project:**
- **Reference Link(s):**

## Proposed Feature / Architectural Concept
<!--- Describe the concept or feature to adapt and why it is beneficial for SigmaOS. -->

## SigmaOS Architectural Integration
<!--- How does this feature fit with SigmaOS's architecture rules? (no_std, capability tokens, WDM driver patterns, Paged/NonPaged pools, standalone file unit tests) -->

## Proposed Implementation Plan & Timebox
- [ ] **Phase 1 (Prototype - 1 Sprint):** Minimal working spike or standalone test.
- [ ] **Phase 2 (Evaluation & Benchmarks):** Measure footprint, syscall/IPC latency, and security impact.
- [ ] **Phase 3 (Integration & CI):** Full implementation with unit/integration tests and CI automation.

## Acceptance Criteria
<!--- What must be proven to consider this feature successfully adopted? -->
- [ ] Standalone unit tests pass cleanly via `rustc --test`.
- [ ] No regressions in microbenchmarks or build footprint.
- [ ] Documentation updated in `docs/` or repository root.
