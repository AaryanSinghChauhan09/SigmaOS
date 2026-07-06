"""
implement_all_md.py
SigmaOS: Write full rich documentation for all stub/broken .md files.
Run from repo root: py scripts/maintenance/implement_all_md.py
"""
import os, textwrap

REPO = os.path.dirname(os.path.abspath(__file__))
# go up 2 dirs: scripts/maintenance/ → repo root
REPO = os.path.dirname(os.path.dirname(REPO))

def w(rel, text):
    path = os.path.join(REPO, rel.replace('/', os.sep))
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, 'w', encoding='utf-8', newline='\r\n') as f:
        f.write(textwrap.dedent(text).lstrip())
    print(f"  [OK] {rel}")

print("\n=== SigmaOS: Implementing All .md Files ===\n")

# ─── init/README.md ──────────────────────────────────────────────────────────
w("init/README.md", """\
# SigmaOS Init System (`sigma-init`)

This module implements **PID 1** — the first process spawned by the kernel after
boot, responsible for orchestrating every subsequent subsystem start-up.

## Runlevels & Initialization Flow

The boot sequence is structured in 5 key phases:

1. **Core Kernel Bootstrap**: Sets up process tables, memory pagers, and the
   Round-Robin / EDF Scheduler.
2. **HAL & Vitals**: Detects bare-metal hardware (PCIe, USB, ACPI) and spins up
   telemetry metrics.
3. **VFS & Storage Mounting**: Mounts the Virtual File System (VFS) and loads the
   `ext4` / `SovereignFS` filesystem drivers.
4. **Network Stack**: Spins up the network interface loopback and prepares
   standard socket connections.
5. **Userland Handoff**: Spawns the CLI shell (`/usr/bin/sh`) to transition
   execution to user space.

## Services Lifecycle

Services are declared in a global registry table and started dynamically based
on designated runlevels.

```c
typedef enum {
    SERVICE_STOPPED,
    SERVICE_STARTING,
    SERVICE_RUNNING,
    SERVICE_FAILED
} service_state_t;

typedef struct {
    const char    *name;
    service_state_t state;
    int (*start)(void);
    int (*stop)(void);
} sigma_service_t;
```

## Key Entry Points

| Symbol | File | Purpose |
|---|---|---|
| `sigma_init_main()` | `init/sigma_init.c` | PID 1 entry point |
| `sigma_run_level(n)` | `init/runlevel.c` | Transition to runlevel `n` |
| `sigma_service_start(svc)` | `init/service.c` | Start an individual service |
| `sigma_service_stop(svc)` | `init/service.c` | Cleanly stop a service |

## Service Registry

Services register themselves at compile time via a linker section macro:

```c
SIGMA_SERVICE_REGISTER(my_service, .start = my_start, .stop = my_stop);
```

## Roadmap

- [x] Basic 5-phase boot sequence
- [x] Service registry table (compile-time)
- [ ] Dynamic service dependencies (DAG-based ordering)
- [ ] Parallel service startup (topological sort)
- [ ] Service restart policies (always / on-failure / never)
- [ ] Watchdog integration (`modules/core/kernel/watchdog.rs`)
- [ ] Journal-based service logs (`modules/tools/diag/logger.rs`)
""")

# ─── modules/core/drivers/README.md ──────────────────────────────────────────
w("modules/core/drivers/README.md", """\
# Σ core/drivers — Sovereign Hardware Driver Layer

Isolates hardware drivers into **capability-gated loadable shards** so a faulty
driver cannot crash the kernel or corrupt unrelated subsystems.

## Architecture

```
┌──────────────────────────────────────────────────────┐
│                  Kernel Core (Ring 0)                │
├──────────────────────────────────────────────────────┤
│  Driver Registry  ←→  SovereignHAL (ext/hal)         │
│      │                                               │
│   ┌──┴──┐  ┌────┐  ┌─────┐  ┌──────┐  ┌────────┐   │
│   │ PCI │  │ USB│  │ GPU │  │Audio │  │  WiFi  │   │
│   └─────┘  └────┘  └─────┘  └──────┘  └────────┘   │
└──────────────────────────────────────────────────────┘
```

## Source Files

| File | Exported Function | Description |
|---|---|---|
| `audio.rs` | `audio_init()` | HD Audio / AC97 subsystem init |
| `gpu.rs` | `gpu_init()` | GPU framebuffer + DRM/KMS stub |
| `pci.rs` | `pci_init()` | PCIe bus enumeration & BAR mapping |
| `usb.rs` | `usb_init()` | XHCI/EHCI USB host controller |
| `wifi.rs` | `wifi_init()` | 802.11 wireless stack (nl80211-style) |
| `sigma_libc.rs` | — | Shared no-alloc primitives |

## API Interface

All driver shards expose a common C-compatible ABI:

```c
// Register a driver with the SovereignHAL registry
void sigma_register_driver(const char *name, sigma_driver_t *drv);

// Allocate DMA-coherent memory for a driver
void *sigma_alloc_dma_region(size_t size);

// Bind an interrupt vector to a handler
int sigma_irq_install(uint32_t vector, void (*handler)(void));

// Core drivers entry points
void pci_init(void);
void usb_init(void);
void gpu_init(void);
void audio_init(void);
void wifi_init(void);
```

## Capability Model

Each driver shard must declare its required capabilities in `module.json`:

```json
{
  "capabilities_required": ["CAP_PCI_ACCESS", "CAP_IRQ_BIND"],
  "capabilities_provided": ["CAP_NIC_DRIVER"]
}
```

## Roadmap

- [x] PCI bus enumeration (`pci.rs`)
- [x] USB XHCI stub (`usb.rs`)
- [x] GPU framebuffer stub (`gpu.rs`)
- [x] Audio HD-Audio stub (`audio.rs`)
- [x] WiFi 802.11 stub (`wifi.rs`)
- [ ] Full DMA ring-buffer implementation (NIC)
- [ ] GPU DRM/KMS mode-setting
- [ ] NVMe storage driver
- [ ] Bluetooth HCI driver
- [ ] Formal DDK API header (`ddk_api.h`)
- [ ] CBMC/Kani safety proofs for DMA paths

## Related Modules

- [`modules/ext/hal`](../../ext/hal/README.md) — Hardware Abstraction Layer
- [`modules/tools/diag`](../../tools/diag/README.md) — Driver diagnostics & tracing
""")

