# SigmaOS Language Implementation Policy

> Full policy: [docs/Language_Implementation_Policy.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/Language_Implementation_Policy.md)

---

## Language Assignment

| Domain | Language | Why |
|---|---|---|
| Kernel core (scheduler, MM, IPC, syscall) | **Rust** `#![no_std]` | Memory safety, zero-cost, no libc |
| HAL / boot / ISR / drivers | **Zig** | Direct MMIO, comptime, zero allocations |
| Formal security (crypto proofs) | **SPARK/Ada** | Proven correctness via `gnatprove` |
| CLI tools / sigma-pkg / sigma-sh | **Nim** | Native binary, ergonomic, no GC |
| Userland daemons | **Rust** (std) | Safe concurrency |
| Cryptography primitives | **Rust** + **SPARK** | Performance + proof |

## Implemented Files (as of v15.0)

| File | Language | Replaces |
|---|---|---|
| `kernel/core/sigma_sched.rs` | Rust | sigma_sched.cpp |
| `kernel/core/sigma_mm.rs` | Rust | sigma_mm.cpp |
| `kernel/core/sigma_syscall_dispatch.rs` | Rust | sigma_syscall_dispatch.cpp |
| `kernel/core/sigma_irq.zig` | Zig | sigma_irq.cpp |
| `kernel/memory/sigma_vmm.zig` | Zig | sigma_vmm.cpp |
| `kernel/security/sigma_pledge.rs` | Rust | SovereignPledge.cpp |
| `kernel/fs/sigma_vfs.rs` | Rust | SovereignVFS.cpp |
| `kernel/net/sigma_net.rs` | Rust | SovereignNetStack.cpp |
| `crypto/sigma_kyber.rs` | Rust | sigma_kyber.cpp |
| `crypto/sigma_dilithium.ads/.adb` | SPARK/Ada | sigma_dilithium.cpp |
| `drivers/display/sigma_vesa.zig` | Zig | sigma_vesa.cpp |
| `drivers/net/sigma_e1000.rs` | Rust | SovereignE1000.cpp |
| `sigma-boot/sigma_boot.zig` | Zig | sigma_boot.c |
| `userland/shell/sigma_shell.nim` | Nim | sigma_shell.cpp |
| `userland/pkg/sigma_pkg.nim` | Nim | sigma_pkg.cpp |
| `userland/init/sigma_init.rs` | Rust | planned C++ |
| `virtualization/ocirunner/sigma_oci.rs` | Rust | planned C++ |

## Rules (enforced by CI)

1. No C or C++ in `kernel/`, `drivers/`, `security/`, `crypto/`

2. `#![no_std]` in all kernel crates

3. No third-party crates in kernel

4. Every `unsafe` block has a justification comment

5. OOP via Traits (Rust) / struct methods (Zig) / contracts (SPARK)

*See full guide: [docs/Language_Implementation_Policy.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/Language_Implementation_Policy.md)*
