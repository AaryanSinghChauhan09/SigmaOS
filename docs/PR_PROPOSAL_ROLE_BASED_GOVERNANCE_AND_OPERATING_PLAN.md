# Pull Request Proposal: Role-Based Operating Plan, Governance, Strategic Wedge Roadmap Role-Based Governance, Strategic Wedge Roadmap & AI-Agent Maintenance Plan AI-Agent Maintenance Plan

**Branch:** `feat/role-based-operating-plan`
**Status:** Approved / Implementation Ready
**Reviewers:** @sigmaos-devs/maintainers @sigmaos-devs/security-officers @sigmaos-devs/reviewers

## Executive Summary

This pull request establishes SigmaOS's role-based governance structure, phased strategic wedge roadmap, and recurring AI-agent maintenance lifecycle. Grounded in a pragmatic reality check, it fundamentally shifts SigmaOS away from superficial feature-parity claims across hundreds of distributions toward an evidence-gated, highly disciplined operating model where every capability claim requires verifiable CI execution logs.

---

## 1. Reality Check & Strategic Wedge

### Reality Check
- **"Defeat every open-source OS in every aspect" isn't achievable.** Linux has thousands of paid maintainers and decades of hardware support; FreeBSD and OpenBSD each have deep specializations. A new OS wins by owning a few areas where it can be provably superior, not by trying to outdo everything simultaneously.
- **Addressing the main gap:** Transitioning from "unit-tested and mock-validated" primitives to real bare-metal and QEMU execution. Numbers like "1.8s boot" and "164 MB idle RAM" only count when backed by verified QEMU/hardware boot serial logs.
- **Eliminating sprawl:** Consolidate overlapping reports, mirrored encyclopedia copies, and overly broad matrices into authoritative sources of truth.

### The 4 Strategic Wedge Pillars
1. **Memory-safe Rust kernel and userland:** An end-to-end memory-safe foundation that no incumbent tier-1 OS distribution provides.
2. **Atomic, rollbackable system updates by default:** A clean, simplified generational update model inspired by NixOS and Fedora Silverblue.
3. **Capability-based security by default:** Fine-grained sandboxing and least privilege (OpenBSD pledge/unveil, FreeBSD Capsicum, Linux Landlock).
4. **Agent-native maintainability:** An operating system structured for automated AI-agent triage, maintenance, fuzzing, and benchmarking under strict human guardrails.

### Upstream Reference Study Matrix
| Area | Reference Projects | Key Takeaways for SigmaOS |
|---|---|---|
| Kernel structure, drivers | Linux (`kernel/`, `drivers/`), FreeBSD, NetBSD | Driver model, device tree, boot flow |
| Security | OpenBSD (`pledge`, `unveil`, W^X, KARL), FreeBSD Capsicum | Minimal-privilege defaults, exploit mitigations |
| Rust in kernels | Redox OS, Linux `rust/`, Theseus, Asterinas | Unsafe boundary contracts, driver ergonomics, ABI |
| Updates and rollback | NixOS, Fedora Silverblue/bootc, FreeBSD boot environments, openSUSE snapper | Atomic generations, A/B rootfs updates |
| Packaging | Arch (`pacman`, AUR), Alpine (`apk`), Void (`xbps`), Nix | Fast resolver, reproducible builds, PQC package signing |
| Init and services | systemd, runit, s6, OpenRC | Dependency graph resolution, rapid parallel boot, supervision |
| Desktop | Wayland compositors (wlroots, Sway, Hyprland), GNOME, KDE | Protocol coverage, minimal input latency, accessibility |
| Installer and UX | Debian installer, Calamares, Fedora Anaconda, Linux Mint | Hardware detection, reliable partitioning, first-run experience |
| Quality process | Linux (syzkaller, KUnit, kselftest, LTP), OpenBSD review culture, reproducible-builds.org | Fuzzing pipelines, regression suites, reproducibility |

---

## 2. Phased Strategic Roadmap

### Phase 0: Ground Truth (Weeks 1–4)
- Add CI that boots the kernel in QEMU across x86_64, AArch64, and RISC-V with serial output assertions.
- Make `FEATURE_STATUS.toml` evidence-based: features count as working only when linked to CI logs.
- Merge duplicate documentation and catalog files into single authoritative sources.

### Phase 1: Boots on Real Hardware (Months 1–3)
- Address foundational hardware and CPU execution gaps: user-mode MSR/paging setup, MMIO mapping for NVMe/AHCI, interrupt-driven VirtIO NIC, ACPI AML interpreter, and SLIT parsing.
- Provide dynamic ELF loading and prioritize Linux ABI compatibility.
- Test across 3–5 physical machines, publishing an evidence-backed Hardware Compatibility List (HCL).

### Phase 2: Usable Day-to-Day (Months 3–6)
- Enable real GPU acceleration (DRM/KMS), persistent filesystem with crash-consistency tests, an installer that handles real disks, and signed package repositories.
- Daily dogfooding: maintain team daily driver usage on VM and at least one dedicated physical laptop.

