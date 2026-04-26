# SigmaOS — CI/CD Pipeline Guide

> How every GitHub Actions workflow is structured, what it validates, and how to interpret results.

---

## 🏗️ Workflow 01 — Sovereign Build & Test

**File**: `.github/workflows/01_Sovereign_Build.yml`  
**Matrix**: `os × profile` → 9 parallel jobs (Ubuntu / macOS / Windows × server / iot / dev)

### What each step does

| Step | Command | Success means |
|------|---------|---------------|
| Purge Legacy Artifacts | `rm -rf build/` | Clean slate for this build |
| Build Orchestrator | `g++ -std=c++20 orchestrator/main.cpp` | Native CLI compiles cleanly |
| Switch Profile | `s-cli profile <name>` | Silicon profile activated |
| Build Lattice | `s-cli build x86_64` | All 5000+ atomic modules reported OK |
| Kernel Tests | `s-cli test --subsystem genesis` | Core allocator/scheduler pass |
| HAL Tests | `s-cli test --subsystem hal` | Driver probe + DMA verified |
| Userland Tests | `s-cli test --subsystem userland` | Process lifecycle validated |
| Benchmarks | `s-cli benchmark --run-all` | Perf + crypto benchmarks pass |

### Reading results


- ✅ Green across all 9 matrix jobs = sovereign build verified
- ❌ Any red = check the failing step output; usually a compile error in `orchestrator/main.cpp`

---

## 🔬 Workflow 02 — Lattice Verification

**File**: `.github/workflows/02_Lattice_Verification.yml`  
**Jobs**: Static Analysis, Formal Proofs (Kani), Entropy Fuzzing

### Static Analysis (cppcheck)

Scans `sigmaos/core/src/`, `suites/S01_Genesis/`, `suites/S04_HAL/`, `suites/S08_Security/` for:

- Memory errors (buffer overflows, use-after-free)
- Null pointer dereferences
- Undefined behavior

> `continue-on-error: true` — warnings are reported but don't block the build

### Formal Proofs (Kani)

Runs Rust Kani model checker on `suites/S08_Security/formal_proofs/`:

- `verify_dma_ipc_non_interference` — proves DMA and IPC cannot corrupt each other
- `verify_dispatch_capability_ownership` — proves capability tokens cannot be forged

> `continue-on-error: true` — Kani runs are advisory; proofs improve over time

### Entropy Fuzzing

Builds `orchestrator/main.cpp` and fuzzes with profiles: `kali`, `tails`, `arch`

- Verifies the CLI handles all profile strings without crashing

---

## 🛡️ Workflow 03 — Native Quality Gate

**File**: `.github/workflows/03_Web_Zenith.yml`

### Steps

1. **cppcheck** — full static analysis on core + HAL + security
2. **Compiler verification** — orchestrator compiles with `-Wall -Wextra -Wpedantic`
3. **Atomic module count** — reports how many `atomic_*.cpp/.hpp` files exist
4. **Sovereignty check** — scans for forbidden `#include <stdlib.h>` etc. in atomic modules

---

## 🚦 Interpreting CI Results at a Glance

```
[✓] = test passed, architecture verified
[*] = running, no verdict yet
[!] = warning — investigate but not fatal
```

## 📈 Key Metrics Tracked Per CI Run


- Atomic module count (target: grows every sprint)
- Zero stdlib imports in atomic modules (must stay at 0)
- Orchestrator compile time (target: < 2s)
- All 9 matrix jobs green (target: 100%)


