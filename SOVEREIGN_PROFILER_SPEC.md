# SOVEREIGN PROFILER SPEC

> **Component**: `kernel/telemetry/sovereign_profiler/` | **Status**: Planned → Implementation

The **SigmaOS Sovereign Profiler** is a high-frequency, zero-overhead observability suite that monitors the health, throughput, and latency of all kernel shards and subsystems. It provides industrial-grade telemetry for performance auditing, shard optimization, and anomaly detection.

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                   SOVEREIGN PROFILER                        │
│                                                             │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐ │
│  │  Ring-Buffer │    │  Aggregator  │    │  Export      │ │
│  │  Collectors  │───▶│  (per-CPU)   │───▶│  Engine      │ │
│  │  (per-shard) │    │              │    │              │ │
│  └──────────────┘    └──────────────┘    └──────┬───────┘ │
│                                                   │         │
│  ┌────────────────────────────────────────────────▼──────┐ │
│  │                    METRICS STORE                       │ │
│  │          /sigma/metrics  (shared memory)               │ │
│  └────────────────────────────────────────────────────────┘ │
│            │                  │                  │           │
│    ┌───────▼─────┐  ┌────────▼────┐  ┌──────────▼───────┐ │
│    │  sigma top  │  │  Zenith HUD │  │  OpenTelemetry   │ │
│    │  (CLI)      │  │  (Desktop)  │  │  Export          │ │
│    └─────────────┘  └─────────────┘  └──────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

---

## Collected Metrics

### 1. Shard Health Vitals

| Metric | Type | Resolution | Description |
|---|---|---|---|
| `shard.cpu_ns` | Counter | per-tick | CPU nanoseconds consumed |
| `shard.ipc_msgs` | Counter | per-tick | sigma-bus messages sent/received |
| `shard.ipc_latency_ns` | Histogram | μs buckets | IPC round-trip time |
| `shard.memory_kb` | Gauge | 100ms | RSS memory usage |
| `shard.page_faults` | Counter | per-tick | Major + minor page faults |
| `shard.state` | Enum | 10ms | INIT / RUNNING / BLOCKED / PANIC |

### 2. Kernel Subsystem Metrics

| Metric | Description |
|---|---|
| `sigma.scheduler.eevdf_slice_ns` | Current EEVDF time-slice per priority class |
| `sigma.mm.alloc_latency_ns` | Memory allocator P50/P95/P99 latency |
| `sigma.ipc.ring_pressure` | sigma-bus ring-buffer fill percentage |
| `sigma.fs.vfs_ops_per_sec` | VFS operation throughput |
| `sigma.net.pkt_rate` | Network packets per second |
| `sigma.crypto.dilithium_sign_ns` | Post-quantum signature latency |

### 3. System-Wide Vitals

```
/sigma/metrics/system
├── cpu_util_percent[]      # per-core utilization
├── load_avg_1m             # 1-minute load average
├── memory_total_kb         # total RAM
├── memory_used_kb          # used RAM
├── swap_used_kb            # swap usage
├── disk_iops[]             # per-device IOPS
└── thermal_celsius[]       # per-zone temperatures
```

---

## Implementation

### Rust Source

```rust
// kernel/telemetry/sovereign_profiler/mod.rs

#![no_std]

use core::sync::atomic::{AtomicU64, Ordering};
use sigma_core::shard::ShardId;

/// Lock-free per-shard metrics ring
#[repr(C, align(64))]  // Cache-line aligned
pub struct ShardMetrics {
    pub shard_id:       u32,
    pub cpu_ns:         AtomicU64,
    pub ipc_msgs_sent:  AtomicU64,
    pub ipc_msgs_recv:  AtomicU64,
    pub memory_kb:      AtomicU64,
    pub page_faults:    AtomicU64,
    pub state:          AtomicU64,  // ShardState as u64
    _pad:               [u8; 16],   // pad to 64 bytes
}

impl ShardMetrics {
    pub fn record_cpu(&self, ns: u64) {
        self.cpu_ns.fetch_add(ns, Ordering::Relaxed);
    }

    pub fn record_ipc_send(&self) {
        self.ipc_msgs_sent.fetch_add(1, Ordering::Relaxed);
    }

    pub fn snapshot(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            cpu_ns:      self.cpu_ns.load(Ordering::Relaxed),
            ipc_sent:    self.ipc_msgs_sent.load(Ordering::Relaxed),
            ipc_recv:    self.ipc_msgs_recv.load(Ordering::Relaxed),
            memory_kb:   self.memory_kb.load(Ordering::Relaxed),
        }
    }
}

/// Shared memory metrics store at /sigma/metrics
pub static METRICS_STORE: MetricsStore = MetricsStore::new();

pub struct MetricsStore {
    shards: [ShardMetrics; 256],  // support up to 256 shards
}

impl MetricsStore {
    pub const fn new() -> Self { /* ... */ }

    pub fn get(&self, id: ShardId) -> &ShardMetrics {
        &self.shards[id.index()]
    }

    /// Broadcast snapshot to Zenith HUD via sigma-bus
    pub fn broadcast_vitals(&self) {
        let vitals: Vec<MetricsSnapshot> = self.shards
            .iter()
            .filter(|s| s.state.load(Ordering::Relaxed) != 0)
            .map(|s| s.snapshot())
            .collect();

        sigma_bus::publish("sigma.metrics.vitals", &vitals);
    }
}
```

