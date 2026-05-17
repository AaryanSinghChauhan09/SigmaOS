# SigmaOS Kernel Developer Handbook

## Version 15.0.0 — Zenith Release

---

## 1. Architecture Overview

SigmaOS uses a **microkernel-inspired shard lattice** where each subsystem is an isolated, independently-compiled shard. The kernel consists of 3 primary layers:

| Layer | Path | Purpose |
|-------|------|---------|
| **LAYER 0** | `kernel/core/hal/` | Hardware Abstraction & Boot |

| **LAYER 1** | `kernel/core/` | IPC, Scheduler, Memory, FS, Net |

| **LAYER 2** | `kernel/core/drivers/` | Hardware Drivers |

---

## 2. Concurrency Model & Formal Verification

### 2.1 Lock Hierarchy (Enforced)

All kernel shards MUST acquire locks in the following order to prevent deadlocks:

```text

1. SovereignMemoryPool::mutex

2. SovereignScheduler::runqueue_lock

3. SovereignNetStack::socket_lock

4. SovereignFS::inode_lock


```text

**Violation** of this order will trigger a `SovereignWatchdog` panic.

### 2.2 Atomic Operations

Prefer `__atomic_*` builtins over mutexes for counter updates:

```cpp

__atomic_fetch_add(&shard_refcount, 1, __ATOMIC_SEQ_CST);


```text

### 2.3 Formal Verification Checklist

Before merging any concurrency-related change:

- [ ] Verified with `ThreadSanitizer` (`-fsanitize=thread`)

- [ ] No `TOCTOU` (time-of-check/time-of-use) patterns introduced

- [ ] Lock ordering documented in shard header comment

- [ ] Stress-tested via `scripts/format_stress_test.sh`

---

## 3. Memory Management Rules

- **No `malloc`/`free`** in kernel shards — use `SovereignMemoryPool::alloc()`

- **RTOS shards** are forbidden from ALL dynamic allocation

- Run `SovereignMemoryPool::profile_leaks()` after each integration test

- Buddy allocation handles blocks ≥ 4KB; slab handles < 4KB objects

---

## 4. Shard Interface Contract

Every shard MUST:

1. Inherit from `SigmaOS::SigmaObject`

2. Implement `type_name()` returning a unique string

3. Use `SigmaSingleton<T>` if stateful

4. Include only relative headers (`../../../include/`)

---

## 5. Regression Test Requirements

All PRs to `main` must pass:

- `scripts/regression_check.sh` — functional correctness

- `scripts/format_stress_test.sh` — concurrency stress

- `scripts/fuzz_pqc.sh` — security/fuzzing

- GitHub Actions: CodeQL scan must return 0 critical alerts

---

## 6. Release Versioning

SigmaOS uses **Semantic Versioning**: `vMAJOR.MINOR.PATCH-FORMAT`

| Tag Example | Meaning |
|-------------|---------|
| `v15.0.0-main` | Core kernel stable release |
| `v15.1.0-app` | App layer feature release |
| `v15.0.1-hotfix` | Security patch |
