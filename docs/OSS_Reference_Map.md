# SigmaOS Open-Source Reference Map

> This document maps every SigmaOS subsystem to the best-of-breed OSS projects
> it draws **architectural inspiration** from.
>
> ⚠️ **Cleanroom rule**: SigmaOS is MIT/BSD licensed. We study interfaces, patterns,
> and ideas from GPL projects — we never copy GPL code. All implementations are
> original. See [CANONICAL_CLEANROOM_ABSORPTION.md](../wiki_repo/CANONICAL_CLEANROOM_ABSORPTION.md).

---

## Kernel & Core

| SigmaOS Subsystem | OSS Reference | What to Study |
|---|---|---|
| Capability ring security | **seL4** (GPLv2 + proofs) | Capability-based access control model, formal verification approach, IPC fastpath design |
| Capability rings | **Fuchsia/Zircon** (BSD/MIT) | Handle table design, capability token passing, process birth/death semantics |
| Syscall gate | **OpenBSD pledge/unveil** (ISC) | Minimal allowlist philosophy, irreversible restriction semantics |
| Formal verification | **seL4 proofs** (Isabelle/HOL) | CAmkES component model, proof-of-correctness methodology for MM + IPC |
| Scheduler | **Linux CFS** (GPLv2) | vruntime + red-black tree concept; implement cleanroom in Rust |
| Scheduler | **FreeBSD ULE** (BSD) | Per-CPU runqueues, CPU affinity, load balancing patterns |
| IPC | **L4 microkernel family** (various) | Synchronous IPC rendezvous, zero-copy shared memory regions |
| Memory management | **OpenBSD mmap** (ISC) | Guard page placement, ASLR entropy design |
| eBPF-equivalent | **Linux eBPF** (GPLv2) | Program verification (DAG check), map types, JIT patterns |
| Self-healing | **Erlang OTP supervisors** (Apache 2) | Supervisor tree restart strategies for shard crash recovery |

---

## Package Manager & Build

| SigmaOS Subsystem | OSS Reference | What to Study |
|---|---|---|
| Content-addressed store | **Nix** (LGPL) | `/nix/store/<hash>-name-version` path layout, closure computation |
| Reproducible builds | **Guix** (GPLv3) | Bootstrap chain, grafts for security updates without rebuild |
| Binary cache | **Nix binary cache** | Substituter protocol, narinfo format, signing |
| Generational rollback | **NixOS** (MIT) | `/nix/var/nix/profiles` symlink chain, `nix-env --rollback` |
| Dependency solver | **apt** SAT solver (GPLv2) | DPKG dependency expression language, conflict resolution |
| Build recipes | **Arch PKGBUILD** (GPL) | `pkgver`, `pkgrel`, `build()`, `check()`, `package()` conventions |
| Package signing | **rpm-sign + GPG** (various) | Detached signature format, keyring management |
| Reproducibility checker | **reprotest** (GPLv2) | Variation dimensions: filesystem, timezone, locale, uid |
| Supply chain | **SLSA framework** (Apache 2) | Level 2 provenance: identify build platform + entry point |
| Transparency log | **Sigstore/Rekor** (Apache 2) | Merkle tree transparency log for package attestation |

---

## Graphics & Display

| SigmaOS Subsystem | OSS Reference | What to Study |
|---|---|---|
| Compositor protocol | **wlroots** (MIT) | Scene-graph API, output layout, input routing — mirror the abstraction, not the code |
| Compositor | **Smithay** (MIT, Rust!) | Direct port candidate — Rust Wayland compositor framework |
| KMS/DRM | **Linux DRM atomic** (GPLv2) | Atomic modesetting state machine, CRTC/plane/connector objects |
| GPU driver interface | **Mesa Gallium3D** (MIT) | `pipe_screen` / `pipe_context` abstraction layer design |
| Vulkan ICD | **Mesa radv / anv** (MIT) | ICD loader interface, Vulkan object lifetime model |
| Font rendering | **FreeType** (FTL/GPLv2) | Glyph rasterisation pipeline — use as interface reference |
| Text layout | **HarfBuzz** (MIT) | Unicode shaping pipeline — cleanroom shaping algorithm |
| Display scaling | **elementary HiDPI** (GPL) | Logical vs physical pixel abstraction |

---

## Networking

| SigmaOS Subsystem | OSS Reference | What to Study |
|---|---|---|
| TCP/IP stack | **lwIP** (BSD) | Zero-copy pbuf chain, netif abstraction — good for embedded profile |
| TCP/IP stack | **smoltcp** (MIT, Rust!) | Direct inspiration candidate — pure Rust, `no_std` TCP/IP |
| DNS/DoH | **Unbound** (BSD) | DNSSEC validation chain, DoH/DoT client model |
| WireGuard | **WireGuard** (GPLv2 kernel, MIT userspace tools) | Cryptokey routing concept, handshake state machine (MIT parts only) |
| Zero-copy DMA | **DPDK** (BSD) | Ring buffer design, DMA mapping patterns for high-throughput NIC |
| CRDT sync | **Automerge** (MIT) | JSON CRDT data model for offline-first state |
| Service mesh | **SPIFFE** (Apache 2) | SVID (SPIFFE Verifiable Identity Document) format + X.509 SVID |
| Network namespaces | **Linux netns** (GPLv2) | Interface for per-process network isolation — cleanroom equivalent |

---

## Filesystem

