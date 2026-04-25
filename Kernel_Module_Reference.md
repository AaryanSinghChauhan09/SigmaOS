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
| `sigma_proc_scheduler.hpp` | OOP Process Scheduler supporting class hierarchies |
| `sigma_proc_scheduler_ai.hpp` | Adaptive AI Scheduler modulating CPU C-states by workload |
| `sigma_rtos_deadline.hpp` | Hard Real-Time EDF Scheduler with microsecond guarantees |
| `sigma_proc_fork.hpp` | Native process cloner and thread spawner |
| `sigma_proc_signal.hpp` | OOP IPC Signal Dispatcher |
| `sigma_proc_trace.hpp` | Process Tracer for debugging and introspection |
| `sigma_mem_pool.h` | 64 MB static arena — alloc/free/used-KB |
| `sigma_mem_audit.h` | Memory stats reporter + leak detector |
| `sigma_net_core.h` | Ethernet frame primitives — header, payload, checksum, stats |
| `sigma_types.h` | Sovereign type definitions (`sigma_u32`, `sigma_u64`, etc.) |
| `sigma_libc.h` | Zero-dependency libc: `sigma_kprint`, `sigma_memcpy`, `sigma_strlen` |
| `sigma_libc_mem.hpp` | OOP encapsulation for memory management with bounds checking |
| `sigma_libc_string.hpp` | OOP string operations avoiding undefined behavior paths |
| `sigma_libc_io.hpp` | Pluggable OOP I/O backend for kernel print routines |
| `sigma_libc_syscall.hpp` | Direct inline assembly syscall invocation dispatcher |
| `SovereignLibC.c` | Legacy monolith (partially modularised → see sigma_libc_*.hpp) |
| `SovereignProcessManager.c` | Legacy monolith (partially modularised → see sigma_proc_*.h/hpp) |
| `SovereignMemoryZenith.c` | Legacy monolith (partially modularised → see sigma_mem_*.h) |
| `SovereignNetMesh.c` | Legacy monolith (partially modularised → see sigma_netmesh_*.hpp) |
| `SovereignShardKernel.c` | Shard kernel bootstrap |

---

## S02_ZenithUI — UI & Compositor (`suites/S02_ZenithUI/`)

| Module File | Responsibility |
|-------------|----------------|
| `sigma_ui_wayland.h` | Wayland-inspired sovereign display compositor (zero libwayland) |
| `sigma_ui_shader_morph.h` | Vulkan Morphic Shaders pushing constants for glassmorphism |
| `sigma_ui_theme_loader.h` | Binary theme loader for instant zero-copy UI theme switching |
| `sigma_ui_profile_switcher.h` | UI profile switcher for toggling Work, Gaming, VR modes |
| `sigma_os_adaptive_profile.hpp` | Profile-driven sovereignty encapsulating settings per user |
| `sigma_ui_shader_holographic.hpp` | Holographic transparency and adaptive blur physics shaders |

---

## S04_HAL — Hardware Abstraction Layer (`suites/S04_HAL/`)

| Module File | Responsibility |
|-------------|----------------|
| `sigma_hal_core.hpp` | Core HAL abstracting CPU architectures (x86_64, ARM64, RISC-V) |
| `sigma_hal_driver_gpu.hpp` | GPU Hardware Abstraction Layer for discrete accelerators |
| `sigma_hal_driver_storage.hpp` | Unified Storage HAL for NVMe, SATA, and eMMC interfaces |
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

## S05_Memory — Memory Management (`suites/S05_Memory/`)

| Module File | Responsibility |
|-------------|----------------|
| `sigma_mem_tcache.h` | jemalloc-inspired thread cache — per-size-class free lists |

---

## S07_Network — Networking Stack (`suites/S07_Network/`)

