# Contributing to SigmaOS

Thank you for your interest in SigmaOS! This guide covers everything you need to get started.

---

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Getting Started](#getting-started)
3. [Project Structure](#project-structure)
4. [Development Workflow](#development-workflow)
5. [Coding Standards](#coding-standards)
6. [Issue Labels](#issue-labels)
7. [Pull Request Process](#pull-request-process)
8. [Testing](#testing)
9. [Documentation](#documentation)

---

## Code of Conduct

Be respectful. Critique code, not people. All contributors must follow the [Contributor Covenant](https://www.contributor-covenant.org/version/2/1/code_of_conduct/).

---

## Getting Started

### Prerequisites

| Tool | Purpose | Version |
|------|---------|---------|
| `x86_64-elf-gcc` | Bare-metal cross compiler | ≥ 12.0 |
| `nasm` | Bootloader assembly | ≥ 2.15 |
| `qemu-system-x86_64` | VM for testing | ≥ 7.0 |
| `make` | Build system | ≥ 4.3 |
| `grub-mkrescue` | ISO generation | ≥ 2.06 |
| `git` | Version control | ≥ 2.40 |

### Setup

```bash
# 1. Fork the repo on GitHub
# 2. Clone your fork
git clone https://github.com/YOUR_USERNAME/SigmaOS.git
cd SigmaOS

# 3. Add upstream remote
git remote add upstream https://github.com/AaryanSinghChauhan09/SigmaOS.git

# 4. Build the default profile
make PROFILE=standalone

# 5. Boot in QEMU
./scripts/qemu-boot.sh standalone
```

---

## Project Structure

```
SigmaOS/
├── kernel/          # Microkernel core (C/C++)
├── drivers/         # Bare-metal drivers
├── net/             # TCP/IP + network stack
├── fs/              # Ext4 + VFS
├── crypto/          # PQC implementations (Kyber, Dilithium)
├── hal/             # Hardware Abstraction Layer
├── zenith_desktop/  # Compositor + desktop environment
├── userland/        # Shell, pkg manager, tools
├── runtime/         # WASM/WASI, Linux compat
├── scripts/         # Build helpers, QEMU launchers
├── docs/            # Technical documentation
├── wiki_repo/       # GitHub Wiki source (synced)
└── .github/         # CI/CD workflows, issue templates
```

---

## Development Workflow

```
main ─── feat/your-feature ─── PR ─── review ─── merge
```

1. **Sync** your fork: `git fetch upstream && git rebase upstream/main`
2. **Branch** off main: `git checkout -b feat/my-feature`
3. **Code** following the standards below
4. **Test** — run the hardware test suite and ensure no regressions
5. **Commit** using Conventional Commits format (see below)
6. **Push** and open a PR

---

## Coding Standards

### Language Rules

- **C** for kernel core (`kernel/`, `net/`, `fs/`) — C11 standard
- **C++** for drivers, userland, Zenith — C++17, no exceptions, no RTTI
- **No external libraries** — zero libc/glibc/musl in kernel space
- **No `#include <stdlib.h>`, `<stdio.h>`, `<string.h>`** — ever

### Type System

Always use the SigmaOS sovereign types from `include/sigma_kernel_types.h`:

```c
// ✅ Correct
sigma_u32  my_count;
sigma_u64  my_address;
sigma_bool my_flag = SIGMA_TRUE;

// ❌ Wrong — violates zero-dependency mandate
uint32_t  my_count;
bool      my_flag = true;
```

### Naming

| Entity | Convention | Example |
|--------|-----------|---------|
| Files | `snake_case` | `sigma_nvme.cpp` |
| Functions (C) | `sigma_verb_noun` | `sigma_map_page()` |
| Classes (C++) | `PascalCase` | `SovereignVMM` |
| Constants | `SIGMA_UPPER_SNAKE` | `SIGMA_HEAP_SIZE` |
| Error codes | `K_ERR_UPPER` | `K_ERR_NOMEM` |

### Commit Format (Conventional Commits)

```
<type>(<scope>): <short description>

[optional body]
[optional footer]
```

| Type | When |
|------|------|
| `feat` | New feature or subsystem |
| `fix` | Bug fix |
| `refactor` | Code restructuring (no behavior change) |
| `docs` | Documentation only |
| `test` | Adding/fixing tests |
| `chore` | Build, CI, tooling |

Example:
```
fix(net): add missing closing brace in TCP_STATE_LISTEN handler

The if(flags & TCP_FLAG_SYN) block was missing its closing } before
the break, causing the SYN_RECV case to merge into the LISTEN case.

Fixes #F-07
```

---

## Issue Labels

| Label | Meaning |
|-------|---------|
| `bug` | Confirmed defect |
| `feat` | New feature request |
| `good-first-issue` | Suitable for new contributors |
| `subsystem:kernel` | Kernel-space change |
| `subsystem:drivers` | Driver change |
| `subsystem:net` | Networking stack |
| `subsystem:zenith` | Desktop compositor |
| `subsystem:crypto` | PQC / security |
| `priority:critical` | Blocks boot or breaks builds |
| `priority:high` | Significant functionality gap |

---

## Pull Request Process

1. **Title** follows `<type>(<scope>): description` format
2. **Description** references the issue: `Fixes #123`
3. All CI checks must pass (build + lint + tests)
4. One approving review required from a maintainer
5. Squash-merge to keep main history clean

---

## Testing

### Hardware Test Suite

```bash
# Run the full hardware test suite (simulated)
make test-hw

# Run for a specific profile
make test-hw PROFILE=iot-arm64
```

The suite is in `kernel/tests/sigma_hw_test.cpp` — add tests there for new drivers.

### Regression Tests

```bash
make test
```

---

## Documentation

- **Wiki** — update `wiki_repo/` for user-facing docs; changes sync to the GitHub Wiki automatically on push
- **Code comments** — every public function needs a doc comment
- **Architecture docs** — major subsystems need a `docs/` entry

---

*Questions? Open a [Discussion](https://github.com/AaryanSinghChauhan09/SigmaOS/discussions) or ping in Issues.*
