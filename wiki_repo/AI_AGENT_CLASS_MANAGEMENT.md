# 🏷️ AI Agent Class Management Protocol for SigmaOS

This document specifies the operational protocols, resource classification algorithms, and security taxonomies for **AI Agents in Class Management** (`Agent-Class`) within the SigmaOS ecosystem.

---

## 🏛️ 1. Resource & Service Class Allocation (`cgroups v2` & `rctl`)

SigmaOS utilizes `Agent-Class` to dynamically classify processes into resource management tiers inspired by Linux cgroups v2 and FreeBSD `rctl`:

```
┌─────────────────────────────────────────────────────────────┐
│             Agent-Class Resource Governor Engine            │
└─────────────────────────────────────────────────────────────┘
         │                          │                         │
         ▼                          ▼                         ▼
┌──────────────────┐      ┌──────────────────┐      ┌──────────────────┐
│  Real-Time Class │      │ Interactive Class│      │ Background Class │
│ • High CPU Shares│      │ • Low Latency    │      │ • Throttled IO   │
│ • Locked Memory  │      │ • GPU Access     │      │ • Idle Shares    │
└──────────────────┘      └──────────────────┘      └──────────────────┘
```

### 📊 Class Allocation Matrix
- **Real-Time System Class (`Class-0`)**:
  - Reserved for microkernel core threads, interrupt handlers, and PQC security drivers. Guaranteed 100% memory pinning and zero scheduling delay.
- **Interactive Userland Class (`Class-1`)**:
  - Assigned to Zenith Desktop compositor, user applications, and display managers. Dynamically prioritized by the BORE interactive scheduler.
- **Batch & Processing Class (`Class-2`)**:
  - Assigned to background compilation, file indexing, and ML training pipelines. Throttled during user interaction to preserve desktop responsiveness.
- **Constrained / Sandbox Class (`Class-3`)**:
  - Assigned to untrusted web downloads, containerized apps, and guest microVMs. Restricted by strict memory limits and eBPF socket filters.

---

## ⚡ 2. Process Scheduling & Priority Class Management

`Agent-Class` continuously classifies process execution behavior and updates scheduler parameters:

1. **EEVDF Deadline Classification**:
   - Computes process virtual runtime deadlines ($V_{runtime}$) and adjusts time slices based on real-time execution demands.
2. **BORE Interactive Sensitivity Scoring**:
   - Analyzes I/O wait times and keyboard/mouse input events to boost interactive application responsiveness.
3. **NUMA Distance Classification**:
   - Maps process classes to specific NUMA memory nodes to prevent cross-socket interconnect bottlenecks.

---

## 🛡️ 3. Security Context & Data Classification (MLS)

`Agent-Class` manages Mandatory Access Control (MAC) data sensitivity levels based on the Bell-LaPadula model:

- **Top Secret (`s3`)**:
  - Cryptographic keys, PQC vault master passwords, and biometric auth hashes.
- **Secret (`s2`)**:
  - System configuration logs, network firewall rules, and internal database records.
- **Confidential (`s1`)**:
  - User documents, email databases, and local file storage.
- **Unclassified (`s0`)**:
  - Public web assets, documentation markdown files, and temporary cache data.

---

## 🎓 4. Educational Classroom & Curriculum Management

In addition to system-level classification, `Agent-Class` manages classroom workflows in educational environments (`NcertScienceTeacherSuite`):

- **Curriculum Classification**:
  - Maps NCERT / CBSE science topics (Classes 6–12 across Physics, Chemistry, Biology, EVS) to student cognitive levels.
- **Automated Virtual Lab Experiments**:
  - Schedules interactive virtual lab simulations (e.g., Ohm's Law, Titration, Photosynthesis) for classroom student rosters.
- **Bloom's Taxonomy Assessment**:
  - Classifies exam questions automatically into Cognitive Levels (*Remembering*, *Understanding*, *Applying*, *Analyzing*, *Evaluating*, *Creating*).

---

## 📊 5. Class Management Observability Scorecard

`Agent-Class` monitors and reports classification metrics over the system message bus:

| Metric | Target | Enforced By |
|---|---|---|
| **Resource Class Enforcement** | 100% Policy Adherence | `cgroups v2` / `rctl` |
| **Interactive Latency Boost** | < 2 ms Input Delay | BORE Scheduler |
| **Data Classification Integrity** | 0 MLS Violations | Sovereign SELinux Engine |
| **Educational Lab Allocation** | 100% Student Roster Match | `NcertScienceTeacherSuite` |

---

This protocol ensures that SigmaOS maintains precise system resource allocation, strict security data classification, and seamless educational classroom management.
