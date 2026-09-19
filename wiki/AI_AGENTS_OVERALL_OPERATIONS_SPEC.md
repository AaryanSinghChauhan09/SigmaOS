# AI Agents Overall Operations Management Specification for SigmaOS

## Abstract
This master specification defines the end-to-end overall operations management framework for AI agents operating within or administering SigmaOS. It integrates all sub-domain specifications (Access Control, Package Management, Service & Operations, Bare-Metal Provisioning, Bottom-Half Interrupts, Buffer Overflow Security, and Clock Replacement) into a unified operational playbook.

---

## 1. Master System Administration Lifecycle

```
                     ┌───────────────────────────────┐
                     │   AI Agent Controller Task   │
                     └───────────────┬───────────────┘
                                     │
         ┌───────────────────────────┼───────────────────────────┐
         ▼                           ▼                           ▼
┌──────────────────┐    ┌─────────────────────────┐    ┌──────────────────┐
│ Security & Access│    │ Core System Operations  │    │ Package & Build  │
│  • Token Auth    │    │  • Service Supervision  │    │  • Universal PKG │
│  • Pledge/Unveil │    │  • Process Scheduling   │    │  • Sandbox Build │
│  • Landlock LSM  │    │  • Sysctl / Power / VFS │    │  • CAS / Rollback│
└──────────────────┘    └─────────────────────────┘    └──────────────────┘
         │                           │                           │
         └───────────────────────────┼───────────────────────────┘
                                     │
                                     ▼
                     ┌───────────────────────────────┐
                     │ SovereignUniversalDistroBridge│
                     └───────────────┬───────────────┘
                                     │
                                     ▼
                     ┌───────────────────────────────┐
                     │    SigmaOS Kernel & MMU       │
                     └───────────────────────────────┘
```

---

## 2. Core Operational Pillars

### 2.1 Access Control & Security Governance
- **Least Privilege**: Agents run under unprivileged, sandboxed execution domains (`agent_domain_t`).
- **Capability Tokens**: Privileged operations require explicit user approval or scoped capability tokens.
- **Memory Protection**: Enforces W^X page permissions (`SovereignKaslrWxAllocator`), stack guard pages, and OpenBSD Retguard XOR canaries (`OpenBsdRetguardEngine`).

### 2.2 System Operations & Service Supervision
- **Service Init Control**: Query, start, and stop system services across Systemd, OpenRC, Runit, and Dinit via `SovereignUniversalDistroBridge`.
- **Hybrid Process Scheduling**: Configure task scheduling policies across Linux 6.12+ `sched_ext` BPF schedulers, Apache NuttX POSIX RT preemption-threshold governors, and CachyOS BORE interactivity tuners.
- **Memory & Clock Replacement**: Manage virtual memory demand paging, CLOCK (Second-Chance) page eviction (`SimpleVMM::perform_clock_replacement_step`), and zram swap compression.

### 2.3 Package Management & Building
- **Universal Package Interoperability**: Convert foreign formats (DEB, RPM, Pacman, APK, XBPS, Ebuild, Nix) to native `SigmaPkg` format.
- **Dry-Run & Pre-Flight Snapshots**: Always run dry-run simulations (`UniversalDryRunSimulator`) and create atomic Btrfs/ZFS snapshots prior to system modifications.

### 2.4 Bare-Metal Provisioning & Emergency Recovery
- **Hardware Auto-Probing**: Discover servers, inspect PCIe/NVMe devices (`SovereignDeviceManager`), and verify UEFI Secure Boot keys.
- **Atomic Image Deployments**: Deploy immutable system rootfs images with sub-millisecond rollback capabilities.
- **Disaster Recovery**: Automated panic handling, vmcore crashdumps (`SovereignKdumpEngine`), and emergency rescue console handoffs (`SigmaRescue`).

---

## 3. Inter-Subsystem Coordination & Event Routing

- **Unified Bridge Dispatch**: Cross-subsystem operations across 24+ core subsystems (Init, VFS, Package, Security, Memory, Network, Drivers, Boot) are dispatched through `SovereignUniversalDistroBridge`.
- **Audit Logging**: All operational events are logged to the append-only journal (`SovereignJournaldBinaryStorageEngine`) with Merkle tree hash chain validation (`Jbd2TransactionLedger`).

---

## 4. Operational Directives & Quality Assurance

1. **Zero-Dependency Mandate**:
   - Maintain strict zero external third-party crate dependencies in `Cargo.toml`. Preserve `#![no_std]` core compatibility.
2. **Mandatory Testing**:
   - Every code change must be verified using the atomic test runner (`./run_sigma_tests.sh`), standalone unit tests, and Python test harness (`pytest tests/`).
3. **Governance Compliance**:
   - Adhere to the Future Development Protocol (`src/governance/future_protocol.rs`) for SIG code reviews, RFC workflows, and living developer wiki updates.

---

## 5. Wiki Synchronization

This document is synchronized across all documentation hubs via `./scripts/sync_wiki.sh`:
- `WIKI/AI_AGENTS_OVERALL_OPERATIONS_SPEC.md`
- `wiki/AI_AGENTS_OVERALL_OPERATIONS_SPEC.md`
- `wiki_repo/AI_AGENTS_OVERALL_OPERATIONS_SPEC.md`

---

*Specification Version: 1.0.0 — SigmaOS Master Operations Architecture*