# ─── modules/core/fs/README.md ───────────────────────────────────────────────
w("modules/core/fs/README.md", """\
# Σ core/fs — Sovereign Filesystem Layer

Abstracts storage handling into a **pluggable VFS layer** supporting multiple
filesystem backends without kernel recompilation.

## Architecture

```
User Process
   └─ sigma_vfs_open("/data/file.txt")
         └─ VFS Router (vfs.rs)
               ├─ SigmaFS  (sigmafs.rs)   ← default sovereign FS
               ├─ Ext4     (ext4.rs)       ← Linux compat layer
               ├─ FAT32    (fat32.rs)      ← removable media
               └─ Web3FS   (web3_persistence.rs) ← IPFS-backed
```

## Source Files

| File | Description |
|---|---|
| `vfs.rs` | Virtual Filesystem Switch — routes open/read/write to backends |
| `sigmafs.rs` | SovereignFS: CoW, journaling, BLAKE3 block integrity |
| `ext4.rs` | Read-compatible Ext4 driver (no journal write support yet) |
| `fat32.rs` | FAT32 r/w driver for removable media |
| `self_opt_fs.rs` | AI-driven self-optimising filesystem layout engine |
| `web3_persistence.rs` | IPFS/Filecoin-backed decentralised storage shim |

## API Interface

```c
// Mount a filesystem backend at a path
int sigma_vfs_mount(const char *path, const char *fs_type, uint32_t flags);

// Open a file — returns sovereign file descriptor
int sigma_vfs_open(const char *path, uint32_t flags, uint32_t mode);

// Read / Write — zero-copy where supported
ssize_t sigma_vfs_read(int fd, void *buf, size_t len);
ssize_t sigma_vfs_write(int fd, const void *buf, size_t len);

// Snapshot the current FS state (SovereignFS only)
int sigma_vfs_snapshot(const char *tag);

// Rollback to a named snapshot
int sigma_vfs_rollback(const char *tag);
```

## SovereignFS On-Disk Layout

```
[Superblock 4K] [Journal 64MB] [Inode Table] [Data Extents ...]
```

- **Copy-on-Write:** every write creates a new extent; old data preserved until
  explicitly pruned — enables instant snapshots.
- **Block Integrity:** each 4 KB block carries a BLAKE3 checksum; the kernel
  rejects tampered blocks at read time.

## Roadmap

- [x] VFS router (`vfs.rs`)
- [x] SovereignFS basic CoW layout (`sigmafs.rs`)
- [x] Ext4 read-only driver (`ext4.rs`)
- [x] FAT32 r/w driver (`fat32.rs`)
- [ ] SovereignFS journal format spec
- [ ] `sfs_mkfs` userland tool
- [ ] SPARK formal proofs for journal replay
- [ ] OverlayFS shim for container layers
- [ ] NVMe queue-depth optimisation in VFS

## Related Modules

- [`modules/core/kernel`](../kernel/README.md) — Kernel memory management
- [`modules/security/isolation`](../../security/isolation/README.md) — FS namespace isolation
""")

# ─── modules/core/kernel/README.md ───────────────────────────────────────────
w("modules/core/kernel/README.md", """\
# Σ core/kernel — Sovereign Microkernel Core

Minimal kernel handling **scheduling, memory management, IPC, syscalls, and
watchdog supervision** — structured as composable sovereign shards rather than a
monolithic blob.

## Architecture

```
              ┌─────────────────────────────────┐
              │          kernel_main.rs          │
              │  (boot → init subsystems → idle) │
              └───────────┬─────────────────────┘
        ┌─────────────────┼──────────────────────┐
     Scheduler         Memory               Security
   (scheduling/)     (memory/)            (security/)
        │                │                     │
      ipc.rs         res_alloc.rs        syscalls.rs
   interrupts.rs     rollback_manager    watchdog.rs
   kernel.rs         self_heal.rs        audit_shard.rs
```

## Key Source Files

| File | Description |
|---|---|
| `kernel.rs` / `kernel_main.rs` | Boot entry — wires all sovereign subsystems |
| `init.rs` | Early hardware init (GDT, IDT, TSS) |
| `interrupts.rs` | IRQ/exception handler table |
| `ipc.rs` | Zero-copy inter-shard message passing |
| `syscalls.rs` | Sovereign syscall dispatcher (non-POSIX ABI) |
| `res_alloc.rs` | Deterministic resource allocator |
| `res_alloc_ai.rs` | AI-assisted resource allocation hints |
| `watchdog.rs` | Hardware-backed watchdog timer integration |
| `self_heal.rs` | Autonomous shard fault detection & restart |
| `rollback_manager.rs` | Snapshot-based kernel state rollback |
| `audit_shard.rs` | Immutable audit log for all syscalls |
| `elf_loader.rs` | ELF binary loader for user-space processes |

## API Interface

```rust
// Kernel entry point (called from bootloader)
#[no_mangle]
pub unsafe extern "C" fn kernel_main() { ... }

// Spawn a new sovereign shard
pub fn shard_spawn(name: &str, caps: &[Capability]) -> ShardId;

// Send a message to another shard (zero-copy)
pub fn shard_send(dst: ShardId, msg: &SovereignMsg) -> KernelResult;

// Syscall dispatcher
#[no_mangle]
pub unsafe extern "C" fn sigma_syscall(nr: u64, args: *const u64) -> i64;
```

## Capability System

Every shard is granted a minimal capability set at spawn time:

```json
{
  "capabilities_required": ["CAP_KERNEL_ROOT"],
  "capabilities_provided": ["CAP_BOOT_CONTEXT"],
  "entry_point": "kernel_main"
}
```

## Roadmap

- [x] IRQ handler table (`interrupts.rs`)
- [x] Zero-copy IPC (`ipc.rs`)
- [x] Syscall dispatcher (`syscalls.rs`)
- [x] Watchdog timer (`watchdog.rs`)
- [x] Self-healing shard restart (`self_heal.rs`)
- [x] AI resource allocation hints (`res_alloc_ai.rs`)
- [ ] Full NUMA-aware memory allocator
- [ ] Formal Kani proofs for IPC non-interference
- [ ] Live kernel patching (hot-patch without reboot)
- [ ] Microkernel split: move drivers fully out of Ring 0

## Sub-Directories

- [`memory/`](memory/) — Paging, slab allocator, NUMA topology
- [`scheduling/`](scheduling/) — Round-Robin, EDF, real-time lanes
- [`security/`](security/) — Capability enforcement hooks
- [`syscalls/`](syscalls/) — Per-syscall implementation shards
- [`hypervisor/`](hypervisor/) — Type-1 hypervisor (VT-x / AMD-V)

## Related Modules

- [`modules/core/drivers`](../drivers/README.md) — Hardware drivers
- [`modules/core/net`](../net/README.md) — Network stack
- [`modules/tools/diag`](../../tools/diag/README.md) — Kernel diagnostics
""")

