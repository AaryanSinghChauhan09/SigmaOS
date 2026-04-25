# SigmaOS — Kernel Module Reference
> **Auto-audited**: This document reflects the actual files in the repository. Last synced: 2026-04-25.

---

## S01_Genesis — Kernel Core (`suites/S01_Genesis/`)

| Module File | Responsibility |
|-------------|----------------|
| `sigma_slab_alloc.h` | O(1) slab allocator — static pool, no malloc |
| `sigma_spinlock.h` | x86_64 XCHG inline-ASM spinlock |
| `sigma_vmm.h` | 2-level page table VMM — map/unmap/translate |
| `sigma_scheduler.h` | RDTSC-timed cooperative round-robin scheduler |
| `sigma_proc_pcb.h` | Process Control Block struct + table alloc/find/free |
| `sigma_proc_spawn.h` | Process spawning — PCB init + x86 RIP jump |
| `sigma_proc_kill.h` | Process termination + zombie reaping |
| `sigma_mem_pool.h` | 64 MB static arena — alloc/free/used-KB |
| `sigma_mem_audit.h` | Memory stats reporter + leak detector |
| `sigma_net_core.h` | Ethernet frame primitives — header, payload, checksum, stats |
| `sigma_types.h` | Sovereign type definitions (`sigma_u32`, `sigma_u64`, etc.) |
| `sigma_libc.h` | Zero-dependency libc: `sigma_kprint`, `sigma_memcpy`, `sigma_strlen` |
| `SovereignLibC.c` | Implementation of Sigma-Libc primitives |
| `SovereignProcessManager.c` | Legacy monolith (partially modularised → see sigma_proc_*.h) |
| `SovereignMemoryZenith.c` | Legacy monolith (partially modularised → see sigma_mem_*.h) |
| `SovereignNetMesh.c` | Legacy monolith (partially modularised → see sigma_net_core.h) |
| `SovereignShardKernel.c` | Shard kernel bootstrap |

---

## S04_HAL — Hardware Abstraction Layer (`suites/S04_HAL/`)

| Module File | Responsibility |
|-------------|----------------|
| `sigma_hal_drivers.hpp` | OOP NVMe + USB HID drivers (ISigmaDriver) |
| `dma_controller.cpp` | DMA ring management |
| `hal_registry.c` | Driver probe + registration |
| `keyboard_master.c` | PS/2 + USB keyboard interrupt handler |
| `arch/hal_exceptions.c` | x86/ARM exception vector table |
| `arch/aarch64/bcm2837_uart.c` | BCM2837 UART driver (Raspberry Pi) |
| `drivers/mock_hal.c` | CI test mock for hardware drivers |
| `drivers/framebuffer_driver.c` | Linear framebuffer renderer |
| `drivers/vga_driver.c` | VGA text-mode driver |
| `drivers/bcm_v3d_npu.c` | BCM V3D GPU/NPU driver |

---

## S08_Security — Security Shards (`suites/S08_Security/`)

| Module File | Responsibility |
|-------------|----------------|
| `sigma_pqc.c` | Kyber/Dilithium post-quantum crypto primitives |
| `sigma_zero_trust.c` | Per-request capability verification |
| `pqc_core.c` | PQC key generation core |
| `audit_master.c` | Silicon-level security event logger |
| `shard_isolation.c` | Process shard isolation enforcer |
| `SovereignSandbox.cpp` | Process sandboxing via capability gating |
| `SovereignLatticePQC.cpp` | PQC lattice-based operations |
| `SovereignHardwareAudit.cpp` | Hardware attestation auditor |
| `ZeroTrustAuthenticator.hpp` | Zero-trust auth interface |
| `formal_proofs/` | Kani-verified IPC/DMA non-interference proofs |

---

## S27_ContainerLattice — Container Isolation (`suites/S27_ContainerLattice/`)

| Module File | Responsibility |
|-------------|----------------|
| `sigma_cgroup.h` | CPU/mem/IO resource group — O(1) admission control |

---

## S28_PerformanceLattice — Performance (`suites/S28_PerformanceLattice/`)

| Module File | Responsibility |
|-------------|----------------|
| `sigma_worksteal.h` | GCD/TBB-inspired work-stealing thread pool |
| `sigma_cache.h` | LRU adaptive cache — RDTSC timestamps, FNV-1a hash |

---

## S30_NeuralPaging — NUMA Memory (`suites/S30_NeuralPaging/`)

| Module File | Responsibility |
|-------------|----------------|
| `sigma_numa.h` | Per-CPU-node NUMA-aware slab allocator |

---

## S31_ImmutableFS — Immutable Storage (`suites/S31_ImmutableFS/`)

| Module File | Responsibility |
|-------------|----------------|
| `sigma_immutable_fs.h` | A/B slot OTA swap + FNV-1a integrity + auto-rollback |

---

## S36_SovereignBPF — Programmable Filters (`suites/S36_SovereignBPF/`)

