# 🏗️ SigmaOS Developer Guide

> **The definitive guide for contributing to the SigmaOS Sovereign Lattice.**

---

## 📋 Table of Contents

- [Prerequisites](#prerequisites)
- [Quick Setup](#quick-setup)
- [Architecture Overview](#architecture-overview)
- [Coding Standards](#coding-standards)
- [Creating a New Shard](#creating-a-new-shard)
- [Security Requirements](#security-requirements)
- [Testing Your Shard](#testing-your-shard)
- [Submitting a Pull Request](#submitting-a-pull-request)

---

## Prerequisites

| Tool | Version | Purpose |
| :--- | :--- | :--- |
| GCC / G++ | 13+ | Primary kernel compiler |
| NASM | 2.15+ | x86_64 assembly |
| Make | 4.0+ | Build orchestration |
| Python 3 | 3.10+ | Lattice coverage tools |
| Git | 2.40+ | Version control |
| Clang-Tidy | 16+ | Static analysis |
| CppCheck | 2.10+ | Security audit |

---

## Quick Setup

```bash
# 1. Clone the repository
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git -b main
cd SigmaOS

# 2. Install toolchain (Ubuntu/Debian)
sudo apt-get install -y gcc g++ nasm make python3 clang-tidy cppcheck

# 3. Build the sovereign kernel
make -j$(nproc)

# 4. Run the lattice coverage report
python3 scripts/lattice_coverage.py

# 5. Run static analysis
cppcheck --enable=warning,style,performance -Iinclude kernel/
```

---

## Architecture Overview

```text
SigmaOS Sovereign Lattice (600+ Shards)
│
├── kernel/
│   ├── core/               ← Sovereign Shards (C++17, zero-dependency)
│   │   ├── SovereignSched.cpp     ← AI Scheduler (NPWO)
│   │   ├── SovereignSecHardener.cpp ← Security Hardener (PLPE)
│   │   ├── SovereignAllocator.cpp  ← Memory Allocator (QBMP)
│   │   └── ...600+ shards
│   ├── ui/                 ← Universal UI Layer (DFO algorithm)
│   └── drivers/            ← Silicon-native hardware drivers
│
├── include/                ← Shard header files (sigma_*.h)
│
├── shards/                 ← Optional/third-party shard packages
│
├── scripts/
│   ├── lattice_coverage.py ← Shard count and modularity metrics
│   └── check_modularity.py ← Zero-dependency enforcement
│
├── .github/workflows/
│   └── sigma_ci.yml        ← Full 6-stage CI/CD pipeline
│
└── SigmaOS.wiki/           ← Documentation submodule
```

---

## Coding Standards

### 1. Zero-Dependency Rule ⚡

```cpp
// ❌ FORBIDDEN — HLL libraries
#include <iostream>
#include <vector>
#include <string>

// ✅ REQUIRED — Sovereign headers only
#include "sigma_types.h"
#include "SovereignLibC.h"
```

### 2. Secure String Operations 🔒

> All string operations MUST use the hardened wrappers from `SovereignLibC.h`

```cpp
// ❌ UNSAFE — Triggers CWE-119
strcpy(dest, src);
sprintf(buf, fmt, ...);

// ✅ SAFE — Bounds-enforced sovereign primitives
sigma_hardened_strcpy(dest, src, MAX_LEN);
sigma_hardened_snprintf(buf, MAX_LEN, fmt, ...);
```

### 3. Input Validation at Every API Boundary 🛡️

```cpp
// Every public function MUST validate inputs (CWE-20 fix)
extern "C" void my_shard_function(const void* data, sigma_u32 size) {
    if (!data || size == 0u) return;  // Guard clause first
    // ... proceed with logic
}
```

### 4. Shard Naming Convention

| Component | Convention | Example |
| :--- | :--- | :--- |
| Header file | `sigma_<name>.h` | `sigma_neural.h` |
| Implementation | `Sovereign<Name>.cpp` | `SovereignNeural.cpp` |
| Init function | `<name>_init()` | `neural_init()` |
| Public APIs | `<name>_<verb>()` | `neural_predict()` |

### 5. Documentation Block

Every shard implementation MUST include:

```cpp
/**
 * SigmaOS Sovereign <Name>
 * Implements a <Algorithm Full Name> (<ACRONYM>) algorithm.
 * ZERO-DEPENDENCY: Strictly bare-metal <description>.
 *
 * Design: OOP-isolated singleton -- Sovereign<Name>Engine.
 */
```

---

## Creating a New Shard

### Step 1 — Create the header

```bash
touch include/sigma_myshard.h
```

```cpp
#ifndef SIGMA_MYSHARD_H
#define SIGMA_MYSHARD_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

void myshard_init(void);
void myshard_execute(const void* input, sigma_u32 size);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_MYSHARD_H */
```

### Step 2 — Create the implementation

```bash
touch kernel/core/SovereignMyShard.cpp
```

### Step 3 — Verify the shard is detected

```bash
python3 scripts/lattice_coverage.py
```

### Step 4 — Run security audit

```bash
cppcheck --enable=warning -Iinclude kernel/core/SovereignMyShard.cpp
```

---

## Security Requirements

All shards **MUST** comply with the following before merging:

- [ ] No `strcpy`, `sprintf`, `gets`, or `scanf` usage
- [ ] All public APIs validate `NULL` inputs
- [ ] No forbidden HLL includes (`<iostream>`, `<string>`, etc.)
- [ ] Buffer sizes explicitly passed — never assumed
- [ ] No global mutable state unless protected by atomic ops

---

## Testing Your Shard

SigmaOS uses a built-in Sovereign Test Lattice for automated regression testing.

1. **Add your test case** to `kernel/core/SovereignTests.cpp`.
2. **Run the tests** by booting the kernel in QEMU:
```bash
make qemu
```
3. **Verify the serial output** for the `[TEST]` tags and the `✅ ALL CORE TESTS PASSED` finality message.

You can also run static analysis locally:
```bash
# Run CppCheck on your shard
cppcheck --enable=all --suppress=missingInclude -Iinclude \
         kernel/core/SovereignMyShard.cpp
```

---

## Submitting a Pull Request

1. Fork the repository.
2. Create a branch: `git checkout -b feat/s-myshard`.
3. Implement your shard following the coding standards above.
4. Add automated tests in `SovereignTests.cpp`.
5. Push and open a PR against `lattice-dev`.
6. The GitHub Actions CI pipeline (`.github/workflows/sigma_ci.yml`) will automatically verify your build and run security audits.
7. All checks must pass ✅ before merging.

---

*SigmaOS is sovereign. So is your contribution.*
