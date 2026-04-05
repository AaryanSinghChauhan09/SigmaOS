# Σ SIGMAOS: Contributor Guidelines

> **Sovereign Protocol**: Every contribution must arrive with a test. No test, no merge.

## 🧪 Test Requirements

All PRs **must** include:

| Type | Requirement |
|------|-------------|
| New shard/feature | Unit test in `tests/test_<feature>.c` |
| Bug fix | Regression test that reproduces the bug before fix |
| Security change | Entry in `tests/test_security_fuzz.c` |
| API change | Updated integration test in `tests/test_integration.c` |
| CI/CD change | Dry-run validation in `.github/workflows/` |

## 🗂️ Test Directory Structure

```text
tests/
├── test_scheduler.c       # Unit: scheduler (MLFQ, preemption, zombie reap)
├── test_integration.c     # Integration: shard-to-shard interactions
├── test_security_fuzz.c   # Security: fuzzing, injection, privilege escalation
├── test_memory.c          # Memory: allocation, pressure, boundary
└── test_automations_dry.sh# Automation: script syntax dry-run
```

## 🏃 Running Tests Locally

```bash
# Compile and run all tests
make test

# Or individually:
gcc -std=c11 -O2 -Wall -o build/test-scheduler     tests/test_scheduler.c
gcc -std=c11 -O2 -Wall -o build/test-integration   tests/test_integration.c
gcc -std=c11 -O2 -Wall -o build/test-security-fuzz tests/test_security_fuzz.c

./build/test-scheduler
./build/test-integration
./build/test-security-fuzz
```

## 🔬 Test Writing Standards

### 1. Self-Contained

Tests must compile with **only** standard C11 headers (`stdio.h`, `stdint.h`, `string.h`).
Never include `kernel/` headers that require freestanding/bare-metal environments.

### 2. Use the Sovereign Test Macro

```c
#define SIGMA_TEST(name, cond) do { \
    if (cond) { printf("  [PASS] %s\n", name); g_passed++; } \
    else { printf("  [FAIL] %s  (line %d)\n", name, __LINE__); g_failed++; } \
} while(0)
```

### 3. Group Tests by Domain

```c
static void test_group_name(void) {
    printf("\n[GROUP] Description\n");
    sched_reset();  // Always reset state before each group
    SIGMA_TEST("test case description", condition);
}
```

### 4. Return Correct Exit Code

```c
int main(void) {
    // ... run groups ...
    return (g_failed == 0) ? 0 : 1;  // CI relies on exit code
}
```

## ⚙️ CI/CD Pipeline

Every push triggers:

| Pipeline | File | Scope |
|----------|------|-------|
| Zenith CI/CD | `sigma_zenith_ci.yml` | Full build, unit tests, multi-arch |
| Security Inspections | `sigmaos_security_inspections.yml` | Secret/unsafe function/shell audits |
| Sovereign Master | `sigmaos_sovereign_master.yml` | Benchmark, static analysis |
| Nightly Stress | `sigma_nightly_stress.yml` | Chaos, ASAN, perf (00:00 UTC) |

## 🔒 Security Testing Requirements

Security-impacting changes require:

- A new test case in `test_security_fuzz.c`
- Entry in the Sovereign Security Checklist:
  - No use of `gets`, `strcpy`, `strcat`, `sprintf`
  - All pointers NULL-checked before use
  - All string operations length-bounded
  - No hardcoded credentials or tokens

## 📏 Code Style

- **Standard**: C11 (`-std=c11`)
- **Warnings**: All code must compile clean under `-Wall -Wextra`
- **Memory**: No dynamic allocation in kernel shards (`malloc` forbidden — use sigma_libc)
- **Naming**: `snake_case` for functions, `UPPER_CASE` for macros, `PascalCase` for types

## 🔃 PR Checklist

- [ ] Tests compile with `gcc -std=c11 -O2 -Wall`
- [ ] All existing tests still pass (`./build/test-*`)
- [ ] New feature has a corresponding test group
- [ ] No hardcoded secrets or credentials
- [ ] No forbidden C functions (`gets`, `strcpy`, etc.)
- [ ] README updated if API surface changed

---

**Σ SIGMAOS**: Sovereign by design. Verified by test. Deployed with zero compromise.