| SigmaOS Subsystem | OSS Reference | What to Study |
|---|---|---|
| CoW filesystem | **btrfs** (GPLv2) | B-tree on-disk format concept, snapshot/subvolume model |
| CoW filesystem | **ZFS / OpenZFS** (CDDL/CDDL) | Dataset model, send/receive incremental snapshots |
| Atomic updates | **OSTree** (LGPLv2) | Composing OS images as immutable git-like objects |
| VFS layer | **9P/Plan 9** (LPL) | Clean file server protocol — minimal VFS interface design |
| Journaling | **JBD2** (GPLv2 concept) | Ordered vs writeback journaling modes |
| FUSE equivalent | **libfuse** (LGPLv2) | Kernel ↔ userspace filesystem protocol design |
| Encrypted FS | **gocryptfs** (MIT) | Per-file AES-GCM encryption with authenticated name encryption |

---

## Security

| SigmaOS Subsystem | OSS Reference | What to Study |
|---|---|---|
| PQC cryptography | **liboqs** (MIT) | Reference vectors + test suites for Kyber-1024 / Dilithium-5 — use for correctness auditing only |
| Secure boot | **systemd-boot** (LGPLv2) | EFI stub loader design, UEFI secure boot chain |
| SELinux-style MAC | **SELinux** (GPLv2) | AVC object class / permission model — cleanroom policy engine |
| Sandboxing | **Landlock** (GPLv2) | Composable LSM rules for filesystem restriction |
| Reproducible builds | **reproducible-builds.org** (various) | SOURCE_DATE_EPOCH, strip-deterministic, normalise ar archives |
| WASM isolation | **Wasmtime** (Apache 2) | WASI capability model, component model boundary semantics |
| WASM isolation | **WasmEdge** (Apache 2) | WASI-NN neural network interface — reference for sigma-ai WASM |
| TPM | **tpm2-tools** (BSD) | ESAPI session management, PCR sealing/unsealing patterns |
| Formal verification | **Coq stdlib** (LGPLv2) | Proof structure for memory safety invariants |

---

## Linux ELF Compatibility Layer

| SigmaOS Subsystem | OSS Reference | What to Study |
|---|---|---|
| Syscall translation | **FreeBSD Linuxulator** (BSD) | Syscall number remapping table design, `linux_syscall_set` |
| Syscall translation | **Darling** (GPL — study only) | macOS ↔ Linux syscall translation patterns |
| ELF loader | **musl libc** (MIT) | ELF dynamic linker internals, TLS model, `dl_iterate_phdr` |
| ABI shim | **WINE** (LGPL) | PE/ELF dual-mode loader, thunking layer design |

---

## AI / ML

| SigmaOS Subsystem | OSS Reference | What to Study |
|---|---|---|
| On-device inference | **llama.cpp** (MIT) | GGUF quantised model format, GGML tensor operations |
| On-device inference | **ONNX Runtime** (MIT) | Execution provider abstraction (CPU/GPU/NPU) |
| NPU HAL | **Linux accel subsystem** (GPLv2) | `DRM_ACCEL_*` IOCTL interface — mirror for sigma NPU class |
| Model packaging | **Hugging Face Hub** (Apache 2) | Model card format, safetensors format |
| Inference server | **Ollama** (MIT) | Model serving API — cleanroom reference for sigma-ai API |
| Neural UI | **Core ML** (Apple proprietary) | On-device adaptive layout concept only |

---

## Containers & Cloud

| SigmaOS Subsystem | OSS Reference | What to Study |
|---|---|---|
| MicroVM | **Firecracker** (Apache 2) | Jailer + VMM architecture, REST API, minimal device model |
| Syscall sandbox | **gVisor** (Apache 2) | Sentry (Go kernel), Gofer (filesystem proxy), syscall interception |
| Container runtime | **kata-containers** (Apache 2) | Hypervisor-based container model, agent protocol |
| Image format | **OCI Image Spec** (Apache 2) | Layer diff format, image manifest, index JSON |
| Orchestration | **Kubernetes API** (Apache 2) | Pod spec, control loop pattern, declarative reconciliation |
| Cloud-init | **cloud-init** (Apache 2/GPLv3) | Datasource abstraction, user-data format, modules |

---

## Installer & Recovery

| SigmaOS Subsystem | OSS Reference | What to Study |
|---|---|---|
| Text/GUI installer | **Alpine setup-alpine** (MIT) | Minimal question-answer installer pattern |
| Declarative install | **NixOS installer** (MIT) | Configuration.nix → system activation |
| Partitioning | **libparted** (GPLv3 — study only) | Partition table models (GPT, MBR), geometry abstraction |
| Recovery | **Rescuezilla** (GPLv3 — study only) | Live rescue environment boot flow |
| Rollback | **OSTree** (LGPLv2) | Generational deployment model |

---

## Notes on GPL Compliance

1. **Never copy GPL code** into SigmaOS source files — SigmaOS is MIT/BSD.
2. **Ideas and interfaces are not copyrightable** — studying a GPL project's design is legal.
3. **LGPL libraries** (libparted, wlroots, Smithay parts) may be dynamically linked by userland apps with proper attribution — check per-library terms.
4. **Cleanroom process**: document what you studied, implement independently, do not reference GPL source during implementation.
5. See [CANONICAL_CLEANROOM_ABSORPTION.md](../wiki_repo/CANONICAL_CLEANROOM_ABSORPTION.md) for the full cleanroom process.

---

*See also: [docs/Open_Source_Drivers.md](Open_Source_Drivers.md) · [CONTRIBUTING.md](../CONTRIBUTING.md) · [LICENCE.md](../wiki_repo/LICENCE.md)*
