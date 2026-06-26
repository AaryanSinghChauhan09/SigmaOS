"""
Sync all new SigmaOS component READMEs → Wiki pages,
enrich them with full architectural detail,
then update the _Sidebar.md.
"""
import os, shutil, textwrap

REPO  = os.path.dirname(os.path.abspath(__file__))
WIKI  = os.path.join(REPO, "..", "SigmaOS_wiki")

# ── helper ──────────────────────────────────────────────────────────────────
def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        f.write(textwrap.dedent(text).lstrip())
    print(f"  [OK] {os.path.relpath(path, REPO)}")

def wp(name):          # wiki page path
    return os.path.join(WIKI, name)

# ── 1. Enrich every component in the main repo ──────────────────────────────
print("\n[1] Enriching component docs in main SigmaOS repo...")

# DDK
write(os.path.join(REPO, "drivers/ddk/README.md"), """\
# Driver Development Kit (DDK)

The **DDK** is SigmaOS's sovereign framework for authoring, testing, and
formally verifying hardware drivers.

## Why a Sovereign DDK?
Linux drivers land in a monolithic kernel where a single bug can crash the
whole system. SigmaOS isolates every driver in its own capability-gated shard
so a faulty NIC driver can't corrupt the filesystem or scheduler.

## Key APIs
| Symbol | Purpose |
|---|---|
| `sigma_register_driver(name)` | Register a driver shard with the HAL registry |
| `sigma_alloc_dma_region(size)` | Allocate physically contiguous, cache-coherent memory |
| `sigma_irq_install(vector, handler)` | Bind an interrupt vector with formal priority checking |

## Directory Layout
```
drivers/ddk/
  ddk_stub.c        ← Minimal compilable template
  ddk_api.h         ← Public DDK header (TODO)
  tests/            ← Formal property tests (TODO)
```

## Roadmap
- [ ] DMA management API
- [ ] IRQ arbitration layer
- [ ] Formal verification harness (CBMC / Frama-C integration)
""")

# HAL
write(os.path.join(REPO, "hal/README.md"), """\
# Hardware Abstraction Layer (HAL)

The **SovereignHAL** provides a single, architecture-agnostic interface that
the kernel uses to talk to hardware. Porting SigmaOS to a new CPU means
implementing one HAL backend — nothing else needs to change.

## Supported Targets
| Architecture | Status |
|---|---|
| x86_64 | ✅ Active |
| AArch64 (ARM64) | 🔧 In-progress |
| RISC-V RV64GC | 📋 Planned |

## Core Abstractions
```c
void hal_init(void);
void hal_set_irq_handler(uint32_t vec, void (*fn)(void));
void hal_flush_tlb(void);
uint64_t hal_get_timestamp_ns(void);
```

## Roadmap
- [ ] AArch64 MMU backend
- [ ] RISC-V SBI wrapper
- [ ] ACPI/DTB discovery
""")

# GPU Compute Toolkit
write(os.path.join(REPO, "graphics/gpu_compute/README.md"), """\
# GPU Compute Toolkit

Sovereign alternative to CUDA/ROCm that does **not** depend on vendor
proprietary user-space libraries.

## Architecture
```
User App
   └─ SigmaGPU API (sovereign, no CUDA/ROCm)
         └─ SovereignHAL GPU backend
               ├─ NVIDIA (via open-firmware commands)
               ├─ AMD (via AMDGPU register map)
               └─ Intel (via i915-compatible open spec)
```

## Goals
- Compute kernels launched via structured shard messages
- Deterministic memory mapping (GPU VRAM ↔ RAM) with cryptographic attestation
- Zero vendor lock-in

## Roadmap
- [ ] Shader compiler (SPIR-V front-end)
- [ ] Command buffer submission
- [ ] Memory allocator (GPU VRAM)
""")

