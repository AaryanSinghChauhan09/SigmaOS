# AGENTS_ACTIVITY_MANAGEMENT.md — AI Agent Activity & Task Management Guidelines for SigmaOS

Welcome, AI Agent! This document defines the standards, architectural models, telemetry rules, and verification protocols for managing, developing, and extending **Activity Tracking, Task Scheduling, Event Logging, and Audit Systems** in **SigmaOS**.

---

## 1. SigmaOS Activity Management Architecture Overview

SigmaOS implements a unified activity and observability framework spanning process execution tracking, structured journald logging, eBPF event tracing, and automated workload scheduling.

### Core Activity & Telemetry Modules
* **Process & Activity Lifecycle (`src/process/`, `src/kernel/processor_management.rs`)**:
  - Process Control Block (`Pcb`) state monitoring (Ready, Running, Blocked, Zombie).
  - CPU hardware performance counters (`HardwarePerfCounters` PMC) measuring instructions, cycles, and cache misses per activity.
* **Structured Logging & Journald Subsystem (`src/logging/structured_logging.rs`)**:
  - Systemd-journald & Syslog inspired key-value structured log records (`StructuredLogEntry`).
  - Automated multi-generation compressed log rotation (`LogRotationEngine`).
  - RFC 5424 / RFC 3164 Syslog UDP/TCP remote forwarding (`RemoteLogForwarder`).
* **eBPF Activity & Security Event Tracing (`src/kernel/ebpf.rs`, `src/tracing/`)**:
  - Ring buffer eBPF event stream capturing syscall activity, file I/O, network socket flows, and process fork/exec calls.
  - Compliance audit logging (`ComplianceAuditLogger`) recording security-relevant state changes.
* **Workload & Cron Event Scheduling (`src/automation/`, `src/system/cron.rs`)**:
  - Vixie Cron / Anacron schedule macro expansion (`@reboot`, `@daily`, `@hourly`).
  - Thundering-herd jitter prevention (`RANDOM_DELAY`) and system load average threshold checks.

---

## 2. Activity Management Guidelines for AI Agents

When modifying or adding activity tracking, logging, or task scheduling logic:

### 1. Structured Logging Field Standards
* **Mandatory Journal Fields**: Every structured activity log entry must supply:
  - `TIMESTAMP_NS`: Nanosecond timestamp since boot.
  - `PRIORITY`: Syslog priority level (0 = Emergency to 7 = Debug).
  - `SYSLOG_IDENTIFIER`: Process or subsystem binary name.
  - `MESSAGE`: Descriptive human-readable activity text.
* **Privacy & Sanitization**: Never record plain-text credentials, authentication tokens, encryption keys, or unsanitized user inputs into activity logs. Use `sanitize_for_log()` helpers.

### 2. Non-Blocking Telemetry & Ring Buffer Safety
* **Zero Allocation in Hot Paths**: Performance counter collection and eBPF activity probes must use bounded ring buffers (`RingBuffer` / `HeapRingBuffer`) to prevent memory allocation overhead or blocking syscall execution.
* **Ring Buffer Overflow Protection**: Drop or overwrite lowest-priority debug events when ring buffers are full under high I/O throughput.

### 3. Resource Quotas & Activity Throttling
* **racct / cgroups v2 Limits**: When observing background activities exceeding CPU or memory thresholds, apply dynamic throttling using FreeBSD `racct` rules or Linux cgroups v2 quotas (`throttle_racct_resource`).

---

## 3. Verification & Testing Protocols

1. **REPL Activity CLI Commands**: Test activity tracking via interactive Shell REPL commands:
   - `journalctl`: Query, filter, and inspect structured journal logs.
   - `logrotate`: Force multi-generation log file rotation.
   - `logger`: Submit test structured log messages.
   - `perf` / `lscpu`: Query hardware PMC performance counters and CPU core topology.
2. **Core Test Runner**: Execute the full test suite runner:
   ```bash
   ./run_sigma_tests.sh
   ```

---

## 4. Pre-Commit Checklist for Activity Changes

Before submitting activity management changes:
- [ ] Confirmed all new log records contain required structured key-value fields.
- [ ] Confirmed no unsanitized user data or credentials are logged.
- [ ] Verified non-blocking ring buffer bounds for high-frequency activity tracing.
- [ ] Executed `./run_sigma_tests.sh` with 100% test pass rate.
- [ ] Requested automated code review using `request_code_review`.
- [ ] Recorded activity management learnings using `initiate_memory_recording`.
