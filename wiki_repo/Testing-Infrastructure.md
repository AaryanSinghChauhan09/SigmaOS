# Testing Infrastructure

SigmaOS uses a four-layer test strategy. The goal: zero bugs escape to a release that have a reproducible test.

---

## Test Categories

```
tests/
├── unit/           C++ unit tests (Google Test, host-mode — no QEMU needed)
├── integration/    Shell scripts — boot in QEMU, measure, verify services
├── fuzz/           libFuzzer harnesses — malformed input finds security bugs
├── openqa/         openQA-style scenario matrix — full OS boot + screenshot
├── kernel/         Kernel-space regression tests (bash + proc filesystem)
├── posix/          POSIX compliance test suite (200 tests)
└── regression/     One test file per fixed CVE/bug — never regress
```

---

## Unit Tests (`tests/unit/`)

Run on the host machine (no VM needed). Fast feedback loop.

```bash

# Build and run all unit tests

cd tests/cpp_host && cmake -B build && cmake --build build
cd build && ctest --output-on-failure

# Run a specific test

./build/test_sigma_sched
./build/test_sigma_net
```

| Test file | What it covers |
|---|---|
| `test_sigma_sched.cpp` | MLFQ scheduler: level demotion, priority boost, anti-starvation |
| `test_sigma_net.cpp` | TLS 1.3, DNS/DoH, DHCP, WPA3/SAE — 22 tests |
| `test_sigma_ipc.cpp` | sigma-bus message delivery, capability gating |
| `test_sigma_fs.cpp` | SemanticFS xattrs, CryptFS key derivation |
| `test_sigma_gst.cpp` | GST calculation correctness (Indian tax law) |
| `test_sigma_pkg.cpp` | Package hash verification, manifest validation |

---

## Fuzz Tests (`tests/fuzz/`)

libFuzzer harnesses that find security bugs before attackers do. Run continuously in CI.

```bash

# Build fuzz target

clang++ -fsanitize=fuzzer,address -std=c++17 -Iinclude \
  tests/fuzz/fuzz_sigma_tcp.cpp \
  kernel/net/sigma_tcpip.c -o fuzz_tcp

# Run with 30-second budget

./fuzz_tcp -max_total_time=30 corpus/tcp/

# Run package fuzzer

clang++ -fsanitize=fuzzer,address -std=c++17 -Iinclude \
  tests/fuzz/fuzz_sigma_pkg.cpp \
  userland/pkg/sigma_acquire.cpp -o fuzz_pkg
./fuzz_pkg -max_total_time=30 corpus/pkg/
```

| Fuzzer | Attack surface |
|---|---|
| `fuzz_sigma_tcp.cpp` | TCP packet injection, option parsing, conntrack hash collisions |
| `fuzz_sigma_pkg.cpp` | Package header parse, manifest JSON, path traversal, signature verify |
| `fuzz_sigma_fs.cpp` | Malformed filesystem images, SigmaFS metadata |

**What fuzzing finds**: buffer overflows, integer overflows, use-after-free, infinite loops in parsers, NULL dereferences on malformed input.

---

## Integration Tests (`tests/integration/`)

Boot the real ISO in QEMU, verify services start, measure timing.

```bash

# Boot test (requires qemu-system-x86_64 and build/sigmaos.iso)

bash tests/integration/test_boot_sequence.sh

# Package install/remove/rollback

bash tests/integration/test_sigma_pkg.sh

# 2-node fleet sync

bash tests/integration/test_fleet_sync.sh
```

Pass criteria for `test_boot_sequence.sh`:

- Kernel boots and PID 1 starts: ✓

- `sigma-healthd` ready within 5 seconds of kernel boot: ✓

- All critical daemons started (busd, trustd, netd, watchdog): ✓

- No FAILED subsystems on first boot: ✓

- No unsigned kpatch modules loaded: ✓

---

## openQA Scenario Matrix (`tests/openqa/`)

Inspired by openSUSE openQA. Each scenario boots the OS in QEMU, runs automated interactions, and compares screenshots against reference "needles".

```bash

# Run a single scenario

python tests/openqa/sigma_scenarios.py zerotrust_revoke x86_64 standalone

# List all scenarios

python -c "from tests.openqa.sigma_scenarios import SCENARIOS; \
           [print(s) for s in SCENARIOS]"
```

**35 scenarios** covering:

- Boot (x86_64, aarch64, RTOS, cloud)

- Security: pledge SIGABRT, unveil ENOENT, ASLR, W^X

- ZeroTrust: allow, deny, revoke (Round 1 regression)

- CryptFS: mount, TPM2 key, wrong PCR (tampered boot)

- Packages: install, remove, rollback, dm-verity tamper

- Network: DHCP, DoH, TLS 1.3, WPA3/SAE, firewall

- Live patch: apply, revert, unsigned rejection

- Regression: one test per fixed bug

---

## Memory Tests (`tests/kernel/test_mm.sh`)

```bash
bash tests/kernel/test_mm.sh

# Tests: mmap/munmap, mprotect PROT_NONE, huge page availability,

#        /proc/self/maps parseable, stack growth, OOM score,

#        overcommit policy, large anonymous allocation (64 MiB)

```

---

## CI Pipeline Integration

The GitHub Actions workflow (`.github/workflows/sigma_ci.yml`) runs:

1. **Build check** — CMake configure + Ninja build, check-stubs

2. **Unit tests** — ctest on host

3. **Fuzz tests** — 30-second budget per harness

4. **Integration tests** — boot test in QEMU (Ubuntu 24.04 runner with KVM)

5. **openQA scenarios** — matrix of critical scenarios

6. **POSIX tests** — `tests/posix/run_posix_tests.sh`

7. **Memory tests** — `tests/kernel/test_mm.sh`

```yaml

# Fuzz targets in CI

fuzz-tests:
  runs-on: ubuntu-24.04
  strategy:
    matrix:
      target: [fuzz_sigma_tcp, fuzz_sigma_pkg]
  steps:
    - run: |
        clang++ -fsanitize=fuzzer,address -Iinclude \
          tests/fuzz/${{ matrix.target }}.cpp -o ${{ matrix.target }}
        ./${{ matrix.target }} -max_total_time=30
```

---

## Regression Test Policy

**Every fixed bug gets a test.** The workflow:

1. Bug reported / CVE filed

2. Reproduce with a minimal test case in `tests/regression/`

3. Fix the bug

4. Verify test passes

5. Test is permanently part of CI — that bug can never silently return

Current regression tests cover:

- `regression_pid1_loop` — PID 1 5-iteration bug (Round 1)

- `regression_sprintf_overflow` — ZeroTrust buffer overflow (Round 1)

- `regression_zt_revocation` — revocation check ordering bug (Round 1)

- `regression_cryptfs_zero_key` — CryptFS derive_key() stub (Issue #44)

- `regression_kyber_misuse` — Kyber used for signatures (Round 7)

---

## Tools Reference

| Tool | Purpose | Install |
|---|---|---|
| libFuzzer | Coverage-guided fuzzing | `clang -fsanitize=fuzzer` |
| AddressSanitizer | Detects memory bugs | `-fsanitize=address` |
| KASAN | Kernel address sanitizer | Build flag `SIGMA_KASAN=1` |
| Google Test | C++ unit test framework | `apt install libgtest-dev` |
| QEMU | VM for integration tests | `apt install qemu-system-x86` |
| pytest | Python test runner | `pip install pytest` |

---

*See also: [Building from Source](Building-from-Source) · [Security Model](Security-Model) · [FAQ](FAQ) · [Contributor Roadmap](Contributor-Roadmap)*