# ─── modules/core/net/README.md ──────────────────────────────────────────────
w("modules/core/net/README.md", """\
# Σ core/net — Sovereign Networking Stack

Standalone networking subsystem with **no dependency on the Linux networking
stack**. Supports classical TCP/IP, post-quantum encrypted channels, and
experimental galactic-scale mesh routing.

## Architecture

```
Application Layer
   └─ socket.rs          (POSIX-style socket API)
         └─ tcpip.rs     (TCP/IP stack)
               ├─ tcp.rs  (TCP state machine)
               ├─ icmp.rs (ICMP echo / error)
               └─ sovereign_net.rs  (SovereignNet overlay)
                     ├─ pqfs.rs       (post-quantum forward secrecy)
                     ├─ mesh_net.rs   (local mesh)
                     └─ galactic_*.rs (long-range mesh routing)
```

## Source Files

| File | Description |
|---|---|
| `socket.rs` | POSIX-compatible socket API (`create`, `bind`, `connect`, `send`, `recv`) |
| `tcp.rs` | TCP state machine (SYN→ESTABLISHED→FIN) |
| `tcpip.rs` | IPv4/IPv6 dual-stack with ARP/NDP |
| `icmp.rs` | ICMP ping / unreachable / redirect |
| `sovereign_net.rs` | Encrypted overlay — wraps TCP frames with ChaCha20-Poly1305 |
| `pqfs.rs` | Post-Quantum Forward Secrecy (Kyber-768 + X25519 hybrid) |
| `mesh_net.rs` | Local gossip-based mesh (IoT / edge) |
| `shard_sync.rs` | Distributed shard-state consensus over the network |
| `consensus.rs` | Byzantine-fault-tolerant consensus primitive |
| `galactic_mesh.rs` | High-latency interplanetary routing layer |

## API Interface

```c
// Create a sovereign socket
int sigma_net_socket(int domain, int type, int protocol);

// Connect to a remote host (blocks until handshake complete)
int sigma_net_connect(int sockfd, const struct sigma_addr *addr);

// Send data — zero-copy if page-aligned
ssize_t sigma_net_send(int sockfd, const void *buf, size_t len, int flags);

// Enable post-quantum encryption on a socket
int sigma_net_enable_pqe(int sockfd, pq_key_pair_t *keys);

// Sovereign init
void init_core_net(void);
```

## Post-Quantum Encryption

All `sovereign_net.rs` channels use a **hybrid** scheme:

| Layer | Algorithm | Standard |
|---|---|---|
| Key Exchange | X25519 + Kyber-768 | NIST ML-KEM |
| Encryption | ChaCha20-Poly1305 | RFC 8439 |
| Integrity | BLAKE3 MAC | — |

## Roadmap

- [x] TCP state machine (`tcp.rs`)
- [x] Socket API (`socket.rs`)
- [x] ICMP (`icmp.rs`)
- [x] Post-quantum encryption stub (`pqfs.rs`)
- [x] Mesh gossip protocol (`mesh_net.rs`)
- [ ] Full IPv6 SLAAC / NDP
- [ ] DHCPv4 / DHCPv6 client
- [ ] DNSSEC resolver integration
- [ ] WireGuard-inspired VPN tunnel
- [ ] Formal Kani proofs for TCP state machine

## Related Modules

- [`modules/core/kernel`](../kernel/README.md) — IPC used by net stack
- [`modules/security/isolation`](../../security/isolation/README.md) — Network namespace isolation
""")

# ─── modules/ext/hal/README.md ───────────────────────────────────────────────
w("modules/ext/hal/README.md", """\
# Σ ext/hal — Hardware Abstraction Layer (SovereignHAL)

Provides a **single, architecture-agnostic interface** that the kernel uses to
communicate with hardware. Porting SigmaOS to a new CPU means implementing one
HAL backend — nothing else needs to change.

## Source Files

| File | Description |
|---|---|
| `hal.rs` | Core HAL trait definitions and dispatch table |
| `hw_detect.rs` | Runtime CPU / ACPI / DTB hardware discovery |
| `accel_hal.rs` | Hardware accelerator HAL (GPU compute / NPU / DSP) |

## Supported Targets

| Architecture | Status |
|---|---|
| x86_64 | ✅ Active |
| AArch64 (ARM64) | 🔧 In-progress |
| RISC-V RV64GC | 📋 Planned |

## API Interface

```c
// Initialise the HAL for the detected platform
void hal_init(void);

// Bind an interrupt vector to a kernel handler
void hal_set_irq_handler(uint32_t vec, void (*fn)(void));

// Flush the CPU TLB (all cores)
void hal_flush_tlb(void);

// Read a nanosecond-precision hardware timestamp
uint64_t hal_get_timestamp_ns(void);

// Map a physical address range into kernel virtual space
void *hal_map_phys(uint64_t phys, size_t size, uint32_t flags);

// Detect platform (returns SIGMA_ARCH_X86_64 / AARCH64 / RISCV64)
sigma_arch_t hal_detect_arch(void);
```

## Hardware Discovery

`hw_detect.rs` interrogates ACPI RSDP / MADT / SRAT on x86_64 and the
Flattened Device Tree on ARM/RISC-V to build a unified topology map:

```
ACPI RSDP → XSDT → MADT   (interrupt routing)
                  → SRAT   (NUMA node topology)
                  → MCFG   (PCIe ECAM base)
```

## Roadmap

- [x] x86_64 HAL backend (TSC, LAPIC, IOAPIC)
- [x] Hardware discovery (`hw_detect.rs`)
- [x] Accelerator HAL stub (`accel_hal.rs`)
- [ ] AArch64 GIC-v3 interrupt controller
- [ ] RISC-V PLIC / CLINT integration
- [ ] ACPI Power Management (S3/S4 sleep states)
- [ ] Secure Enclave HAL (SGX / TrustZone)

## Related Modules

- [`modules/core/drivers`](../../core/drivers/README.md) — Drivers that use HAL
- [`modules/core/kernel`](../../core/kernel/README.md) — Kernel that drives HAL
""")

# ─── modules/ext/plugins/README.md ───────────────────────────────────────────
w("modules/ext/plugins/README.md", """\
# Σ ext/plugins — Sovereign Plugin & Extension Framework

Allows third-party capabilities to be injected into SigmaOS as **cryptographically
verified, capability-gated capsules** without rebuilding the kernel.

## Source Files

| File | Description |
|---|---|
| `extension_api.rs` | Public trait definitions for all plugin types |
| `capsule.rs` | Capsule packaging: sign, verify, load, unload |
| `auto_driver_builder.rs` | AI-assisted driver scaffolding from hardware IDs |
| `policy_modules.rs` | Runtime policy injection (MAC rules, firewall) |

## Plugin Capsule Format

A capsule is a signed tar-like archive:

```
capsule.shard
├── manifest.json   # name, version, capabilities, author, signature
├── code.wasm       # or code.elf (sovereign ELF)
└── policy.sigma    # optional MAC policy additions
```

## Extension API

```rust
pub trait SigmaPlugin: Send + Sync {
    fn name(&self) -> &'static str;
    fn init(&mut self, ctx: &mut PluginContext) -> SigmaResult<()>;
    fn shutdown(&mut self) -> SigmaResult<()>;
}

// Register a plugin at runtime
pub fn plugin_register(capsule_path: &str) -> PluginId;

// Query loaded plugins
pub fn plugin_list() -> &[PluginInfo];

// Unload a plugin cleanly
pub fn plugin_unload(id: PluginId) -> SigmaResult<()>;
```

## Security Model

1. Every capsule must carry an **Ed25519 signature** from a key in the Sovereign
   Trust Root.
2. The kernel validates the signature before mapping any code page.
3. Capabilities are enforced at IPC call boundaries — a plugin cannot exceed its
   declared capability set.

## Roadmap

- [x] Extension API trait (`extension_api.rs`)
- [x] Capsule signing & verification (`capsule.rs`)
- [x] AI driver scaffolding stub (`auto_driver_builder.rs`)
- [x] Policy module injection (`policy_modules.rs`)
- [ ] WASM capsule JIT execution (Cranelift backend)
- [ ] Hot-reload (swap capsule version without reboot)
- [ ] Capsule sandboxing via `modules/tools/sandbox`
- [ ] GUI app-store frontend for capsule discovery

## Related Modules

- [`modules/tools/sandbox`](../../tools/sandbox/README.md) — Capsule isolation
- [`modules/security/access_control`](../../security/access_control/README.md) — Capability enforcement
""")

