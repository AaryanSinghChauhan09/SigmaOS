# SigmaOS — Kernel Module Reference

> Complete reference for all atomic silicon modules. Each entry: one file, one function, zero dependencies.

---

## 🧬 S01_Genesis — Kernel Core

| Module | File | Function | Status |
|--------|------|----------|--------|
| Slab Allocator | `sigma_slab_alloc.h` | O(1) kernel memory allocation from static pool | ✅ Live |
| Spinlock | `sigma_spinlock.h` | x86 XCHG inline-ASM mutual exclusion | ✅ Live |
| Virtual Memory Manager | `sigma_vmm.h` | 2-level page table map/unmap/translate | ✅ Live |
| Round-Robin Scheduler | `sigma_scheduler.h` | RDTSC-timed cooperative task scheduling | ✅ Live |

---

## 🔌 S04_HAL — Hardware Abstraction Layer

| Module | File | Function | Status |
|--------|------|----------|--------|
| NVMe Driver | `sigma_hal_drivers.hpp` | PCIe BAR init + DMA queue management | ✅ Live |
| USB HID Driver | `sigma_hal_drivers.hpp` | USB endpoint enumeration + interrupt polling | ✅ Live |
| IRQ Dispatcher | `atomic_hal_irq.hpp` | Hardware interrupt routing (Timer/KB/NIC) | ✅ Live |
| DMA Controller | `dma_controller.cpp` | Direct Memory Access ring management | ✅ Live |

---

## 🔐 S08_Security — Security Shards

| Module | File | Function | Status |
|--------|------|----------|--------|
| Capability Tokens | `sigma_caps.h` | Zero-trust mint/check/revoke | ✅ Live |
| PQC Crypto | `sigma_pqc.c` | Kyber/Dilithium quantum-safe primitives | ✅ Live |
| Zero-Trust Auth | `sigma_zero_trust.c` | Per-request capability verification | ✅ Live |
| Audit Master | `audit_master.c` | Silicon-level security event logging | ✅ Live |
| Sandbox | `SovereignSandbox.cpp` | Process isolation via capability gating | ✅ Live |
| Formal Proofs | `formal_proofs/` | Kani-verified IPC non-interference | ✅ Live |

---

## 📦 S27_ContainerLattice — Container / Isolation

| Module | File | Function | Status |
|--------|------|----------|--------|
| CGroup Controller | `sigma_cgroup.h` | CPU/mem/IO resource group admission | ✅ Live |

---

## ⚡ S28_PerformanceLattice — Perf Optimization

| Module | File | Function | Status |
|--------|------|----------|--------|
| Work-Stealing Pool | `sigma_worksteal.h` | GCD-inspired per-CPU deque with steal | ✅ Live |

---

## 🗂️ S31_ImmutableFS — Immutable File System

| Module | File | Function | Status |
|--------|------|----------|--------|
| Immutable FS | `sigma_immutable_fs.h` | A/B slot OTA swap + FNV-1a integrity + auto-rollback | ✅ Live |

---

## 📡 S36_SovereignBPF — Programmable Filters

| Module | File | Function | Status |
|--------|------|----------|--------|
| BPF Filter Chain | `sigma_bpf.h` | Programmable packet/event filter (no JIT VM) | ✅ Live |

---

## 🔄 S42_RawIPC — Inter-Process Communication

| Module | File | Function | Status |
|--------|------|----------|--------|
| SPSC Ring Buffer | `sigma_ring_buffer.h` | Lock-free single-producer/consumer queue | ✅ Live |
| Async I/O Ring | `sigma_aio.h` | io_uring-inspired SQ/CQ submission ring | ✅ Live |

---

## 🎟️ S43_SovereignCaps — Capabilities

| Module | File | Function | Status |
|--------|------|----------|--------|
| Capability Tokens | `sigma_caps.h` | Token mint, permission check, revocation | ✅ Live |

---

## ⚙️ Core OOP Interfaces (`sigmaos/core/src/`)

| Interface | File | Purpose |
|-----------|------|---------|
| `ISigmaModule` | `atomic_sigma_oop_base.hpp` | Abstract base: init/execute/shutdown |
| `ISigmaDriver` | `atomic_sigma_oop_base.hpp` | Abstract driver: probe_hardware/enable_dma |
| `ICallback` | `atomic_sigma_oop_base.hpp` | User-defined automation functor |
| `IProcess` | `atomic_sigma_process.hpp` | Process lifecycle: run/block/terminate |
| `IInterruptHandler` | `atomic_hal_irq.hpp` | Hardware IRQ handler interface |
| `ICommand` | `orchestrator/main.cpp` | CLI command: matches/execute |

---

## 🛠️ CLI Commands

```
s-cli profile <work|gaming|vr>     Activate Morphic UI silicon profile
s-cli build <arch>                 Compile all atomic modules for target
s-cli test --subsystem <name>      Run regression suite for subsystem
s-cli benchmark --run-all          Full perf + security benchmark
s-cli forge                        Generate new shard on-demand
s-cli link                         Bio-telemetry cognitive sync
```

---

## 📊 CI Pipeline Status

| Workflow | Trigger | What it validates |
|----------|---------|-------------------|
| `01_Sovereign_Build.yml` | Push/PR | Cross-platform build (Ubuntu/macOS/Windows) × 3 profiles |
| `02_Lattice_Verification.yml` | Push/PR | cppcheck static analysis + Kani formal proofs + fuzzing |
| `03_Web_Zenith.yml` | Push/PR | Native quality gate: compile check + sovereignty audit |
