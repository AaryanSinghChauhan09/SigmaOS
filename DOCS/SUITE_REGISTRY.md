# SigmaOS: Canonical Sovereign Suite Registry

> **Authority Document** — All kernel suite directories must match this manifest.
> Duplicate suite IDs are a build-time error.

## Suite Numbering Conflicts Identified & Resolved

The following duplicate suite IDs were detected and require directory consolidation:

| Suite Slot | **CANONICAL Name** | Remove Alias |
|---|---|---|
| S01 | `S01_SiliconFoundation` | ~~S01_Genesis~~ |
| S05 | `S05_Memory` | ~~S05_Storage → S06_Storage~~ |
| S08 | `S08_Compliance` | ~~S08_Security~~ |
| S12 | `S12_Parallelism` | ~~S12_Ecosystem~~ |
| S21 | `S21_Virtualization` | ~~S21_EternalState~~ |

---

## Canonical 33-Suite Lattice

| Suite | Name | Responsibility |
|---|---|---|
| S00 | SovereignCore | Micro-kernel, IPC, Immutability Guard |
| S01 | SiliconFoundation | Memory paging, buddy/slab allocator |
| S02 | ZenithUI | Zenith Desktop rendering pipeline |
| S03 | Orchestrator | System call dispatch, scheduler |
| S04 | HAL | Hardware abstraction (GPU, Wi-Fi, USB) |
| S05 | Memory | Virtual memory, swap, page-fault handling |
| S06 | Storage | Block device VFS, QNFS, RAM-disk |
| S07 | Network | TCP/IP, IPv6, VPN, IDS |
| S08 | Compliance | MAC, Audit, Authentication |
| S09 | Intelligence | AI inference, neural pattern engine |
| S10 | Registry | Zero-trust namespacing, shard registry |
| S11 | Virtualization | WASM JIT, containers, VM |
| S12 | Parallelism | SIMD, multi-core dispatch |
| S13 | Sentience | SoulMolding, consciousness layer |
| S14 | Transcendence | Math primitives, neural compression |
| S15 | DevNexus | POSIX bridge, marketplace, shard manager |
| S16 | SoulMolding | Identity & adaptive behavior |
| S17 | BioNexus | Bio-signal synchronizer |
| S18 | QuantumLink | QKD, quantum-secure comms |
| S19 | SelfEvolution | Hot-patch, autonomous update |
| S20 | Interconnect | Hyper-Link, cross-cluster IPC |
| S21 | Virtualization | was duplicate — merged into S11 |
| S22 | SimulationNexus | Physics/world simulation |
| S23 | OmniNexus | Cross-OS interop fabric |
| S24 | GlobalDebugger | Kernel-level debug tracer |
| S25 | ZeroKernel | Bare-metal fallback core |
| S26 | OmniFabric | Fabric routing, data-plane |
| S27 | NeuralLink | Brain-computer interface layer |
| S28 | OmniBus | Unified hardware/software bus |
| S29 | LatticeMerge | Live lattice state merging |
| S30 | Supremacy | Supremacy Signature, code signing |
| S31 | GlobalGovernance | Zero-entropy consensus mechanism |
| S32 | UnifiedSovereignty | Lattice-wide sovereignty protocol |
| S33 | TerminalFulfillment | Shell, eternal state declaration |

---

> **Rule:** Every shard `#include` must resolve to one and only one suite header.
