# SigmaOS Governance Framework & Role-Based Operating Plan

Welcome to the **SigmaOS Governance Framework**. This document defines the governance rules, strategic reality check, wedge prioritization, 8-step execution guide, reference subsystem study matrix, recurring AI-agent maintenance plan, and 12-role operating model for the SigmaOS project.

---

## Reality Check & Strategic Differentiator Wedges

1. **Strategic Focus Over Total Domination**: Winning in every aspect against Linux, FreeBSD, and OpenBSD simultaneously is unachievable due to decades of ecosystem momentum. SigmaOS wins by establishing provable technical superiority in specific, strategic "wedges":
   - **Memory-Safe Rust Kernel & Userland**: A native `#![no_std]` Rust microkernel and userland eliminating C memory safety risks.
   - **Atomic & Rollbackable Updates**: Generation-based A/B system updates with zero-downtime transactional rollback by default (inspired by NixOS, Fedora Silverblue/bootc, and FreeBSD boot environments).
   - **Capability Security by Default**: Process sandboxing enforcing minimal privileges via `pledge`/`unveil`, FreeBSD Capsicum capability rights, and Linux Landlock LSM policies.
   - **AI Agent Maintainability**: Machine-readable telemetry, TOML task schemas, strict CI test gates, and automated regression ratchets enabling safe maintenance by AI agents.
2. **Evidence-Based Ground Truth**: Mocks and unit test pass rates alone do not establish OS readiness. Features are marked "working" in `FEATURE_STATUS.toml` only when backed by verifiable CI logs, QEMU boot serial outputs, or physical hardware execution evidence.
3. **Docs Sprawl Consolidation**: Overlapping parity documents, mirrored encyclopedia copies, and unused status files are consolidated into single authoritative sources of truth in `docs/` and `GOVERNANCE.md`.

---

## Step-by-Step Execution Plan

1. **Step 1: Decide what SigmaOS is for (Week 1)**
   - Team meeting to pick target user (desktop, server, embedded, or security-focused).
   - Select core wedge differentiators and project licensing.
   - **Output**: `VISION.md`.

2. **Step 2: Make the project honest and clean (Weeks 1–3)**
   - Consolidate doc sprawl into single docs hierarchy.
   - Set up QEMU multi-arch boot tests in CI (x86_64, AArch64, RISC-V).
   - Change `FEATURE_STATUS.toml` to link to CI execution artifacts.
   - Update `README.md` with honest real-hardware status.
   - **Output**: CI boot badge & evidence-linked status file.

3. **Step 3: Assign study areas (Week 2)**
   - Assign team members/agents to reference OS study areas (Linux, FreeBSD, NetBSD, OpenBSD, Redox, NixOS, Alpine, Arch, systemd, wlroots).
   - **Output**: Assigned study matrix.

4. **Step 4: Study each project the same way (Weeks 2–6)**
   - Analyze architecture, codebase, bug trackers, and CVE histories.
   - Write a 1-page design note per subsystem gap.
   - Convert notes into prioritized backlog issues with acceptance criteria.
   - **Output**: Design notes & prioritized issue backlog.

5. **Step 5: Fix known gaps first (Months 1–3)**
   - User-mode transition, NVMe/AHCI MMIO mapping, VirtIO networking, dynamic ELF loader (`ld-sigma`), ACPI AML, DRM/KMS graphics.
   - **Output**: QEMU & physical hardware test logs.

6. **Step 6: Make it usable day to day (Months 3–6)**
   - System installer, persistent crash-consistent filesystem, signed package repos, daily dogfooding.
   - **Output**: Usable daily installation & dogfooding issue log.

7. **Step 7: Measure against others and publish (Months 6–12)**
   - Competitive benchmarks against Debian, Arch, Alpine, FreeBSD, OpenBSD.
   - Continuous kernel fuzzing pipeline & reproducible builds.
   - **Output**: Monthly public scoreboard.

8. **Step 8: Build a community (Ongoing)**
   - Maintain `CONTRIBUTING.md`, "good first issue" labels, fast PR review turnaround.
   - **Output**: Community growth & release cadence.

---

## Part 1: Learning from Linux and BSD Ecosystems

### Subsystem Study Matrix

| Area | Research Targets | Core Architectural Takeaways |
| :--- | :--- | :--- |
| **Kernel Structure & Drivers** | Linux (`kernel/`, `drivers/`), FreeBSD, NetBSD | Driver model, Device Tree (FDT), ACPI AML parsing, unified boot flow |
| **Security & Sandboxing** | OpenBSD (`pledge`, `unveil`, W^X, KARL), FreeBSD Capsicum | Minimal-privilege defaults, capability rights, W^X mitigations |
| **Rust in Kernels** | Redox OS, Linux `rust/`, Theseus, Asterinas | Unsafe FFI boundaries, safe driver wrappers, ABI stability |
| **Updates & Rollback** | NixOS, Fedora Silverblue/bootc, FreeBSD boot environments, openSUSE snapper | Atomic state generations, A/B updates, CoW snapshot rollbacks |
| **Packaging & Repos** | Arch (`pacman`, AUR), Alpine (`apk`), Void (`xbps`), Nix | Fast resolver, reproducible builds, package signing |
| **Init & Services** | systemd, runit, s6, OpenRC | Dependency graph parsing, fast parallel initialization, process supervision |
| **Desktop & Compositing** | Wayland compositors (wlroots, Sway, Hyprland), GNOME, KDE | Wayland protocol coverage, sub-millisecond input latency, WCAG 2.1 AAA accessibility |
| **Installer & UX** | Debian Installer, Calamares, Fedora Anaconda, Linux Mint | Hardware auto-detection, non-destructive partition resizing, onboarding wizard |
| **Quality & Verification** | Linux (`syzkaller`, `KUnit`, `kselftest`), OpenBSD review culture, reproducible-builds.org | Continuous syscall fuzzing, regression test ratchets, bit-for-bit build reproducibility |

