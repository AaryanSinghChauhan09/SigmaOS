# SigmaOS â€” CI/CD Pipeline Guide

> How every GitHub Actions workflow is structured, what it validates, and how to interpret results.

---

## ðŸ—ï¸ Workflow 01 â€” Sovereign Build & Test

**File**: `.github/workflows/01_Sovereign_Build.yml`

**Matrix**: `os Ã— profile` â†’ 9 parallel jobs (Ubuntu / macOS / Windows Ã— server / iot / dev)

### What each step does | Step | Command | Success means | |------|---------|---------------| | Purge Legacy Artifacts | `rm -rf build/` | Clean slate for this build | | Build Orchestrator | `g++ -std=c++20 orchestrator/main.cpp` | Native CLI compiles cleanly | | Switch Profile | `s-cli profile <name>` | Silicon profile activated | | Build Lattice | `s-cli build x86_64` | All 5000+ atomic modules reported OK | | Kernel Tests | `s-cli test --subsystem genesis` | Core allocator/scheduler pass | | HAL Tests | `s-cli test --subsystem hal` | Driver probe + DMA verified | | Userland Tests | `s-cli test --subsystem userland` | Process lifecycle validated | | Benchmarks | `s-cli benchmark --run-all` | Perf + crypto benchmarks pass | ### Reading results

- âœ… Green across all 9 matrix jobs = sovereign build verified
- âŒ Any red = check the failing step output; usually a compile error in `orchestrator/main.cpp`

---

## ðŸ”¬ Workflow 02 â€” Lattice Verification

**File**: `.github/workflows/02_Lattice_Verification.yml`

**Jobs**: Static Analysis, Formal Proofs (Kani), Entropy Fuzzing

### Static Analysis (cppcheck)

Scans `sigmaos/core/src/`, `suites/S01_Genesis/`, `suites/S04_HAL/`, `suites/S08_Security/` for:

- Memory errors (buffer overflows, use-after-free)
- Null pointer dereferences
- Undefined behavior

> `continue-on-error: true` â€” warnings are reported but don't block the build

### Formal Proofs (Kani)

Runs Rust Kani model checker on `suites/S08_Security/formal_proofs/`:

- `verify_dma_ipc_non_interference` â€” proves DMA and IPC cannot corrupt each other
- `verify_dispatch_capability_ownership` â€” proves capability tokens cannot be forged

> `continue-on-error: true` â€” Kani runs are advisory; proofs improve over time

### Entropy Fuzzing

Builds `orchestrator/main.cpp` and fuzzes with profiles: `kali`, `tails`, `arch`

- Verifies the CLI handles all profile strings without crashing

---

## ðŸ›¡ï¸ Workflow 03 â€” Native Quality Gate

**File**: `.github/workflows/03_Web_Zenith.yml`

### Steps

1. **cppcheck** â€” full static analysis on core + HAL + security
2. **Atomic module count** â€” reports how many `atomic_*.cpp/.hpp` files exist
3. **Sovereignty check** â€” scans for forbidden `#include <stdlib.h>` etc. in atomic modules

---

## ðŸš¦ Interpreting CI Results at a Glance

```

[âœ“] = test passed, architecture verified
[*] = running, no verdict yet
[!] = warning â€” investigate but not fatal

```

## ðŸ“ˆ Key Metrics Tracked Per CI Run

- Atomic module count (target: grows every sprint)
- Zero stdlib imports in atomic modules (must stay at 0)
- Orchestrator compile time (target: < 2s)
- All 9 matrix jobs green (target: 100%)
