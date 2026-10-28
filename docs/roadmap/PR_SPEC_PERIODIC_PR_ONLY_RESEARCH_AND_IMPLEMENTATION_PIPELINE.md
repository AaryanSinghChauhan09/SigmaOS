# PR Spec: Periodic PR-Only Research-and-Implementation Pipeline Specification

## Subsystem: Governance & PR Pipeline Engine

**Specification Version:** 1.0.0
**Target Engine:** `src/governance/periodic_pr_only_research_pipeline_engine.rs`

## Overview

This specification formalizes the periodic research-and-implementation operating model for SigmaOS. All capabilities and missing components are researched, implemented, tested, documented, and promoted through isolated PR cycles backed by empirical runtime evidence.

## Governance Architecture

### 1. PR-Only Lifecycle
All modifications must traverse the 7-stage PR pipeline:
1. Issue creation (`study_issue.yml` or `missing_component_issue.yml`)
2. Research & ADR PR (`study/` branch)
3. Implementation PR (`feat/` or `fix/` branch)
4. Test & evidence PR (`test/` branch)
5. Documentation & status update PR (`docs/` branch)
6. Automated CI verification & reviewer sign-off
7. Merged commit & closed tracking issue

### 2. Cadence Rules
- **Daily:** Build/test/boot health checks and smoke tests
- **Weekly:** Upstream mechanism study and ADR generation
- **Biweekly:** Component revalidation and regression testing
- **Monthly:** Full missing component delivery with runtime proof
- **Quarterly:** Architecture review and gap register maintenance

### 3. Missing Component Priorities
- **P0:** Reproducible CI, QEMU boot validator, Bootloader integration, Kernel entry & serial logging
- **P1:** Physical memory manager, Virtual memory/paging, Interrupts & timer, Scheduler, Syscall interface, Init & shell
- **P2:** VFS, RAM-disk filesystem, Processes & signals, Pipes & IPC, Installer, Atomic update & rollback, Security capabilities
- **P3:** Drivers (virtio), Networking, Desktop, Package manager, Distro compatibility

### 4. Verification Gate Commands
```bash
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
make check
make test
make format
```

### 5. Review & Merging Directives
- **Zero Direct Commits:** All changes enter via pull requests.
- **Evidence-Gated Status:** `CAPABILITY_MATRIX.toml` and `FEATURE_STATUS.toml` promotion requires linked test logs.
- **No Parity Claims:** Reference upstream project naming does not imply identical compatibility without explicit test evidence.
