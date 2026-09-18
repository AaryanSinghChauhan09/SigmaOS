# AGENTS_AUTOMATIC_MANAGEMENT.md — AI Agent Automatic & Autonomous Management Guidelines for SigmaOS

Welcome, AI Agent! This document defines the standards, architectural models, safety guardrails, and verification protocols for managing, developing, and extending **Autonomous Systems, Self-Healing Automation, and Automatic Background Management** in **SigmaOS**.

---

## 1. SigmaOS Autonomous System Architecture Overview

SigmaOS incorporates self-managing, autonomous subsystems that maintain system health, optimize resource usage, heal kernel faults, and automate maintenance without requiring human intervention.

### Core Autonomous Management Domains
* **Kernel & Cryptographic Self-Healing (`src/nextgen.rs`, `src/resilience/`)**:
  - `PqcSelfHealing` & `KernelPatchVerificationEngine`: Automatic livepatch verification, thread stack integrity restoration, and post-quantum cryptographic self-healing.
  - Automatic Btrfs/ZFS CoW snapshot rollback guard on failed updates (`SnapperTransactionGuard`).
* **Automated Package & Storage Maintenance (`src/package/cache.rs`, `src/sigpkg/nixos.rs`)**:
  - `PackageCacheEngine` & `NixProfileStore`: Automatic content-addressed store (CAS) blob garbage collection, unreferenced generation pruning, and cache size bounds enforcement.
* **Automated Log Rotation & Storage Cleanup (`src/logging/structured_logging.rs`, `src/system/cleanup.rs`)**:
  - `LogRotationEngine`: Multi-generation compressed log rotation based on byte size or age limits.
  - `SystemCleanupManager`: Automated temporary file, log archive, and cache directory purging.
* **Automated Thermal, Power & Performance Adaptation (`src/kernel/processor_management.rs`, `src/nextgen_innovations.rs`)**:
  - `GarudaPerformanceTweakEngine` & `ThermalGovernorState`: Automatic CPU governor profile switching (Performance vs Powersave) based on AC power status and thermal sensor telemetry.
  - `PolicyAdaptiveEventScheduler`: Workload latency prediction and automatic thread priority tuning.

---

## 2. Guidelines for Autonomous Systems Development

When developing or extending automatic background management logic:

### 1. Bounded Automation Loops
* **Oscillation & Thundering Herd Prevention**: Ensure automated feedback loops (e.g. thermal throttling or cache pruning) employ hysteresis or jitter delays (`RANDOM_DELAY`) to prevent rapid state oscillation.
* **Non-Disruptive Background Execution**: Automated cleanup and garbage collection tasks must run at low scheduling priority (e.g., `SCHED_IDLE` or nice +19) to prevent interfering with real-time UI compositing or user interactive tasks.

### 2. Atomic & Fail-Closed Rollback Guarantees
* **Pre-Change Snapshots**: Any automated system modification (such as livepatching, kernel parameter tuning, or package updates) must create an atomic CoW snapshot (`btrfs` / `ZFS` / `Snapper`) before applying changes.
* **Fail-Closed Fallback**: If an automated state update fails verification, automatically revert to the previous verified generation (`NixProfileStore` rollback) and alert structured journal logs.

### 3. Auditability of Autonomous Actions
* All automated system interventions (e.g., automatic process termination, log rotation, thermal governor switching) must issue a structured journal record (`StructuredLogEntry`) with `SYSLOG_IDENTIFIER=sigma-auto-manager`.

---

## 3. Verification & Testing Protocols

1. **REPL Automation CLI Commands**: Inspect and trigger autonomous engines via interactive Shell REPL commands:
   - `init` / `sv`: Test lightweight init process supervisor and crash recovery.
   - `journalctl` / `logrotate`: Verify automated log rotation and storage bounds.
   - `cpufreq` / `perf`: Inspect automatic thermal governor and CPU frequency adjustments.
2. **Core Test Runner Execution**:
   ```bash
   ./run_sigma_tests.sh
   ```

---

## 4. Pre-Commit Checklist for Automation Changes

Before submitting autonomous management or self-healing changes:
- [ ] Verified non-disruptive scheduling priority for background maintenance loops.
- [ ] Verified atomic CoW snapshot creation prior to automated system updates.
- [ ] Confirmed all autonomous actions produce structured audit journal logs.
- [ ] Executed `./run_sigma_tests.sh` with 100% test pass rate.
- [ ] Requested automated code review using `request_code_review`.
- [ ] Recorded automation learnings using `initiate_memory_recording`.
