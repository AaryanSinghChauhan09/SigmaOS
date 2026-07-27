# SigmaOS CI Pipeline Guide

> Every commit to `main` runs 30+ CI checks. This page explains each one
> and how to fix common failures.

---

## CI Architecture

```
PR opened / push to main
         │
         ├── sigma_dev_workflow.yml ──── commit-lint
         │                         ├── rust fmt + clippy
         │                         ├── nim check
         │                         ├── SPDX headers
         │                         ├── unsafe audit
         │                         ├── cargo-audit (CVEs)
         │                         ├── SBOM generation
         │                         └── AI PR summary (PR only)
         │
         ├── sigma_multiarch_ci.yml ──── x86_64 kernel build
         │                         ├── aarch64 kernel build
         │                         ├── riscv64gc kernel build
         │                         ├── sigma-sh build (host)
         │                         ├── QEMU smoke boot (x86_64)
         │                         ├── OCI compat (hello-world, alpine)
         │                         └── reproducible build check
         │
         ├── pr_quality_gate.yml ─────── conventional commit lint
         │                         ├── format check
         │                         ├── license header check
         │                         └── build matrix
         │
         └── [nightly only] ─────────── syscall fuzz (30s)
                                   ├── cargo-audit full scan
                                   └── kabi regression check
```

---

## How to Fix Common CI Failures

### ❌ `commit-lint` fails

```
✖ subject may not be empty [subject-empty]
✖ type may not be empty [type-empty]
```

**Fix:** Your commit message doesn't follow Conventional Commits.

```bash
# Amend your last commit message
git commit --amend
# Enter: feat(kernel): your description here
```

---

### ❌ `rustfmt check` fails

```
Diff in kernel/core/sigma_pledge.rs at line 42
```

**Fix:**
```bash
# Format all Rust files
cargo +nightly fmt

# Or just the specific file
rustfmt +nightly kernel/core/sigma_pledge.rs

# Re-stage and amend
git add -u && git commit --amend --no-edit
```

---

### ❌ `clippy` fails

```
error: this looks like you are trying to swap `a` and `b`
```

**Fix:** Address the warning or suppress with a documented reason:
```rust
// clippy is wrong here because X
#[allow(clippy::manual_swap)]
```

---

### ❌ `SPDX headers` fails

```
MISSING SPDX: kernel/core/my_new_file.rs
```

**Fix:** Add to the top of your file:
```rust
// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/my_new_file.rs — description
```

---

### ❌ `cargo-audit` fails

```
error[RUSTSEC-2024-XXXX]: vulnerability in crate-name
```

**Fix:** Update the affected dependency:
```bash
cargo update -p crate-name
# Or pin to a safe version in Cargo.toml:
# crate-name = "=1.2.3"
```

---

### ❌ `QEMU smoke boot` fails

```
[qemu-smoke] WARN — no recognisable output
```

**Fix:** The kernel didn't print any expected output. Check:
1. `arch/boot/sovereign_boot.asm` — does it print to serial?
2. `kernel/core/sigma_irq.rs` — is serial console initialized (`serial_puts`)?
3. `kernel/src/main.rs` — does it call `print_str("Welcome...")`?

The QEMU boot test is currently **non-blocking** (warning only) until the kernel produces a stable boot.

---

### ❌ `ABI check` fails

```
❌ REMOVED (1 symbols — ABI BREAK): fn sigma_request_irq
```

**Fix:**
- If removal is intentional: bump `kabi/src/version.rs`, run `python kabi/check.py snapshot`
- If accidental: restore the symbol or add a compatibility shim

---

### ❌ `cross-build aarch64` fails

```
error[E0554]: #![feature(abi_x86_interrupt)] is not supported on aarch64
```

**Fix:** Wrap x86-specific code in `#[cfg(target_arch = "x86_64")]`:
```rust
#[cfg(target_arch = "x86_64")]
pub unsafe fn sigma_pic_init(...) { ... }
```

---

## Running the Full CI Suite Locally

```bash
# 1. Rust checks
cargo fmt --check
cargo clippy -- -D warnings
cargo test

# 2. SPDX headers
find kernel security -name '*.rs' | while read f; do
  head -3 "$f" | grep -q SPDX || echo "MISSING: $f"
done

# 3. cargo-audit
cargo install cargo-audit && cargo audit

# 4. ABI check
python kabi/check.py check

# 5. QEMU boot (if QEMU installed)
./qemu-boot.sh smoke

# 6. Nim check
nim check userland/tools/sigma_top.nim
```

---

## CI Badges

Add to your fork's README:

```markdown
![CI](https://github.com/AaryanSinghChauhan09/SigmaOS/workflows/SigmaOS%20Developer%20Workflow/badge.svg)
![Multi-Arch](https://github.com/AaryanSinghChauhan09/SigmaOS/workflows/SigmaOS%20Multi-Arch%20CI/badge.svg)
```

---

## Nightly Fuzz Results

Fuzz runs are uploaded as CI artifacts. Download from:
`Actions → nightly run → Artifacts → sigmaos-fuzz-results`

If a fuzz crash is found:
1. Download the crash input from the artifact
2. Open an issue with label `bug` `security` `fuzz-found`
3. Reference `SECURITY_POLICY.md` if it's a security-sensitive crash

---

*See also: [CONTRIBUTING.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/CONTRIBUTING.md) ·
[Developer Workflow](Developer-Workflow-And-Governance) ·
[Security Model](Security-Model)*


---
## Merged from CI_Pipeline_Guide.md
# CI Pipeline Guide

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

1

> `continue-on-error: true`  warnings are reported but don't block the build

1

Runs Rust Kani model checker on `suites/S08_Security/formal_proofs/`:

1

> `continue-on-error: true`  Kani runs are advisory; proofs improve over time

1

Builds `orchestrator/main.cpp` and fuzzes with profiles: `kali`, `tails`, `arch`

1

---

1

**File**: `.github/workflows/03_Web_Zenith.yml`

1

1. **cppcheck**  full static analysis on core + HAL + security

2. **Atomic module count**  reports how many `atomic_*.cpp/.hpp` files exist

3. **Sovereignty check**  scans for forbidden `#include <stdlib.h>` etc. in atomic modules

---

1

1

[?] = test passed, architecture verified
[*] = running, no verdict yet
[!] = warning  investigate but not fatal

1

1

1

1
