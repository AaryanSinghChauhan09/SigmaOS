# AI Agent Guidelines: Problem Management in SigmaOS

## Overview
This document defines guidelines for AI agents working on **Problem Management**, fault diagnosis, root cause analysis, crash recovery, and automated incident resolution in SigmaOS.

SigmaOS integrates proactive AI diagnostics, self-healing driver supervision, structured kernel crash dumps, and predictive telemetry to detect, isolate, and remediate system anomalies before they impact system stability.

---

## 1. Problem Management Architectural Subsystems

AI agents interacting with problem management in SigmaOS must interface with the following core subsystems:

| Subsystem | Location | Description |
| :--- | :--- | :--- |
| **`SigmaPulse`** | `src/futuristic_modules.rs` | Real-time system telemetry engine providing AI predictive failure alerts, anomaly scoring, and health telemetry. |
| **`SigmaCortex`** | `src/futuristic_modules.rs` | Cognitive kernel workflow interpreter that analyzes crash traces, stack frames, and execution logs to propose fixes. |
| **`ReincarnationServer`** | `src/open_source_os_gap_closure.rs` | MINIX 3 inspired self-healing driver supervisor that automatically restarts panicked or hung kernel driver threads without dropping system state. |
| **`LinuxCoreDumpFilterEngine`** | `src/compatibility/linux_distro_parity.rs` | Core dump collector generating compressed ELF coredump headers according to `/proc/sys/kernel/core_pattern`. |
| **`JournalEntry` / `Journald`** | `src/init/systemd_init.rs`, `src/distro/linux_bsd_distro_gaps.rs` | Structured binary journal log indexing, fast field searching, and rotation. |
| **`SecurityAdvisoryTracker`** | `src/arch_kernel_inspirations.rs` | CVE advisory tracking, package vulnerability classification (`Vulnerable`, `Fixed`, `Unaffected`), and automated patch verification. |

---

## 2. Problem Diagnosis & Incident Workflow

When an issue, bug, or crash is reported, AI agents must follow this structured 5-stage problem management protocol:

```
+-------------------+     +---------------------+     +-----------------------+
| 1. Incident       | --> | 2. Telemetry & Log  | --> | 3. Root Cause         |
|    Detection      |     |    Extraction       |     |    Isolation          |
+-------------------+     +---------------------+     +-----------------------+
                                                              |
+-------------------+     +---------------------+             v
| 5. Regression     | <-- | 4. Automated Patch  | <--------------------------+
|    Verification   |     |    & Verification   |
+-------------------+     +---------------------+
```

### Stage 1: Incident Detection & Logging
- Monitor `SigmaPulse` telemetry streams for anomaly scores exceeding `0.85`.
- Retrieve binary journal logs using `SovereignJournaldBinaryStorageEngine` or standard unit journal streams.

### Stage 2: Telemetry & Log Extraction
- Parse kernel panics, register dumps (`CpuContextState`), and ELF core dumps via `LinuxCoreDumpFilterEngine`.
- Extract thread call stacks and memory addresses.

### Stage 3: Root Cause Isolation ("Diagnose Before Changing")
- **Rule:** Never attempt package or environment changes before identifying the root cause.
- Trace panics to the originating module (e.g., driver memory dereference, lock contention, memory leak, or ABI mismatch).

### Stage 4: Automated Patch & Verification
- Apply targeted code changes at the source level. Never modify build artifacts (`dist/`, `build/`).
- Use atomic live patching (`AtomicTrampolineGenerator` in `src/distro/nextgen.rs`) for zero-downtime hotfixes when applicable.

### Stage 5: Regression Verification
- Run tests using `./run_sigma_tests.sh`.
- Ensure zero performance or security regressions across all 21 supported Linux/BSD distro modes.

---

## 3. Crash Dump & Kernel Panic Protocols

When working with kernel panics and core dumps:
1. **Header Inspection:** Validate ELF coredump headers produced by `LinuxCoreDumpFilterEngine`.
2. **Context Reconstruction:** Reconstruct the saved register state across target architectures (`X86Context`, `X64Context`, `Arm64Context`, `Riscv64Context`, `LoongArch64Context`, `Ppc64Context`, `S390xContext`).
3. **No-Panic Recovery:** In driver code, prefer returning `Result<T, DriverError>` over invoking `panic!()`. Use `ReincarnationServer` to isolate untrusted device drivers in unprivileged sandboxes.

---

## 4. Security Advisory & CVE Problem Tracking

When addressing security vulnerabilities:
1. **Advisory Lookup:** Check package status against `SecurityAdvisoryTracker`.
2. **Severity Gating:** Priority 1 critical vulnerabilities (CVSS >= 9.0) require immediate patch deployment and reproducible build validation (`ReproducibleBuildVerdict`).
3. **Quorum Signoff:** Validate package signoffs using `PackageSignoff` maintainer quorums prior to pushing updates to core package repositories.

---

## 5. AI Agent Self-Assessment Checklist

Before marking any problem management task as complete, AI agents must verify:

- [ ] Has the root cause been explicitly diagnosed and documented?
- [ ] Were source files modified rather than build artifacts?
- [ ] Has `./run_sigma_tests.sh` been executed and all tests confirmed passing?
- [ ] Is `#![no_std]` zero-dependency compliance maintained across edited files?
- [ ] Are logs and telemetry updated to reflect the resolved issue state?