# ─── modules/ext/runtimes/README.md ──────────────────────────────────────────
w("modules/ext/runtimes/README.md", """\
# Σ ext/runtimes — Sovereign Language Runtimes

Hosts sovereign runtimes for languages other than C/Rust that can safely
execute inside SigmaOS user space without requiring the Linux ABI.

## Supported Runtimes

| Runtime | Language | Format | Status |
|---|---|---|---|
| `sigma-wasm` | WebAssembly | `.wasm` | 🔧 In-progress |
| `sigma-luajit` | Lua 5.4 | `.lua` | 📋 Planned |
| `sigma-python` | Python 3.x | `.py` | 📋 Planned |
| `sigma-zig` | Zig | `.zig` ELF | ✅ Native (no shim needed) |

## WebAssembly Runtime (`sigma-wasm`)

SigmaOS's primary portable app format — a sovereign, libc-free WASM interpreter
and JIT that sandboxes apps with hardware capability tokens.

### Execution Model

```
.wasm module
  └─ sigma-wasm validator   (type checking, memory bounds)
        └─ Interpreter      (boot-time, no JIT dependency)
              └─ Cranelift JIT (post-boot, hot path optimisation)
                    └─ Native shard (capability-gated)
```

### WASI Sovereign Mapping

| WASI Syscall | SigmaOS Translation |
|---|---|
| `fd_read` | `sigma_vfs_read()` |
| `fd_write` | `sigma_vfs_write()` |
| `sock_send` | `sigma_net_send()` |
| `clock_time_get` | `hal_get_timestamp_ns()` |

## API Interface

```c
// Load and validate a WASM module
wasm_module_t *sigma_wasm_load(const uint8_t *bytes, size_t len);

// Instantiate with a capability token
wasm_instance_t *sigma_wasm_instantiate(wasm_module_t *m, cap_token_t cap);

// Call an exported function
int64_t sigma_wasm_call(wasm_instance_t *inst, const char *fn, ...);

// Destroy an instance (frees memory)
void sigma_wasm_destroy(wasm_instance_t *inst);
```

## Roadmap

- [ ] WASM binary validator (MVP spec)
- [ ] Stack-machine interpreter (for early boot)
- [ ] Cranelift JIT backend integration
- [ ] WASI → SigmaOS syscall mapping table
- [ ] Lua 5.4 interpreter port (no C stdlib)
- [ ] Python 3 minimal port (for scripting tools)
- [ ] Runtime hot-swap (update runtime without reboot)

## Related Modules

- [`modules/ext/plugins`](../plugins/README.md) — WASM capsule packaging
- [`modules/tools/sandbox`](../../tools/sandbox/README.md) — Runtime sandboxing
""")

# ─── modules/perf/bench/README.md ────────────────────────────────────────────
w("modules/perf/bench/README.md", """\
# Σ perf/bench — Sovereign Benchmarking Suite

Micro- and macro-benchmarks for every critical SigmaOS subsystem. Results are
used to catch performance regressions before they reach `main`.

## Benchmark Categories

| Category | Scope | Key Metric |
|---|---|---|
| `kernel/syscall` | Syscall dispatcher round-trip | ns/call |
| `kernel/ipc` | Zero-copy IPC throughput | GB/s |
| `mm/alloc` | Slab allocator latency | ns/op |
| `mm/page_fault` | Minor page fault handling | µs |
| `fs/read_seq` | Sequential read (SovereignFS) | MB/s |
| `fs/write_rand` | Random write 4K blocks | IOPS |
| `net/tcp_tx` | TCP transmit throughput | Gb/s |
| `scheduler/switch` | Context switch latency | ns |

## Running Benchmarks

```bash
# Run all benchmarks
just bench

# Run only IPC benchmarks
just bench -- kernel/ipc

# Generate flamegraph
just bench-flamegraph

# Compare against baseline
just bench-compare baseline.json
```

## Benchmark Harness

Benchmarks use a minimal no-alloc harness built on `perf_event_open`:

```c
sigma_bench_start("syscall_roundtrip");
for (int i = 0; i < BENCH_ITERS; i++) {
    sigma_syscall(NR_SIGMA_NOOP, NULL);
}
sigma_bench_end();   // prints: "syscall_roundtrip: 120ns/op"
```

## CI Integration

All benchmarks run nightly on bare-metal CI (not in QEMU — timing is not
meaningful in a VM). A regression of > 5% triggers a blocking CI alert.

```yaml
# .github/workflows/bench.yml
on:
  schedule:
    - cron: '0 2 * * *'   # 02:00 UTC nightly
```

## Roadmap

- [ ] Syscall round-trip benchmark
- [ ] IPC throughput benchmark
- [ ] Memory allocator latency suite
- [ ] Filesystem IOPS benchmark
- [ ] Network TCP throughput benchmark
- [ ] Automated regression detection (± 5% threshold)
- [ ] Flamegraph generation pipeline
- [ ] Historical results dashboard (GitHub Pages)

## Related Modules

- [`modules/perf/scheduler`](../scheduler/README.md) — Scheduler performance
- [`modules/perf/mm`](../mm/README.md) — Memory manager benchmarks
""")