# SovereignFS
write(os.path.join(REPO, "fs/sovereign_fs/README.md"), """\
# SovereignFS — Sovereign Journaling Filesystem

**SovereignFS** (SFS) is SigmaOS's own filesystem designed for deterministic
write latency, cryptographic block integrity, and native rollback support.

## Design Principles
1. **Copy-on-Write (CoW):** Every write creates a new extent; old data is
   preserved until explicitly pruned. Enables instant snapshots.
2. **Journaling:** All metadata changes are journaled before data is written
   — survives power failures cleanly.
3. **Block Integrity:** Each 4 KB block carries a BLAKE3 checksum; the kernel
   rejects tampered blocks at read time.

## On-Disk Layout
```
[Superblock 4K] [Journal 64MB] [Inode Table] [Data Extents ...]
```

## Roadmap
- [ ] Superblock & journal format specification
- [ ] `sfs_mkfs` tool
- [ ] Kernel VFS integration
- [ ] Snapshot/rollback API
""")

# IPv6
write(os.path.join(REPO, "net/ipv6/README.md"), """\
# Sovereign IPv6 Stack

Full dual-stack (IPv4 + IPv6) network implementation with no dependency on
the Linux networking subsystem.

## Features
- Stateless Address Auto-Configuration (SLAAC)
- NDP (Neighbor Discovery Protocol)
- Flow Labels for QoS
- IPSec integration point

## Roadmap
- [ ] ICMPv6 core
- [ ] DHCPv6 client
- [ ] Multicast routing
""")

# VPN
write(os.path.join(REPO, "net/vpn/README.md"), """\
# Sovereign VPN

Kernel-level, WireGuard-inspired VPN tunnel with post-quantum key exchange.

## Why Kernel-Level?
Userspace VPN daemons add syscall overhead on every packet. SigmaOS handles
encryption/decryption directly in the network shard, achieving line-rate
throughput.

## Cryptography
- **Key Exchange:** X25519 (classical) + Kyber-768 (post-quantum, hybrid)
- **Data Encryption:** ChaCha20-Poly1305
- **Authentication:** BLAKE3 MAC

## Roadmap
- [ ] Tunnel establishment protocol
- [ ] Peer management
- [ ] PQC key-exchange integration
""")

# Mesh Networking
write(os.path.join(REPO, "net/mesh/README.md"), """\
# Sovereign Mesh Networking

Decentralized mesh protocol for IoT edge nodes and defense-grade deployments
where a central router cannot be assumed.

## Protocol Design
- Gossip-based peer discovery (no central authority)
- Per-hop encryption using shared sovereign identity tokens
- Topology-aware routing (minimise hops, maximise redundancy)

## Use Cases
- Military battlefield networks (air-gapped, self-healing)
- Industrial IoT sensor grids
- Disaster-relief communication infrastructure

## Roadmap
- [ ] Gossip protocol implementation
- [ ] Routing table convergence algorithm
- [ ] Integration with Sovereign Identity Manager
""")

# DNS
write(os.path.join(REPO, "net/dns/README.md"), """\
# Sovereign DNS Resolver

Encrypted, privacy-preserving DNS resolver integrated directly into the
SigmaOS networking shard.

## Features
- **DNS-over-TLS (DoT)** and **DNS-over-HTTPS (DoH)** support
- Local shard-based caching (no external resolver required)
- DNSSEC validation
- Sovereign split-horizon: internal `.sigma` domains resolved locally

## Roadmap
- [ ] DoT client implementation
- [ ] DNSSEC chain validator
- [ ] Local authority for `.sigma` TLD
""")

# POSIX Compat
write(os.path.join(REPO, "tools/compat/README.md"), """\
# POSIX Compatibility Shim

A lightweight **opt-in** translation layer that allows existing POSIX/Linux
applications to run on SigmaOS without modification, while keeping the
sovereign kernel clean.

## Architecture
```
Linux ELF Binary
   └─ SigmaCompat loader (intercepts syscalls)
         └─ Translates POSIX syscalls → Sovereign Syscall ABI
               └─ SigmaOS Kernel
```

## What It Covers
| POSIX Syscall | Sovereign Translation |
|---|---|
| `open()` | `sigma_vfs_open()` |
| `read()` / `write()` | `sigma_io_*()` |
| `fork()` | `sigma_spawn_shard()` |
| `pthread_*` | Sovereign task primitives |

## What It Does NOT Cover
- `ioctl()` calls that touch hardware directly (forbidden by capability model)
- Signals that violate the determinism contract

## Roadmap
- [ ] ELF loader with syscall interception
- [ ] `mmap()` translation
- [ ] Dynamic linker shim
""")