### eBPF Probes

For kernel-level instrumentation without overhead:

```c
// kernel/bpf/profiler_probes.bpf.c

#include <linux/bpf.h>
#include <bpf/bpf_helpers.h>

struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(max_entries, 256);
    __type(key, u32);
    __type(value, u64);
} shard_cpu_ns SEC(".maps");

// Probe: measure shard entry/exit CPU time
SEC("kprobe/sigma_shard_enter")
int probe_shard_enter(struct pt_regs *ctx) {
    u32 shard_id = (u32)PT_REGS_PARM1(ctx);
    u64 *ts = bpf_map_lookup_elem(&shard_cpu_ns, &shard_id);
    if (ts) *ts = bpf_ktime_get_ns();
    return 0;
}

SEC("kretprobe/sigma_shard_enter")
int probe_shard_exit(struct pt_regs *ctx) {
    u32 shard_id = (u32)PT_REGS_PARM1(ctx);
    u64 elapsed = bpf_ktime_get_ns() - /* start time */;
    // Accumulate to per-CPU counter
    bpf_map_update_elem(&shard_cpu_ns, &shard_id, &elapsed, BPF_ANY);
    return 0;
}
```

---

## CLI Interface

```bash
# Real-time shard vitals (like `top` but for shards)
sigma top

# Output:
# SHARD              CPU%   MEM_KB   IPC_MSG/s  STATE
# CoreLattice         3.2%    2048     45,230    RUNNING
# SigmaScheduler      1.1%     512     12,100    RUNNING
# NetworkStack        8.7%    8192    120,500    RUNNING
# HelloWorld          0.0%       4         12    IDLE
# SigmaShield         0.2%    1024      5,400    RUNNING

# Historical stats (last 60s)
sigma stats --shard CoreLattice --window 60s

# Export to OpenTelemetry (Prometheus scrape endpoint)
sigma metrics export --format prometheus --port 9090

# Show IPC latency histogram
sigma metrics histogram --metric ipc_latency_ns --shard NetworkStack
# P50:  245ns
# P95:  890ns
# P99: 2100ns
# MAX: 5400ns
```

---

## Zenith HUD Integration

Vitals are broadcast to the **Zenith Header** (desktop HUD) via sigma-bus at 10Hz:

```
┌──────────────────────────────────────────────────────────────┐
│  CPU: ██░░░ 22%  MEM: 4.2/16GB  NET: ↑12MB/s ↓45MB/s       │
│  TEMP: 48°C  SHARDS: 47 active  IPC: 180K msg/s  UPTIME: 5d │
└──────────────────────────────────────────────────────────────┘
```

---

## OpenTelemetry Export

```yaml
# /etc/sigma/otel-exporter.yaml
exporters:
  prometheus:
    endpoint: "0.0.0.0:9090"
  otlp:
    endpoint: "https://otel-collector.internal:4317"
    tls:
      insecure: false

metrics:
  prefix: "sigmaos_"
  include:
    - "shard.*"
    - "sigma.scheduler.*"
    - "sigma.mm.*"
    - "sigma.net.*"
```

---

## Performance Overhead

The profiler is designed for zero overhead in production:

| Mode | CPU Overhead | Memory | Latency Impact |
|---|---|---|---|
| Off | 0% | 0 | 0ns |
| Minimal (counters only) | <0.1% | 256KB | <5ns per event |
| Full (histograms) | <0.5% | 4MB | <20ns per event |
| Debug (all probes) | <2% | 16MB | <100ns per event |

**Default**: Minimal mode in production, Full mode in dev/beta builds.

---

## Integration with Kernel Autotuner

The profiler feeds directly into the `sigma_kernel_autotuner`:

```rust
// kernel/sigma_kernel_autotuner.rs
pub fn tune_cycle(metrics: &MetricsStore) {
    let cpu_pressure = metrics.system_cpu_util();
    let ipc_pressure = metrics.ipc_ring_pressure();

    // Adjust EEVDF scheduler slice based on CPU pressure
    if cpu_pressure > 85.0 {
        scheduler::reduce_slice(SLICE_REDUCTION_STEP);
    } else if cpu_pressure < 40.0 && ipc_pressure < 20.0 {
        scheduler::increase_slice(SLICE_INCREASE_STEP);
    }
}
```

---

## Roadmap

- [x] Lock-free per-shard atomic counters
- [x] `/sigma/metrics` shared memory store
- [x] `sigma top` CLI tool
- [ ] eBPF probe integration (Q3)
- [ ] Histogram support for IPC latency (Q3)
- [ ] OpenTelemetry export (Q4)
- [ ] Zenith HUD real-time sparklines (Q4)
- [ ] Anomaly detection with neural core (Year 2)
