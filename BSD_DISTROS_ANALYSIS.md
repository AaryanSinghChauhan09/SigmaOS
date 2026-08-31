# BSD Distributions Analysis for SigmaOS

> Deep analysis of FreeBSD, OpenBSD, NetBSD, and DragonFlyBSD features,
> innovations, and implementation techniques that SigmaOS can adopt.

***

## Table of Contents

1.  [BSD Family Overview](#bsd-family-overview)
2.  [FreeBSD](#freebsd)
3.  [OpenBSD](#openbsd)
4.  [NetBSD](#netbsd)
5.  [DragonFlyBSD](#dragonflybsd)
6.  [Feature Adoption Matrix](#feature-adoption-matrix)
7.  [Implementation Priorities](#implementation-priorities)

***

## BSD Family Overview

The BSD (Berkeley Software Distribution) systems share a common heritage from
the original Unix source code but have diverged significantly in focus:

| System | Philosophy | Key Strength |
|--------|-----------|-------------|
| FreeBSD | Production server OS | Stability, ZFS, Jails |
| OpenBSD | Security-first | Pledge/Unveil, W^X, ASLR |
| NetBSD | Maximum portability | Runs on >70 platforms |
| DragonFlyBSD | SMP performance | HAMMER2, fine-grained locking |

All four use the MIT/BSD license, making code study and algorithm adoption
straightforward.

***

## FreeBSD

### Overview

FreeBSD is the most widely deployed BSD. It powers PlayStation 4/5, Nintendo
Switch, Netflix's CDN, WhatsApp's servers, and is the basis for macOS/iOS.

### Key Innovations for SigmaOS

#### 1. Jails (FreeBSD 4.0, 2000)

The original lightweight virtualisation mechanism. Each jail has:

*   Private root filesystem
*   Private network stack (optional)
*   Private hostname
*   Process isolation (can't see processes outside jail)
*   `jail_attach()` to enter from outside

**SigmaOS adoption:** Full – `src/virtualization/container.rs`
**Extensions added:**

*   Nested jails (jail-in-jail)
*   Per-jail CPU quota via cgroups
*   OCI container runtime built on jails

#### 2. Capsicum Capability Mode

Processes enter capability mode (`cap_enter()`); all subsequent operations use
capabilities (open file descriptors + permissions) rather than global names.
This is more powerful than POSIX capabilities because it limits ambient authority.

**SigmaOS adoption:** Full – `src/security/capability.rs`
**Extension:** Capability tokens are cryptographically signed, allowing delegation
between processes.

#### 3. ZFS Port

FreeBSD ported Sun's ZFS in 2007, years before it appeared in Linux. ZFS provides:

*   Copy-on-write
*   End-to-end checksums (detects silent data corruption)
*   RAID-Z (software RAID without a RAID controller)
*   Snapshots (instant, space-efficient)
*   Compression (LZ4, ZSTD, GZIP)
*   Deduplication
*   Send/receive for replication

**SigmaOS adoption:** Partial – `src/filesystem/cow_snapshot.rs` implements CoW
snapshots. Full ZFS-compatible implementation is in the roadmap.
**Algorithm adopted:** BLAKE3 for block checksums (faster than ZFS's SHA-256/Fletcher-4)

#### 4. kqueue Event System

`kqueue()` provides a unified event notification system for:

*   File descriptor I/O readiness
*   File system events
*   Process events
*   Signal delivery
*   Timer events

**SigmaOS adoption:** `src/event/mod.rs` + `src/kernel/ipc.rs`
The SigmaOS async runtime uses a kqueue-inspired event loop.

#### 5. FreeBSD's Network Stack

FreeBSD's network stack is the gold standard for production performance:

*   Very fine-grained locking (per-connection mutexes)
*   NUMA-aware socket buffers
*   Zero-copy sendfile
*   TCP offload engine (TOE) support

**SigmaOS adoption:** `src/net/tcpip_stack.rs` – implemented from scratch with
these design principles.

#### 6. pkg Package Manager

FreeBSD's `pkg` is a binary package manager with:

*   Remote package repositories
*   Dependency resolution
*   Atomic transactions
*   Conflict detection
*   Query interface (what package provides a file?)

**SigmaOS adoption:** `sigpkg` borrows pkg's transaction model.

#### 7. DTrace

DTrace is a dynamic tracing framework (originally from Solaris) that FreeBSD
ported. Key capabilities:

*   Probe points throughout kernel and userspace
*   D language for probe specifications
*   Minimal overhead when probes not active

**SigmaOS adoption (planned):** `src/tracing/sigma_trace.rs` – SigmaTrace is the
SigmaOS equivalent, using eBPF-inspired probes.

#### 8. LinuxKPI Compatibility Layer

FreeBSD can run Linux kernel modules through LinuxKPI. SigmaOS's equivalent is
the DDE (Driver Development Environment) in `src/drivers/dde.rs`.

***

## OpenBSD

### Overview

OpenBSD's mantra is "Only two remote holes in the default install, in a heck
of a long time." It is the most security-hardened general-purpose OS in existence.

### Key Innovations for SigmaOS

#### 1. W^X (Write XOR Execute)

Pages are either writable or executable, never both. This prevents
code-injection attacks. W^X is enforced at the hardware level (NX bit).

**SigmaOS adoption:** Enforced in `src/kernel/paging.rs` – no page can be
W+X simultaneously. The compiler chain (`src/toolchain/`) never produces W+X
segments.

#### 2. ASLR (Address Space Layout Randomisation)

OpenBSD has the most aggressive ASLR in any OS:

*   Stack randomisation: 128-bit entropy
*   Heap randomisation: 64-bit entropy
*   Library base: randomised on every load
*   Text segment: randomised (requires PIE)
*   mmap: fully randomised

**SigmaOS adoption:** `src/kernel/paging.rs` – 128-bit ASLR for stack,
64-bit for heap, randomised mmap base.

#### 3. `pledge(2)` System Call

A process declares which groups of system calls it needs. Violations are fatal.
Groups include: `stdio`, `rpath`, `wpath`, `exec`, `inet`, `dns`, `tty`, etc.

This is one of the most elegant security mechanisms in any OS.

**SigmaOS adoption:** Full – `src/security/sigma_pledge.rs`
**Extensions:**

*   Compile-time pledge verification via proc macro
*   Pledge logging (record all invocations)
*   Per-thread pledge (sub-pledge for worker threads)

#### 4. `unveil(2)` System Call

Narrows the visible filesystem. Any path not unveiled is invisible (ENOENT).

**SigmaOS adoption:** Full – `src/security/sigma_unveil.rs`
**Extensions:**

*   Regex-based unveil paths
*   Recursive unveil with depth limit
*   Unveil inheritance control for child processes

#### 5. OpenBSD Malloc (`malloc(3)`)

OpenBSD's allocator is security-hardened:

*   Pages returned to OS when freed (prevents use-after-free exploitation)
*   Guard pages between allocations
*   Randomised allocation addresses
*   Chunk headers stored separately from data (prevents heap metadata overwrites)

**SigmaOS adoption:** `src/klib/custom_allocator.rs` includes guard pages and
separate metadata storage. The buddy allocator has randomised chunk selection.

#### 6. `securelevel(7)`

System-wide security level from 0 (insecure) to 2 (highly secure):

*   Level 1: No raw disk/memory access, no kernel patches at runtime
*   Level 2: Also no reducing file flags

**SigmaOS adoption:** Full – `src/security/securelevels.rs` with levels 0–3.

#### 7. LibreSSL

OpenBSD forked OpenSSL to create LibreSSL (cleaner, more auditable).
The key lesson: sometimes the right answer is to rewrite from scratch.

**SigmaOS lesson applied:** SigmaOS implements its own cryptographic primitives
in `src/kernel/crypto/mod.rs` rather than linking to any external crypto library.

#### 8. OpenBSD Ports Tree

OpenBSD's ports tree has extremely strict quality control. Every port must:

*   Build reproducibly
*   Have no known CVEs
*   Not run unnecessary services

**SigmaOS adoption:** `sigpkg` has a quality gate in `src/sigpkg/verifier.rs`.

***

## NetBSD

### Overview

NetBSD's slogan is "Of course it runs NetBSD." It supports the most hardware
platforms of any OS: 70+ architectures including VAX, sun2, shark, and many
embedded platforms.

### Key Innovations for SigmaOS

#### 1. Cross-Platform HAL

NetBSD achieves portability through a strict HAL (machine-dependent / machine-
independent split). Every architecture provides the same `machine/...` headers.

**SigmaOS adoption:** `src/arch/hal.rs` – clean HAL following NetBSD's model.
Currently: x86-64, aarch64. Target: RISC-V, MIPS, PowerPC.

#### 2. Rump Kernels

Rump kernels allow NetBSD kernel subsystems to run as userspace libraries.
This enables testing kernel code without rebooting or emulation overhead.

**SigmaOS adoption:** Every kernel module can be compiled with `feature = "rump"`
to run in userspace. Used extensively in `tests/integration_test.rs`.

#### 3. pkgsrc

NetBSD's package system works on any Unix-like OS. It's more portable than
Homebrew, apt, or pacman.

**SigmaOS adoption concept:** `sigpkg` is designed to be portable (run on Linux
to cross-build SigmaOS packages).

#### 4. NetBSD's Exploit Mitigations

NetBSD's `uvm` (Unified Virtual Memory) has:

*   PAXASLR – ASLR with high entropy
*   PAXMPROTECT – W^X enforcement
*   PAXSEGVGUARD – segfault guard pages
*   Stack-smashing protection

**SigmaOS adoption:** All mitigations adopted in `src/kernel/paging.rs` and
`src/klib/uvm.rs`.

#### 5. `audio(4)` Framework

NetBSD has a clean audio driver framework that separates hardware-specific code
from the generic audio pipeline.

**SigmaOS adoption concept:** `src/audio/mod.rs` follows this architecture.

***

## DragonFlyBSD

### Overview

DragonFlyBSD forked from FreeBSD 4.8 in 2003. Its focus is on SMP performance
via fine-grained locking and message-passing.

### Key Innovations for SigmaOS

#### 1. HAMMER2 Filesystem

HAMMER2 is DragonFlyBSD's flagship filesystem:

*   **Multi-master clustering:** Multiple machines can have writable copies
*   **PFS (pseudo-filesystems):** Per-user, per-jail, per-snapshot namespaces
*   **Deduplication:** Content-addressed block storage
*   **Snapshots:** Instant, space-efficient, recursive
*   **Data encoding:** Compression + checksum at the block level

**SigmaOS adoption:** `src/filesystem/sigma_fs.rs` – SigmaFS implements:

*   PFS namespaces
*   BLAKE3-based deduplication
*   CoW snapshots
*   Compression (LZ4 default, ZSTD optional)

#### 2. LWKT (LightWeight Kernel Threads)

DragonFlyBSD moved from coarse-grained SMP locking to a message-passing model
between CPUs. Each CPU runs its own kernel thread scheduler.

**SigmaOS adoption:** `src/kernel/ipc.rs` – each kernel subsystem has a message
queue. Hot paths (interrupt handlers) are lock-free using ring buffers.

#### 3. `vkernel` (Virtual Kernel)

A `vkernel` is a DragonFlyBSD kernel running in userspace as a process. This
enables safe kernel development without bare-metal hardware.

**SigmaOS adoption:** The QEMU smoke test (`scripts/qemu_smoke_test.py`) provides
an equivalent capability.

#### 4. Slab Allocator

DragonFlyBSD's KMALLOC uses a per-CPU slab allocator for small objects,
eliminating contention on the global allocator lock.

**SigmaOS adoption:** `src/kernel/slab_allocator.rs` – per-CPU slab allocator
for objects < 2 KB. Objects > 2 KB use the buddy allocator.

#### 5. `dsynth` Build System

DragonFlyBSD's `dsynth` builds packages in parallel across multiple CPUs using
process isolation. Each package build is isolated in its own filesystem namespace.

**SigmaOS adoption:** `src/buildfarm/mod.rs` – SigmaBuildFarm parallelises
package builds using jails.

***

## Feature Adoption Matrix

| Feature | FreeBSD | OpenBSD | NetBSD | DragonFlyBSD | SigmaOS Status |
|---------|---------|---------|--------|-------------|----------------|
| Jails / Capsicum | ✅ | - | - | - | ✅ Implemented |
| pledge / unveil | - | ✅ | - | - | ✅ Implemented |
| W^X | ⚠️ | ✅ | ✅ | ✅ | ✅ Implemented |
| ASLR | ⚠️ | ✅ | ✅ | ✅ | ✅ Implemented |
| ZFS / CoW FS | ✅ | - | - | HAMMER2 | ⚠️ Partial |
| Slab allocator | ✅ | ✅ | ✅ | ✅ | ✅ Implemented |
| Fine-grained locking | ✅ | - | - | ✅ | ⚠️ Partial |
| kqueue events | ✅ | ✅ | ✅ | ✅ | ✅ Implemented |
| DTrace | ✅ | - | - | - | ⚠️ SigmaTrace |
| Rump kernels | - | - | ✅ | ✅ | ⚠️ Partial |
| Reproducible builds | - | ✅ | - | - | ✅ Implemented |
| Secure malloc | - | ✅ | - | - | ✅ Implemented |
| securelevel | ✅ | ✅ | ✅ | ✅ | ✅ Implemented |
| PFS namespaces | - | - | - | ✅ | ✅ Implemented |
| Deduplication | ZFS | - | - | HAMMER2 | ⚠️ Partial |
| pkg manager quality | ✅ | ✅ | ✅ | ✅ | ⚠️ In progress |
| Driver compatibility | ✅ | - | ✅ | - | ⚠️ DDE partial |

***

## Implementation Priorities

### Priority 1 (Current Sprint)

1.  Complete ZFS-compatible send/receive in SigmaFS
2.  Add per-thread pledge support
3.  Implement regex-based unveil paths

### Priority 2 (Next Quarter)

1.  Full DTrace-compatible tracing (SigmaTrace expansion)
2.  Rump kernel mode for all kernel modules
3.  Per-CPU slab allocator for all size classes

### Priority 3 (6-Month Horizon)

1.  HAMMER2-compatible clustering protocol
2.  Complete capsicum capability delegation
3.  RISC-V port (inspired by NetBSD's portability work)

***

*Last updated: 2026-08-04*