| Module File | Responsibility |
|-------------|----------------|
| `sigma_net_tcp.h` | lwIP-inspired lightweight TCP stack (zero heap allocation) |
| `sigma_net_vpn.h` | WireGuard-inspired sovereign VPN tunnel (ZKP handshake) |
| `sigma_net_dns.h` | Unbound-inspired DNS resolver (DNSSEC-aware, TTL expiry) |
| `sigma_net_driver_ethernet.hpp` | OOP Ethernet driver base class + DMA inline ASM hooks |
| `sigma_net_driver_wifi.hpp` | OOP Wi-Fi driver interface + AP scanning/auth abstraction |
| `sigma_net_core_socket.h` | POSIX-like zero-copy socket API implementation |
| `sigma_net_firewall.h` | O(N) stateless IP matching firewall rule engine |
| `sigma_netmesh_routing.hpp` | OOP Mesh Router utilizing Optimized Link State Routing |
| `sigma_netmesh_topology.hpp` | OOP Mesh Topology Discoverer via Layer 2 Heartbeats |
| `sigma_netmesh_security.hpp` | Zero-Trust Mesh Security Enforcer utilizing ZKP caps |

---

## S08_Security — Security Shards (`suites/S08_Security/`)

| Module File | Responsibility |
|-------------|----------------|
| `sigma_pqc.c` | Legacy monolith (partially modularised → see sigma_pqc_*.h) |
| `sigma_pqc_keygen.h` | PQC Keypair generation |
| `sigma_pqc_sign.h` | PQC FNV-1a backed cryptographically secure signature |
| `sigma_pqc_verify.h` | PQC cryptographic verification |
| `sigma_formal_proof.hpp` | Mathematical native Design-by-Contract invariant checker |
| `sigma_sec_audit_runtime.hpp` | Runtime zero-trust auditing Engine |
| `sigma_sec_sandbox_quantum.hpp` | Quantum-safe micro-VM app isolation via ASM MSR hooks |
| `sigma_sec_firewall_adaptive.hpp` | Adaptive Mesh Firewall responding to global threat levels |
| `sigma_sec_integrity.hpp` | Continuous Integrity Verification hashing |
| `sigma_zero_trust.c` | Per-request capability verification |
| `sigma_sec_tpm.h` | TPM 2.0-inspired PCR banks + attestation (zero tpm2-tools) |
| `sigma_sec_mac.h` | SELinux-inspired mandatory access control policy table |
| `sigma_sec_crypto_quantum.h` | Quantum-safe primitives via Montgomery Reduction ASM |
| `sigma_sec_audit.h` | Automated silicon-level security event logger |
| `sigma_sec_sandbox.h` | Capability-based app isolation container (IProcess wrapper) |
| `pqc_core.c` | PQC key generation core |
| `audit_master.c` | Silicon-level security event logger |
| `shard_isolation.c` | Process shard isolation enforcer |
| `SovereignSandbox.cpp` | Process sandboxing via capability gating |
| `SovereignLatticePQC.cpp` | PQC lattice-based operations |
| `SovereignHardwareAudit.cpp` | Hardware attestation auditor |
| `ZeroTrustAuthenticator.hpp` | Zero-trust auth interface |
| `formal_proofs/` | Kani-verified IPC/DMA non-interference proofs |

---

## S12_Ecosystem — Multimedia & Extensions (`suites/S12_Ecosystem/`)

| Module File | Responsibility |
|-------------|----------------|
| `sigma_media_codec.h` | FFmpeg-inspired codec registry (plugin-free C functions) |

---

## S15_DevNexus — Developer Tools (`suites/S15_DevNexus/`)

| Module File | Responsibility |
|-------------|----------------|
| `sigma_dev_test.h` | Catch2-inspired native unit test framework |
| `sigma_dev_build.h` | Make/Ninja-inspired dependency graph builder |
| `sigma_dev_gdb.h` | GDB-inspired debug stub (INT3 software breakpoints) |

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
| `sigma_perf_profiler.h` | Linux perf-inspired RDTSC profiler zones |
| `sigma_perf_shadow.h` | Valgrind-inspired shadow memory leak detector |
| `sigma_perf_isolator.h` | Resource isolation dynamically restricting CPU freq and memory |
| `sigma_os_behavior_adaptive.hpp` | Behavior-adaptive OS caching heuristics |

---

## S30_NeuralPaging — NUMA Memory (`suites/S30_NeuralPaging/`)

| Module File | Responsibility |
|-------------|----------------|
| `sigma_numa.h` | Per-CPU-node NUMA-aware slab allocator |

---

## S31_ImmutableFS — Immutable Storage (`suites/S31_ImmutableFS/`)

