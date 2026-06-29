# SigmaOS Architecture

One-page block diagram and subsystem descriptions for contributors.

---

## System Stack (Top-Down)

```
╔══════════════════════════════════════════════════════════════════╗
║                        USER APPLICATIONS                        ║
║        sigma-shell  ·  browser  ·  apps  ·  containers          ║
╠══════════════════════════════════════════════════════════════════╣
║                     ZENITH DESKTOP (Ring 3)                     ║
║      Compositor  ·  GPU Pipeline  ·  AI Scheduler  ·  IPC Bus   ║
╠══════════════════════════════════════════════════════════════════╣
║                   SYSCALL INTERFACE (int 0x80 / syscall)        ║
║        Seccomp-profile-filtered  ·  Capability-checked          ║
╠══════════════════════════════════════════════════════════════════╣
║                     SIGMAOS KERNEL (Ring 0)                     ║
║  ┌─────────────┐  ┌────────────┐  ┌──────────┐  ┌───────────┐  ║
║  │  Scheduler  │  │ Page Alloc │  │   VFS    │  │  Net Core │  ║
║  │  (EEVDF)    │  │ Buddy/SLAB │  │ (sigma-  │  │  (QUIC+   │  ║
║  │             │  │ /TLSF      │  │  vfs)    │  │   TCP/IP) │  ║
║  └─────────────┘  └────────────┘  └──────────┘  └───────────┘  ║
║  ┌─────────────┐  ┌────────────┐  ┌──────────────────────────┐  ║
║  │  Zero-Trust │  │  IPC/Caps  │  │  Metrics Exporter        │  ║
║  │  Enforcer   │  │  (Cap-IPC) │  │  /sigma/metrics (procfs) │  ║
║  └─────────────┘  └────────────┘  └──────────────────────────┘  ║
╠══════════════════════════════════════════════════════════════════╣
║               SOVEREIGN HAL  (Zig — architecture leaf)          ║
║    x86_64 APIC/IOAPIC  ·  ARM64 GIC  ·  RISC-V CLINT/PLIC      ║
╠══════════════════════════════════════════════════════════════════╣
║                  HARDWARE / FIRMWARE                            ║
║     CPU · RAM · NVMe · GPU · TPM · ACPI · UEFI / coreboot      ║
╚══════════════════════════════════════════════════════════════════╝
```

---

## Privilege Rings

| Ring | Name | Runs | Direct HW Access |
|------|------|------|-----------------|
| Ring 0 / EL1 / S-Mode | **Kernel** | Scheduler, allocators, VFS, net stack, device drivers | ✅ Full |
| Ring 3 / EL0 / U-Mode | **Userland** | Shell, compositor, apps, containers | ❌ Syscall-only |
| EL2/EL3 / M-Mode | **Firmware** | UEFI / coreboot / SBI | ✅ Full |

---

## Key Subsystems

### Kernel (`kernel/`)
- **Scheduler**: EEVDF by default; `realtime` and `mixed-criticality` via `sigma.toml`.
- **Allocator**: Buddy (default), SLAB (high-throughput), TLSF (real-time/IoT).
- **IPC**: Capability-based message passing — no shared memory by default.
- **VFS**: Log-structured primary FS + read-only verified overlay for airgapped builds.
- **Metrics**: Zero-overhead virtual exporter at `/sigma/metrics` — see `kernel/core/metrics.rs`.

### Security (`security/`)
- **Zero-Trust enforcer**: pledge/unveil port, AVC matrix, namespace jailing.
- **Ada/SPARK proofs**: Formal verification for memory verifier and capability checker.
- **PQC**: Kyber-1024 hybrid TLS 1.3 in the network stack.

### HAL (`hal/`)
- Written in Zig for deterministic leaf compilation.
- One HAL binary per architecture — x86_64, AArch64, RISC-V 64GC.

### FFI Boundary (`kabi/`)
- All cross-language structs are defined here as `#[repr(C)]` Rust types.
- Zig, Nim, and Ada modules import from the generated C header — never hand-write ABI.

### Desktop (`desktop/` + `graphics/`)
- Zenith compositor: tiling + floating + full-screen modes.
- GPU pipeline: Vulkan-like abstraction over hardware drivers.
- AI scheduler: cooperative ML workload priority in `zenith_desktop.js`.

---

## Language Map

```
kernel/         Rust (no_std)
hal/            Zig
security/       Ada/SPARK (proof) + Rust (runtime)
drivers/        Rust + Zig (leaf drivers)
userland/       Rust + Nim
tools/          Rust
sigma-web/      HTML + Vanilla CSS/JS
kabi/           Rust #[repr(C)] → C header for all consumers
```

See [LANGUAGE_POLICY.md](./LANGUAGE_POLICY.md) for the full ABI contract.