# ─── modules/perf/mm/README.md ───────────────────────────────────────────────
w("modules/perf/mm/README.md", """\
# Σ perf/mm — Memory Manager Performance Optimisations

Houses the high-performance memory management implementation for SigmaOS,
including the **buddy allocator**, **slab cache**, and **NUMA-aware page
allocator**.

## Allocator Stack

```
sigma_malloc(size)
   └─ Slab Cache (< 512 bytes, O(1) fixed-size slabs)
         └─ Buddy Allocator (≥ 512 bytes, power-of-two zones)
               └─ NUMA Page Allocator (selects nearest memory node)
                     └─ Physical Frame Allocator (bitmap)
```

## Key Design Points

| Feature | Detail |
|---|---|
| **Allocation** | O(1) slab, O(log N) buddy |
| **NUMA** | Allocates from the NUMA node closest to the requesting CPU |
| **Fragmentation** | Buddy coalescing keeps external fragmentation < 5% |
| **Safety** | Guard pages + canary values detect buffer overflows |
| **No libc** | `#![no_std]` — zero dependency on glibc/musl |

## API Interface

```c
// Allocate `size` bytes (kernel heap)
void *sigma_alloc(size_t size);

// Free a kernel heap pointer
void sigma_free(void *ptr);

// Allocate physically contiguous pages (for DMA)
void *sigma_alloc_pages(size_t order);

// Map a physical range into kernel virtual address space
void *sigma_map_phys(phys_addr_t phys, size_t size);

// Initialise the memory manager (called from kernel_main)
void init_perf_mm(void);
```

## NUMA Topology

SigmaOS discovers NUMA node topology from the ACPI SRAT table and maintains a
per-node free-list:

```
Node 0 (CPU 0-7,  RAM 0–64 GB)
Node 1 (CPU 8-15, RAM 64–128 GB)
```

Allocations prefer the local node; spill to remote only when local is exhausted.

## Roadmap

- [ ] Buddy allocator implementation
- [ ] Slab cache (fixed-size object pools)
- [ ] NUMA-aware page allocator
- [ ] Guard page + canary overflow detection
- [ ] Memory pressure callbacks (OOM handler)
- [ ] Transparent huge pages (THP) support
- [ ] Kani formal proofs: no double-free, no use-after-free

## Related Modules

- [`modules/core/kernel/memory`](../../core/kernel/memory/) — Page-table management
- [`modules/perf/bench`](../bench/README.md) — Memory allocator benchmarks
""")

# ─── modules/perf/scheduler/README.md ────────────────────────────────────────
w("modules/perf/scheduler/README.md", """\
# Σ perf/scheduler — Hybrid Sovereign Scheduler

Extends the base Round-Robin and EDF schedulers with **NUMA awareness,
real-time lanes, energy efficiency, and AI-driven workload prediction**.

## Scheduler Class Hierarchy

```
SovereignScheduler (abstract)
  ├─ RoundRobin     — fair-share for interactive tasks
  ├─ EDF            — Earliest Deadline First for real-time shards
  ├─ RTLane         — hard real-time, preempts all other lanes
  ├─ NUMAFair       — CFS analogue, NUMA-topology aware
  └─ EcoLane        — battery/power-optimised (ARM big.LITTLE)
```

## Scheduling Policy Selection

A shard declares its scheduling class in the spawn request:

```rust
shard_spawn(SpawnRequest {
    name: "audio_daemon",
    sched_class: SchedClass::RTLane { deadline_us: 5_000 },
    cpu_affinity: CpuSet::node(0),
    ..Default::default()
});
```

## AI Prediction Engine

`modules/core/kernel/res_alloc_ai.rs` feeds scheduling hints:

- Lightweight LSTM trained on historical shard CPU/memory patterns
- Runs in a sandboxed inference shard (no GPU required at boot)
- Inference latency < 50 µs on baseline x86_64
- Signals pre-warming of cache lines for known bursty workloads

## API Interface

```c
// Yield the current CPU timeslice
void sigma_sched_yield(void);

// Set real-time deadline for a shard
int sigma_sched_set_rt(sigma_shard_id_t id, uint64_t deadline_us);

// Get CPU usage statistics for a shard
sigma_cpu_stats_t sigma_sched_stats(sigma_shard_id_t id);

// Initialise the scheduler subsystem
void init_perf_scheduler(void);
```

## Context Switch Latency Targets

| Class | Target Latency |
|---|---|
| RTLane | < 5 µs |
| EDF | < 50 µs |
| RoundRobin | < 500 µs |
| EcoLane | Best-effort |

## Roadmap

- [x] Round-Robin base scheduler
- [x] EDF scheduler with deadline enforcement
- [ ] RTLane preemption guarantees (< 5 µs verified)
- [ ] NUMA topology detector integration
- [ ] CPU frequency governor (P-state / DVFS)
- [ ] AI prediction hook from `res_alloc_ai.rs`
- [ ] Formal scheduling analysis (response-time analysis)
- [ ] `schedtool`-compatible CLI for shard priority adjustment

## Related Modules

- [`modules/core/kernel`](../../core/kernel/README.md) — Kernel scheduler host
- [`modules/perf/bench`](../bench/README.md) — Context-switch benchmark
""")

# ─── modules/security/access_control/README.md ───────────────────────────────
w("modules/security/access_control/README.md", """\
# Σ security/access_control — Mandatory Access Control & Audit

Sovereign alternative to SELinux and AppArmor with **deterministic, lattice-based
policy evaluation** and an immutable audit chain.

## Source Files

| File | Description |
|---|---|
| `audit_chain.rs` | Append-only, BLAKE3-chained audit log of all policy decisions |

## Access Control Model

SigmaOS uses a **Lattice-Based Access Control** (LBAC) model:

- Every **subject** (shard) carries a clearance label `{confidentiality, integrity}`.
- Every **object** (file, socket, IPC endpoint) carries a sensitivity label.
- Operations are permitted only when the lattice partial order is satisfied.

```
ALLOW browser_shard READ  /media          # read public data
DENY  browser_shard ANY   /sys            # no kernel inspection
ALLOW ssh_daemon    BIND  net:22          # bind privileged port
```

## Audit Chain

Every policy decision is appended to a BLAKE3-linked chain:

```
Entry N: { timestamp, subject, object, action, decision, hash(Entry N-1) }
```

This makes the audit log **tamper-evident** — any modification breaks the hash
chain and is detected at verification time.

## API Interface

```c
// Check if a shard is allowed to perform an action on an object
int sigma_mac_check(shard_id_t subj, object_id_t obj, mac_action_t action);

// Append an audit entry (called automatically by mac_check)
void sigma_audit_log(const sigma_audit_entry_t *entry);

// Verify integrity of the entire audit chain
int sigma_audit_verify(void);

// Load a MAC policy file
int sigma_mac_load_policy(const char *policy_path);

// Initialise MAC subsystem
void init_security_access_control(void);
```

## Policy Language

```
# Allow app_shard to read /home, write /tmp
ALLOW app_shard   READ  /home
ALLOW app_shard   WRITE /tmp
DENY  app_shard   ANY   /etc/shadow

# Network policies
ALLOW web_shard   CONNECT net:443
DENY  web_shard   BIND    net:*
```

## Roadmap

- [x] Audit chain with BLAKE3 linking (`audit_chain.rs`)
- [ ] Policy compiler (text → binary rule table)
- [ ] Kernel enforcement hook in syscall dispatcher
- [ ] Label assignment to all shards at boot
- [ ] Policy hot-reload (without reboot)
- [ ] GUI policy editor for Zenith Desktop

## Related Modules

- [`modules/security/isolation`](../isolation/README.md) — Process isolation
- [`modules/core/kernel`](../../core/kernel/README.md) — Syscall enforcement hooks
""")