# WebAssembly Runtime
write(os.path.join(REPO, "runtime/wasm/README.md"), """\
# WebAssembly Runtime

A sovereign, libc-free WebAssembly (WASM) interpreter/JIT that allows
cross-platform apps to run sandboxed on SigmaOS.

## Why WASM?
WASM gives SigmaOS a portable app format that is:
1. **Architecture-neutral** — one binary runs on x86_64, ARM64, and RISC-V
2. **Capability-sandboxed** — WASM modules can only access resources they
   have been explicitly granted
3. **POSIX-free** — no libc needed

## Execution Model
```
.wasm module → SigmaWASM validator → JIT compiler → native shard
```

## Roadmap
- [ ] WASM binary validator (MVP spec)
- [ ] Interpreter (for boot, no JIT)
- [ ] Cranelift-based JIT backend
- [ ] WASI (WebAssembly System Interface) sovereign mapping
""")

# Hybrid Scheduler
write(os.path.join(REPO, "scheduling/hybrid/README.md"), """\
# Hybrid Sovereign Scheduler

Extends SigmaOS's Round-Robin and EDF schedulers with:
- **NUMA awareness** — tasks affined to local memory nodes
- **Real-time (RT) lanes** — hard deadlines guaranteed
- **Energy efficiency** — frequency scaling for idle workloads
- **AI prediction** — ML model pre-fetches task requirements

## Class Hierarchy
```
SovereignScheduler
  ├─ RTLane      (hard real-time, EDF)
  ├─ NUMAFair    (NUMA-aware CFS analogue)
  └─ EcoLane     (battery/power optimised)
```

## Roadmap
- [ ] RTLane preemption guarantees
- [ ] NUMA topology detector
- [ ] CPU frequency governor integration
""")

# MAC
write(os.path.join(REPO, "security/mac/README.md"), """\
# Mandatory Access Control (MAC)

Sovereign alternative to SELinux and AppArmor with **deterministic policy
evaluation** (no probabilistic caching, no race conditions).

## Model
SigmaOS uses a **Lattice-Based Access Control** model where every subject
(process shard) and object (resource) has a clearance label. Operations are
only permitted when the lattice partial order is satisfied.

## Policy Language
```
# Example: allow browser shard to read /media, deny /sys
ALLOW browser_shard READ /media
DENY  browser_shard ANY  /sys
```

## Roadmap
- [ ] Label assignment to all shards at boot
- [ ] Policy compiler (text → binary rule table)
- [ ] Kernel enforcement hook in syscall dispatcher
""")

# Identity
write(os.path.join(REPO, "security/identity/README.md"), """\
# Cryptographic Identity Manager

Zero-trust process identity: every shard receives a cryptographically signed
identity token at spawn time. No token = no resource access.

## Token Structure
```json
{
  "shard_id": "uuid-v4",
  "capabilities": ["NET_BIND", "FS_READ:/media"],
  "issued_at": 1748200000,
  "signature": "<ED25519 sig over above fields>"
}
```

## Lifecycle
1. Kernel spawns shard → generates ephemeral key pair
2. Trust Root signs the token
3. Every IPC / syscall presents token → verified in O(1) via cached pubkey

## Roadmap
- [ ] ED25519 token issuance
- [ ] Token revocation list (CRL equivalent)
- [ ] PQC upgrade path (Dilithium)
""")

# PQC
write(os.path.join(REPO, "security/pqc/README.md"), """\
# Quantum-Safe Cryptography Toolkit

Prepares SigmaOS for the post-quantum era by integrating NIST-selected PQC
algorithms at the kernel level.

## Algorithms Integrated
| Purpose | Algorithm | NIST Status |
|---|---|---|
| Key Encapsulation | Kyber-768 / ML-KEM | ✅ Standard |
| Digital Signatures | Dilithium3 / ML-DSA | ✅ Standard |
| Hashing | BLAKE3 | 🔧 Best-in-class |

## Integration Points
- VPN key exchange (Kyber hybrid with X25519)
- Shard identity tokens (Dilithium signatures)
- Shard manifest signatures (SPM verifier)

## Roadmap
- [ ] Kyber KEM integration in VPN
- [ ] Dilithium signature in Identity Manager
- [ ] Side-channel hardening (constant-time implementations)
""")

