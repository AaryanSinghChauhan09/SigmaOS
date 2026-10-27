# PR Proposal: Turn SigmaOS into a Repeatable Agent Operating Loop

## Executive Summary

SigmaOS is an experimental, Rust-based "AI-native OS" repository with kernel, userspace, desktop, packaging, and tooling modules. Rather than adding further unverified features, this proposal operationalizes SigmaOS as a **repeatable agent operating loop**: small, verifiable tasks; evidence-gated status; and distro-style release discipline modeled on Linux distributions (Arch, Mint, Redox, FreeBSD, OpenBSD).

## Distro-Style Operating Model

| Distro Practice | SigmaOS Agent Equivalent |
|---|---|
| Package maintainer owns one package | One agent owns one subsystem or module boundary |
| ArchWiki-style reproducible procedure | Every task has a machine-readable runbook |
| Release branches and tagged versions | Agent changes land through PRs with test evidence |
| Stable/testing/unstable channels | `qemu-verified`, `model-only`, and `blocked` capability states |
| ISO build and smoke test | Boot/QEMU smoke test gates every integration change |
| Security advisories | Regression, capability, and security-impact reports |
| Reproducible builds | Deterministic build inputs, lockfiles, and recorded toolchain versions |

## 7 Scoped Agent Roles

1. **Repository Steward (`steward`)**: Daily CI checks, stale branch audits, conventional commit formatting, and documentation drift tracking.
2. **Build and Test Sentinel (`sentinel`)**: Automated clean builds (`cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, `make check`), flaky test detection, and machine-readable test manifests.
3. **Boot and Runtime Validator (`boot-validator`)**: ISO build in clean container, QEMU x86_64 boot-to-shell, serial log capture, and boot matrix maintenance.
4. **Capability Auditor (`capability-auditor`)**: Cross-references claims in `CAPABILITY_MATRIX.toml` and `FEATURE_STATUS.toml` against runtime/test evidence, generating public truth tables.
5. **Kernel and Subsystem Agent (`kernel-subsystem`)**: Modular ownership over scheduler, memory, VFS, syscalls, IPC, drivers, and security capabilities.
6. **Package and Distro Compatibility Agent (`package-compat`)**: Package format schemas (`.deb`, `.rpm`, Arch, BSD), sandboxed dry-run package parsing, and dependency resolution.
7. **Documentation and Onboarding Agent (`docs-onboarding`)**: Readme command verification, "Verified Today" dashboard, and contributor onboarding.

## 9-Stage Task Lifecycle

```text
1. SELECT TASK → Read task schema and ownership boundaries
2. READ MATRIX → Check capability matrix and current test coverage
3. ISOLATE BRANCH → Create scoped branch/workspace
4. IMPLEMENT MINIMAL CHANGE → Apply focused modification
5. RUN SCOPED TESTS → Execute subsystem test suite
6. RUN REPO CHECKS → Run ./run_sigma_tests.sh
7. PRODUCE EVIDENCE ARTIFACT → Generate log/manifest in reports/
8. UPDATE STATUS WITH EVIDENCE → Update matrix status if evidence supports
9. SUBMIT PR WITH ROLLBACK → Open PR with risk assessment and rollback steps
```

## 6-Level Evidence Status Vocabulary

- `unimplemented`: No credible implementation found.
- `model-only`: Unit-tested model exists, but no connected runtime path.
- `integration-pending`: Code exists but is not connected end-to-end to boot path.
- `qemu-verified`: Passes automated QEMU boot and runtime test.
- `hardware-verified`: Tested on named physical hardware with logs on record.
- `blocked`: Known dependency or design blocker.

## 90-Day Execution Plan

- **Days 1–14 (Establish Truth)**: Create agent registry, task schema, report directories, and capability audit dashboard.
- **Days 15–35 (Trustworthy Build)**: Automate clean builds, test manifests, and mechanical failure repairs.
- **Days 36–60 (Prove One Boot Path)**: QEMU x86_64 boot-to-shell validation and serial log matrix.
- **Days 61–75 (Connect Subsystems)**: Minimum boot path subsystem integration and runtime trace evidence.
- **Days 76–90 (Distro Release Discipline)**: Release channels (`dev`, `nightly`, `validated`) and sandboxed package metadata audit.

## Verification & Proof Standard

- All 10 unit and integration tests in `tests/test_repeatable_agent_operating_loop.rs` pass.
- Rust engine `src/distro/repeatable_agent_operating_loop_pr.rs` is `#![no_std]` compliant and exported in `src/distro/mod.rs`.
