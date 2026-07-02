# SigmaOS Performance Architecture

SigmaOS is designed to be measurably faster than Linux distributions through silicon-aware tuning, lock-free algorithms, and profile-guided optimisation.

---

## Performance Targets (v16.0 Apex)

| Metric | Target | Current |
|--------|--------|---------|
| Boot time (NVMe → desktop) | < 2 s | N/A (no boot yet) |
| Context switch latency | < 50 ns | N/A |
| Kyber-1024 ops/sec (AVX-512) | ≥ 5.8 M | N/A |
| Idle RAM (full desktop) | < 150 MB | N/A |
| TCP throughput (loopback) | ≥ 10 Gbps | N/A |
| Filesystem write (NVMe) | ≥ 2 GB/s | N/A |
| IRQ latency (RTOS profile) | < 10 µs | N/A |

---

## Scheduler Performance (`kernel/sched/`)

### Lock-Free Runqueue
- CAS-based atomic operations — no scheduler lock contention
- Cache-line aligned task control blocks
- Per-CPU runqueues — minimise cross-CPU migration

### NUMA Awareness
- Reads ACPI SRAT table at boot
- Prefers local-memory task placement
- Migration threshold based on load imbalance

### CFS Clone (vruntime)
- Red-black tree O(log n) insertion/removal
- Virtual runtime accounts for priority differences
- Bandwidth throttling for cgroup CPU quotas

---

## Memory Performance (`kernel/memory/`)

### Buddy Allocator
- Power-of-2 page frames, O(log n) alloc/free
- Free list per order, per CPU (reduces locking)
- Coalescing on free — reduces fragmentation

### Slab Allocator
- Per-type object caches, aligned to cache lines
- SLUB-style batching — amortises alloc overhead
- Coloring to reduce cache aliasing

### Page Table Walk Optimisation
- TLB shootdowns only on modified mappings
- Huge pages (2 MB) for kernel text/data
- PCID support — avoids full TLB flush on context switch

---

## Network Performance (`net/`)

### Zero-Copy Paths
- Receive: DMA directly into user buffer (VirtIO-net)
- Send: scatter-gather DMA, no intermediate copy

### io_uring Equivalent (`kernel/io/sigma_uring.cpp`) — Phase G
- Async I/O submission/completion ring — zero syscall for hot paths
- Registered buffers + fixed files — eliminates fd lookup overhead

### TCP Optimisations
- TSO (TCP Segmentation Offload) — NIC does segmentation
- GRO (Generic Receive Offload) — coalesce received segments
- Nagle algorithm with configurable cork

---

## SIMD Acceleration (`performance-optimized` branch)

### AVX-512 (x86_64)
- **Kyber-1024 NTT**: 256-bit polynomial operations vectorised
- **Memory operations**: `sigma_memcpy`, `sigma_memset` using 512-bit registers
- **Zenith compositor**: matrix transforms, pixel blending (Phase G)

### ARM NEON
- **Kyber NEON NTT**: 128-bit SIMD for ARM Cortex-A72+
- **AES-NI equivalent**: ARMv8 crypto extensions for AES-GCM

### Auto-detection at Runtime
```cpp
if (sigma_cpu_has_avx512()) {
    kyber_ntt = kyber_ntt_avx512;
} else if (sigma_cpu_has_avx2()) {
    kyber_ntt = kyber_ntt_avx2;
} else {
    kyber_ntt = kyber_ntt_generic;
}
```

---

## Profile-Guided Optimisation (PGO)

```bash
# Step 1: Instrument build
make PROFILE=pgo-instrument all

# Step 2: Run representative workload
qemu-system-x86_64 -cdrom build/sigmaos.iso -m 2G ... (run workloads)

# Step 3: Rebuild with profile data
make PROFILE=pgo-optimise all
```

PGO improves branch prediction, inlining decisions, and code layout for ~10-15% throughput improvement on measured workloads.

---

## Clear Linux–Inspired Tuning (`cmake/sigma_hardening.cmake`)

Compiler flags applied to production builds:
```cmake
-O3 -march=native -mtune=native
-fprofile-use
-fno-plt                    # avoid PLT trampolines
-fstack-protector-strong
-D_FORTIFY_SOURCE=2
-flto=thin                  # LTO for cross-module inlining
```

---

## Performance Monitoring (`sigmad-metrics`)

Real-time performance data exposed at `/run/sigma/metrics.sock`:

```bash
# View live metrics (Prometheus format)
curl --unix-socket /run/sigma/metrics.sock /metrics

# Key metrics exported:
# sigma_context_switches_total
# sigma_scheduler_latency_ns{quantile="0.99"}
# sigma_page_faults_total{type="minor|major"}
# sigma_tcp_bytes_total{direction="rx|tx"}
# sigma_pqc_ops_total{algorithm="kyber|dilithium"}
```

---

*See also: [Kernel](Kernel) · [Branch-Development-Roadmap](Branch-Development-Roadmap#performance-optimized) · [Networking](Networking)*
