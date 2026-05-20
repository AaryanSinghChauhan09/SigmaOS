# CI Pipeline Guide


> How every GitHub Actions workflow is structured, what it validates, and how to interpret results.

---


**File**: `.github/workflows/01_Sovereign_Build.yml`

**Matrix**: `os × profile` ? 9 parallel jobs (Ubuntu / macOS / Windows × server / iot / dev)



---


**File**: `.github/workflows/02_Lattice_Verification.yml`

**Jobs**: Static Analysis, Formal Proofs (Kani), Entropy Fuzzing


Scans `sigmaos/core/src/`, `suites/S01_Genesis/`, `suites/S04_HAL/`, `suites/S08_Security/` for:



> `continue-on-error: true`  warnings are reported but don't block the build


Runs Rust Kani model checker on `suites/S08_Security/formal_proofs/`:


> `continue-on-error: true`  Kani runs are advisory; proofs improve over time


Builds `orchestrator/main.cpp` and fuzzes with profiles: `kali`, `tails`, `arch`


---


**File**: `.github/workflows/03_Web_Zenith.yml`


1. **cppcheck**  full static analysis on core + HAL + security

2. **Atomic module count**  reports how many `atomic_*.cpp/.hpp` files exist

3. **Sovereignty check**  scans for forbidden `#include <stdlib.h>` etc. in atomic modules

---



[?] = test passed, architecture verified
[*] = running, no verdict yet
[!] = warning  investigate but not fatal




