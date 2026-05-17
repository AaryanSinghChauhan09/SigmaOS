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

### Sigma SystemCtl (`tools/sigma_systemctl.cpp`)

Service management and init orchestrator.

- Deterministic, parallelized dependency resolution for background daemons
- Replaces legacy systemd with atomic, C++ native logic

**C API:** `sysctl_init()`, `sysctl_start()`, `sysctl_stop()`, `sysctl_status()`

### Sigma Container Engine (`tools/sigma_container_engine.cpp`)

Daemonless, rootless container engine.

- Directly orchestrates memory bounds and kernel namespaces
- Sandboxes workloads using Sovereign boundaries rather than cgroups

**C API:** `container_init()`, `container_run()`, `container_stop()`

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

### Sigma Archive Suite (`tools/sigma_archive_suite.cpp`)

Compression and encryption utilities.

- PQC-encrypted archives
- zstd-equivalent compression speeds
- Zero-dependency extraction

**C API:** `archive_init()`, `archive_compress()`, `archive_extract()`

### Sigma Notification Center (`tools/sigma_notifications.cpp`)

Unified alerts across shards.

- Cross-shard event bus for UI-independent notifications
- Priority queues (NORMAL, HIGH, CRITICAL)
- Decoupled from the GUI stack

**C API:** `notify_init()`, `notify_push()`, `notify_clear()`

### Sigma Clipboard Hub (`tools/sigma_clipboard.cpp`)

Multi-clipboard manager.

- Cross-shard copy/paste ring buffer
- PQC-encrypted memory allocation
- Access history of previous clipboard states

**C API:** `clipboard_init()`, `clipboard_copy()`, `clipboard_list()`

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

### Sigma Policy Engine (`tools/sigma_policy_engine.cpp`)

Enforce enterprise rules across the lattice.

- Zero-trust RBAC and immutable policy evaluation
- Checks permissions dynamically across kernel and shards

**C API:** `policy_init()`, `policy_load()`, `policy_check()`

### Sigma Enterprise Recovery (`tools/sigma_enterprise_recovery.cpp`)

Fleet rollback utilities.

- Atomic rollbacks for entire networked clusters
- RDMA broadcast commands for snapshot syncing
- Immutable rollback hashes

**C API:** `recovery_init()`, `recovery_register()`, `recovery_rollback()`

### Sigma Sovereign Cloud OS (`tools/sigma_sovereign_cloud.cpp`)

Hybrid bare-metal + cloud orchestration.

- Treat remote instances exactly like local shards
- Deploy bare-metal hypervisors remotely
- Live migration of active shards across the network

**C API:** `cloudos_init()`, `cloudos_deploy()`, `cloudos_migrate()`

### Sigma Mirror Manager (`tools/sigma_mirror_manager.cpp`)

Sovereign distribution mirrors.

- Decentralised, cryptographic package seeding
- Finds the fastest PQC-attested mirror on the network

**C API:** `mirror_init()`, `mirror_add()`, `mirror_sync()`

---

## 🌐 IoT & Embedded

### Sigma Sensor Fusion (`tools/sigma_sensor_fusion.cpp`)

Combine multiple IoT sensor streams.

- Deterministic multiplexing of hardware inputs
- Scaled Kalman/AHRS simulation framework
- Supports up to 64 concurrent sensor inputs

**C API:** `fusion_init()`, `fusion_register()`, `fusion_update()`, `fusion_process()`, `fusion_dump()`

### Sigma Edge ML (`tools/sigma_edge_ml.cpp`)

Deploy lightweight ML models on IoT devices.

- Hardware-accelerated ML inference
- No external Python dependencies
- Fast deterministic execution paths

**C API:** `edgeml_init()`, `edgeml_load()`, `edgeml_infer()`

### Sigma Robotics Planner (`tools/sigma_robotics_planner.cpp`)

Pathfinding and control utilities.

- Deterministic real-time kinematics
- A* pathfinding algorithms for motor vectors
- Obstacle avoidance engine

**C API:** `robotics_init()`, `robotics_set_target()`, `robotics_step()`

### Sigma Automotive Diagnostics (`tools/sigma_auto_diag.cpp`)

CAN bus + OBD integration.

- Deterministic real-time vehicle telemetry parsing
- Requests standard OBD-II PIDs directly from hardware

**C API:** `autodiag_init()`, `autodiag_connect()`, `autodiag_read_obd()`

### Sigma Smart Grid Manager (`tools/sigma_smart_grid.cpp`)

IoT utilities for energy networks.

- Real-time decentralised energy load balancing
- Peak shaving logic
- Distributed node power tracking

**C API:** `smartgrid_init()`, `smartgrid_register()`, `smartgrid_balance()`

### Sigma Edge Vision (`tools/sigma_edge_vision.cpp`)

Computer vision toolkit for IoT.

- Hardware-accelerated pixel inference at the edge
- Direct hardware camera integration

**C API:** `edgevis_init()`, `edgevis_attach()`, `edgevis_process()`

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

### Sigma Build Farm (`tools/sigma_build_farm.cpp`)

Distributed shard compilation.

- Parallel compilation across RDMA-connected nodes
- Automatic horizontal scaling of compile tasks
- Distributed dependency graph resolution

**C API:** `buildfarm_init()`, `buildfarm_connect()`, `buildfarm_dispatch()`

### Sigma API Gateway (`tools/sigma_api_gateway.cpp`)

Sovereign API orchestration.

- PQC-attested routing for all microkernel IPC services
- Distributed endpoint management

**C API:** `apigate_init()`, `apigate_add()`, `apigate_handle()`

### Sigma Blockchain Hub (`tools/sigma_blockchain.cpp`)

Distributed ledger integration.

- PQC-hardened smart contract validation at the OS level
- OS-native ledger synchronization

**C API:** `blockchain_init()`, `blockchain_sync()`, `blockchain_validate()`

### Sigma Debug CLI (`tools/sigma_debug_cli.cpp`)

Advanced debugging commands.

- Sovereign introspection into running shards
- Dump core registers and analyze IPC payloads

**C API:** `debugcli_init()`, `debugcli_attach()`, `debugcli_dump()`

### Sigma Dev Dashboard (`tools/sigma_dev_dashboard.cpp`)

GUI for developer telemetry.

- Unified real-time view of IPC bounds, heap limits, and traces
- Visually toggled developer overlay

**C API:** `devdash_init()`, `devdash_toggle()`, `devdash_feed()`

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

### Sigma Education Profile (`tools/sigma_education.cpp`)

Classroom management shards.

- Deterministic workspace lock-in for testing and focus
- Disables clipboard and restricts network access in exam mode

**C API:** `edu_init()`, `edu_exam_mode()`, `edu_broadcast()`

### Sigma Accessibility Hub (`tools/sigma_accessibility.cpp`)

One-click accessibility presets.

- Deep hardware/UI integration for screen readers
- Triggers compositor high contrast and color inversion
- Magnifier zoom logic built into the display server

**C API:** `access_init()`, `access_reader()`, `access_contrast()`, `access_magnify()`

### Sigma Adaptive Input (`tools/sigma_adaptive_input.cpp`)

Voice, gesture, and haptic controls.

- Multi-modal sovereign input parsing
- Hardware-direct haptic intensity orchestration

**C API:** `adaptin_init()`, `adaptin_voice()`, `adaptin_gesture()`, `adaptin_haptic()`

### Sigma Personalisation CLI (`tools/sigma_personalisation.cpp`)

Manage user preferences via terminal.

- Deterministic setting application with live compositor updates
- Unified configuration dump commands

**C API:** `person_init()`, `person_set()`, `person_dump()`

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