# Hypervisor
write(os.path.join(REPO, "virtualization/hypervisor/README.md"), """\
# Sovereign Bare-Metal Hypervisor

A Type-1 hypervisor built directly into SigmaOS to run isolated guest VMs
without depending on Linux KVM.

## Design
- **VT-x / AMD-V** on x86_64; **EL2** on AArch64
- Each guest VM is a capability-gated shard — the hypervisor is just another
  kernel module, not a privileged monolith
- Live migration via SovereignFS snapshot deltas

## Roadmap
- [ ] VMCS/VMCB setup (x86_64)
- [ ] Guest memory isolation (EPT / NPT)
- [ ] Virtio-net / Virtio-blk para-virtual devices
- [ ] Live migration prototype
""")

# Containers
write(os.path.join(REPO, "virtualization/containers/README.md"), """\
# Sovereign Container Framework

Linux-independent container implementation that uses SigmaOS shard isolation
instead of Linux namespaces and cgroups.

## Comparison with Docker/OCI
| Feature | Docker (Linux) | SovereignContainers |
|---|---|---|
| Isolation | Linux namespaces | Shard capability model |
| Resource limits | cgroups | Sovereign scheduler quotas |
| Image format | OCI tar layers | Shard bundles (CoW SFS extents) |
| Runtime | runc | sigma-run |

## Roadmap
- [ ] `sigma-run` container runtime
- [ ] Shard bundle format (OCI-compatible import)
- [ ] Networking namespace analogue via mesh shards
""")

# AI Scheduler
write(os.path.join(REPO, "core/ai_scheduler/README.md"), """\
# AI Scheduling Engine

Predictive resource allocator that uses an on-device ML model to anticipate
workload bursts and pre-warm CPU/memory resources.

## Model Architecture
- Lightweight LSTM trained on historical shard behaviour
- Runs entirely in a sandboxed inference shard (no GPU required at boot)
- Inference latency < 50 µs on baseline x86_64

## Integration Points
- Feeds scheduling hints to `scheduling/hybrid/`
- Monitors memory pressure and pre-triggers compaction
- Signals VPN shard of expected burst traffic

## Roadmap
- [ ] Feature extraction pipeline (CPU/mem/io counters)
- [ ] Model training harness (offline, on reference hardware)
- [ ] Kernel hook for hint injection
""")

# Self-Healing
write(os.path.join(REPO, "kernel/self_healing/README.md"), """\
# Self-Healing Kernel

Autonomous fault recovery inspired by biological immune systems — when a
kernel shard crashes, the system diagnoses the fault and restarts only the
affected component without rebooting.

## Recovery Flow
```
Fault detected (watchdog / page-fault / assertion)
   ↓
Fault classifier (heuristic + ML)
   ↓
Quarantine faulty shard (revoke capabilities)
   ↓
Clean restart from last good snapshot (SovereignFS)
   ↓
Telemetry report filed to Sovereign Audit Log
```

## Roadmap
- [ ] Watchdog timer integration
- [ ] Snapshot restore from SovereignFS
- [ ] Fault classifier training data collection
""")

print("  [OK] All main-repo READMEs enriched.\n")

# ── 2. Copy → Wiki pages (detailed) ─────────────────────────────────────────
print("[2] Copying enriched docs to wiki...")

