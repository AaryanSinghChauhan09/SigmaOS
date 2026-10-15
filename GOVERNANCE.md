# SigmaOS Governance & Role-Based Operating Plan

Welcome to the **SigmaOS Governance Framework**. This document defines the governance rules, strategic roadmap, recurring AI-agent maintenance plan, and 12-role operating model for the SigmaOS project.

---

## Getting Started: Bootstrap Phase

To prevent organizational overload during early development, SigmaOS boots its role-based operating model using a phased rollout:

1. **Initial Active Roles (Weeks 1–4)**:
   - **Triager**: `@sigmaos-devs/triagers`
   - **Reviewer**: `@sigmaos-devs/reviewers`
   - **Updater**: `@sigmaos-devs/updaters`
   - **Tester**: `@sigmaos-devs/testers-fuzzers`
2. **Weekly Review & Scaling**: Run with these four core active roles for four weeks. Review cadence workload metrics weekly, adjust cadences based on actual human and agent load, and incrementally activate the remaining eight roles.

---

## Assigned Human & Agent Roles Summary

| Role | Mission | Main Cadence | Human Team / Handle | Agent Suitability |
| :--- | :--- | :--- | :--- | :--- |
| **Maintainer** | Owns subsystem health, reviews PRs, holds merge authority | Daily | `@sigmaos-devs/maintainers` | Human-led, agent-assisted |
| **Updater** | Keeps toolchain, crates, CI base images, and upstream current | Daily / Weekly | `@sigmaos-devs/updaters` | Mostly agent |
| **Reviewer** | Checks code for correctness, safety, performance, and style | Per PR | `@sigmaos-devs/reviewers` | Agent first pass, human approval |
| **Consolidator** | Eliminates code duplication, dead modules, and stale docs | Weekly | `@sigmaos-devs/consolidators` | Mostly agent |
| **Researcher** | Studies reference OSes (Linux, BSDs) and writes design notes | Weekly / Monthly | `@sigmaos-devs/researchers` | Agent drafts, human decides |
| **Triager** | Sorts issues, reproduces bugs, and sets severity | Daily | `@sigmaos-devs/triagers` | Mostly agent |
| **Tester/Fuzzer** | Operates test suites, continuous fuzzing, and HCL testing | Daily / Nightly | `@sigmaos-devs/testers-fuzzers` | Agent, with humans for physical hardware |
| **Security Officer** | Manages advisories, `unsafe` audits, and signing keys | Daily / Monthly | `@sigmaos-devs/security-officers` | Human-led |
| **Release Manager** | Builds ISOs, executes release gates, signs binaries | Per release | `@sigmaos-devs/release-managers` | Agent builds, human signs off |
| **Docs Steward** | Synchronizes documentation and README with CI evidence | Weekly | `@sigmaos-devs/docs-stewards` | Agent drafts, human checks |
| **Benchmarker** | Operates performance pipelines and publishes scoreboards | Weekly / Monthly | `@sigmaos-devs/benchmark-officers` | Agent |
| **Community Manager** | Onboards contributors, answers issues, publishes updates | Daily | `@sigmaos-devs/community-managers` | Human |

---

## Reality Check & Differentiator Wedges

1. **Focus Over Total Domination**: Winning in every aspect against Linux, FreeBSD, and OpenBSD simultaneously is unachievable due to decades of ecosystem momentum. SigmaOS wins by establishing provable technical superiority in specific, strategic "wedges":
   - **Memory-Safe Kernel & Userland**: A native `#![no_std]` Rust microkernel with uncontained C memory safety eliminated.
   - **Atomic & Rollbackable Updates**: Generation-based A/B system updates with zero-downtime transactional rollback by default (inspired by NixOS, Fedora Silverblue/bootc, and FreeBSD boot environments).
   - **Capability Security by Default**: Process sandboxing enforcing minimal privileges via `pledge`/`unveil`, Capsicum capability rights, and Landlock LSM policies.
   - **AI Agent Maintainability**: Machine-readable telemetry, TOML task schemas, strict CI test gates, and automated regression ratchets enabling safe maintenance by AI agents.
2. **Evidence-Based Ground Truth**: Mocks and unit test pass rates alone do not establish OS readiness. Features are marked complete in `FEATURE_STATUS.toml` only when backed by verifiable CI logs, QEMU boot serial outputs, or physical hardware execution evidence.
3. **Docs Sprawl Consolidation**: Overlapping parity documents and mirrored encyclopedia files are consolidated into single sources of truth.

---

## Part 1: Learning from Linux and BSD Ecosystems

### Subsystem Reference Matrix