# ─── modules/security/isolation/README.md ────────────────────────────────────
w("modules/security/isolation/README.md", """\
# Σ security/isolation — Sovereign Process Isolation (S-Sandbox)

Provides **zero-trust, capability-gated execution environments** for all
applications running on SigmaOS. No process can exceed its declared capability
set — not even with a kernel exploit.

## Source Files

| File | Description |
|---|---|
| `sandbox.rs` | Shard sandbox: spawn, constrain, monitor, destroy |

## Isolation Model

```
Untrusted App (WASM / ELF)
   └─ sigma_sandbox_create()         ← assigns capability token
         └─ Shard Boundary (hardware ring separation)
               ├─ FS namespace  (only allowed paths visible)
               ├─ Net namespace (only declared ports reachable)
               ├─ IPC namespace (only approved shard IDs)
               └─ Memory domain (IOMMU-enforced, no cross-shard DMA)
```

## API Interface

```c
typedef struct {
    sigma_u32 container_id;
    bool      network_access;
    bool      fs_access;
    sigma_u32 memory_limit_mb;
    char     *allowed_paths[16];
    uint16_t  allowed_ports[16];
} sigma_sandbox_config_t;

// Create a new sandboxed container
sigma_u32 sandbox_create(const sigma_sandbox_config_t *cfg);

// Execute a binary inside the sandbox
int sandbox_execute(sigma_u32 id, const char *binary_path);

// Check if a syscall is allowed inside this sandbox
int sandbox_check_syscall(sigma_u32 id, sigma_u32 syscall_nr);

// Destroy a sandbox and reclaim resources
void sandbox_destroy(sigma_u32 id);

// Validate a MAC policy decision for this sandbox
int sandbox_validate_mac(sigma_u32 id, const char *subj, const char *obj, const char *action);

// Initialise the sandbox subsystem
void init_security_isolation(void);
```

## Syscall Allowlist

Each sandbox defines an explicit **syscall allowlist**. Any call not on the list
is blocked at the dispatcher and logged to the audit chain:

```json
{
  "syscall_allowlist": ["sigma_vfs_read", "sigma_vfs_write", "sigma_net_send"],
  "syscall_denylist":  ["sigma_exec", "sigma_ptrace"]
}
```

## Roadmap

- [x] Sandbox create / execute / destroy lifecycle (`sandbox.rs`)
- [x] Syscall allowlist enforcement
- [ ] IOMMU-enforced DMA isolation
- [ ] seccomp-BPF equivalent for Sovereign ABI
- [ ] Sandbox live introspection API
- [ ] Escape detection via invariant checking
- [ ] Formal proof: sandbox capability confinement (Isabelle/HOL)

## Related Modules

- [`modules/security/access_control`](../access_control/README.md) — MAC policies
- [`modules/ext/plugins`](../../ext/plugins/README.md) — Plugin capsule sandboxing
""")

# ─── modules/security/secure_boot/README.md ──────────────────────────────────
w("modules/security/secure_boot/README.md", """\
# Σ security/secure_boot — Cryptographic Secure Boot

Ensures that every component loaded during the SigmaOS boot sequence is
**cryptographically verified** against the Sovereign Trust Root before execution.

## Boot Chain of Trust

```
UEFI Firmware (OEM key)
   └─ SigmaOS UEFI shim (signed with Sovereign Root CA)
         └─ sigma-boot (Rust UEFI bootloader)
               └─ Kernel image verification (Ed25519 + SHA-512)
                     └─ initramfs verification (BLAKE3)
                           └─ kernel_main()
```

## Verification Algorithm

1. Load component into memory.
2. Compute BLAKE3 hash of the raw bytes.
3. Verify Ed25519 signature (from the Sovereign Root CA public key embedded
   in the bootloader).
4. If verification fails → halt with error code and log to TPM event log.
5. If verification passes → transfer execution.

## Rollback Protection

Each verified component carries a **monotonic version counter** stored in the
TPM NV index. Downgrade attacks are rejected:

```c
if (component_version < tpm_read_nv(NV_MIN_VERSION)) {
    secure_boot_halt("Rollback attack detected");
}
```

## API Interface

```c
// Verify a binary image before loading
int secure_boot_verify(const uint8_t *image, size_t len,
                       const uint8_t *sig, size_t sig_len);

// Update the minimum version counter in TPM NV
int secure_boot_update_version(uint32_t component_id, uint32_t new_version);

// Read the TPM event log (for audit purposes)
int secure_boot_read_event_log(sigma_tpm_event_t *out, size_t *count);

// Initialise secure boot subsystem
void init_security_secure_boot(void);
```

## Key Management

| Key | Purpose | Storage |
|---|---|---|
| Sovereign Root CA | Signs all official SigmaOS releases | HSM / offline |
| Platform Key (PK) | UEFI Secure Boot anchor | UEFI NVRAM |
| Key Exchange Key (KEK) | Update DB / DBX | UEFI NVRAM |
| Signing Key | Per-component CI signing | GitHub OIDC → HSM |

## Roadmap

- [ ] Ed25519 signature verification (UEFI phase)
- [ ] BLAKE3 hash chain (initramfs → kernel → rootfs)
- [ ] TPM 2.0 PCR extension and measurement log
- [ ] Rollback counter in TPM NV
- [ ] Measured Boot report (for remote attestation)
- [ ] Post-quantum upgrade path (Dilithium3 signatures)

## Related Modules

- [`modules/security/access_control`](../access_control/README.md) — Runtime MAC
- [`modules/security/isolation`](../isolation/README.md) — Process sandbox
""")

