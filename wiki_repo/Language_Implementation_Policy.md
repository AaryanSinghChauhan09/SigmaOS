# SigmaOS Language Implementation Policy

> Canonical policy for language choice across every subsystem.
> All new code follows this policy. All legacy C/C++ is being migrated.

---

## Language Assignment by Domain

| Domain | Language | Rationale |
|--------|----------|-----------|
| Kernel core (scheduler, MM, IPC, syscall) | **Rust** (`#![no_std]`) | Memory safety, zero-cost abstractions, `no_std` |
| HAL / boot / ISR stubs | **Zig** | Comptime, direct hardware access, no hidden allocations |
| Formal-critical security (MACs, crypto proofs) | **SPARK/Ada** | Formal verification via `gnatprove`, proof-carrying code |
| Build scripts / CLI tools / sigma-pkg | **Nim** | Ergonomic, compiles to C-free native, JS backend for tools |
| Userland daemons (sigmad-*, sigma-sh) | **Rust** (`std` allowed) | Safe concurrency, ecosystem |
| GPU/display drivers | **Zig** | Direct MMIO, no hidden runtime, comptime register maps |
| Network stack | **Rust** (`#![no_std]` + smoltcp patterns) | Type-safe protocol state machines |
| Cryptography primitives | **Rust** + **SPARK** verification | Proven correctness + performance |
| AI/ML inference runtime | **Rust** + **Zig** SIMD | AVX-512/NEON without libc |

---

## Prohibited Practices (enforced by CI `make check-lang`)

1. **No C or C++** — zero C/C++ files in `kernel/`, `drivers/`, `security/`, `crypto/`

2. **No `unsafe` without justification comment** — every `unsafe` block documents why

3. **No `std`/`libc` in kernel crates** — `#![no_std]` enforced

4. **No pre-defined stdlib functions in kernel** — implement from scratch

5. **No third-party crates in kernel** — only `sigma-*` internal crates

6. **OOP via Traits** — use Rust Traits + Structs; no inheritance anti-patterns

7. **No dynamic dispatch in hot paths** — prefer generics over `dyn Trait`

---

## Migration Status (C/C++ → Modern Languages)

| File/Subsystem | Old Language | New Language | Status |
|---|---|---|---|
| `kernel/core/sigma_sched.cpp` | C++ | Rust | ⬜ Planned |
| `kernel/core/sigma_mm.cpp` | C++ | Rust | ⬜ Planned |
| `kernel/core/sigma_irq.cpp` | C++ | Zig | ⬜ Planned |
| `kernel/core/sigma_syscall_dispatch.cpp` | C++ | Rust | ⬜ Planned |
| `drivers/display/sigma_vesa.cpp` | C++ | Zig | ⬜ Planned |
| `drivers/gpu/sigma_i915.cpp` | C++ | Zig | ⬜ Planned |
| `drivers/gpu/sigma_amdgpu.cpp` | C++ | Zig | ⬜ Planned |
| `drivers/net/sigma_e1000.cpp` | C++ | Rust | 🔄 In Progress |
| `drivers/storage/sigma_ahci.cpp` | C++ | Rust | ⬜ Planned |
| `kernel/security/sigma_pledge.cpp` | C++ | Rust + SPARK | ⬜ Planned |
| `kernel/fs/sigma_vfs.cpp` | C++ | Rust | ⬜ Planned |
| `userland/sigma_sh.cpp` | C++ | Nim | ⬜ Planned |
| `userland/sigma_pkg.cpp` | C++ | Nim/Rust | ⬜ Planned |
| `sigma-boot/sigma_boot.c` | C | Zig | ⬜ Planned |

---

## OOP Patterns in Rust

```rust
// Trait = interface/abstract class
pub trait SdfDriver: Send + Sync {
    fn probe(dev: &DeviceId) -> bool where Self: Sized;
    fn init(&mut self) -> SdfResult<()>;
    fn shutdown(&mut self);
    fn name(&self) -> &'static str;
}

// Struct = concrete class
pub struct E1000Driver {
    mmio_base: usize,
    rx_ring:   RxRing,
    tx_ring:   TxRing,
}

impl SdfDriver for E1000Driver {
    fn probe(dev: &DeviceId) -> bool {
        dev.vendor == 0x8086 && matches!(dev.device, 0x100E | 0x100F)
    }
    fn init(&mut self) -> SdfResult<()> {
        self.reset(); self.setup_rings()?; Ok(())
    }
    fn shutdown(&mut self) { self.flush(); }
    fn name(&self) -> &'static str { "sigma-e1000" }
}
```

---

## Zig HAL Pattern (no hidden allocations)

```zig
// arch/x86_64/apic.zig — APIC init, zero-cost, comptime register offsets
const APIC_BASE: usize = 0xFEE00000;

pub const Apic = struct {
    base: usize,

    pub fn init(self: *Apic) void {
        self.base = APIC_BASE;
        self.write(0xF0, 0x1FF); // Spurious interrupt vector
    }

    pub fn write(self: *Apic, offset: usize, val: u32) void {
        const ptr: *volatile u32 = @ptrFromInt(self.base + offset);
        ptr.* = val;
    }

    pub fn read(self: *Apic, offset: usize) u32 {
        const ptr: *const volatile u32 = @ptrFromInt(self.base + offset);
        return ptr.*;
    }
};
```

---

## SPARK/Ada Pattern (verified crypto)

```ada
-- security/sigma_dilithium.adb
-- SPARK proof: no buffer overflow, no integer overflow
package body Sigma.Dilithium
  with SPARK_Mode => On
is
   procedure Sign
     (Message   : in  Byte_Array;
      SecretKey : in  Secret_Key;
      Signature : out Signature_Type)
   with
     Pre  => Message'Length > 0 and Message'Length <= Max_Msg_Len,
     Post => Signature.Valid = True
   is
   begin
      -- provably correct signing implementation
      null; -- TODO: implement
   end Sign;
end Sigma.Dilithium;
```

---

## Nim CLI Pattern (sigma-pkg tool)

```nim

# userland/sigma_pkg/main.nim

# Compiles to native binary, no GC in kernel, no libc dependency

import std/parseopt  # stdlib allowed in userland tools

type PackageCmd = enum
  Install, Remove, Search, List, Info, Build, Verify

proc main() =
  var p = initOptParser()
  var cmd: PackageCmd
  var pkgName: string

  for kind, key, val in p.getopt():
    case kind
    of cmdArgument:
      case key
      of "install": cmd = Install
      of "remove":  cmd = Remove
      of "search":  cmd = Search
      else: pkgName = key
    else: discard

  case cmd
  of Install: installPackage(pkgName)
  of Remove:  removePackage(pkgName)
  else: echo "Usage: sigma-pkg <install|remove|search> <name>"

main()
```

---

*See also: [CONTRIBUTING.md](../CONTRIBUTING.md) · [docs/License_Map.md](License_Map.md) · [LANGUAGE_POLICY.md](../LANGUAGE_POLICY.md)*
