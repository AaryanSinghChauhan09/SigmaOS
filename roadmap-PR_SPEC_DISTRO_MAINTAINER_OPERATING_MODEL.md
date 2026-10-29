# Specification: Distro-Maintainer Operating Model and Evidence-Gated Execution System

- **Spec Title**: `PR_SPEC_DISTRO_MAINTAINER_OPERATING_MODEL`
- **Related Proposal**: `docs/PR_PROPOSAL_DISTRO_MAINTAINER_OPERATING_MODEL.md`
- **PR Branch**: `feature/distro-maintainer-operating-model`
- **Status**: Approved / Specification Ready

## Specification Overview

This specification details the architecture, agent role contracts, task schema, contract directory layout, policy rules, release channels, and 90-day implementation roadmap for SigmaOS's Distro-Maintainer Operating Model.

## 1. Subsystem Agent Role Contracts

### 1.1 Repository Steward (`repository-steward`)
- **Scope**: Entire repository root, `.github/`, `docs/`, `.agents/`, `reports/steward/`.
- **Permissions**: Read all, write reports, create issues, submit documentation PRs.
- **Cadence**: Daily (CI & PR inspection), Weekly (stale work audit).

### 1.2 Build and Test Sentinel (`build-test-sentinel`)
- **Scope**: `src/`, `tests/`, `tools/`, `Makefile`, `run_sigma_tests.sh`, `Cargo.toml`, `reports/build/`, `reports/test/`.
- **Permissions**: Execute build tools, run unit & integration tests, create mechanical fix PRs.
- **Cadence**: Per PR & Daily.

### 1.3 Boot and Runtime Validator (`boot-runtime-validator`)
- **Scope**: `boot/`, `kernel/`, `installer/`, `iso_root/`, `scripts/`, `reports/boot/`.
- **Permissions**: Build ISO, invoke QEMU, capture serial logs & screenshots, publish boot matrix.
- **Cadence**: Daily.

### 1.4 Capability Auditor (`capability-auditor`)
- **Scope**: `CAPABILITY_MATRIX.toml`, `FEATURE_STATUS.toml`, `README.md`, `docs/`, `reports/capability/`.
- **Permissions**: Read all source and test logs, update capability files with evidence links, flag unevidenced claims.
- **Cadence**: Daily & Weekly.

### 1.5 Kernel and Subsystem Agents (`kernel-subsystem-agents`)
- **Scope**: Subsystem-specific source directories (`src/kernel/`, `src/fs/`, `src/vfs/`, `src/driver/`, etc.).
- **Permissions**: Write within subsystem boundary, run subsystem tests, produce traces.
- **Rule**: Interface boundary changes require cross-agent integration review.

### 1.6 Package and Distro Compatibility Agent (`package-compatibility-agent`)
- **Scope**: `pkgbuild/`, `src/compatibility/`, `tools/sigma_pacman_compat.rs`, `reports/package/`.
- **Permissions**: Parse package metadata, dry-run dependency trees in sandboxed containers.
- **Rule**: No host execution of untrusted scripts or foreign binaries.

### 1.7 Documentation and Onboarding Agent (`docs-onboarding-agent`)
- **Scope**: `README.md`, `docs/`, `CONTRIBUTING.md`, `wiki/`, `reports/capability/verified_today.md`.
- **Permissions**: Sync docs with Makefiles, generate "What Works Today" dashboard.

## 2. 9-Stage Task Execution Schema

Every automated and recurring task adheres to the following sequence:

1. **SELECT**: Pick next job from `.agents/tasks/open/` based on priority and agent boundary.
2. **PLAN**: Evaluate impacted files, execution commands, rollback procedure, and evidence requirements.
3. **ISOLATE**: Work in an isolated Git branch (`agent/<role>/<task_id>`).
4. **EXECUTE**: Apply minimal code, test, or config changes.
5. **VERIFY**: Execute `cargo check`, `./run_sigma_tests.sh`, or QEMU boot test.
6. **RECORD**: Store exit code, command, execution duration, commit hash, and log artifacts in `reports/`.
7. **UPDATE STATUS**: Promote status in `CAPABILITY_MATRIX.toml` or `FEATURE_STATUS.toml` only if evidence is attached.
8. **REVIEW**: Submit PR with linked evidence. Human review mandatory for security, boot, installer, and kernel changes.
9. **FOLLOW UP**: On verification failure, move task to `.agents/tasks/blocked/` with log excerpt and reproduction command.

## 3. Directory Layout Contract

The repository enforces the following layout for agent operations:

```text
.agents/
├── registry.toml
├── policies/
│   ├── safety.md
│   ├── code-change.md
│   ├── test-evidence.md
│   └── release.md
├── runbooks/
│   ├── build-test.md
│   ├── qemu-boot.md
│   ├── capability-audit.md
│   ├── package-compat.md
│   └── docs-sync.md
└── tasks/
    ├── open/
    ├── in-progress/
    ├── done/
    └── blocked/

reports/
├── build/
├── test/
├── boot/
├── capability/
├── installer/
├── package/
└── security/

tools/
├── agent_cli.py
├── capability_audit.py
├── boot_matrix.py
└── evidence_signer.py
```

## 4. Mandatory Rules & Guardrails

- **Rule 1 (Evidence Before Status)**: Claims require attached, dated log evidence.
- **Rule 2 (Boundary Isolation)**: Agents do not edit outside declared ownership paths.
- **Rule 3 (Atomic Commits)**: Small, single-concern PRs.
- **Rule 4 (Reproducibility)**: Every report logs commit SHA, toolchain version, OS, timestamps, and hashes.
- **Rule 5 (Human Gate)**: Human approval mandatory for high-impact security, boot, installer, release, and script changes.
- **Rule 6 (Sandboxing)**: Third-party package scripts execute in restricted sandboxes.
- **Rule 7 (No Silent Failure)**: Failed execution creates a detailed `.agents/tasks/blocked/` task file.
- **Rule 8 (Versioned Claims)**: Stale claims (>30 days without revalidation) are downgraded.
- **Rule 9 (Security Review)**: Dedicated security audit for crypto, capability, and privilege boundaries.
- **Rule 10 (Runtime Proof Dominance)**: Runtime QEMU proof supersedes unit test logic proof for OS capability claims.

## 5. Release Channels

- `dev`: Active development branches passing unit tests.
- `nightly`: Automated daily build passing clean checkout tests and QEMU x86_64 boot-to-shell smoke test.
- `validated`: Verified release candidate with full boot matrix, installer test, known issue list, and rollback notes.
- `experimental`: Bleeding-edge features, clearly labeled and isolated.