# ─── modules/tools/diag/README.md ────────────────────────────────────────────
w("modules/tools/diag/README.md", """\
# Σ tools/diag — Sovereign Diagnostics Toolkit

Provides **structured logging, profiling, and syscall tracing** for the SigmaOS
kernel and user-space shards.

## Source Files

| File | Description |
|---|---|
| `logger.rs` | Ring-buffer structured logger (`SIGMA_LOG_*` macros) |
| `profiler.rs` | CPU cycle + PMU counter-based profiler |
| `syscall_tracer.rs` | Strace-equivalent syscall intercept + timeline recorder |

## Logger

Zero-allocation ring-buffer logger using a lock-free SPSC queue:

```rust
// Log levels: TRACE, DEBUG, INFO, WARN, ERROR, FATAL
sigma_log!(INFO, "kernel", "shard {id} started in {elapsed_us}µs");

// Structured key-value pairs
sigma_log!(WARN, "net", "tcp_retransmit";
    "shard" => id, "seq" => seq_num, "count" => retries);
```

Output format (journal-compatible):
```
2026-07-05T14:30:00.000Z INFO  kernel  shard 42 started in 1200µs
```

## Profiler

Hardware PMU-based profiler using `perf_event_open` equivalents:

```c
// Start profiling a shard
profiler_start(shard_id, PROFILER_CPU_CYCLES | PROFILER_CACHE_MISS);

// Snapshot current counters
profiler_snapshot_t snap = profiler_snapshot(shard_id);
// snap.cpu_cycles, snap.cache_misses, snap.branch_mispredicts

// Stop and emit flamegraph data
profiler_stop(shard_id, "output.fg");
```

## Syscall Tracer

```c
// Attach to a running shard
tracer_attach(shard_id);

// All syscalls are now recorded to a ring buffer
// sigma_vfs_open("/etc/config", O_RDONLY) → 3 [1.2µs]
// sigma_net_send(3, buf, 1024, 0) → 1024 [8.4µs]

// Detach and dump trace
tracer_detach(shard_id);
tracer_dump("trace.json");
```

## API Interface

```c
void init_tools_diag(void);

// Logger
void sigma_log(log_level_t level, const char *module, const char *msg, ...);

// Profiler
void profiler_start(shard_id_t id, uint32_t events);
profiler_snapshot_t profiler_snapshot(shard_id_t id);
void profiler_stop(shard_id_t id, const char *output_path);

// Tracer
int tracer_attach(shard_id_t id);
void tracer_dump(const char *path);
```

## Roadmap

- [x] Ring-buffer structured logger (`logger.rs`)
- [x] PMU-based profiler stub (`profiler.rs`)
- [x] Syscall tracer stub (`syscall_tracer.rs`)
- [ ] Flamegraph generation (Brendan Gregg format)
- [ ] Log shipping to Sovereign Audit Chain
- [ ] Distributed tracing (OpenTelemetry-compatible spans)
- [ ] Interactive TUI dashboard (`sigma-top`)

## Related Modules

- [`modules/core/kernel`](../../core/kernel/README.md) — Kernel log sources
- [`modules/security/access_control`](../../security/access_control/README.md) — Audit chain
""")

# ─── modules/tools/loader/README.md ──────────────────────────────────────────
w("modules/tools/loader/README.md", """\
# Σ tools/loader — Sovereign Module Loader

Handles **dynamic loading, verification, and lifecycle management** of SigmaOS
kernel modules and user-space shards at runtime.

## Source Files

| File | Description |
|---|---|
| `module_loader.rs` | Core loader: ELF/WASM parse, verify, link, execute |

## Loading Pipeline

```
Request: load("sigma-net.shard")
   │
   ├─ 1. Fetch from Sovereign Package Registry (or local path)
   │
   ├─ 2. Verify Ed25519 signature against Trust Root
   │
   ├─ 3. Parse ELF / WASM headers
   │
   ├─ 4. Resolve capability requirements
   │       └─ Query kernel: does caller hold CAP_MODULE_LOAD?
   │
   ├─ 5. Map code into isolated memory domain (IOMMU-backed)
   │
   ├─ 6. Patch relocations + link against sovereign libc
   │
   └─ 7. Call module entry point with capability token
```

## API Interface

```c
// Load and start a module from a path
module_handle_t sigma_module_load(const char *path, cap_token_t caller_caps);

// Unload a module cleanly (calls shutdown hook)
int sigma_module_unload(module_handle_t handle);

// Query a loaded module's exported symbols
void *sigma_module_sym(module_handle_t handle, const char *symbol);

// List all currently loaded modules
int sigma_module_list(module_info_t *out, size_t max_count);

// Verify a module archive (without loading)
int sigma_module_verify(const char *path);

// Initialise the loader subsystem
void init_tools_loader(void);
```

## Module Manifest

Every loadable module ships with `module.json`:

```json
{
  "name": "sigma-net",
  "version": "1.2.0",
  "entry_point": "sigma_net_init",
  "capabilities_required": ["CAP_NET_BIND", "CAP_IRQ_BIND"],
  "capabilities_provided": ["CAP_SOCKET_API"],
  "signature": "<Ed25519 over (name|version|sha256(code))>"
}
```

## Hot-Reload

The loader supports **hot-module replacement** for non-critical modules:

1. Load new version into a shadow domain.
2. Quiesce the old version (drain in-flight requests).
3. Atomically swap the dispatch table pointer.
4. Unload the old version.

## Roadmap

- [x] ELF loader + relocation (`module_loader.rs`)
- [ ] WASM capsule loader (Cranelift JIT)
- [ ] Signature verification integration
- [ ] Hot-reload (shadow domain swap)
- [ ] Dependency graph resolver (topological sort)
- [ ] Module version compatibility checks (semver)

## Related Modules

- [`modules/ext/plugins`](../../ext/plugins/README.md) — Plugin capsule format
- [`modules/security/isolation`](../../security/isolation/README.md) — Module sandboxing
""")

# ─── modules/tools/sandbox/README.md ─────────────────────────────────────────
w("modules/tools/sandbox/README.md", """\
# Σ tools/sandbox — Sovereign Testing Sandbox

A **safe, instrumented execution environment** for running untrusted code,
fuzz targets, and integration tests without risking the production kernel state.

## Use Cases

| Use Case | Description |
|---|---|
| **Fuzz testing** | Execute AFL++/libFuzzer harnesses in an isolated shard |
| **CI integration tests** | Run full kernel subsystem tests in a QEMU-backed sandbox |
| **Third-party capsules** | Test unverified plugins before signing |
| **Kernel regression tests** | Snapshot state → run test → verify → rollback |

## Architecture

```
Test Runner
   └─ sandbox_create(config)
         └─ Isolated shard (separate memory domain, IOMMU)
               ├─ Fake hardware (QEMU device model via virtio)
               ├─ Snapshot of kernel state (SovereignFS CoW)
               └─ Instrumented syscall interceptor (for coverage)
```

## API Interface

```c
typedef struct {
    const char *name;
    uint64_t   memory_limit_mb;
    bool       network_enabled;
    bool       fs_writable;
    const char *rootfs_snapshot;    // SovereignFS snapshot tag
} sandbox_config_t;

// Create a testing sandbox
sandbox_t *sandbox_create(const sandbox_config_t *cfg);

// Execute a binary inside the sandbox, return exit code
int sandbox_run(sandbox_t *sb, const char *binary, char *const argv[]);

// Capture all syscall events from the sandbox
int sandbox_trace_syscalls(sandbox_t *sb, syscall_trace_cb_t cb);

// Reset sandbox to its initial snapshot (for repeated test runs)
int sandbox_reset(sandbox_t *sb);

// Destroy the sandbox and free all resources
void sandbox_destroy(sandbox_t *sb);

// Initialise the sandbox subsystem
void init_tools_sandbox(void);
```

## Fuzz Integration

```bash
# Build a fuzz harness
sigma build --fuzz target/fuzz_net_parser

# Run under AFL++
sigma fuzz --target fuzz_net_parser --timeout 3600

# Triage a crash
sigma sandbox run --repro crash-001.bin --target fuzz_net_parser
```

## Roadmap

- [ ] Basic shard isolation (`sandbox_create` / `sandbox_destroy`)
- [ ] SovereignFS snapshot-based reset (`sandbox_reset`)
- [ ] Syscall trace interceptor (`sandbox_trace_syscalls`)
- [ ] AFL++ / libFuzzer integration harness
- [ ] QEMU-backed hardware simulation (virtio-blk, virtio-net)
- [ ] Coverage-guided fuzzing via KCOV equivalent
- [ ] Distributed sandbox pool for CI parallelism

## Related Modules

- [`modules/security/isolation`](../../security/isolation/README.md) — Production isolation
- [`modules/tools/diag`](../diag/README.md) — Syscall tracing
- [`modules/perf/bench`](../../perf/bench/README.md) — Performance regression testing
""")

