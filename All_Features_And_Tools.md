# Complete Features and Sovereign Tools Reference

This document serves as the master index for every capability, Shard, and tool engineered natively into SigmaOS. Our philosophy guarantees that each of these features runs at absolute bare-metal C11/Assembly efficiency, completely stripped of legacy bloat, interpreters, and generic standard libraries.

## 🛡️ Sovereign Kernel Foundations

| Foundation Module | Description | Location / Execution |
|---|---|---|
| **Zero-Dependency Kernel** | The x86_64 architecture avoids `glibc`/`musl` entirely. Custom ring-0 bootloader and pure intrinsic memory handlers manage system allocation. | `SovereignKernelZenith.asm`, `main.c` |
| **SovereignLibC** | The native library replacing standard C headers (`stdio.h`, `stdlib.h`), executing pure direct `syscalls`. | `SovereignLibC.asm` |
| **SigmaOOP Polymorphism** | C11 structure-based Object-Oriented polymorphism utilized via `CLASS_DECLARE` macros. | `SovereignOmniShard.h` |
| **Virtual File System (VFS)** | Sub-nanosecond UNIX-style file mapping utilizing direct memory-mapped snapshots, deduplication, and full rollback features. | `SigmaVFS.js`, `sigma_std.c` |
| **Zero-Copy Memory Manager** | PMM/VMM implementation combined with slab allocators explicitly built to bypass standard OS bottlenecks. | `SovereignProcessManager.c` |

## 🧩 Shard-On-Demand (SOD) Ecosystem

The Shard architecture executes functionalities directly in Ring-0 and unwinds them instantly when not needed.

| Shard | Primary Function | Target Persona / Sector |
|---|---|---|
| **High Frequency Trading (HFT)** | Native DMA ingress/egress bypassing the TCP/IP ring, nanosecond latency arbitrage execution. | Quantitative Finance |
| **Bio-Informatics Core** | AVX-512 SIMD-accelerated K-mer counting and memory-mapped fast sequencing (FASTQ/BAM). | Computational Biology |
| **Amnesic Forensic Scruber** | Overrides volatile memory caches; DOD-5220.22-M storage wiping and audit obfuscation. | Cyber-Forensics / SecOps |
| **AI Distributor Protocol** | Ring-0 IPC framework routing multi-model LLM inferences to system tools natively. | AI Research |
| **Sovereign ML / DSA Core** | Embedded matrix multiplication mapping directly to OpenCL/GPU hardware layers, problem visualization. | Data Science / ML |
| **Post-Quantum Cryptography** | PQC Lattice encryption integrated into fundamental IPC communications using LWE algorithms. | General Security |
| **Hypervisor Zenith** | Qubes-style domain isolation virtualization natively in C11. | Privacy |

## 🛠️ Omni Shell & Sovereign Custom Tools

We do not use standard GNU coreutils. All tools are native `C11` or strictly controlled UI orchestrator scripts.

| Sovereign Tool | Primary Function | Equivalent Replaced |
|---|---|---|
| `sigma_invoke` | The foundation CLI command. Mounts, compiles, and loads `.c` Shards directly into system memory. | `apt-get`, `pacman` |
| `sigma_auto_optimizer` | The memory daemon that silently checks slab fragmentation and executes process rebalancing. | `cron`, `systemd` |
| `omni_shell` | The 400+ command native POSIX-compatible terminal interface loaded with AI extensions and native parsing. | `bash`, `zsh` |
| `xclicker` | Hardware-native low-level input simulator for macro automation. | `xdotool`, `autohotkey` |
| `remote_bot` | Native Remote Procedure Call network executor with absolute zero-dependency mapping. | `ssh`, `netcat` |
| `gaming` | Pre-allocation tool locking CPU cores and dynamically releasing graphic V-RAM for hardware rendering isolation. | `gamemode` | 
| `system_cleaner` | Executes rapid cache dumping, forensic wiping, and history sanitization in C11. | `shred`, `bleachbit` |

## 🌌 Zenith-Gold UX & Application Architecture

| UI Component | Description |
|---|---|
| **JS DOM UI Orchestrator** | A completely bespoke window manager built natively in Javascript (`index.js`). Floating windows, dynamic resizing, and theming. |
| **Persona-Aware Adaption** | The kernel and UI seamlessly morph (allocating quotas, adapting themes, shifting CPU locks) when switching user mode paradigms. |
