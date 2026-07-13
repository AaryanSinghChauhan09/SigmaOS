# SigmaOS LLVM Toolchain Integration Guide

> **Status**: 📋 Planned | **Audience**: SigmaOS Developers | **Component**: `sigma-build`

---

## Overview

This guide details how SigmaOS integrates LLVM/Clang as its primary compiler toolchain for both kernel and userland development. It covers build profiles, sanitizer usage, LTO/PGO optimization, and reproducible build verification.

---

## 1. Toolchain Components

| Component | Version | Role |
|:----------|:--------|:-----|
| `clang` | 18.x | C/C++ frontend (for C shims) |
| `rustc` | nightly | Primary Rust compiler |
| `lld` | 18.x | Linker (deterministic, fast) |
| `llvm-ar` | 18.x | Archiver |
| `llvm-strip` | 18.x | Symbol stripper |
| `llvm-objcopy` | 18.x | Object file manipulation |

---

## 2. Build Profiles

### Development Profile (default)

```bash
$ sigma build
# Equivalent flags:
# -O1 -g -fsanitize=address,undefined -fno-omit-frame-pointer
# Fast compile, maximal debug info, sanitizers on
```

### Staging Profile

```bash
$ sigma build --profile staging
# Equivalent: -O2 -g1 (minimal debug)
# No sanitizers. Used for integration testing.
```

### Production Profile (LTO + PGO)

```bash
# Phase 1: Instrument
$ sigma build --profile pgo-instrument
$ sigma test --run-benchmarks      # generates profraw data

# Phase 2: Optimize
$ sigma build --profile production
# Equivalent: -O3 -flto=full -fprofile-use=merged.profdata
# + -march=native -fvectorize -fslp-vectorize
```

---

## 3. Sanitizer Reference

| Sanitizer | Flag | Detects |
|:----------|:-----|:--------|
| AddressSanitizer | `-fsanitize=address` | Heap/stack overflow, UAF |
| UndefinedBehavior | `-fsanitize=undefined` | Integer overflow, null deref |
| ThreadSanitizer | `-fsanitize=thread` | Data races |
| MemorySanitizer | `-fsanitize=memory` | Uninitialized reads |
| LeakSanitizer | `-fsanitize=leak` | Memory leaks |

> [!NOTE]
> Sanitizers are enabled by default in `development` profile.
> Production builds have sanitizers removed for performance.

---

## 4. Reproducible Builds Verification

```bash
$ sigma build --reproducible-check
Σ [BUILD] Build 1 complete: blake3:a1b2c3...
Σ [BUILD] Build 2 complete: blake3:a1b2c3...  ← identical
✓ Reproducible build verified
```

The CI pipeline runs this check on every PR against `main`.
