# 🏗️ SigmaOS Architecture

## System Layer Model

```
┌─────────────────────────────────────────────────────────┐
│                    USER APPLICATIONS                     │
│              (Native, Flatpak, AppImage, AUR)           │
├─────────────────────────────────────────────────────────┤
│                   DESKTOP ENVIRONMENT                    │
│          Zenith Compositor + Sigma Shell + Wayland      │
├─────────────────────────────────────────────────────────┤
│                    SYSTEM SERVICES                       │
│    sigma-init │ D-Bus │ Polkit │ sigma-journal │ NTP    │
├─────────────────────────────────────────────────────────┤
│                     S-AI LAYER                          │
│      Orchestrator │ LLM Router │ Sigma Copilot │ NLU   │
├─────────────────────────────────────────────────────────┤
│                   SECURITY LAYER                        │
│    SELinux │ AppArmor │ Sentinel │ pledge │ Seccomp    │
├─────────────────────────────────────────────────────────┤
│                    SIGMA KERNEL                         │
│   EEVDF/BORE │ Memory │ IPC │ VFS │ Drivers │ eBPF    │
├─────────────────────────────────────────────────────────┤
│                     HARDWARE                            │
│     x86_64 │ ARM64 │ RISC-V │ UEFI │ ACPI │ PCIe     │
└─────────────────────────────────────────────────────────┘
```

## Kernel Architecture

SigmaOS uses a **hybrid kernel** design — combining the performance of a monolithic kernel with the modularity of a microkernel:

- **Critical path** (scheduler, memory, IPC) runs in kernel space for speed
- **Drivers and subsystems** can be loaded as kernel modules
- **eBPF** allows safe user-defined kernel extensions without recompilation

## Memory Architecture

### Paging Model
- **x86_64**: 4-level page tables (PML4 → PDPT → PD → PT)
- **ARM64**: 4-level translation tables (TTBRx)
- **Page sizes**: 4KB, 2MB (huge pages), 1GB (giant pages)

### Memory Zones
| Zone | Purpose |
|------|---------|
| DMA Zone | < 16MB, for legacy DMA hardware |
| DMA32 Zone | < 4GB, for 32-bit DMA devices |
| Normal Zone | > 4GB, general purpose |
| High Zone | Temporary mappings (x86_32 legacy) |

### Key Algorithms
- **Buddy Allocator**: Power-of-2 block allocation for physical pages
- **Slab Allocator**: Object caches for frequently allocated kernel structures
- **kswapd**: Background daemon for memory reclamation using LRU lists
- **KSM**: Kernel Same-page Merging for VM/container memory deduplication
- **CoW**: Fork uses copy-on-write; pages only duplicated on first write

## Scheduler Architecture

SigmaOS implements multiple scheduler classes in priority order:

1. **Stop class** — per-CPU stop tasks (highest priority)
2. **Deadline class** — EDF for real-time periodic tasks
3. **Realtime class** — FIFO/RR for soft real-time
4. **Fair class (EEVDF+BORE)** — normal interactive and batch tasks
5. **Idle class** — runs only when nothing else is ready

### EEVDF Algorithm
> Picks the **eligible** process with the **earliest virtual deadline**

- **Eligible**: process whose `virtual_runtime ≤ system_vtime`
- **Virtual deadline**: `vruntime + (time_slice / weight)`
- **BORE enhancement**: CPU-burst penalty increases virtual deadline for batch tasks

### NUMA-Aware Scheduling
- Each NUMA node has its own run queue
- Work-stealing balances load across CPUs
- CPU cache affinity respected where possible

## IPC Mechanisms

| Mechanism | Use Case | Performance |
|-----------|----------|-------------|
| Shared Memory | Large data transfer | ~0 copy overhead |
| Pipes | Sequential data streams | Low latency |
| Unix Sockets | Local service communication | Low latency |
| D-Bus | System service APIs | Moderate |
| io_uring | Async I/O submission | Very high throughput |
| eBPF Maps | Kernel ↔ userspace data | Near-zero overhead |

## Security Architecture

### Defence in Depth
```
Application Level:  pledge() + unveil() + Seccomp-BPF
MAC Level:         SELinux policies + AppArmor profiles  
Kernel Level:       KSPP hardening + W^X + KASLR + SMEP/SMAP
Boot Level:         UEFI Secure Boot + TPM PCR sealing
Network Level:      eBPF firewall + Zero-Trust + WireGuard
Crypto Level:       Post-quantum (Kyber + Dilithium) + TLS 1.3
```

## S-AI Architecture

```
User Request
     ↓
Orchestrator (task decomposition + agent routing)
     ↓
LLM Router (selects best local model for subtask)
     ↓
Specialist Agents (code, analysis, system, security)
     ↓
Response Aggregation
     ↓
User
```

**Local LLM Backends**:
- llama.cpp (GGUF models)
- Ollama
- LM Studio
- vLLM (GPU-accelerated)