| Module File | Responsibility |
|-------------|----------------|
| `sigma_fs_ext.hpp` | Legacy EXT2/3/4 filesystem compatibility wrapper |
| `sigma_fs_btrfs.hpp` | BTRFS subvolume and snapshot compatibility |
| `sigma_fs_sovereign.hpp` | Native Sovereign Immutable Filesystem for cryptographically verified persistence |
| `sigma_immutable_fs.h` | A/B slot OTA swap + FNV-1a integrity + auto-rollback |

---

## S32_SystemTools — Standard Utilities (`suites/S32_SystemTools/`)

| Module File | Responsibility |
|-------------|----------------|
| `sigma_sys_busybox.h` | BusyBox-inspired UNIX utilities (cat, grep, wc, etc.) |
| `sigma_sec_musl.h` | musl-inspired hardened libc extensions (safe memcpy, etc.) |

---

## S36_SovereignBPF & Package Registry (`suites/S36_*/`)

| Module File | Responsibility |
|-------------|----------------|
| `sigma_bpf.h` | eBPF-inspired programmable filter chain (no JIT) |
| `sigma_pkg.h` | Capability-gated, hash-verified package install/remove |
| `sigma_package_registry.h` | Package registry index and lookup |

---

## S37_SovereignWire & ZKP (`suites/S37_*/`)

| Module File | Responsibility |
|-------------|----------------|
| `sigma_zkp.h` | Fiat-Shamir sigma protocol — commit/challenge/respond/verify |
| `sigma_zkp_attestation.h` | ZKP-based attestation for capability tokens |
| `sigma_netfilter.h` | iptables-inspired zero-copy packet firewall |

---

## S41_SiliconBoot — Boot & Self-Healing (`suites/S41_SiliconBoot/`)

| Module File | Responsibility |
|-------------|----------------|
| `sigma_boot_init.hpp` | Sovereign Bootloader native long-mode transition |
| `sigma_boot_handoff.hpp` | Clean handoff from bootloader to kernel main |
| `sigma_boot_securecheck.hpp` | Post-quantum boot image signature verification |
| `sigma_boot_driverload.hpp` | Native PCI bus scanning and driver injection engine |
| `sigma_auto_watchdog.hpp` | OOP Self-healing daemon integrating with ICallback |
| `sigma_auto_rollback.hpp` | OOP Rollback automation logic triggering CPU IDT faults |
| `sigma_auto_update.hpp` | OOP Seamless update engine managing A/B boot slots |
| `sigma_auto_userfn.hpp` | User-Defined automation hooks (auto-backup, etc.) |
| `sigma_auto_watchdog.h` | Legacy C bindings for watchdog |
| `sigma_auto_rollback.h` | Legacy C bindings for rollback |
| `sigma_fw_update.h` | Legacy C bindings for firmware updates |
| `sigma_sys_cron.h` | cron-inspired sovereign task scheduler |

---

## S42_RawIPC — Inter-Process Communication (`suites/S42_RawIPC/`)

| Module File | Responsibility |
|-------------|----------------|
| `sigma_ipc_channel.hpp` | Zircon-slayer capability-handle zero-copy asynchronous channel |
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
| S01_Genesis kernel headers | 27 |
| S04_HAL hardware modules | 13 |
| S08_Security shards | 23 |
| DevTools/Ecosystem/UI/Tools | 14 |
| Container/Perf/NUMA | 7 |
| Storage/FS | 5 |
| Networking | 12 |
| Security (ZKP/Caps/BPF/PKG) | 6 |
| Self-Healing & Boot | 12 |
| IPC/AIO | 3 |
| Core OOP atomic modules | 24 |
| **Total Native Modules** | **146+** |

---

## CI Workflows

| Workflow | File | What It Validates |
|----------|------|-------------------|
| Sovereign Build | `01_Sovereign_Build.yml` | Cross-platform build × 9 matrix jobs |
| Lattice Verification | `02_Lattice_Verification.yml` | cppcheck + Kani proofs + fuzzing |
| Native Quality Gate | `03_Web_Zenith.yml` | Compile check + sovereignty audit |
| Dev Coverage | `04_Sigma_Dev_Coverage.yml` | Module count + OOP audit + ASM count |
