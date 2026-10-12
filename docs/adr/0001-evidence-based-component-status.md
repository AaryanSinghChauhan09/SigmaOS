# ADR 0001: Evidence-Based Component Status Classification

## Context and Problem Statement

SigmaOS is an experimental Rust operating system with kernel, userspace, desktop, packaging, and tooling layers. Historically, code presence or passing unit tests was sometimes conflated with complete subsystem implementation. However, passing unit tests alone does not prove bootability, disk installer stability, driver support, or kernel integration.

## Decision Drivers

- Need to distinguish between theoretical/model implementation and verified runtime execution.
- Need honest status reporting in `FEATURE_STATUS.toml`, `CAPABILITY_MATRIX.toml`, and README files.
- Need automated CI enforcement to prevent premature claims of hardware or platform completion.

## Decision Outcome

SigmaOS adopts an evidence-based status policy:

1. **Unverified / Conceptual**: Code exists in repository, unit tests may pass, but runtime path has not been executed on target hardware or QEMU.
2. **Partial / Model Verified**: Component unit tests and simulation integration tests pass; isolated execution is verified.
3. **Runtime Verified**: Component runtime path has been exercised in QEMU or real hardware with recorded log/artifact evidence.
4. **Production / Release Ready**: Component passes full regression, fault injection, security review, and recovery tests.

No component may be promoted to "Runtime Verified" or "Complete" without attached execution logs or CI artifacts.

## Consequences

- Prevents false confidence in unsupported drivers or unintegrated kernel modules.
- Clarifies priority workstreams for contributors and automated AI agents.
