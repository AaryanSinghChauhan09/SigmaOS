# ⚡ Performance Tuning & Kernel Architecture

SigmaOS is engineered for ultra-low-latency desktop responsiveness and high-throughput server workloads. It incorporates performance innovations from **CachyOS (BORE Scheduler)**, **Linux MGLRU**, **eBPF XDP**, and a zero-dependency safe Rust kernel (`#![no_std]`).

---

## 🏎️ CachyOS BORE Scheduler (`CachyOsBoreScheduler`)

SigmaOS uses the **Burst-Oriented Response Enhancer (BORE)** task scheduler (`src/sched/bore.rs`).

### Key Characteristics:
- **Interactive Burst Detection**: Dynamically calculates task "burstiness" to prioritize desktop UI rendering and audio processing during background multi-core compilation jobs.
- **EEVDF Virtual Deadlines**: Combines Earliest Eligible Virtual Deadline First (EEVDF) math with BORE scoring to guarantee sub-millisecond frame dispatching for the Zenith compositor.

### Tuning Scheduler Parameters:
```bash
# Query active scheduler metrics
sysctl kernel.sched_bore_score

# Adjust burstiness sensitivity (0 = disabled, 1 = default, 2 = aggressive)
sysctl kernel.sched_bore_score=1
```

---

## 🔋 CPU Governors & Power Management (`CachyOsAutoFreqEngine`)

SigmaOS dynamically switches CPU scaling governors based on system load and power source:

| Governor | Description | Target Workload |
|----------|-------------|-----------------|
| `performance` | Max clock frequency, minimum latency | Gaming, Compiling, Video Rendering |
| `schedutil` / `balanced` | Dynamic frequency scaling based on load | General Desktop Usage |
| `powersave` | Lower clock frequency, maximum battery life | Mobile / On-Battery Usage |

### Configuration:
```bash
# Set CPU scaling governor to performance
sysctl power.cpu_governor="performance"

# Query Energy Performance Preference (EPP)
sysctl power.energy_perf_bias
```

---

## 🧠 Memory Reclamation & Swap (`MGLRU` + ZRAM)

SigmaOS optimizes memory management (`src/mm/`) through:

1. **Multi-Generational LRU (MGLRU)**: Groups physical memory pages into generational sets based on access frequency, reducing page reclaim CPU overhead by up to 40%.
2. **Copy-on-Write (CoW) Demand Paging**: Zero-copy page fault resolution with Kernel Samepage Merging (KSM) deduplication.
3. **ZRAM Compressed Swap**: Compresses swap pages in RAM using ZSTD before writing to disk, doubling effective system RAM capacity.

```bash
# Query MGLRU page generation metrics
sysctl vm.mglru_stats

# Enable ZRAM compressed swap
sigctl enable zram
```

---

## 🌐 eBPF & XDP Zero-Copy Networking

SigmaOS bypasses the traditional kernel network stack for high-rate packet processing using **eBPF and eXpress Data Path (XDP)** (`src/net/xdp.rs`):

- **Lock-Free Ring Buffers (`BPF_MAP_TYPE_RINGBUF`)**: Transfers packet events between kernel shards and userland without mutex lock contention.
- **Zero-Copy Driver Ring**: Direct NIC DMA transfer into userland memory buffers.
