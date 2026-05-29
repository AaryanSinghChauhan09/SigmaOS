# SigmaOS Differentiation Blueprint (2026-2027)

This blueprint turns SigmaOS vision into an execution plan that outpaces SteamOS, Clear Linux, NixOS, Fedora CoreOS, Flatcar, Solus, Rescuezilla, and RancherOS by combining **sovereign low-level subsystems** with **polished user-facing experience**.

## 1) Strategic Positioning

SigmaOS wins only if it is better in both axes:

- **Technical sovereignty:** own critical runtime layers (boot, net, container isolation, libc primitives, driver path).
- **UX and operations quality:** polished Zenith desktop, resilient recovery, automation, and transparent docs/release workflows.

## 2) Current Gaps (Most Material)

- **Hardware support breadth:** NIC path improving; GPU/Wi-Fi/Bluetooth and broad platform enablement still limited.
- **Container execution depth:** native namespace/cgroup intent exists in CLI, but full kernel orchestrator enforcement must be completed end-to-end.
- **Recovery UX:** rollback + resilient fallback path exists, but guided recovery UX (text + GUI) is still early.
- **Desktop completeness:** Zenith compositor/toolkit exists, but cohesive windowing/personalization flow needs production hardening.
- **Build/package reproducibility:** strong direction exists, but registry-backed deterministic package lifecycle and validation pipeline needs formalization.

## 3) Phase Plan

## Phase A - Hardening Core Sovereignty (0-90 days)

- **Networking:** complete RX/TX driver-to-stack loop for active NICs; enforce single socket ABI ownership.
- **Containers:** finish orchestrator handling for native namespace/cgroup spec from `sigma-pod run-native`.
- **Boot resilience:** promote safe-mode policy to default for repeated failed boot attempts.
- **Minimal libc path:** prioritize memory, string, and formatted output routines on hot codepaths.

Exit criteria:
- Boot success >= 99% in CI virtual profiles.
- Packet TX/RX integration tests passing on at least two NIC targets.
- Native pod launch applies namespace and cgroup limits in kernel path.

## Phase B - Product-Grade UX + Recovery (90-180 days)

- **Zenith polish:** stable compositor loop, predictable window placement, accessibility hooks, and deterministic input handling.
- **Auto-tiling WM integration:** first-party tiling policy in Zenith with profile-aware defaults.
- **Personalization engine:** declarative profile (`~/.sigma_profile`) + theme/layout presets.
- **Recovery surface:** recovery assistant for rollback/snapshot selection and diagnostics export.

Exit criteria:
- UI smoke suite passes across standard profile matrix.
- Recovery flow can restore known-good boot without manual kernel edits.
- Profile-driven desktop state reliably restored after reboot.

## Phase C - Ecosystem + Transparency (180-365 days)

- **Sovereign registry maturity:** deterministic build recipes, signatures, and provenance checks.
- **Automation hooks:** update/backup/recovery orchestration scripts and scheduler integration.
- **GitHub-first transparency:** changelog discipline, docs updates per subsystem change, CI gating for kernel/driver/UI paths.

Exit criteria:
- Every subsystem PR includes test evidence + docs update.
- Release candidates include validated rollback and recovery evidence.
- Wiki pages map directly to maintained subsystem owners.

## 4) Competitor-Specific Surpass Strategy

- **SteamOS:** beat with sovereign graphics + predictable low-latency desktop stack and integrated dev-oriented tooling.
- **Clear Linux:** beat with silicon-aware scheduling + cross-vendor tuning, not vendor-specific optimization only.
- **NixOS:** beat with sovereign reproducible recipes + policy-backed registry + first-party operational automation.
- **Fedora CoreOS / Flatcar:** beat with immutable-style updates + automatic rollback + stronger local recovery.
- **RancherOS:** beat with native container-first orchestration that does not depend on Docker daemon semantics.
- **Solus:** beat with cohesive Zenith UX and first-class personalization/declarative profile flow.
- **Rescuezilla/SystemRescue:** beat with integrated recovery mode and rollback diagnostics built into normal operation lifecycle.

## 5) Operating Rules

- No new subsystem is "done" unless it has:
  - runtime tests,
  - recovery behavior,
  - docs update,
  - ownership declaration.
- Prefer replacing dependency hot paths incrementally over risky all-at-once rewrites.
- Treat reliability and polish as equal release gates.