| Module File | Responsibility |
|-------------|----------------|
| `sigma_bpf.h` | eBPF-inspired programmable filter chain (no JIT) |

---

## S36_SovereignPackageRegistry — Package Manager (`suites/S36_SovereignPackageRegistry/`)

| Module File | Responsibility |
|-------------|----------------|
| `sigma_pkg.h` | Capability-gated, hash-verified package install/remove |
| `sigma_package_registry.h` | Package registry index and lookup |

---

## S37_ZeroKnowledgeProofLayer — ZKP (`suites/S37_ZeroKnowledgeProofLayer/`)

| Module File | Responsibility |
|-------------|----------------|
| `sigma_zkp.h` | Fiat-Shamir sigma protocol — commit/challenge/respond/verify |
| `sigma_zkp_attestation.h` | ZKP-based attestation for capability tokens |

---

## S37_SovereignWire — Networking (`suites/S37_SovereignWire/`)

| Module File | Responsibility |
|-------------|----------------|
| `sigma_netfilter.h` | Zero-copy packet firewall — wildcard bitmask rules |

---

## S41_SiliconBoot — Self-Healing Boot (`suites/S41_SiliconBoot/`)

| Module File | Responsibility |
|-------------|----------------|
| `sigma_auto_rollback.h` | RDTSC-timestamped snapshots + capability-gated restore |

---

## S42_RawIPC — Inter-Process Communication (`suites/S42_RawIPC/`)

| Module File | Responsibility |
|-------------|----------------|
| `sigma_ring_buffer.h` | Lock-free SPSC ring buffer |
| `sigma_aio.h` | io_uring-inspired async I/O SQ/CQ ring |

---

## S43_SovereignCaps — Capabilities (`suites/S43_SovereignCaps/`)

| Module File | Responsibility |
|-------------|----------------|
| `sigma_caps.h` | Capability token mint/check/revoke (zero-trust) |

---

## Core OOP Modules (`sigmaos/core/src/atomic_*`)

| File | Responsibility |
|------|----------------|
| `atomic_sigma_oop_base.hpp` | Abstract: `ISigmaModule`, `ISigmaDriver`, `ICallback` |
| `atomic_sigma_process.hpp` | Abstract: `IProcess` — run/block/terminate |
| `atomic_hal_irq.hpp` | IRQ dispatcher — Timer/KB/NIC handlers |
| `atomic_sigma_net_oop.cpp` | OOP NIC driver (ISigmaDriver) |
| `atomic_sigma_vfs_oop.cpp` | OOP VFS with user-defined I/O hooks |
| `atomic_sigma_auto_oop.cpp` | Automation hook manager (ICallback) |
| `atomic_sigma_alloc.cpp` | Custom arena allocator |
| `atomic_sigma_sched.cpp` | Inline-ASM context switch |
| `atomic_sigma_crypto.cpp` | SIMD/ASM crypto packet parser |
| `atomic_sigma_build.cpp` | Native toolchain linker stub |
| `atomic_sec_audit.cpp` | Security audit shard |
| `atomic_sec_encrypt.cpp` | Encryption shard |
| `atomic_sec_firewall.cpp` | Firewall enable shard |
| `atomic_mem_trace.cpp` | Memory arena leak tracer |
| `atomic_mem_prune.cpp` | Memory pruner |
| `atomic_sched_profile.cpp` | RDTSC scheduler profiler |
| `atomic_net_secure_connect.cpp` | Secure connection shard |
| `atomic_media_load_codec.cpp` | Codec loader shard |
| `atomic_perf_balance.cpp` | Performance balancer |
| `atomic_comp_split.cpp` | Component splitter utility |
| `atomic_ui_init.cpp` | UI initialisation shard |
| `atomic_ui_morph.cpp` | Morphic UI profile switcher |
| `atomic_ledger_audit.cpp` | Immutable state ledger audit |
| `atomic_subsystem_load.cpp` | Subsystem bootstrap loader |

---

## Module Count Summary

| Category | Count |
|----------|-------|
| S01_Genesis kernel headers | 17 |
| S04_HAL hardware modules | 10 |
| S08_Security shards | 10 |
| Container/Perf/NUMA | 3 |
| Storage/FS | 2 |
| Networking | 2 |
| Security (ZKP/Caps/BPF/PKG) | 6 |
| Self-Healing | 1 |
| IPC/AIO | 2 |
| Core OOP atomic modules | 24 |
| **Total** | **77+** |

---

## CI Workflows

| Workflow | File | What It Validates |
|----------|------|-------------------|
| Sovereign Build | `01_Sovereign_Build.yml` | Cross-platform build × 9 matrix jobs |
| Lattice Verification | `02_Lattice_Verification.yml` | cppcheck + Kani proofs + fuzzing |
| Native Quality Gate | `03_Web_Zenith.yml` | Compile check + sovereignty audit |
| Dev Coverage | `04_Sigma_Dev_Coverage.yml` | Module count + OOP audit + ASM count |
