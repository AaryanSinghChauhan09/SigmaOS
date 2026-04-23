# Contributing to SigmaOS Sovereign Lattice

Thank you for your interest in contributing to **SigmaOS** — the world's most advanced sovereign, bare-metal operating system! 🚀

## Table of Contents
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Shard Architecture](#shard-architecture)
- [Submitting Changes](#submitting-changes)
- [Code Style](#code-style)
- [Reporting Bugs](#reporting-bugs)
- [Feature Requests](#feature-requests)

---

## Getting Started

### Prerequisites
| Tool | Version | Purpose |
|------|---------|---------|
| `nasm` | 2.14+ | Assembling sovereign ASM primitives |
| `g++` / `clang++` | 11+ | Compiling C++ shard modules |
| `ld` (binutils) | 2.36+ | Linking the kernel binary (Linux) |
| `node` | 18+ | Running `repair_build.js` (shim synthesis) |
| `qemu-system-x86_64` | optional | Testing the kernel in a VM |

### Quick Build
```bash
# 1. Clone the repository
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# 2. Synthesize canonical headers (ALWAYS run before building)
node repair_build.js

# 3. Build the Sovereign Lattice
chmod +x ./build_sovereign.sh
./build_sovereign.sh

# 4. Run integrity tests
chmod +x ./run_sigma_tests.sh
./run_sigma_tests.sh
```

---

## Development Setup

### Repository Structure
```
SigmaOS/
├── suites/              # 641 Sovereign Shards (S01–S33)
│   ├── S01_Genesis/     # Core kernel entry, HAL, LibC
│   ├── S03_Orchestrator/# Process management, scheduler
│   ├── S04_HAL/         # Hardware Abstraction Layer
│   ├── S07_Network/     # Networking stack
│   ├── S30_Supremacy/   # Supremacy finality kernel
│   ├── S33_GlobalLatticeMesh/ # P2P mesh synchronization
│   └── include/         # Canonical header shims
├── build_sovereign.sh   # Build orchestrator (v28.0)
├── repair_build.js      # Self-healing header shim generator
├── run_sigma_tests.sh   # Sovereign Atomic Test Runner
├── WIKI/                # Documentation and roadmap
└── .github/workflows/   # CI/CD pipeline
```

---

## Shard Architecture

Each **Shard** is a self-contained sovereign module:

```c
// Example: Creating a new shard in suites/S34_MyShard/shard_init.c
#include "sigma_kernel_types.h"
#include "sigma_libc.h"

void shard_init_S34_MyShard(void) {
    sigma_log("[S34] My Sovereign Shard initializing...");
    // Your shard logic here
}
```

**Rules for new shards:**
1. Place your shard in `suites/SXX_Name/`
2. Run `node repair_build.js` — it will automatically create any needed header shims
3. Your shard will be auto-discovered by `build_sovereign.sh`
4. Stem-based deduplication: each unique filename is compiled only once

---

## Submitting Changes

1. **Fork** the repository
2. **Create a branch**: `git checkout -b feat/my-shard`
3. **Make your changes** following the Code Style guide below
4. **Run the build**: `./build_sovereign.sh`
5. **Run tests**: `./run_sigma_tests.sh`
6. **Commit**: `git commit -m "feat: Add S34 MyShard — description"`
7. **Push** and open a Pull Request

### Commit Message Format
```
<type>: <short description>

Types: feat | fix | perf | docs | refactor | test | ci
```

---

## Code Style

- **C/C++**: No standard library — use `sigma_*` APIs exclusively
- **Headers**: Always include `sigma_kernel_types.h` first
- **ASM**: Use NASM syntax, `elf64` format, `[BITS 64]` (or `[BITS 32]` for Multiboot)
- **Naming**: `sigma_` prefix for kernel functions, `SIGMA_` for macros
- **No `malloc`/`printf`**: Use `sigma_malloc` / `sigma_print` instead

---

## Reporting Bugs

Use the [Bug Report template](.github/ISSUE_TEMPLATE/bug_report.md) and include:
- Build log output (from `./build_sovereign.sh`)
- Your OS and toolchain versions (`nasm --version`, `g++ --version`)
- The specific shard or file causing the issue

---

## Feature Requests

Use the [Feature Request template](.github/ISSUE_TEMPLATE/feature_request.md) and describe:
- Which Phase (1–7) your feature belongs to
- Which Suite (S01–S33) it would live in
- How it maintains the **Zero-Dependency Sovereignty** design principle


## SigmaOS Modular Architecture
When contributing, please place your code in the appropriate module under `modules/`. We follow a strict Microkernel Architecture approach. 
- Core: `modules/core`
- Security: `modules/security`
- Perf: `modules/perf`
- Ext: `modules/ext`
- Tools: `modules/tools`

Read our [Design Philosophy](WIKI/DESIGN_PHILOSOPHY.md) before submitting PRs.
