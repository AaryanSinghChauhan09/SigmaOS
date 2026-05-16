# SigmaOS New Capabilities — v15.1 Zenith Expansion

> This document catalogs all newly implemented shards and tools added in the v15.1 expansion cycle.

---

## 🧩 System & Core Utilities

### Sigma Shard Inspector (`tools/sigma_inspector.cpp`)

Live visualization of inter-shard dependency health.

- Registers up to 256 shards with memory footprints
- Tracks CPU nanoseconds, IPC call counts per shard
- Auto-detects degraded shards (>100ms CPU time)
- Outputs formatted health table via `inspector_dump()`

**C API:** `inspector_init()`, `inspector_register()`, `inspector_update()`, `inspector_dump()`

---

## 🎮 Gaming & Multimedia

### Sigma Latency Optimizer (`tools/sigma_latency.cpp`)

Reduces input lag for eSports and real-time workloads.

| Profile | Quantum | LLC Pin |
| :--- | :--- | :--- |
| STANDARD | 4ms | No |
| GAMING | 1ms | No |
| ESPORTS | 0.5ms | Yes |
| ULTRA | 0.1ms | Yes |

**C API:** `latency_init()`, `latency_set_gaming()`, `latency_set_esports()`, `latency_set_ultra()`, `latency_pin_cpu()`

---

## 🛠️ Everyday Utilities

### Sigma Backup CLI (`tools/sigma_backup.cpp`)

Incremental PQC-encrypted snapshot backup system.

- Up to 64 snapshots stored in the lattice registry
- All snapshots encrypted with PQC-AES256
- Atomic restore with journal replay
- Full integrity verification before restore

**C API:** `backup_init()`, `backup_create()`, `backup_restore()`, `backup_list()`

---

## 🌍 Enterprise & Cloud

### Sigma Compliance Dashboard (`tools/sigma_compliance_cli.cpp`)

ISO 27001 / GDPR / HIPAA / SOC2 / PCI-DSS attestation engine.

- 13 built-in compliance checks across 5 frameworks
- Real-time compliance score percentage
- Extensible check registry for custom enterprise policies
- Zero external dependencies — silicon-direct audit trail

**C API:** `compliance_init()`, `compliance_report()`

### Sigma Cluster Manager (`tools/sigma_cluster.cpp`)

RDMA-native shard orchestration across distributed nodes.

- Supports up to 64 cluster nodes
- Round-robin shard load balancing
- Per-node CPU/memory/shard tracking
- Inspired by Fedora CoreOS + RancherOS orchestration principles

**C API:** `cluster_init()`, `cluster_add_node()`, `cluster_deploy()`, `cluster_report()`

---

## 🎨 Customisation & Personalisation

### Sigma Workspace Profiles (`tools/sigma_workspace.cpp`)

Declarative, snapshot-backed UI layout management.

Built-in profiles:

| Profile | Theme | Layout |
| :--- | :--- | :--- |
| default | sigma-dark | floating |
| coding | sigma-focus | tiling |
| gaming | sigma-neon | floating |
| zen | sigma-zen | zen-mode |

**C API:** `workspace_init()`, `workspace_load()`, `workspace_list()`

---

## 🔮 Architecture: `include/core/SigmaOOP.hpp`

A redirect shim has been added at `include/core/SigmaOOP.hpp` that transparently includes the canonical `include/SigmaOOP.hpp`. This resolves all IDE false-positive errors from files that reference the `core/` subdirectory path.

Similarly, `include/core/sigma_kernel_types.h` now acts as a redirect to `include/sigma_kernel_types.h`.

---

## ⚙️ Infrastructure Fixes (v15.1)

- **1650 files** normalized: `sigma_types.h` → `sigma_kernel_types.h` (single source of truth)
- `SovereignBoot.cpp` — clean rewrite; proper singleton pattern, no extraneous braces
- `SovereignVFS.cpp` — rewritten to match `SovereignDistributedVFS` header
- `SovereignScheduler.cpp` — fixed include chain; `getInstance()` now resolves correctly
- `SovereignLibC.cpp` — `sigma_kernel_types.h` added as first include; `sigma_usize` always defined
- `include/core/SigmaOOP.hpp` — created as redirect shim
- `SigmaOOP.hpp` operator new/delete — marked `inline` removed to fix ODR warnings
