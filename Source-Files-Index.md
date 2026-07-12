# SigmaOS Source Files Index

> All implemented source files by language and subsystem.

---

## Rust (`#![no_std]` — Kernel)

| File | Subsystem | Description |
|---|---|---|
| `kernel/core/sigma_sched.rs` | Scheduler | MLFQ + EDF + CFS — full Rust impl |
| `kernel/core/sigma_mm.rs` | Memory | Buddy allocator + Slab (kmalloc) |
| `kernel/core/sigma_syscall_dispatch.rs` | Syscalls | 32-syscall dispatch table |
| `kernel/security/sigma_pledge.rs` | Security | sigma_pledge + sigma_unveil |
| `kernel/fs/sigma_vfs.rs` | Filesystem | VFS layer + Tmpfs |
| `kernel/net/sigma_net.rs` | Networking | Ethernet + IPv4 + ARP + UDP |
| `crypto/sigma_kyber.rs` | Crypto | Kyber-1024 KEM skeleton + NTT |
| `drivers/net/sigma_e1000.rs` | NIC Driver | Intel e1000 Gigabit |

## Zig (HAL / Boot / Drivers)

| File | Subsystem | Description |
|---|---|---|
| `kernel/core/sigma_irq.zig` | IRQ | IDT + APIC + IRQ dispatch |
| `kernel/memory/sigma_vmm.zig` | VM | x86-64 4-level paging + W^X |
| `sigma-boot/sigma_boot.zig` | Bootloader | UEFI EFI stub (replaces C) |
| `drivers/display/sigma_vesa.zig` | Display | VESA/GOP framebuffer driver |
| `arch/x86_64/paging.zig` | Arch | x86-64 paging (existing) |

## Nim (Userland Tools)

| File | Tool | Description |
|---|---|---|
| `userland/shell/sigma_shell.nim` | sigma-sh | Sovereign shell — full REPL |
| `userland/pkg/sigma_pkg.nim` | sigma-pkg | Package manager with SHA-256 |

## Rust (Userland Daemons)

| File | Daemon | Description |
|---|---|---|
| `userland/init/sigma_init.rs` | sigma-init | PID 1 + service supervisor |
| `virtualization/ocirunner/sigma_oci.rs` | sigma-container | OCI runtime |

## SPARK/Ada (Formal Crypto)

| File | Subsystem | Description |
|---|---|---|
| `crypto/sigma_dilithium.ads` | Crypto | Dilithium-5 spec with contracts |
| `crypto/sigma_dilithium.adb` | Crypto | Dilithium-5 body (provable) |

---

## Language Distribution

```
Rust    ≈ 65%  (kernel core, security, net, crypto, userland daemons)
Zig     ≈ 20%  (HAL, boot, interrupt handling, drivers, paging)
Nim     ≈ 10%  (CLI tools, shell, package manager)
SPARK   ≈  5%  (formal crypto verification)
```

### No C or C++ in any of the above. All C++ stubs have been replaced.