| Area | Research Targets | Core Architectural Takeaways |
| :--- | :--- | :--- |
| **Kernel & Drivers** | Linux (`kernel/`, `drivers/`), FreeBSD, NetBSD | Driver model abstraction, FDT/ACPI parsing, unified boot flow |
| **Security** | OpenBSD (`pledge`, `unveil`, W^X, KARL), FreeBSD Capsicum, Landlock | Minimal-privilege defaults, capability delegation, W^X enforcement |
| **Rust in Kernels** | Redox OS, Linux `rust/`, Theseus, Asterinas | Safe FFI wrappers, zero-copy driver buffers, ABI stability |
| **Updates & Rollback** | NixOS, Fedora Silverblue/bootc, FreeBSD boot environments | Atomic state generations, CoW snapshot rollbacks |
| **Packaging** | Arch (`pacman`, AUR), Alpine (`apk`), Void (`xbps`), Nix | Fast dependency resolution, reproducible builds, PQC package signatures |
| **Init & Supervision** | systemd, runit, s6, OpenRC | Dependency graph parsing, fast parallel initialization, process supervision |
| **Desktop** | Wayland compositors (wlroots, Sway, Hyprland), GNOME, KDE | Sub-millisecond latency, Wayland protocol coverage, WCAG 2.1 AAA accessibility |
| **Installer & UX** | Debian Installer, Calamares, Fedora Anaconda, Linux Mint | Hardware auto-detection, non-destructive partition resizing, onboarding wizard |
| **Quality & Verification** | Linux (`syzkaller`, `KUnit`, `kselftest`), OpenBSD review culture | Continuous syscall fuzzing, regression test ratchets, bit-for-bit build reproducibility |

### Borrowing & Licensing Rules
- **Licensing Boundaries**: Do not copy GPL code (Linux, GNOME) into non-GPL components. Code from permissively licensed projects (BSD, MIT, Apache 2.0) may be adapted with explicit attribution.
- **Design Notes Requirement**: Every borrowed design must be documented in a 1-page design note covering what it does, why it works, the simplest Rust implementation, and testable acceptance criteria.

---

## Phased Roadmap

### Phase 0: Ground Truth (Weeks 1–4)
- Automated CI booting the kernel in QEMU across x86_64, AArch64, and RISC-V with serial output assertions.
- Convert `FEATURE_STATUS.toml` entries to evidence-linked statuses.
- Consolidate duplicate documentation into single authoritative sources.

### Phase 1: Real Hardware Boot (Months 1–3)
- Address kernel gaps: user-mode MSR/paging transition, MMIO mapping for NVMe/AHCI, VirtIO NIC driver, ACPI AML interpreter, SLIT parsing.
- Dynamic ELF loader (`ld-sigma`) and prioritized Linux ABI translation layer.
- Maintain a public Hardware Compatibility List (HCL) covering 3–5 physical test machines.

### Phase 2: Usability & Dogfooding (Months 3–6)
- Native DRM/KMS graphics pipeline, persistent filesystem with power-loss crash consistency testing, signed package repos, functional installer.
- Daily dogfooding in a primary VM and on a physical test laptop.

### Phase 3: Benchmark Victory & Hardening (Months 6–12)
- Publish monthly head-to-head scoreboards against Debian, Arch, Alpine, FreeBSD, and OpenBSD on identical hardware.
- Continuous kernel fuzzing via `fuzz/` targets, reproducible build verification, and third-party security audits.

---

## Monthly Scoreboard Metrics

1. Cold boot time to interactive shell (seconds).
2. Idle RAM consumption (MB).
3. Verified physical hardware platforms on HCL.
4. Continuous fuzz-hours without kernel panic.
5. Reported CVE count and mean time to fix (hours).
6. Update and generation rollback time (seconds).
7. Reproducible build percentage (%).
8. Package repository availability count.

---

## Part 2: Recurring AI-Agent Maintenance Plan

### Maintenance Cadence

- **Per Commit / PR**: `cargo check`, `clippy`, full test suite, QEMU boot check (x86_64/AArch64/RISC-V), `unsafe` diff analysis, license/secret scan.
- **Daily**: Syscall fuzzing run, crash triage, dependency CVE scan, nightly ISO boot validation, issue queue triage.
- **Weekly**: Scoreboard benchmark execution, upstream watcher digest, stale PR/issue cleanup, dead code removal, status file audit against CI artifacts.
- **Monthly**: Hardware compatibility re-testing, competitive benchmark rerun, security review of `unsafe` blocks, roadmap re-prioritization.
- **Quarterly**: Release candidate build, reproducible-build verification, external security audit, agent retrospective.

### Mandatory AI-Agent Guardrails

1. **Human Approval Gate**: AI agents may propose changes but **must never self-merge** PRs touching the kernel, cryptography, bootloader, installer, or release signing keys.
2. **Evidence Requirement**: No feature or task may be marked complete without a linked, reproducible CI log or execution artifact.
3. **Least Privilege & Isolation**: Agents operate with scoped access tokens, zero access to signing keys, and no direct push access to `main`.
4. **Reviewable PR Caps**: Agent PRs must be small, single-purpose, accompanied by unit/integration tests, and capped per day to maintain review velocity.
5. **Regression Ratchet**: Every fixed bug requires an added test. Test coverage, `unsafe` count, and performance metrics cannot regress without explicit human override.
6. **Immutable Audit Trail**: Every agent action is recorded in an immutable ledger with one-command rollback.
7. **Public Honesty**: Public documentation and README must accurately state current real-hardware and QEMU execution status.

