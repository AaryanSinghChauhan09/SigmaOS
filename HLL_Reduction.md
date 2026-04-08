# ⚡ HLL Reduction Policy

**HLL (High-Level Language) Reduction** is one of the core sovereignty mandates of SigmaOS. Every layer of the kernel must progressively eliminate dependency on high-level language abstractions, moving toward pure C11 and Assembly.

---

## The Three Forbidden Dependencies

| Type | Examples | Status |
| --- | --- | --- |
| **Standard Libraries** | `stdio.h`, `stdlib.h`, `string.h` | ❌ Forbidden in kernel |
| **OS-Specific APIs** | `windows.h`, `unistd.h` | ❌ Forbidden everywhere |
| **Runtime Environments** | Python, Node.js, Java in kernel | ❌ Forbidden in kernel |

---

## Allowed Replacements

| Forbidden | Sovereign Replacement |
| --- | --- |
| `printf()` | `sigma_printf()` in `sigma_libc.c` |
| `malloc()` / `free()` | `sigma_alloc()` / `sigma_free()` via PMM slab allocator |
| `memcpy()` / `memset()` | `sigma_memcpy()` / `sigma_memset()` in `sigma_std.c` |
| `strlen()` / `strcmp()` | `sigma_strlen()` / `sigma_strcmp()` in `sigma_libc.c` |
| `exit()` | Inline `__asm__ volatile("mov $60, %rax\n\t syscall\n\t")` |
| `uint32_t` | `sigma_u32` from `sigma_types.h` |

---

## Audit Tool: `SovereignBuildMaster.c`

Automatically scans all `.c` and `.h` files in the project for violations:

```c
// Checks for any of these patterns:
const char* forbidden[] = {
    "#include <stdio.h>",
    "#include <stdlib.h>",
    "#include <string.h>",
    "#include <windows.h>",
    "#include <unistd.h>"
};
```

Run before every build:

```powershell
gcc -o SovereignBuildMaster sovereign_tools/SovereignBuildMaster.c
./SovereignBuildMaster
```

**If any violation is detected, the build is halted.**

---

## Build System Flags

`build.ps1` enforces HLL reduction at the compiler level:

```powershell
gcc -nostdlib -ffreestanding -std=c11 `
    -fno-builtin -fno-stack-protector `
    -Wall -Wextra `
    -T kernel/sigma.ld `
    -o sigma.elf ...
```

| Flag | Effect |
| --- | --- |
| `-nostdlib` | No standard library linked |
| `-ffreestanding` | No hosted environment assumptions |
| `-fno-builtin` | No compiler built-in function substitution |
| `-std=c11` | Enforces C11 standard only |

---

## HLL Reduction Progress

| Module | HLL Score (lower = better) | Status |
| --- | --- | --- |
| `kernel/` core | 0 external deps (Pure C11) | ✅ Sovereign |
| `kernel/modules/` | Zero-HLL (C11 OOP) | ✅ Verified (v175.0) |
| `libc/` | 0 external deps | ✅ Sovereign |
| `sovereign_tools/` | 0 external deps | ✅ Sovereign |
| `ecosystem/` shards | Pure C11 (Transitioned) | ✅ Verified (v175.0) |
| `index.js` (UI) | Browser JS only | ✅ Acceptable |
| `build.ps1` | PowerShell (host-only) | ✅ Acceptable |

---

## Why This Matters

By eliminating HLL dependencies:

1. **No supply chain attacks** — no external packages can be compromised
2. **Predictable performance** — no hidden allocations or GC pauses
3. **True portability** — compiles on any C11-compliant toolchain
4. **Security auditability** — every function is user-defined and inspectable