### Phase 3: Win the Wedge (Months 6–12)
- Publish automated monthly comparative benchmarks against Debian, Arch, Alpine, FreeBSD, and OpenBSD on identical hardware (boot time, idle RAM, syscall latency, rollback speed, attack-surface metrics).
- Continuous fuzzing pipeline via `fuzz/` reporting zero reproducible kernel panics over long fuzz-hours.
- Independent security audits and 100% reproducible builds.

---

## 3. 12 Maintenance Roles in the Operating Model

| Role | Mission | Main Cadence | Leadership Profile |
|---|---|---|---|
| **Maintainer** | Owns subsystem health, architectural boundaries, and code review merges. | Daily | Human-led, agent-assisted |
| **Updater** | Keeps dependencies, toolchain, advisories, and upstream knowledge current. | Daily/Weekly | Mostly agent |
| **Reviewer** | Inspects every pull request for correctness, security, style, and regression coverage. | Per PR | Agent first pass, human approval |
| **Consolidator** | Fights sprawl, removes dead code, consolidates docs, and prunes unused feature flags. | Weekly | Mostly agent |
| **Researcher** | Studies Linux/BSD upstream projects and prepares concise 1-page design notes. | Weekly/Monthly | Agent drafts, human decides |
| **Triager** | Sorts issues, eliminates duplicates, reproduces bugs, and sets severity levels. | Daily | Mostly agent |
| **Tester/Fuzzer** | Executes test suites, continuous fuzz campaigns, and hardware test checklists. | Daily/Nightly | Agent automated, humans for hardware |
| **Security Officer** | Audits unsafe blocks, manages disclosure pipelines, and maintains signing keys. | Daily/Monthly | Human-led |
| **Release Manager** | Drives stabilization freezes, builds ISO images, verifies reproducibility, and signs releases. | Per release | Agent builds, human signs off |
| **Docs Steward** | Ensures documentation matches code truth, verifies links, and updates status claims. | Weekly | Agent drafts, human checks |
| **Benchmarker** | Runs comparative scoreboard benchmarks across target OS distributions. | Weekly/Monthly | Agent |
| **Community Manager** | Handles community questions, contributor onboarding, and good-first-issue labeling. | Daily | Human |

---

## 4. Weekly Operational Calendar

- **Monday (Triage & Upstream):** Triage sweep, weekly planning, and upstream advisory/change digest generated by Updaters.
- **Tuesday–Wednesday (Core Building & Review):** Active subsystem implementation and review; Researcher publishes design notes.
- **Thursday (Consolidation & Benchmarking):** Consolidator doc and code pruning; Docs Steward sync; Benchmarker performance run.
- **Friday (Release & Retrospective):** Release candidate gate verification; weekly health report; team retrospective.

---

## 5. 8-Step Team Execution Plan

1. **Step 1: Decide what SigmaOS is for (Week 1)**: Formulate target user profile, wedge specialization, and license model. Output: `VISION.md`.
2. **Step 2: Make the project honest and clean (Weeks 1–3)**: Consolidate duplicate docs, deploy QEMU serial boot CI, convert `FEATURE_STATUS.toml` to evidence-linked claims, and update README. Output: `FEATURE_STATUS.toml`.
3. **Step 3: Assign study areas (Week 2)**: Designate subsystem leads and study rotations across reference projects. Output: `GOVERNANCE.md`.
4. **Step 4: Study each project the same way (Weeks 2–6)**: Inspect architectures, track common CVEs, and produce 1-page design notes with acceptance tests. Output: Design notes & issue backlog.
5. **Step 5: Fix known gaps first (Months 1–3)**: Resolve user-mode MSRs, MMIO mappings, VirtIO NIC, ACPI parsing, and dynamic ELF loader. Output: Hardware Compatibility List (HCL).
6. **Step 6: Make it usable day to day (Months 3–6)**: Ship reliable installer, persistent filesystem, signed repos, and enforce daily dogfooding. Output: Dogfooding feedback & bug records.
7. **Step 7: Measure against others and publish (Months 6–12)**: Execute comparative benchmarks against Linux/BSD, run continuous fuzzing, and prepare reproducible builds. Output: Monthly Scoreboard Report.
8. **Step 8: Build a community (Ongoing)**: Maintain CONTRIBUTING.md, swift PR review cycles, and transparent release notes. Output: Release Notes & Roadmap.

---

## 6. 7 Mandatory Governance Guardrails

1. **Human Approval:** Mandatory for modifications touching kernel, crypto, bootloader, installer, or cryptographic keys.
2. **Evidence-Gated Success:** No feature may be marked complete without a verified, linked CI artifact (e.g. QEMU serial boot log).
3. **Sandbox and Least Privilege:** Autonomous agents operate with scoped tokens, no key access, and no direct pushes to main.
4. **Small, Reviewable PRs:** PRs must be modular with tests; daily PR limits prevent reviewer overload.
5. **Regression Ratchet:** Every bug fix requires a test; test coverage, unsafe block count, and benchmarks cannot regress without explicit human override.
6. **Audit Log & Revertibility:** Every agent action is logged with single-command rollback capability.
7. **Honest Public Reporting:** Documentation and README state only what boots and functions today on actual machines.