# ─── pkg/spm/README.md ───────────────────────────────────────────────────────
w("pkg/spm/README.md", """\
# Sovereign Package Manager (SPM / `sigpkg`)

SPM is the cryptographically secure, deterministic package manager for SigmaOS.
Unlike `apt` or `dnf` — which rely on global shared state and binary blobs —
SPM manages isolated **shards** using a recipe-based, reproducible build system.

## Core Principles

1. **Cryptographic Verifiability:** No shard is installed without passing a
   strict Ed25519 digital signature check against the Sovereign Trust Root.
2. **Deterministic Rollbacks:** Upgrades are atomic. Any failure immediately
   reverts the state pointer via SovereignFS CoW snapshots.
3. **Dependency Isolation:** Shards do not pollute a global `/usr/lib`.
   Dependencies are strictly mapped via the Shard Manifest.
4. **Reproducible Builds:** The same `.srecipe` file + same source commit
   always produces a bit-for-bit identical shard binary.

## Components

| File | Description |
|---|---|
| `cli.py` | User-facing CLI — `sigpkg install`, `remove`, `search`, `rollback` |
| `verifier.py` | Cryptographic core — verifies Ed25519 signatures, hash chains |
| `schema/shard_manifest.json` | JSON schema for valid `.shard` package structures |

## Example Usage

```bash
# Install a shard from the Sovereign Registry
sigpkg install SovereignNet

# Verify a locally downloaded shard
sigpkg verify SovereignNet

# Atomic rollback to the previous state
sigpkg rollback

# Search the registry
sigpkg search "web server"

# Build a shard from source recipe
sigpkg build ./my_app.srecipe

# List installed shards
sigpkg list --installed
```

## Shard Manifest Format

```json
{
  "name": "SovereignNet",
  "version": "2.1.0",
  "arch": ["x86_64", "aarch64"],
  "capabilities_required": ["CAP_NET_BIND"],
  "dependencies": [],
  "sha256": "abc123...",
  "signature": "<Ed25519 over (name|version|sha256)>"
}
```

## Roadmap

- [x] CLI skeleton (`sigpkg install`, `remove`, `rollback`)
- [x] Signature verifier (`verifier.py`)
- [x] Shard manifest schema
- [ ] Registry server implementation
- [ ] Reproducible build toolchain integration
- [ ] Delta updates (only ship changed extents)
- [ ] Multi-architecture cross-build support
- [ ] GUI package browser for Zenith Desktop

## Related Modules

- [`modules/tools/loader`](../../modules/tools/loader/README.md) — Module loader
- [`modules/security/secure_boot`](../../modules/security/secure_boot/README.md) — Signature verification
""")

# ─── wiki_repo/DRIVER_ECOSYSTEM.md (fix inline code fence) ───────────────────
driver_eco_path = os.path.join(REPO, "wiki_repo", "DRIVER_ECOSYSTEM.md")
w("wiki_repo/DRIVER_ECOSYSTEM.md", """\
# DRIVER ECOSYSTEM

SigmaOS ships hardware drivers as sovereign kernel shards — zero binary blobs,
zero kernel-update breakage.

## Network Interface Controller (NIC)

Auto-detects and programs the NIC at boot via PCIe endpoint analysis:

| Hardware | PCIe ID | Mode |
|---|---|---|
| VirtIO-Net (QEMU) | `1AF4:1000` | DMA ring buffer |
| RTL8139 (bare-metal) | `10EC:8139` | BMCR register programming |

```c
nic_init();
nic_probe(0x1AF4, 0x1000); // VirtIO-Net
nic_transmit(payload, length);
```

## WiFi Driver

Kernel-level 802.11 WPA3 stack:

```c
wifi_init();
wifi_scan(channels, SIGMA_WIFI_ALL);
wifi_connect("MySSID", wpa3_psk);
```

## Unknown Hardware Transpiler

`SovereignHWTranspiler` handles unknown PCIe devices:

```rust
let transpiler = SovereignHWTranspiler::new(pcie_id);
transpiler.generate_driver_stub()?;
```

## Roadmap

- [x] VirtIO-Net DMA ring buffer driver
- [x] RTL8139 BMCR driver
- [x] 802.11 WPA3 stub
- [ ] NVIDIA GPU open-firmware driver
- [ ] AMD AMDGPU driver
- [ ] Intel i915 open-spec driver
- [ ] Formal DDK verification harness
""")

# ─── wiki_repo/Networking-Shard.md (close unclosed mermaid block) ────────────
w("wiki_repo/Networking-Shard.md", """\
# Sovereign Networking Shard (S-NET)

The Networking Shard is a modular, hot-swappable TCP/IP stack implemented
independently from the monolithic kernel core. It provides secure sockets and
strict network isolation for SigmaOS.

## Architecture Diagram

```mermaid
graph TD
    A[Userland App] --> |Z-SYSCALL| B(S-NET Socket API)
    B --> C{PQC Engine}
    C --> |Encrypted| D[TCP/IP Stack]
    C --> |Unencrypted| D
    D --> E[Sovereign HAL]
    E --> F[Hardware NIC]
```

## Key Features

- **TCP/IP Stack**: Full IPv4 (and future IPv6) implementation.
- **Secure Sockets**: Built-in integration with the Post-Quantum Cryptography
  (PQC) engine for default-encrypted packet transmission.
- **Hot-swappable**: The network driver and stack can be restarted or updated
  without rebooting the kernel.

## API Examples

### Creating a Socket

```c
int fd = sigma_net_socket(AF_SIGMA, SOCK_STREAM, IPPROTO_TCP);
sigma_net_connect(fd, &remote_addr);
sigma_net_send(fd, buf, len, 0);
sigma_net_close(fd);
```

### Enabling Post-Quantum Encryption

```c
sigma_net_enable_pqe(fd, &my_kyber_keypair);
```

## Roadmap

- [x] TCP/IP stack (`tcp.rs`, `tcpip.rs`)
- [x] PQC engine stub (`pqfs.rs`)
- [x] Socket API (`socket.rs`)
- [ ] IPv6 SLAAC / NDP
- [ ] DHCPv6 client
- [ ] WireGuard-inspired VPN tunnel
""")

print("\n=== All .md files implemented successfully! ===\n")
