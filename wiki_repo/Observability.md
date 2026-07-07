# Observability Roadmap & Spec

## 1. Zero-Overhead Kernel Telemetry
Observability is integrated directly into the kernel scheduler (`sigma_rr_sched.rs`), memory allocation subsystems, and virtual filesystems (`sigma_vmm.rs`, `sigma_vfs_core.rs`).
- **Tracepoints**: Lockless, static tracepoints write event logs directly to per-CPU ring buffers in physical memory.
- **Exporters**: A native daemon (`sigma-prom-agent`) translates ring buffer data into standard Prometheus metrics.

## 2. Metrics & Dashboards
The monitoring daemon exposes an HTTP `/metrics` endpoint parsing system statistics:
- CPU core utilization and task queue length.
- Slab cache fragmentation levels.
- Page faults and memory allocations.
- Sandbox MicroVM performance quotas.

## 3. Anomaly Detection & Auto-Rollback
- **Threshold Monitors**: Detect post-upgrade issues like high context switch loops, out-of-memory errors, and device read failures.
- **Self-Healing Loop**: If metrics exceed critical thresholds during the post-install verification period, the system triggers the `sigpkg` rollback state machine, reverting the active partition to the previous stable snapshot.

## 4. Roadmap Phases
- **Phase 1 (0–3m)**: Define kernel tracepoint macros and initialize lockless memory buffers.
- **Phase 2 (3–6m)**: Launch the Prometheus metrics exporter daemon.
- **Phase 3 (6–9m)**: Design Grafana dashboard setups and alert metric rules.
- **Phase 4 (9–12m)**: Link the telemetry monitoring system to the active package manager rollback triggers.

## 5. Contributor Guidelines
- Add tracepoints to all new drivers and core system modules.
- Ensure tracing calls do not allocate memory on the heap.
