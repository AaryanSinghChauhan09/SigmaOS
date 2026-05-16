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

### Sigma Log Visualizer (`tools/sigma_log_visualizer.cpp`)

Interactive TUI charts for system logs.

- Aggregates kernel ring buffer and shard telemetry
- Supports CPU usage lines, memory footprints, and error density heatmaps
- Zero-dependency graph generation

**C API:** `logvis_init()`, `logvis_render()`

### Sigma Kernel Tuner (`tools/sigma_tuner.cpp`)

Live kernel parameter adjustment without reboots.

- Modifies kernel tunables in real-time
- Validates inputs against safe boundaries
- Read-only support for protected variables

**C API:** `tuner_init()`, `tuner_set()`, `tuner_list()`

### Sigma Energy Saver (`tools/sigma_energy.cpp`)

Intelligent power scaling for laptops/servers.

- 4 modes: PERFORMANCE, BALANCED, POWERSAVE, ULTRA_ECO
- Automatically switches profile based on battery % and AC status
- Modifies CPU frequency bounds and device sleep toggles

**C API:** `energy_init()`, `energy_set_state()`, `energy_force_powersave()`, `energy_report()`

### Sigma Secure Boot Manager (`tools/sigma_secure_boot.cpp`)

Sovereign boot validation with PQC.

- Attests boot images using PQC-Kyber/Dilithium signatures
- Strict enforcement and auditing modes
- Pre-loads platform keys into memory

**C API:** `secboot_init()`, `secboot_verify()`, `secboot_set_mode()`, `secboot_report()`

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

### Sigma Game Library (`userland/apps/SigmaGameLibrary.cpp`)

Curated open-source gaming hub with direct hardware access.

- Launch titles directly with zero-overhead graphics
- Automatically toggles eSports Latency Optimizer for competitive games
- Tracks local vs cloud installations

**C API:** `gamelib_init()`, `gamelib_launch()`, `gamelib_list()`

### Sigma VR Studio (`tools/sigma_vr_studio.cpp`)

VR workspace for productivity.

- Sovereign spatial compositor for HMDs
- Stereo 4K at 120Hz deterministic tracking
- Spawn and recenter floating application windows

**C API:** `vrstudio_init()`, `vrstudio_connect()`, `vrstudio_spawn()`, `vrstudio_recenter()`

### Sigma GPU Profiler (`tools/sigma_gpu_profiler.cpp`)

Real-time graphics performance analysis.

- Silicon-direct VRAM and shader pipeline telemetry
- Thermal throttling threshold warnings
- Lightweight lock-free monitoring

**C API:** `gpuprof_init()`, `gpuprof_update()`, `gpuprof_dump()`

### Sigma Audio Mixer (`tools/sigma_audio_mixer.cpp`)

Sovereign sound routing and mixing.

- Lock-free zero-copy audio pipeline
- Deterministic mixing guarantees (no PulseAudio/PipeWire stutter)
- Hardware-direct volume orchestration

**C API:** `audio_init()`, `audio_register()`, `audio_set_vol()`, `audio_master_vol()`, `audio_list()`

---

## 🛠️ Everyday Utilities

### Sigma Backup CLI (`tools/sigma_backup.cpp`)

Incremental PQC-encrypted snapshot backup system.

- Up to 64 snapshots stored in the lattice registry
- All snapshots encrypted with PQC-AES256
- Atomic restore with journal replay
- Full integrity verification before restore

**C API:** `backup_init()`, `backup_create()`, `backup_restore()`, `backup_list()`

### Sigma Cloud Sync (`tools/sigma_cloud_sync.cpp`)

Sovereign sync with GitHub/Dropbox/OneDrive/SigmaS3.

- End-to-End PQC Encryption before transport
- Batch syncing across multiple heterogeneous providers
- Zero-knowledge data orchestration

**C API:** `cloudsync_init()`, `cloudsync_add()`, `cloudsync_execute()`

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

## 🌐 IoT & Embedded

### Sigma Sensor Fusion (`tools/sigma_sensor_fusion.cpp`)

Combine multiple IoT sensor streams.

- Deterministic multiplexing of hardware inputs
- Scaled Kalman/AHRS simulation framework
- Supports up to 64 concurrent sensor inputs

**C API:** `fusion_init()`, `fusion_register()`, `fusion_update()`, `fusion_process()`, `fusion_dump()`

---

## 🔬 Experimental & Developer Tools

### Sigma Test Lab (`tools/sigma_test_lab.cpp`)

Automated regression and fuzzing suite.

- High-throughput IPC fuzzer
- Sovereign memory leak detector
- Continuous integration of kernel/shard boundaries

**C API:** `testlab_init()`, `testlab_run_suite()`, `testlab_report()`

### Sigma Quantum Simulator (`tools/sigma_quantum_simulator.cpp`)

PQC and quantum workload simulation.

- Hardware-accelerated tensor network simulation for qubits
- Validates post-quantum cryptography algorithms
- Tracks active circuits and simulated state vectors

**C API:** `quantum_init()`, `quantum_load()`, `quantum_execute()`

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