---

## Detailed 12-Role Operating Model

### 1. Maintainer (`@sigmaos-devs/maintainers`)
- **Mission**: Owns subsystem health, architectural standards, and merge authority.
- **Tasks**: Daily PR reviews, weekly backlog grooming, monthly subsystem health reports.
- **Limits**: No maintainer merges their own PR without secondary approval.

### 2. Updater (`@sigmaos-devs/updaters`)
- **Mission**: Keeps dependencies, Rust toolchain, crates, and upstream advisories current.
- **Tasks**: Daily vulnerability scans, weekly crate/toolchain updates, weekly upstream Linux/BSD digests.
- **Limits**: One dependency per PR; no auto-merge on critical subsystem dependencies.

### 3. Reviewer (`@sigmaos-devs/reviewers`)
- **Mission**: Checks every PR for correctness, capability security, safety comments, and performance impact.
- **Tasks**: First-pass automated checks, human review within 48 hours.
- **Limits**: Agent reviewers can block or comment but never merge.

### 4. Consolidator (`@sigmaos-devs/consolidators`)
- **Mission**: Eliminates code duplication, dead modules, and redundant documentation to maintain repository clarity.
- **Tasks**: Weekly code/docs cleanup PRs, monthly removal of untested legacy features.
- **Limits**: Must link CI evidence proving code/doc is obsolete before removal.

### 5. Researcher (`@sigmaos-devs/researchers`)
- **Mission**: Studies reference OS architectures and writes actionable 1-page design notes.
- **Tasks**: Weekly analysis of Linux/BSD subsystems, monthly gap analysis against scoreboards.
- **Limits**: Strict adherence to non-GPL copying rules with source citations.

### 6. Triager (`@sigmaos-devs/triagers`)
- **Mission**: Manages issue queues, deduplicates bug reports, and verifies reproduction steps.
- **Tasks**: Daily issue labeling and severity assignment (S0–S3), weekly stale issue sweeps.

### 7. Tester/Fuzzer (`@sigmaos-devs/testers-fuzzers`)
- **Mission**: Maintains test pipelines, continuous kernel fuzzers, and hardware test checklists.
- **Tasks**: Nightly full test suites, QEMU multi-arch boot validation, continuous fuzzing campaigns.

### 8. Security Officer (`@sigmaos-devs/security-officers`)
- **Mission**: Oversees vulnerability disclosure, `unsafe` Rust audits, capability boundaries, and key management.
- **Tasks**: Daily security advisory tracking, weekly `unsafe` diff reviews, monthly key rotation.
- **Limits**: Release signing keys remain strictly in human possession.

### 9. Release Manager (`@sigmaos-devs/release-managers`)
- **Mission**: Coordinates release freezes, ISO image creation, reproducible build checks, and release publishing.
- **Tasks**: Pre-release freezes, release candidate builds, post-release regression monitoring.

### 10. Docs Steward (`@sigmaos-devs/docs-stewards`)
- **Mission**: Keeps user manuals, API docs, and `README.md` synchronized with verified code state.
- **Tasks**: Weekly doc compile check, monthly update of Hardware Compatibility List from CI artifacts.

### 11. Benchmarker (`@sigmaos-devs/benchmark-officers`)
- **Mission**: Measures system performance, flags regressions, and publishes competitive scoreboards.
- **Tasks**: Weekly performance regression tracking, monthly multi-OS benchmark runs.

### 12. Community Manager (`@sigmaos-devs/community-managers`)
- **Mission**: Welcomes contributors, manages onboarding guides, and maintains community communication.
- **Tasks**: Daily response to user questions, weekly "good first issue" labeling, monthly progress updates.

---

## Inter-Role Workflows & Weekly Operational Calendar

```
Researcher (Design Note) ---> Triager (Issue Backlog) ---> Contributor / Fixer (PR)
                                                                 |
                                                                 v
Docs Steward <--- Maintainer (Merge) <--- Tester / Benchmarker <--- Reviewer (Audit & Guardrails)
```

| Day | Core Operational Focus |
| :--- | :--- |
| **Monday** | Triage sweep, planning, Upstream Watcher digest publication. |
| **Tuesday–Wednesday** | Feature development and code review; Researcher publishes design notes. |
| **Thursday** | Consolidator and Docs Steward cleanup PRs; weekly benchmark run. |
| **Friday** | Scheduled release candidate build, weekly health report generation, operational retrospective. |

---

## Mandatory Rules for Every Role

1. **Evidence Over Claims**: Nothing is marked "complete" without a linked CI log or execution artifact.
2. **Human Override Gate**: AI agents propose; humans approve changes to kernel, crypto, bootloader, installer, or keys.
3. **Scoped Least Privilege**: Agents use minimal token permissions and cannot push directly to `main`.
4. **Atomic PRs**: Small, single-purpose pull requests with attached unit/integration tests.
5. **Regression Prevention**: Every fixed bug adds a regression test.