COPIES = {
    "pkg/spm/README.md":                    "Sovereign-Package-Manager.md",
    "tools/sigma-cc/README.md":             "SigmaCC-Toolchain.md",
    "fs/sovereign_fs/README.md":            "SovereignFS.md",
    "net/vpn/README.md":                    "Sovereign-VPN.md",
    "net/mesh/README.md":                   "Sovereign-Mesh-Networking.md",
    "net/ipv6/README.md":                   "Sovereign-IPv6.md",
    "net/dns/README.md":                    "Sovereign-DNS.md",
    "tools/compat/README.md":               "POSIX-Compatibility-Shim.md",
    "runtime/wasm/README.md":               "WebAssembly-Runtime.md",
    "scheduling/hybrid/README.md":          "Hybrid-Scheduler.md",
    "security/mac/README.md":               "Mandatory-Access-Control.md",
    "security/identity/README.md":          "Cryptographic-Identity-Manager.md",
    "security/pqc/README.md":               "Post-Quantum-Cryptography-Toolkit.md",
    "virtualization/hypervisor/README.md":  "Bare-Metal-Hypervisor.md",
    "virtualization/containers/README.md":  "Sovereign-Containers.md",
    "core/ai_scheduler/README.md":          "AI-Scheduling-Engine.md",
    "kernel/self_healing/README.md":        "Self-Healing-Kernel.md",
    "drivers/ddk/README.md":               "Driver-Development-Kit.md",
    "hal/README.md":                        "Hardware-Abstraction-Layer.md",
    "graphics/gpu_compute/README.md":       "GPU-Compute-Toolkit.md",
}

for src_rel, wiki_name in COPIES.items():
    src = os.path.join(REPO, src_rel)
    dst = os.path.join(WIKI, wiki_name)
    shutil.copy2(src, dst)
    print(f"  [OK] {src_rel} -> {wiki_name}")

print()

# ── 3. Update _Sidebar.md ────────────────────────────────────────────────────
print("[3] Updating _Sidebar.md...")

SIDEBAR = os.path.join(WIKI, "_Sidebar.md")
with open(SIDEBAR, "r", encoding="utf-8") as f:
    content = f.read()

NEW_SECTION = """
## 🔧 Hardware & System Tooling

- [**Driver Development Kit (DDK)**](Driver-Development-Kit)

- [**Hardware Abstraction Layer (HAL)**](Hardware-Abstraction-Layer)

- [**GPU Compute Toolkit**](GPU-Compute-Toolkit)

- [SovereignFS (Journaling / CoW Filesystem)](SovereignFS)

## 🌐 Sovereign Networking Stack

- [**IPv6 Stack**](Sovereign-IPv6)

- [**VPN (Post-Quantum Tunneling)**](Sovereign-VPN)

- [**Mesh Networking (IoT/Defense)**](Sovereign-Mesh-Networking)

- [**Sovereign DNS Resolver**](Sovereign-DNS)

## 🛠️ Developer Toolchain & Ecosystem

- [**SigmaCC Compiler & Toolchain**](SigmaCC-Toolchain)

- [**Sovereign Package Manager (SPM)**](Sovereign-Package-Manager)

- [**POSIX Compatibility Shim**](POSIX-Compatibility-Shim)

- [**WebAssembly Runtime**](WebAssembly-Runtime)

## 🔒 Security Framework

- [**Mandatory Access Control (MAC)**](Mandatory-Access-Control)

- [**Cryptographic Identity Manager**](Cryptographic-Identity-Manager)

- [**Post-Quantum Cryptography Toolkit**](Post-Quantum-Cryptography-Toolkit)

## ⚡ Performance & Scheduling

- [**Hybrid Scheduler (NUMA / RT / Eco)**](Hybrid-Scheduler)

- [**AI Scheduling Engine**](AI-Scheduling-Engine)

## 🖥️ Virtualization

- [**Bare-Metal Hypervisor**](Bare-Metal-Hypervisor)

- [**Sovereign Containers**](Sovereign-Containers)

## 🚀 Bold Future Directions

- [**Self-Healing Kernel**](Self-Healing-Kernel)

"""

# Insert before existing "## 🚀 Strategic Vision" section
ANCHOR = "## 🚀 Strategic Vision"
if NEW_SECTION.strip() not in content:
    content = content.replace(ANCHOR, NEW_SECTION + ANCHOR)
    with open(SIDEBAR, "w", encoding="utf-8") as f:
        f.write(content)
    print("  [OK] _Sidebar.md updated.")
else:
    print("  [INFO] _Sidebar.md already up-to-date.")

print("\nAll done!")
