# AGENTS_ASSIGNMENT_MANAGEMENT.md — AI Agent Assignment & Task Routing Guidelines for SigmaOS

Welcome, AI Agent! This document defines the standards, architectural models, capability bounds, and routing protocols for managing, developing, and extending **Task Assignment, Core Pinning, Capability Binding, and Domain Ownership Systems** in **SigmaOS**.

---

## 1. SigmaOS Assignment Subsystem Overview

SigmaOS incorporates multi-dimensional assignment systems ranging from real-time thread CPU affinity assignment to security capability delegation and Special Interest Group (SIG) domain ownership assignment.

### Core Assignment Domains
* **Process CPU & NUMA Affinity Assignment (`src/kernel/processor_management.rs`)**:
  - `SmpTopologyManager` & `NumaAffinityMap`: Pinning processes and threads to specific physical CPU cores (Performance P-Cores vs Efficiency E-Cores) and NUMA memory nodes (`taskset` parity).
  - Work-stealing scheduler queue assignment (`src/scheduler/`).
* **Security Privilege & Capability Assignment (`src/security/capability.rs`, `src/security/sigma_unveil.rs`)**:
  - Assigning capability tokens (`LinuxCapability`, `Permission`) to sandboxed processes and `.sigma-app` packages.
  - OpenBSD `pledge` and `unveil` file path permission assignments.
* **Special Interest Group (SIG) & CODEOWNERS Assignment (`Governance.md`, `CODEOWNERS`)**:
  - **SIG-Kernel**: Low-level kernel scheduling, VMM paging, and driver architecture assignments.
  - **SIG-Drivers**: PCIe, NVMe, USB, Audio, and Wi-Fi driver assignments.
  - **SIG-Apps & Shards**: Package management, universal adapters, and marketplace shard assignments.
  - **SIG-Security**: Cryptography, PQC attestation, pledge/unveil, and SELinux/AppArmor policy assignments.
* **Build Job & Cluster Worker Assignment (`src/distro/developer.rs`, `src/cluster/`)**:
  - Distributing package compilation jobs across cluster nodes and build sandbox containers (`PackageBuildService`).

---

## 2. Assignment Guidelines for AI Agents

When modifying or implementing assignment and task routing logic in SigmaOS:

### 1. NUMA & Core Affinity Safety
* **Valid Core Bitmasks**: Ensure CPU mask assignments do not reference offline or out-of-bounds core IDs.
* **Heterogeneous Core Aware**: Assign latency-critical audio/compositor threads to Performance (P) cores and background compression/indexing daemons to Efficiency (E) cores.

### 2. Least Privilege Capability Assignment
* **Minimal Scope**: When assigning capabilities (`Permission::FileRead`, `Permission::NetworkTcp`), grant only the minimal set required for task completion.
* **Non-Escalation**: Never elevate capability tokens or remove pledge/unveil restrictions without explicit authorization checks.

### 3. Subsystem Domain Ownership
* Ensure changes modifying specific subsystems adhere to the corresponding SIG governance rules in `Governance.md` and `CODEOWNERS`.

---

## 3. Verification & Testing Protocols

1. **REPL Assignment CLI Commands**: Test task affinity and capability assignments via interactive Shell REPL commands:
   - `taskset`: Inspect and set process CPU core affinity bitmasks.
   - `lscpu`: Verify SMP topology and NUMA node core distribution.
   - `pledge` / `jail`: Test sandboxed privilege assignments.
2. **Core Test Runner Execution**:
   ```bash
   ./run_sigma_tests.sh
   ```

---

## 4. Pre-Commit Checklist for Assignment Changes

Before submitting assignment or task routing modifications:
- [ ] Confirmed core affinity bitmasks are validated against active CPU topology.
- [ ] Confirmed capability token assignments follow the principle of least privilege.
- [ ] Verified SIG domain alignment according to `Governance.md` and `CODEOWNERS`.
- [ ] Executed `./run_sigma_tests.sh` with 100% test pass rate.
- [ ] Requested automated code review using `request_code_review`.
- [ ] Recorded assignment learnings using `initiate_memory_recording`.
