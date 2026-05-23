# 🛠️ Developer Guide: Build, Test & Contribute

> This guide covers how to build SigmaOS from source, run it in QEMU, write zero-dependency kernel shards, and submit contributions.

---

## Prerequisites

| Tool | Purpose |
|:--|:--|
| `x86_64-elf-gcc` | Cross-compiler (no host libc linkage) |
| `nasm` | x86 assembler for boot stages |
| `qemu-system-x86_64` | Hardware emulator for testing |
| `make` | Build orchestration |
| `xorriso` | ISO creation |
| `grub-pc-bin` | Bootloader |
| `node` + `npm` | Vitest UI/telemetry test suite |

---

## Build Instructions

```bash
# 1. Clone the repository
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# 2. Install JS dependencies (for Vitest tests only)
npm install

# 3. Compile the kernel
make clean && make all

# 4. Boot in QEMU
qemu-system-x86_64 \
  -cdrom build/sigmaos.iso \
  -serial stdio \
  -m 2G \
  -net nic,model=e1000
```

---

## Running Tests

```bash
# JS/UI tests (82 tests, must all pass)
npm run test

# Zero-dependency audit (must show 0 violations)
grep -r "#include <" kernel/ tools/ usr/ --include="*.cpp"
```

---

## Writing a Zero-Dependency Shard

Every kernel shard must follow these rules:

### ✅ Allowed
```cpp
// 1. Declare your own types
typedef unsigned int u32;

// 2. Use inline assembly for hardware access
__asm__ volatile("hlt");

// 3. Use __builtin_* GCC intrinsics
__builtin_va_list args;

// 4. Write sovereign utility functions
static void sovereign_memset(void* ptr, u8 val, u32 n) { ... }
```

### ❌ Forbidden
```cpp
#include <stdio.h>    // FORBIDDEN — predefined library
#include <stdlib.h>   // FORBIDDEN
#include <string.h>   // FORBIDDEN
#include <stdint.h>   // FORBIDDEN — define your own u8/u32/u64

using namespace std;  // FORBIDDEN
printf("hello");      // FORBIDDEN — use sigma_vga_printf()
malloc(size);         // FORBIDDEN — use sigma_alloc()
free(ptr);            // FORBIDDEN — use sigma_free()
memcpy(a, b, n);      // FORBIDDEN — use sovereign_memcpy()
```

### Shard Template

```cpp
/*
 * Σ SigmaOS Zenith — [Shard Name]
 * Absorbs: [Linux/Other inspiration]
 * Zero-Dependency: No libc, no stdlib, no predefined headers.
 */

typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;
typedef unsigned long long u64;

/* Sovereign utilities (write only what you need) */
static void sovereign_memset(void* p, u8 v, u32 n) {
    u8* b = (u8*)p; while (n--) *b++ = v;
}

/* ... your implementation ... */

extern "C" void sigma_[shard]_init() {
    /* Entry point */
}
```

---

## Commit Convention

```
type(scope): description

Types: feat, fix, docs, chore, refactor, perf
Scope: kernel, drivers, fs, scheduler, usr, tools, wiki

Examples:
  feat(drivers): add Realtek RTL8139 NIC driver
  fix(scheduler): correct EDF deadline wraparound
  docs(wiki): add developer guide
```

---

## Branch Workflow

```bash
# Always rebase — never merge
git pull --rebase origin main

# Create a feature branch
git checkout -b feat/sigma-nvme-driver

# Push and open PR
git push origin feat/sigma-nvme-driver
```
