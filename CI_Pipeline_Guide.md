1


> How every GitHub Actions workflow is structured, what it validates, and how to interpret results.

---


1


**File**: `.github/workflows/01_Sovereign_Build.yml`

**Matrix**: `os × profile` ? 9 parallel jobs (Ubuntu / macOS / Windows × server / iot / dev)


1



1


---


1


**File**: `.github/workflows/02_Lattice_Verification.yml`

**Jobs**: Static Analysis, Formal Proofs (Kani), Entropy Fuzzing


1


Scans `sigmaos/core/src/`, `suites/S01_Genesis/`, `suites/S04_HAL/`, `suites/S08_Security/` for:


1


<<<<<<< HEAD

1


=======
>>>>>>> c682b9ae193869d405d851dfbeb13314cb964f9f
> `continue-on-error: true` — warnings are reported but don't block the build


1


Runs Rust Kani model checker on `suites/S08_Security/formal_proofs/`:


1


> `continue-on-error: true` — Kani runs are advisory; proofs improve over time


1


Builds `orchestrator/main.cpp` and fuzzes with profiles: `kali`, `tails`, `arch`


1


---


1


**File**: `.github/workflows/03_Web_Zenith.yml`


1


<<<<<<< HEAD


1. **cppcheck** — full static analysis on core + HAL + security
2. **Atomic module count** — reports how many `atomic_*.cpp/.hpp` files exist



=======
1. **cppcheck** — full static analysis on core + HAL + security
2. **Atomic module count** — reports how many `atomic_*.cpp/.hpp` files exist
>>>>>>> c682b9ae193869d405d851dfbeb13314cb964f9f
3. **Sovereignty check** — scans for forbidden `#include <stdlib.h>` etc. in atomic modules

---


1



1


[?] = test passed, architecture verified
[*] = running, no verdict yet
[!] = warning — investigate but not fatal


1



1



1
<<<<<<< HEAD



1
=======
>>>>>>> c682b9ae193869d405d851dfbeb13314cb964f9f