### Licensing & Borrowing Rules
- **Non-GPL Copying Restrictions**: Do not copy GPL source code (Linux, GNOME) into non-GPL project components. Code from permissively licensed projects (BSD, MIT, Apache 2.0) may be adapted with full attribution.
- **Design Notes Requirement**: Every borrowed subsystem requires a 1-page design note specifying purpose, architecture, minimal Rust implementation, and testable acceptance criteria.

---

## Phased Roadmap

### Phase 0: Ground Truth (Weeks 1–4)
- Multi-arch QEMU boot CI (x86_64, AArch64, RISC-V) with serial output assertions.
- Convert `FEATURE_STATUS.toml` to evidence-linked statuses.
- Consolidate documentation sprawl into `docs/` hierarchy.

### Phase 1: Boots on Real Hardware (Months 1–3)
- Address kernel gaps: user-mode MSR/paging, NVMe/AHCI MMIO mapping, VirtIO NIC driver, ACPI AML interpreter, SLIT parsing.
- Dynamic ELF loader (`ld-sigma`) and prioritized Linux ABI translation layer.
- Maintain public Hardware Compatibility List (HCL) covering 3–5 physical test machines.

### Phase 2: Usability & Dogfooding (Months 3–6)
- Native DRM/KMS graphics pipeline, persistent filesystem with power-loss crash consistency testing, signed package repos, functional installer.
- Daily dogfooding in a primary VM and on a physical test laptop.

### Phase 3: Win the Wedge (Months 6–12)
- Publish head-to-head monthly performance scoreboards against Debian, Arch, Alpine, FreeBSD, and OpenBSD on identical hardware.
- Continuous kernel fuzzing via `fuzz/` targets, reproducible build verification, and third-party security audits.

---

## Monthly Scoreboard Metrics

1. Cold boot time to interactive shell (seconds).
2. Idle RAM consumption (MB).
3. Verified physical hardware platforms on HCL.
4. Continuous fuzz-hours without kernel panic.
5. Reported CVE count and mean time to fix (hours).
6. Update generation rollback time (seconds).
7. Reproducible build percentage (%).
8. Package repository availability count.

---

## Part 2: Recurring AI-Agent Maintenance Plan

### Maintenance Cadence

- **Every Commit / PR**: `cargo check`, `clippy`, full test suite, QEMU boot check (x86_64/AArch64/RISC-V), `unsafe` diff analysis, license/secret scan.
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
6. **Immutable Audit Trail**: Every agent action is recorded in an immutable ledger with one-command rollback capability.
7. **Public Honesty**: Public documentation and README must accurately state current real-hardware and QEMU execution status.

---

## 12-Role Operating Model

| Role | Mission | Main Cadence | Human / Agent Handle | Limits & Guardrails |
| :--- | :--- | :--- | :--- | :--- |
| **Maintainer** | Owns subsystem health & merge authority | Daily | `@sigmaos-devs/maintainers` | Secondary approval needed for own PRs |
| **Updater** | Keeps toolchain, crates, & upstream current | Daily / Weekly | `@sigmaos-devs/updaters` | One dependency per PR; no kernel auto-merge |
| **Reviewer** | Audits PRs for safety, security, & performance | Per PR | `@sigmaos-devs/reviewers` | Agent can block, human must approve merge |
| **Consolidator** | Eliminates code duplication & doc sprawl | Weekly | `@sigmaos-devs/consolidators` | Link evidence proving target is obsolete |
| **Researcher** | Studies reference OSes & writes design notes | Weekly / Monthly | `@sigmaos-devs/researchers` | Respect non-GPL rules; cite all sources |
| **Triager** | Sorts issues, reproduces bugs, sets severity | Daily | `@sigmaos-devs/triagers` | Daily queue SLA; polite stale sweeps |
| **Tester/Fuzzer** | Operates test suites, fuzzers, & HCL checklists | Daily / Nightly | `@sigmaos-devs/testers-fuzzers` | Minimize crashes into regression tests |
| **Security Officer** | Manages advisories, `unsafe` audits, & keys | Daily / Monthly | `@sigmaos-devs/security-officers` | Signing keys strictly human-held |
| **Release Manager** | Coordinates builds, gates, & ISO publishing | Per release | `@sigmaos-devs/release-managers` | Agent builds, human signs off |
| **Docs Steward** | Synchronizes docs & README with CI evidence | Weekly | `@sigmaos-devs/docs-stewards` | Sync claims with verified CI artifacts |
| **Benchmarker** | Operates performance tracking & scoreboards | Weekly / Monthly | `@sigmaos-devs/benchmark-officers` | Identical hardware benchmark comparisons |
| **Community Manager**| Onboards contributors & publishes updates | Daily | `@sigmaos-devs/community-managers` | Polite communication & fast PR triage |

---

## Inter-Role Workflows & Weekly Operational Calendar

```
Researcher (Design Note) ---> Triager (Issue Backlog) ---> Contributor / Builder (PR)
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

## Bootstrap Rollout Strategy

To prevent overload during initial implementation, SigmaOS activates four core roles during weeks 1–4:
1. **Triager**: `@sigmaos-devs/triagers`
2. **Reviewer**: `@sigmaos-devs/reviewers`
3. **Updater**: `@sigmaos-devs/updaters`
4. **Tester**: `@sigmaos-devs/testers-fuzzers`

After 4 weeks, assess team velocity and activate the remaining eight roles.
