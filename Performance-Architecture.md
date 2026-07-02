# Performance Architecture

This page documents the low-latency and high-throughput design decisions in SigmaOS, sourced from the `performance-optimized` branch. These techniques apply across all profiles but are most aggressively enabled in the `release/standalone` and `release/rtos` builds.

---

## Scheduling: Minimum Latency Context Switches

Context switches are the single largest source of scheduling latency. SigmaOS minimises the cost by:

- **Preserving only live registers** — the switch saves only the registers actually in use by the preempted task, not the full x86_64 general-purpose register file. Dead registers are not saved.
- **Inline assembly swap** — `SovereignScheduler::swapContextRegisters` executes directly on CPU stacks without a function call frame, eliminating call/ret overhead.
- **Timer tunable at runtime** — the scheduler time slice is a live sysctl:

```bash
# Read current time slice (default: 10 ms)
sigma-sysctl kernel.sched.timeslice_ms

# Reduce to 5 ms for interactive workloads
sigma-sysctl kernel.sched.timeslice_ms=5
```

---

## Memory: Lockless O(1) Allocation

### Physical Memory Manager (PMM)

The PMM uses a **quick bitmap (QBMP)** allocator:
- 8-byte-aligned bitmap over the full physical memory map.
- `alloc_frame()` scans for the first free bit — O(1) amortised with BSFL/TZCNT CPU instructions.
- No locking required for single-CPU allocation; SMP requires a single CAS on the bitmap word.

### Slab Allocator (planned for Phase 2)

The upcoming slab allocator eliminates heap fragmentation for kernel objects:
- Fixed-size buckets per object type (task structs, VFS nodes, socket descriptors).
- Lockless free-list per CPU (no global lock on the hot path).
- Objects never cross page boundaries — cache-line friendly.

### Copy-on-Write (CoW) Pages

Fork is O(1) — the child shares the parent's page table entries with `W` bits cleared. Physical copy happens only on first write (page fault path). This makes `sigma_process.spawn()` fast regardless of parent memory footprint.

---

## NUMA Awareness

On multi-socket machines, the scheduler automatically balances workloads to minimise cross-socket memory accesses:

```
CPU 0 (socket 0)  ──── local memory
CPU 1 (socket 0)  ──── local memory
      │                    │
      └──── QPI link ───────┘
CPU 2 (socket 1)  ──── remote memory  ← 2-4× latency penalty
```

`SovereignScheduler::balanceNUMANodes()` pins threads to the socket that owns the majority of their working set. This is measured continuously via hardware performance counters (PMU) and adjusts dynamically without manual configuration.

Runtime tunable:
```bash
sigma-sysctl kernel.sched.numa_balance=1   # enable (default: on)
```

---

## SIMD Auto-Vectorisation

The kernel and klib are compiled with:
- `-mavx512f` on x86_64 where available (detected at boot via CPUID)
- `-march=armv8-a+sve` on ARM64

This accelerates:
- **Cryptographic routines** — AES-NI for CryptFS, SHA-NI for audit log hashing
- **Memory operations** — `sigma_memcpy` / `sigma_memset` use VMOVDQU512 for 64-byte-at-a-time transfers
- **Neural UI Engine** — inference feature vector dot products use AVX-512 FMA

The build system detects CPU capabilities at compile time:
```makefile
CFLAGS += $(shell scripts/detect_avx.sh)   # emits -mavx512f or -mavx2 or -msse4.2
```

---

## Profile-Guided Optimisation (PGO)

The `performance-optimized` branch adds a two-stage PGO build:

```bash
# Stage 1: build with instrumentation
cmake -B build -DSIGMA_PGO=instrument
make -C build -j$(nproc)

# Run representative workload to collect profile data
./build/sigmaos.bin --pgo-workload tests/pgo_workload.sh

# Stage 2: rebuild with profile data
cmake -B build -DSIGMA_PGO=use
make -C build -j$(nproc)
```

PGO moves hot syscall dispatch paths into the instruction cache's most-fetched cache lines, reducing branch mispredictions in the scheduler and VFS hot paths by ~15–20% on workstation builds.

---

## Lock-Free IPC (SPSC Ring Buffers)

Inter-process communication between kernel shards uses **single-producer single-consumer (SPSC) ring buffers**:

```
Producer (shard A)          Consumer (shard B)
    │                              │
    └──── write head ──────────────┘
              ↑                    ↑
           cache line 0       cache line 1
           (producer-owned)   (consumer-owned)
```

Design properties:
- **No locking** — head and tail pointers are in separate cache lines, eliminating false sharing.
- **Zero-copy** — large messages are passed by pointer into a shared memory segment; only the pointer goes through the ring.
- **Bounded latency** — fixed ring size means producers never block indefinitely.

Runtime tunable:
```bash
sigma-sysctl kernel.ipc.ring_size=4096    # entries per ring (power of 2)
```

---

## Firewall and Network Fast Path

The `sigma_shield` firewall evaluates rules in O(1) per packet using a hash table keyed on the 5-tuple (src_ip, dst_ip, src_port, dst_port, protocol):

- **No linear scan** of rule chains — rule hit rate is monitored and hot rules are promoted to the front of a small L1-cached fast-path array.
- **Conntrack counter is always accurate** — decremented on connection close (Bug #19 regression test covers this).
- **Packet processing target**: < 1 ms per packet under normal load.

---

## Benchmarking

Run the built-in benchmark suite:

```bash
# Scheduler context switch latency
sigma-sysctl kernel.bench.context_switch=1

# Memory allocation throughput
./build/tests/kernel/bench_pmm

# TCP round-trip latency (loopback)
./build/tests/kernel/bench_tcp_loopback

# Syscall dispatch overhead
./build/tests/kernel/bench_syscall
```

Compare results with the baseline in `tests/performance_profiler.test.js`.

---

*See also: [Kernel Architecture](Kernel) · [Branch Guide](Branch-Guide) · [Building from Source](Building-from-Source)*
