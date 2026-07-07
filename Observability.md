# SigmaOS System Observability & Self-Healing

## Overview
SigmaOS incorporates a local observability stack designed for low-overhead telemetry gathering and autonomous self-healing. System metrics are exposed via native endpoints compatible with Prometheus and Grafana, while the anomaly detection daemon monitors kernel telemetry to trigger automated snapshot rollbacks on critical failures.

## Observability & Self-Healing Flow
```
 [System/Kernel Metrics] ──► [sigmad-monitor (Prometheus exporter)]
                                       │
                                       ▼
 [Autonomous Self-Healer] ◄────────────┤
         │                             ▼
         ▼                      [Grafana Dashboard]
 [Anomaly Detected?]
         │
         └──► Yes ──► Terminate Process / Rollback Snapshot
```

## System Properties
Telemetry configurations are defined in `/etc/sigma/observability.conf`:
```toml
[observability]
enabled = true
export_interval = "15s"
prometheus_port = 9100

[self_healing]
enabled = true
memory_critical_threshold_percent = 95
auto_rollback_on_panic = true
```

## Technical Implementation
The monitoring daemon records kernel performance parameters directly from memory ring-buffers.

```rust
// kernel/mm/sigma_vmm.rs
pub fn get_memory_utilization() -> MemoryStats {
    let total_pages = get_total_physical_pages();
    let free_pages = get_free_physical_pages();
    MemoryStats {
        total: total_pages * PAGE_SIZE,
        free: free_pages * PAGE_SIZE,
        used: (total_pages - free_pages) * PAGE_SIZE,
    }
}
```

## Roadmap & Milestones
- **Phase 1 (Months 0-3)**: Prometheus metrics exporter daemon for cpu/memory utilization.
- **Phase 2 (Months 3-6)**: Integrated Grafana dashboard service pre-packaged.
- **Phase 3 (Months 6-9)**: Self-healing watchdog daemon that restarts critical services.
- **Phase 4 (Months 9-12)**: Automated ZFS/Btrfs snapshot rollback on core file corruption detection.
