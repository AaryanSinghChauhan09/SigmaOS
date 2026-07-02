# SigmaOS Performance Targets

> Concrete, measurable performance goals with measurement methodology.
> Every target is verified by `sigma-bench` in CI.

---

## Boot Performance

| Milestone | v0.1 Target | v1.0 Target | v2.0 Target | Measurement |
|---|---|---|---|---|
| UEFI → kernel_main | < 500ms | < 300ms | < 200ms | HPET timestamp |
| Kernel init | < 300ms | < 200ms | < 100ms | jiffies delta |
| sigma-init + daemons | < 1000ms | < 700ms | < 400ms | PID 1 log |
| Compositor first frame | < 600ms | < 400ms | < 200ms | vblank counter |
| **Boot to desktop** | **< 2.5s** | **< 1.5s** | **< 1.0s** | Stopwatch |
| Suspend → resume | N/A | < 500ms | < 200ms | ACPI event |

---

## Scheduler & Process

| Metric | Target | Measurement |
|---|---|---|
| Context switch latency | < 50ns | sigma-bench context-switch |
| Task creation (fork) | < 1µs | sigma-bench fork |
| Syscall dispatch overhead | < 200ns | sigma-bench syscall-noop |
| IPC round-trip (sigma-bus) | < 500ns | sigma-bench ipc-ping |
| IRQ latency (RTOS profile) | < 10µs | GPIO toggle + oscilloscope |
| Scheduler tick overhead | < 5% CPU | perf stat |

---

## Memory

| Metric | Target | Measurement |
|---|---|---|
| Idle RAM (sigma-sh only) | < 32 MB | /proc/meminfo |
| Idle RAM (full desktop) | < 256 MB | /proc/meminfo |
| Kernel binary (stripped) | < 2 MB | `size kernel.elf` |
| Minimal ISO size | < 100 MB | `du -sh *.iso` |
| kmalloc latency (slab hit) | < 50ns | sigma-bench kmalloc |
| Page fault (warm TLB) | < 100ns | sigma-bench pagefault |
| OOM kill to recovery | < 200ms | Log timestamp |

---

## Filesystem

| Metric | Target | Measurement |
|---|---|---|
| SigmaFS sequential read | > 1 GB/s (NVMe) | sigma-bench disk-seq |
| SigmaFS sequential write | > 800 MB/s (NVMe) | sigma-bench disk-seq |
| SigmaFS random 4K read | > 100K IOPS | sigma-bench disk-rand |
| Ext4 mount (100K files) | < 200ms | `time mount` |
| sigma-pkg install (10MB) | < 1s | time sigma-pkg install |
| /proc/stat read | < 1µs | sigma-bench procfs |

---

## Networking

| Metric | Target | Measurement |
|---|---|---|
| TLS 1.3 handshake | < 5ms (LAN) | sigma-bench tls-handshake |
| DNS resolution (DoH cached) | < 2ms | sigma-bench dns-resolve |
| UDP ping RTT (LAN) | < 0.5ms | sigma-bench udp-ping |
| sigma-pkg download (1MB) | < 100ms (GbE) | sigma-bench pkg-dl |
| Firewall per-packet (cached) | < 100ns | sigma-bench firewall |
| TCP throughput (localhost) | > 10 Gbps | sigma-bench tcp-throughput |
| WireGuard overhead | < 5% vs plaintext | iperf3 comparison |

---

## Cryptography

| Metric | Target | Measurement |
|---|---|---|
| SHA-256 throughput | > 500 MB/s | sigma-bench sha256 |
| Kyber-1024 keygen | < 0.5ms | sigma-bench kyber-keygen |
| Kyber-1024 encaps | < 0.5ms | sigma-bench kyber-enc |
| Dilithium-5 sign | < 2ms | sigma-bench dilithium-sign |
| Dilithium-5 verify | < 1ms | sigma-bench dilithium-verify |
| AES-256-GCM (AVX-512) | > 5 GB/s | sigma-bench aes-gcm |
| sigma-pkg verify (1MB) | < 10ms | sigma-bench pkg-verify |

---

## Desktop / UI

| Metric | Target | Measurement |
|---|---|---|
| Frame time (1080p) | < 16ms (60fps) | sigma-bench compositor |
| Frame time (4K) | < 8ms (120fps) | sigma-bench compositor-4k |
| Input → screen latency | < 5ms | High-speed camera test |
| Window open animation | 250ms at 60fps | Profiler |
| App launch (sigma-edit) | < 300ms | time sigma-edit |
| Notification display | < 100ms | Log timestamp |
| Settings open | < 150ms | Profiler |
| Workspace switch | < 100ms | Profiler |
| Launcher search (1000 apps) | < 10ms | sigma-bench launcher |

---

## Package Manager

| Metric | Target | Measurement |
|---|---|---|
| `sigma-pkg list` | < 50ms | time sigma-pkg list |
| `sigma-pkg search` (5000 pkgs) | < 100ms | sigma-bench pkg-search |
| `sigma-pkg install` (local) | < 500ms | sigma-bench pkg-install |
| Signature verification | < 20ms/pkg | sigma-bench pkg-verify |
| Reproducible rebuild diff | Bit-identical | sigma-pkg rebuild |

---

## Regression Policy

Any PR that causes a regression > 10% in any P1 metric (boot, frame time, input latency) **must be fixed or reverted** before merge.

CI automatically runs `sigma-bench` on every PR and comments the results.

---

## Benchmark Commands

```bash
# Run all benchmarks
sigma-bench all

# Specific categories
sigma-bench boot          # boot time (requires real hardware)
sigma-bench memory        # allocator benchmarks
sigma-bench network       # networking stack
sigma-bench crypto        # cryptographic operations
sigma-bench compositor    # frame rendering
sigma-bench scheduler     # context switch, fork, IPC
sigma-bench storage       # disk I/O

# Compare against baseline
sigma-bench all --compare baseline.json

# Generate HTML report
sigma-bench all --output report.html
```

---

*See also: [docs/UI_UX_Performance_Plan.md](UI_UX_Performance_Plan.md) · [docs/Hardware_CI_Matrix.md](Hardware_CI_Matrix.md)*
