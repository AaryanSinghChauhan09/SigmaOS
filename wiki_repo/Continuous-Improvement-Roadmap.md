# SigmaOS — Continuous Improvement Roadmap
## Versioning · Code Review · Testing Philosophy · Documentation System
## Automation Pipelines Deep-Dive · CLI UX Patterns · Feedback Loops

Continues from all previous roadmap documents. Covers the remaining
engineering dimensions not yet addressed in earlier docs.


---

## 1. Versioning Strategy

### VS1 — Semantic Versioning Policy

```
Format: vMAJOR.MINOR.PATCH[-channel]
Example: v16.0.3-apex, v15.1.0-stable, v17.0.0-rc1

MAJOR: Incompatible kernel ABI or shard manifest change
MINOR: New features, new profession apps, new hardware support
PATCH: Bug fixes, security patches (via sigma-kpatch where possible)
channel: stable | rc | beta | nightly
```

| Rule | Enforcement | Branch | 
| ------ | ------------ | -------- | 
| No MAJOR bump without 90-day deprecation notice | Release gate | all | 
| PATCH releases must not break `SIGMA_STABLE` symbols | `make check-abi` CI | all | 
| `SIGMA_EXPERIMENTAL` APIs can break in MINOR | Documented in sigma_abi.h | all | 
| Nightly builds auto-tagged `vX.Y.Z-nightly-YYYYMMDD` | `.github/workflows/sigma_release.yml` | all | 
| RC must pass all Q0 quality gates | `sigma_quality_check.sh --strict` | `prepare-sigmaos-launch` | 
| Every release has signed SBOM | `scripts/gen_sbom.sh` | all | 

### VS2 — Branch-to-Version Mapping

| Git branch | Version series | Auto-tag | 
| ----------- | --------------- | --------- | 
| `main` | v15.x stable | On CI green | 
| `tools-dev` | v15.1-dev | Nightly | 
| `kernel-exp` | v16.0-dev | Nightly after boot | 
| `release/standalone` | v16.0-standalone-rc | On CI green | 
| `release/cloud` | v17.0-cloud-rc | On CI green | 
| `release/mobile` | v16.0-mobile-rc | After RPi4 boot | 
| `prepare-sigmaos-launch` | v15.1.0 | Manual tag | 

### VS3 — Changelog Automation

```bash
# Auto-generate from conventional commits:
sigma_automation.sh gen-changelog

# Format (CHANGELOG.md):
# ## v16.0.0-apex — 2026-Q4
# ### Features
#   - feat(kernel): MLFQ scheduler with MCS budget accounting
#   - feat(drivers): VirtIO-GPU DRM/KMS compositor pipeline
# ### Bug fixes
#   - fix(crypto): CryptFS Argon2id replaces zero-byte derive_key (#44)
# ### Security
#   - security(pqc): Replace PRNG placeholders with real NTT
# ### Breaking changes
#   - BREAKING(shard): SigmaShardManifest v2 requires recover() callback
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| Conventional commit validator | `.conform.yaml` | all | Already exists — enforce in CI | 
| Auto-changelog CI job | `.github/workflows/sigma_release.yml` | all | Run `gen_changelog.sh` on tag push | 
| CHANGELOG.md in repo root | `CHANGELOG.md` | all | Updated on every release | 
| Breaking change detection | `.github/workflows/sigma_ci.yml` | all | Scan commit messages for `BREAKING:` | 

---

## 2. Code Review Standards

### CR1 — Review Checklist (enforced via PR template)

Every PR to `main` or `release/*` must answer:

```markdown
## Code Review Checklist
### Correctness
- [ ] Logic is correct for happy path
- [ ] Error paths return correct sigma_err_t codes
- [ ] No silent failures (all errors propagated or logged)
- [ ] No undefined behaviour (UB sanitizer clean)

### Security
- [ ] No hardcoded credentials or secrets
- [ ] SPDX-License-Identifier present on new files
- [ ] No raw pointer arithmetic in public APIs
- [ ] PII (Aadhaar/PAN) masked in any log output

### OOP / Design
- [ ] Single responsibility: class does one thing
- [ ] Depends on abstractions, not implementations (DIP)
- [ ] [[nodiscard]] on all error-returning functions
- [ ] RAII: resources released in destructor

### Testing
- [ ] Unit test added or updated
- [ ] Edge cases covered (empty input, OOM, timeout)
- [ ] No test-only code compiled into production binary

### Documentation
- [ ] CURRENT_PROBLEMS_MANIFEST.md updated if fixing a known issue
- [ ] Public API has Doxygen comment (brief + @param + @return)
- [ ] Man page updated if CLI changed
- [ ] wiki_repo/ updated if new feature or behaviour change
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| PR template with checklist | `.github/PULL_REQUEST_TEMPLATE.md` | all | Mandatory checkboxes — CI fails if unchecked | 
| Auto-assign reviewers | `.github/CODEOWNERS` | all | Subsystem owners auto-requested | 
| Review turnaround SLO | Community policy | all | First review within 48 hours | 
| Stale PR auto-close | `.github/workflows/` | all | No activity 90 days → auto-close with message | 
| Review coverage metric | CI | all | PRs with 0 approvals cannot merge to main | 

### CR2 — Architecture Review Process (ADR)

Big decisions get an Architecture Decision Record before implementation:

```markdown
# ADR-001: Scheduler Algorithm Choice
## Status: Accepted
## Context: Need scheduler for 600-shard lattice
## Decision: MLFQ + MCS budget accounting (seL4-inspired)
## Rationale:
  - MLFQ: fairness for interactive workloads
  - MCS: real-time budget for safety-critical shards
  - Formal verification path via Frama-C
## Consequences:
  - Positive: provably bounded latency for RT shards
  - Negative: more complex than round-robin; implement in phases
## Alternatives rejected:
  - Pure CFS: no hard RT guarantees
  - Preemptive EDF only: starvation for non-RT tasks
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| ADR directory | `docs/adr/` | `docs-update` | One `.md` file per major architectural decision | 
| ADR index | `docs/adr/README.md` | `docs-update` | Table of all ADRs with status | 
| Required ADR topics | `docs/adr/` | `docs-update` | Scheduler, PQC algo, SDF design, sigma-bus protocol, .spkg format | 
| ADR review process | `CONTRIBUTING.md` | `docs-update` | Any change affecting 3+ files requires ADR | 

---

## 3. Testing Philosophy

### TP1 — Test Pyramid for SigmaOS

```
         /\
        /  \
       / E2E\     ← 10%: OpenQA QEMU scenarios (boot_default, profession apps)
      /------\
     / Integr \   ← 20%: sigma-wine W1, GSTN sandbox, ABDM sandbox, fleet
    /----------\
   /  Unit Tests \  ← 70%: Every module, every function, every error path
  /--------------\
```

| Layer | Count target | Tool | Branch | 
| ------- | ------------- | ------ | -------- | 
| Unit tests | ≥ 200 | GTest via CMake | all | 
| Integration tests | ≥ 30 | Shell scripts + QEMU | all | 
| E2E scenarios | ≥ 10 | OpenQA sigma_scenarios.py | all | 
| Fuzz targets | ≥ 10 | AFL++ / libFuzzer | all | 
| Property-based | ≥ 5 | Custom proptest.cpp | `kernel-exp` | 
| Performance benchmarks | ≥ 6 | sigma-perf bench suite | `performance-optimized` | 
| Chaos tests | ≥ 7 | tests/chaos/*.sh | all | 

### TP2 — Test Naming Convention

```cpp
// Unit test: TEST(SubsystemName, BehaviourUnderCondition)
TEST(BuddyAllocator, AllocFreeSinglePageSucceeds) { ... }
TEST(BuddyAllocator, AllocFailsOnOOM) { ... }
TEST(SigmaCA, ComputeGSTR1WithValidGSTINSucceeds) { ... }
TEST(SigmaCA, ComputeGSTR1WithInvalidGSTINReturnsErrInval) { ... }

// Fuzz target: fuzz_<subsystem>_<input_type>
// fuzz_vfs_path.cpp
// fuzz_sigma_ca_gstin.cpp

// Chaos test: test_<scenario>_<expected_outcome>.sh
// test_nic_crash_driver_restarts.sh
// test_3boot_fail_rollback_triggers.sh
```

### TP3 — Test-Driven Development for Critical Paths

For these subsystems, write tests FIRST before implementation:

| Subsystem | Reason | Required tests before coding | 
| ----------- | -------- | ------------------------------ | 
| `sigma_argon2id.cpp` | Security-critical; wrong impl = no encryption | KAT vectors from RFC 9106 | 
| `sigma_kyber1024.cpp` | PQC correctness | NIST KAT test vectors | 
| `sigma_tls_handshake.cpp` | Network security | RFC 8446 test vectors | 
| `sigma_accounts_post()` | Financial data | Double-entry invariant tests | 
| `sigma_pe_loader.cpp` | Binary loading | Known-good PE files | 
| VFS path normaliser | Security | Path traversal fuzz corpus | 

---

## 4. Documentation System

### DS1 — Documentation Architecture

```
docs/                     ← Developer reference
  adr/                    ← Architecture Decision Records
  api/html/               ← Doxygen auto-generated
  examples/               ← Working code examples
  man/                    ← Man pages for all 55 CLI tools
  tutorials/              ← Step-by-step guides

wiki_repo/                ← User-facing wiki (GitHub Wiki)
  roadmaps/               ← All roadmap documents
  guides/                 ← How-to guides per profession
  reference/              ← API quick-reference

CONTRIBUTING.md           ← Contributor guide
CHANGELOG.md              ← Auto-generated release notes
CURRENT_PROBLEMS_MANIFEST.md ← Active issues tracker
```

### DS2 — Doc-per-PR Policy

```yaml
# .github/PULL_REQUEST_TEMPLATE.md mandatory section:
## Documentation
- [ ] New public API has Doxygen comment (/// @brief, @param, @return)
- [ ] CURRENT_PROBLEMS_MANIFEST.md updated if fixing known issue
- [ ] Man page updated if any CLI command changed
- [ ] wiki_repo/ page updated if user-visible behaviour changed
- [ ] Example added to docs/examples/ if new capability introduced
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| Doxygen CI job | `.github/workflows/sigma_ci.yml` | all | Fail if public API missing `@brief` | 
| Man page generator | `scripts/sigma_docs_cli.sh` | `docs-update` | Auto-gen from `--help` output + hand-written sections | 
| Example test runner | `scripts/sigma_docs_cli.sh` | `docs-update` | `sigma-docs test-examples` — verify all examples compile | 
| Wiki freshness check | `scripts/sigma_quality_check.sh` | all | Warn if wiki_repo 7+ days behind main | 
| Broken link scanner | `scripts/sigma_docs_cli.sh` | `docs-update` | `sigma-docs check` — scan all .md for dead links | 

### DS3 — India-Language Documentation

```bash
# Every profession app guide available in 6 Indian languages:
sigma-docs serve --lang hi    # Hindi docs server
sigma-docs serve --lang ta    # Tamil

# File structure:
wiki_repo/
  hi/                      # Hindi translations
    Sigma-CA-Guide.md
    Sigma-Agri-Guide.md
  ta/                      # Tamil translations
    ...
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| Hindi getting-started | `wiki_repo/hi/Getting-Started.md` | `docs-update` | Translate 5-command quick-start to Hindi | 
| Hindi sigma-ca guide | `wiki_repo/hi/Sigma-CA-Guide.md` | `docs-update` | Step-by-step GSTR-3B filing in Hindi | 
| Tamil sigma-agri guide | `wiki_repo/ta/Sigma-Agri-Guide.md` | `docs-update` | MSP + PMFBY in Tamil | 
| i18n doc CI | `.github/workflows/sigma_ci.yml` | `docs-update` | Warn if English doc updated but Hindi not | 
| Community translation tracker | `wiki_repo/Translation-Status.md` | `docs-update` | Table: page × language × status | 

---

## 5. Automation Pipeline Deep-Dive

### AP1 — Full CI/CD Flow

```
Developer pushes code
  └── GitHub Actions triggers sigma_ci.yml
        ├── lint (clang-tidy + markdownlint)
        ├── build (3 profiles × CMake + Ninja)
        ├── unit-tests (GTest, all modules)
        ├── fuzz (AFL++ 30s each target)
        ├── security-scan (CodeQL + secrets grep)
        ├── reproducible-build (2 builds → SHA256 diff)
        └── QEMU boot (standalone + microkernel + cloud)

On merge to main:
  └── sigma_quality_check.sh --strict
        └── sigma_automation.sh wiki-sync
              └── sigma_git_sync.sh (push wiki_repo)

On release tag vX.Y.Z:
  └── make iso (reproducible)
        └── sigma_automation.sh sign-release
              └── sigma_automation.sh sbom
                    └── sigma_automation.sh publish
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| Wire QEMU to real boot | `.github/workflows/sigma_qemu.yml` | all | Replace `echo` stubs with real qemu invocation | 
| Nightly benchmark CI | `.github/workflows/sigma_ci.yml` | `performance-optimized` | `@daily` schedule: bench_boot + bench_pqc + bench_sched | 
| India Stack sandbox weekly | `.github/workflows/sigma_ci.yml` | `release/standalone` | `@weekly`: GSTN + ABDM sandbox API tests | 
| Release pipeline | `.github/workflows/sigma_release.yml` | `prepare-sigmaos-launch` | Full: ISO → sign → SBOM → publish → notify | 
| Auto-label by subsystem | `.github/workflows/` | all | Label `kernel/net/zenith/compat` by path | 

### AP2 — sigma-automation.sh Complete Command Set

```bash
# Current (all real):
sigma_automation.sh backup
sigma_automation.sh update
sigma_automation.sh update-check
sigma_automation.sh recovery-check
sigma_automation.sh meta-check
sigma_automation.sh wiki-sync
sigma_automation.sh quality-check [--strict]

# To add:
sigma_automation.sh release          # tag + sign + publish ISO
sigma_automation.sh sign-release     # ML-DSA-87 sign ISO
sigma_automation.sh qemu-test        # real QEMU boot test
sigma_automation.sh perf-bench       # run 6 benchmarks
sigma_automation.sh sbom             # CycloneDX SBOM generate
sigma_automation.sh india-sync       # refresh offline India data
sigma_automation.sh fleet-sync       # push policy to all devices
sigma_automation.sh lint             # clang-tidy + markdownlint
sigma_automation.sh gen-changelog    # CHANGELOG.md from git log
sigma_automation.sh clean            # remove build artefacts
sigma_automation.sh size-check       # verify module size budgets
sigma_automation.sh dep-scan         # foreign dependency audit
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| `cmd_release()` | `scripts/sigma_automation.sh` | `prepare-sigmaos-launch` | `git tag -s vX.Y.Z` + ISO build + sign + upload | 
| `cmd_qemu_test()` | `scripts/sigma_automation.sh` | all | `qemu-system-x86_64 -cdrom SigmaOS.iso` + assert prompt | 
| `cmd_size_check()` | `scripts/sigma_automation.sh` | all | `size` each .so + kernel image vs budget table | 
| `cmd_dep_scan()` | `scripts/sigma_automation.sh` | all | Grep for `#include <openssl/`, `#include <glib.h>` etc. | 

---

## 6. Reducing Predefined Functions & Libraries

### PF1 — sigma-nanolib (Zero-Dependency Standard Library)

**Goal:** Every kernel and profession app function uses sigma-nanolib instead
of `<string.h>`, `<stdlib.h>`, `<stdio.h>`, or any system libc.

**Current:** `sigma_libc.h` and `klib/include/sigma_nanolib.h` exist.
Not all code uses them — many files still `#include <string.h>`.

#### sigma-nanolib complete function inventory

```cpp
// klib/include/sigma_nanolib.h — complete sovereign standard library

// Memory
void* sigma_memcpy(void* dst, const void* src, sigma_usize n);
void* sigma_memmove(void* dst, const void* src, sigma_usize n);
void* sigma_memset(void* ptr, int c, sigma_usize n);
int   sigma_memcmp(const void* a, const void* b, sigma_usize n);
void* sigma_memchr(const void* s, int c, sigma_usize n);
void  sigma_memzero(void* ptr, sigma_usize n);      // secure zero
void  sigma_secure_memzero(volatile void* ptr, sigma_usize n); // no-opt

// String
sigma_usize sigma_strlen(const char* s);
char*  sigma_strcpy(char* dst, const char* src);
char*  sigma_strncpy(char* dst, const char* src, sigma_usize n);
char*  sigma_strcat(char* dst, const char* src);
int    sigma_strcmp(const char* a, const char* b);
int    sigma_strncmp(const char* a, const char* b, sigma_usize n);
int    sigma_strcasecmp(const char* a, const char* b);
char*  sigma_strchr(const char* s, int c);
char*  sigma_strstr(const char* hay, const char* needle);
sigma_usize sigma_strlcpy(char* dst, const char* src, sigma_usize sz);

// Formatting (no printf — sigma-specific)
sigma_usize sigma_snprintf(char* buf, sigma_usize sz, const char* fmt, ...);
sigma_usize sigma_vsnprintf(char* buf, sigma_usize sz, const char* fmt, va_list ap);
void sigma_printf(const char* fmt, ...);    // to sigma-log, not stdout

// Conversion
sigma_s64  sigma_atoi(const char* s);
sigma_u64  sigma_atoull(const char* s);
sigma_f64  sigma_atof(const char* s);
char*      sigma_itoa(sigma_s64 v, char* buf, int base);
char*      sigma_ulltoa(sigma_u64 v, char* buf, int base);

// Unicode (India Stack needs UTF-8/UTF-16)
sigma_usize sigma_utf8_len(const char* s);
sigma_u32   sigma_utf8_decode(const char** s);
sigma_usize sigma_utf8_encode(sigma_u32 cp, char* out);
sigma_usize sigma_utf16_to_utf8(const sigma_u16* src, sigma_usize slen,
                                  char* dst, sigma_usize dlen);

// Math (no libm)
sigma_s64  sigma_abs(sigma_s64 x);
sigma_u64  sigma_min(sigma_u64 a, sigma_u64 b);
sigma_u64  sigma_max(sigma_u64 a, sigma_u64 b);
sigma_u64  sigma_clamp(sigma_u64 v, sigma_u64 lo, sigma_u64 hi);
sigma_u64  sigma_align_up(sigma_u64 v, sigma_u64 align);
sigma_u64  sigma_align_down(sigma_u64 v, sigma_u64 align);
sigma_bool sigma_is_power_of_2(sigma_u64 v);
sigma_u64  sigma_next_power_of_2(sigma_u64 v);
sigma_u32  sigma_popcount(sigma_u64 v);
sigma_u32  sigma_clz(sigma_u64 v);    // count leading zeros
sigma_u32  sigma_ctz(sigma_u64 v);    // count trailing zeros

// Sorting / searching
void sigma_qsort(void* base, sigma_usize nmemb, sigma_usize size,
                  int (*cmp)(const void*, const void*));
void* sigma_bsearch(const void* key, const void* base, sigma_usize nmemb,
                     sigma_usize size, int (*cmp)(const void*, const void*));

// Time (no gettimeofday)
sigma_u64 sigma_uptime_ns(void);     // nanoseconds since boot
sigma_u64 sigma_tsc_read(void);      // raw TSC value
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| Complete `sigma_nanolib.h` | `klib/include/sigma_nanolib.h` | `tools-dev` | All functions above declared | 
| `sigma_nanolib.cpp` implementations | `klib/sigma_nanolib.cpp` | `tools-dev` | Pure C++17, zero `#include <string.h>` | 
| `sigma_secure_memzero` (volatile) | `klib/sigma_nanolib.cpp` | `tools-dev` | Prevents compiler optimising out crypto key wipes | 
| UTF-8/UTF-16 converters | `klib/sigma_nanolib.cpp` | `tools-dev` | Needed by sigma-wine NT path → VFS path | 
| `sigma_printf` → sigma-log | `klib/sigma_nanolib.cpp` | `tools-dev` | Replaces `printf` — routes to serial + sigma-log | 
| CI: scan for forbidden includes | `.github/workflows/sigma_ci.yml` | all | Fail if `#include <string.h>` found in kernel/ or crypto/ | 
| Migrate kernel/*.c to sigma-nanolib | `kernel/core/` | `kernel-exp` | Replace `memcpy/memset/strlen` with sigma_ equivalents | 
| Migrate crypto/*.cpp | `crypto/` | `performance-optimized` | No libc in PQC code | 

### PF2 — Replacing Predefined C++ STL in Hot Paths

SigmaOS uses STL in non-kernel code but replaces it on hot paths:

| STL function | Sigma replacement | Reason | File | 
| ------------- | ----------------- | -------- | ------ | 
| `std::string` | `sigma_string_t` (SBO, stack-alloc) | No heap for short strings | `klib/sigma_string.h` | 
| `std::vector<T>` | `sigma_array<T, N>` (fixed-size) | No dynamic realloc in kernel | `klib/sigma_array.h` | 
| `std::unordered_map` | `sigma_hashmap<K,V>` (open addressing) | Cache-friendly, no indirection | `klib/sigma_hashmap.h` | 
| `std::function` | `sigma_fn<Ret(Args...)>` (no heap) | No heap allocation for callbacks | `klib/sigma_fn.h` | 
| `std::shared_ptr` | Explicit reference counting | Predictable destruction | Manual in drivers | 
| `std::mutex` | `sigma_spinlock_t` (CAS) | Lock-free for kernel | `klib/sigma_lockfree.h` | 
| `std::sort` | `sigma_qsort()` | Zero external dep | `klib/sigma_nanolib.cpp` | 

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| `sigma_string_t` (SBO ≤ 15 chars) | `klib/sigma_string.h` | `tools-dev` | Inline buffer for short strings, no `new` | 
| `sigma_array<T,N>` fixed container | `klib/sigma_array.h` | `tools-dev` | Stack-allocated, bounds-checked array | 
| `sigma_hashmap<K,V>` open addressing | `klib/sigma_hashmap.h` | `performance-optimized` | Robin Hood hashing, cache-line aligned | 
| `sigma_fn<>` zero-alloc callable | `klib/sigma_fn.h` | `tools-dev` | No heap, fits in 32 bytes | 
| CI: STL audit in kernel/ | `.github/workflows/sigma_ci.yml` | `kernel-exp` | Fail if `<vector>`, `<string>`, `<map>` found in kernel/ | 

---

## 7. User Experience (UX) Design System

### UX1 — Zenith Design Language

SigmaOS has one visual language: **Zenith Design Language (ZDL)**.
Every pixel of UI — system dialogs, profession apps, CLI output — follows it.

```
ZDL Core Principles:
  1. Clarity over decoration   — every element serves a purpose
  2. India-first aesthetics    — warm colours, Indian script support
  3. Performance-first         — never sacrifice responsiveness for beauty
  4. Accessibility by default  — WCAG 2.2 AA is the floor, not the ceiling
  5. Contextual density        — show more detail when user needs it
```

#### ZDL Colour System

```toml
# Zenith Design Language — canonical colour tokens

[color.base]
background        = "#1E1E2E"   # Catppuccin Mocha Base (dark)
surface           = "#313244"   # Surface0
overlay           = "#45475A"   # Surface1
border            = "#585B70"   # Surface2

[color.accent]
primary           = "#CBA6F7"   # Mauve (primary action)
secondary         = "#89B4FA"   # Blue (info)
success           = "#A6E3A1"   # Green
warning           = "#FAB387"   # Peach
error             = "#F38BA8"   # Red
india_saffron     = "#FF9933"   # India flag saffron (special)
india_green       = "#138808"   # India flag green (special)

[color.text]
primary           = "#CDD6F4"   # Text
secondary         = "#A6ADC8"   # Subtext1
disabled          = "#6C7086"   # Surface2
inverse           = "#1E1E2E"   # On accent

[color.light]
background        = "#EFF1F5"   # Catppuccin Latte Base
surface           = "#FFFFFF"
primary           = "#8839EF"   # Mauve light
india_saffron     = "#E67300"
india_green       = "#0E6B06"
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| ZDL colour tokens TOML | `zenith_desktop/themes/zdl-dark.sigma-theme` | `release/standalone` | Canonical dark theme as source of truth | 
| ZDL light variant | `zenith_desktop/themes/zdl-light.sigma-theme` | `release/standalone` | Latte-based light theme | 
| ZDL India variant | `zenith_desktop/themes/zdl-india.sigma-theme` | `release/standalone` | Saffron/white/green India flag palette | 
| WCAG 2.2 AA checker | `zenith_desktop/theme/sigma_theme_engine.cpp` | `release/standalone` | Every colour pair must meet 4.5:1 contrast | 
| Token usage enforcement | `scripts/check_theme_tokens.py` | `release/standalone` | Fail if hardcoded hex found outside theme files | 

#### ZDL Typography

```toml
[typography]
font_ui         = "Noto Sans"             # Latin, all scripts
font_devanagari = "Noto Sans Devanagari"  # Hindi, Marathi, Sanskrit
font_tamil      = "Noto Sans Tamil"
font_telugu     = "Noto Sans Telugu"
font_bengali    = "Noto Sans Bengali"
font_mono       = "JetBrains Mono"        # code, terminal
font_size_base  = 13                      # px equivalent
font_size_sm    = 11
font_size_lg    = 16
font_size_xl    = 20
line_height     = 1.5
```

#### ZDL Spacing & Motion

```toml
[spacing]
xs  = 4    # within component
sm  = 8    # between related elements
md  = 16   # between sections
lg  = 24   # between groups
xl  = 32   # page margins

[motion]
duration_fast   = 100   # ms — hover, toggle
duration_normal = 200   # ms — open, close, switch
duration_slow   = 400   # ms — page transition
easing          = "cubic-bezier(0.4, 0, 0.2, 1)"  # Material easing
reduce_motion   = false  # respect prefers-reduced-motion
```

### UX2 — UI Component Library (sigma-ui)

**New directory:** `zenith_desktop/ui/`

Every interactive element is a reusable component:

```cpp
// zenith_desktop/ui/sigma_button.cpp
class SigmaButton : public ISigmaWidget {
    sigma_err_t render(SigmaCanvas& c) override;
    sigma_err_t on_click(sigma_point_t pos) override;
    void set_label(const char* label);
    void set_variant(ButtonVariant v); // PRIMARY | SECONDARY | DANGER | GHOST
    void set_disabled(bool d);
    void set_loading(bool l);          // spinner while async operation runs
};

// zenith_desktop/ui/sigma_input.cpp
class SigmaInput : public ISigmaWidget {
    void set_placeholder(const char* hint);
    void set_type(InputType t);   // TEXT | NUMBER | CURRENCY_INR | GSTIN | AADHAAR
    void set_validator(sigma_fn<bool(const char*)> fn);
    void set_formatter(sigma_fn<void(char*, size_t)> fn);  // format as ₹1,23,456
};

// zenith_desktop/ui/sigma_table.cpp
class SigmaTable : public ISigmaWidget {
    void add_column(const char* header, sigma_u32 width_pct);
    void add_row(const char** cells, sigma_u32 ncols);
    void set_sortable(bool s);
    void set_filterable(bool f);
    void on_row_click(sigma_fn<void(sigma_u32 row)> fn);
};
```

| Component | File | Branch | Priority | 
| ----------- | ------ | -------- | --------- | 
| SigmaButton | `zenith_desktop/ui/sigma_button.cpp` | `release/standalone` | 🔴 Every app needs this | 
| SigmaInput (with GSTIN/PAN formatters) | `zenith_desktop/ui/sigma_input.cpp` | `release/standalone` | 🔴 Every profession app | 
| SigmaTable (sortable, filterable) | `zenith_desktop/ui/sigma_table.cpp` | `release/standalone` | 🟠 sigma-ca invoice list | 
| SigmaDialog (modal) | `zenith_desktop/ui/sigma_dialog.cpp` | `release/standalone` | 🟠 Confirmation prompts | 
| SigmaToast (notification) | `zenith_desktop/ui/sigma_toast.cpp` | `release/standalone` | 🟠 GST filing success | 
| SigmaProgress (bar + spinner) | `zenith_desktop/ui/sigma_progress.cpp` | `release/standalone` | 🟠 Package install | 
| SigmaCard (info card) | `zenith_desktop/ui/sigma_card.cpp` | `release/standalone` | 🟡 Dashboard widgets | 
| SigmaChart (bar/line/pie) | `zenith_desktop/ui/sigma_chart.cpp` | `release/standalone` | 🟡 GST trend chart | 
| SigmaCalendar | `zenith_desktop/ui/sigma_calendar.cpp` | `release/standalone` | 🟡 Filing deadline picker | 
| SigmaBadge (verified DID) | `zenith_desktop/ui/sigma_badge.cpp` | `release/standalone` | 🟡 "CA ✓" credential | 

### UX3 — Profession App UX Patterns

Every profession app follows the same layout:

```
┌─────────────────────────────────────────────────────────┐
│  sigma-ca                              🔔  👤 CA: Arjun ✓ │
├──────────────┬──────────────────────────────────────────┤
│              │                                          │
│  Navigation  │           Main Content Area              │
│  ──────────  │                                          │
│  Dashboard   │  [Primary action button]                 │
│  Clients  ←  │                                          │
│  GST Filing  │  ┌─────────────────────────────────┐   │
│  ITR         │  │ Data table (sortable, filterable)│   │
│  Audit       │  │ with Indian number formatting    │   │
│  Settings    │  └─────────────────────────────────┘   │
│              │                                          │
└──────────────┴──────────────────────────────────────────┘
```

| Pattern | Rule | Applied to | 
| --------- | ------ | ----------- | 
| Navigation sidebar | Left sidebar, 200px wide, icons + labels | All profession apps | 
| Page header | Title + primary CTA button + user DID badge | All screens | 
| Data tables | Sortable columns, filter bar, ₹ formatted | sigma-ca, sigma-accounts | 
| India number format | ₹1,23,45,678 (lakh/crore) | All monetary displays | 
| Date format | 28 June 2026 (dd Month YYYY) | All date displays | 
| Success toast | Green, 3 s auto-dismiss, action button | After GST filing, invoice create | 
| Error dialog | Red border, error code + human message + fix suggestion | All errors | 
| Loading state | Skeleton screen (not spinner) for initial load | Dashboard, reports | 

### UX4 — CLI UX Patterns (deeper)

```bash
# Progress with percentage and ETA:
sigma-pkg install sigma-ca
[████████░░░░░░░░░░░░]  42%  sigma-ca 2.1 MB/5.0 MB  ETA: 3s

# Success confirmation with summary:
sigma-ca gst file --period 2026-06
✓ GSTR-3B filed successfully
  GSTIN:   27ABCDE1234F1Z5
  Period:  June 2026
  ARN:     AA2706250007XXX
  Filed:   28 June 2026, 14:32 IST
  Next due: 20 July 2026

# Warning with actionable next steps:
sigma-agri enam register --fpo --district Amritsar
⚠ FPO registration requires SFAC certificate
  You have: Aadhaar ✓  Bank account ✓  Land records ✓
  Missing:  SFAC certificate
  Next step: sigma-gov sfac apply --district Amritsar
  Or:        sigma-agri enam register --as-individual

# Structured JSON output (--json flag):
sigma-agri msp --crop wheat --year 2026 --json
{
  "crop": "wheat",
  "category": "Rabi",
  "year": 2026,
  "msp_per_qtl": 2425,
  "msp_per_tonne": 24250,
  "cost_a2fl": 1506.0,
  "return_pct": 61.0,
  "procurement_agency": "FCI"
}
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| Progress bar with ETA | `userland/tools/sigma_progress_bar.cpp` | `tools-dev` | VT100 animated bar, ETA from download speed | 
| `--json` on every command | All CLI tools | `tools-dev` | Structured output for scripting and automation | 
| Success summary format | All mutating commands | `tools-dev` | Standard block: ✓ title + key-value summary | 
| Error with fix suggestion | All CLI tools | `tools-dev` | Error message + "Next step:" guidance | 
| Colour detection | All CLI tools | `tools-dev` | `isatty(1)` — colour in terminal, plain in pipe | 
| India number formatter | `userland/locales/sigma_l10n.cpp` | `release/standalone` | `sigma_format_inr(1234567)` → "₹12,34,567" | 
| Spinner for async ops | `userland/tools/sigma_spinner.cpp` | `tools-dev` | Braille Unicode spinner while waiting | 

---

## 8. User Interface Architecture

### UI1 — Zenith Rendering Pipeline

```
App draw call
  │
  ▼ sigma-display protocol (IPC)
  │
  ▼ Compositor receives surface update
  │   ZenithCompositor::composite_window()
  │   → Porter-Duff alpha blend (AVX-512/NEON)
  │   → Layer merge (windows + overlay + cursor)
  │
  ▼ Vulkan command buffer recording
  │   VkCmdDraw* → triangle strip for each window rect
  │   Text: HarfBuzz shape → FreeType2 glyph → atlas upload
  │   Effects: Vulkan compute shader (blur, shadow)
  │
  ▼ Present to DRM/KMS
  │   drmModeSetCrtc → page flip
  │   VRR/FreeSync: drmModeAtomicCommit with variable refresh
  │
  ▼ Display (1 frame max @ 120 Hz)
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| Porter-Duff alpha blend (SIMD) | `zenith_desktop/compositor/sigma_compositor.cpp` | `release/standalone` | AVX-512 8-pixel-wide OVER operator | 
| Vulkan command buffer pre-record | `zenith_desktop/compositor/sigma_vk_frame.cpp` | `release/standalone` | Pre-record per window, submit on vblank | 
| HarfBuzz text shaping integration | `zenith_desktop/compositor/sigma_font.cpp` | `release/standalone` | Complex scripts: Devanagari conjuncts | 
| FreeType2 glyph atlas | `zenith_desktop/compositor/sigma_font.cpp` | `release/standalone` | Upload once at startup, reuse every frame | 
| Vulkan compute blur shader | `zenith_desktop/shaders/blur.comp` | `release/standalone` | Gaussian blur for glassmorphism effect | 
| VRR/FreeSync support | `drivers/graphics/sigma_kms.cpp` | `drivers-dev` | `DRM_CAP_ADDFB2_MODIFIERS` + atomic commit | 
| Frame time metric | `zenith_desktop/compositor/sigma_compositor.cpp` | `release/standalone` | Rolling p50/p95/p99 frame time logged | 

### UI2 — Input Handling Architecture

```
Hardware event (keyboard/pointer/touch)
  │
  ▼ SDF Input Driver (Ring-3)
  │   PS/2 keyboard: scan code → keysym
  │   USB HID: HID report → keysym
  │   Touch: multitouch → pointer events
  │
  ▼ sigma-input daemon (userland)
  │   Key repeat timer
  │   IME integration (Inscript/phonetic)
  │   Accessibility (sticky keys, slow keys)
  │
  ▼ sigma-display protocol (IPC)
  │   Event → focused window
  │
  ▼ App receives event
  │   WM: focus change, tiling resize
  │   App: key input, pointer click
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| sigma-input daemon | `userland/daemons/sigma_inputd.cpp` | `release/standalone` | Consolidate keyboard + pointer events | 
| Key repeat timer | `userland/daemons/sigma_inputd.cpp` | `release/standalone` | 400 ms delay, 30 Hz repeat rate (configurable) | 
| IME event injection | `userland/ime/sigma_ime_cli.cpp` | `release/standalone` | IME inserts composed text via sigma-input | 
| Sticky keys accessibility | `userland/daemons/sigma_inputd.cpp` | `release/standalone` | Single-key modifier sequences | 
| Touch → pointer normalisation | `userland/daemons/sigma_inputd.cpp` | `release/mobile` | Touch events → pointer events for existing apps | 
| Multi-touch gesture recognition | `userland/daemons/sigma_inputd.cpp` | `release/mobile` | Pinch-to-zoom, 3-finger swipe workspace | 
| Input event CI test | `tests/ui/test_input.sh` | `release/standalone` | Inject synthetic events, verify app receives them | 

### UI3 — Window Management Quality

**Current:** BSP/master-stack real, remove_window BSP rebuild is TODO, animation stubs.

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| BSP tree rebuild on `remove_window` | `zenith_desktop/wm/sigma_tiling_wm.cpp` | `release/standalone` | Fix TODO: walk tree, remove leaf, rebalance | 
| Fibonacci layout mode | `zenith_desktop/wm/sigma_tiling_wm.cpp` | `release/standalone` | Spiral partition: first window 50%, next 50% of remainder | 
| Animated window spawn (opacity) | `zenith_desktop/compositor/sigma_compositor.cpp` | `release/standalone` | 200ms fade-in from 0.0 to 1.0 opacity | 
| Animated layout switch | `zenith_desktop/compositor/sigma_compositor.cpp` | `release/standalone` | Smooth 300ms transition between layouts | 
| Snap-to-edge (Windows-style) | `zenith_desktop/wm/sigma_tiling_wm.cpp` | `release/standalone` | Drag to edge → snap to 50% half-screen | 
| Multi-monitor spanning | `zenith_desktop/wm/sigma_tiling_wm.cpp` | `release/standalone` | Windows can span across two monitors | 
| WM smoke test suite | `tests/ui/test_tiling_wm.cpp` | `release/standalone` | 20 scenarios: add/remove/focus/fullscreen/float | 

---

## 9. Security — Additional Depth

### SEC1 — Security Audit Trail Quality

```bash
# Every security event produces a tamper-evident log entry:
sigma-audit log --filter security
# Output:
# 2026-06-28T14:32:01+05:30 [INFO] [sigma-mac] open("/sigma/data/ca.db") ALLOW
#   subject: sigma-ca[PID:1234] DID:arjun123
#   object:  /sigma/data/ca.db
#   rule:    policy.ca.data.read
#   sig:     ML-DSA-87:abc123...  ← every entry signed

# Verify chain of custody:
sigma-audit verify
# ✓ 1,247 entries verified. Chain intact
# ✓ No gaps detected. No tampering detected
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| ML-DSA-87 per log entry | `kernel/security/sigma_immutable_audit_trail.cpp` | all | Sign every entry individually | 
| Hash chain (each entry includes prev hash) | `kernel/security/sigma_immutable_audit_trail.cpp` | all | Tamper detection: any modification breaks chain | 
| WORM hardware register backup | `kernel/security/sigma_immutable_audit_trail.cpp` | `release/standalone` | Critical entries written to write-once register | 
| Audit replay tool | `userland/tools/sigma_audit_cli.cpp` | `tools-dev` | Verify full chain from genesis entry | 
| CERT-In 6-hour report | `userland/tools/sigma_audit_cli.cpp` | `release/standalone` | Auto-format incident report for MeitY | 

### SEC2 — sigma-ids (Intrusion Detection) Quality

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| Behavioural baseline learning | `kernel/security/sigma_ids.cpp` | `release/cloud` | 7-day baseline before alerting | 
| Anomaly scoring model | `kernel/security/sigma_anomaly_detector.cpp` | `release/cloud` | Isolation Forest on syscall frequency | 
| ML-powered false positive reduction | `userland/ai/sigma_ai_daemon.cpp` | `release/standalone` | sigma-ai confirms anomaly before alert | 
| STIX 2.1 threat intelligence | `userland/tools/sigma_siem_cli.cpp` | `release/cloud` | Import STIX feeds from CERT-In | 
| Automated quarantine | `kernel/security/sigma_ids.cpp` | `release/cloud` | Anomalous process → sigma-pod with no-network cap | 

---

## 10. Tools Ecosystem Roadmap

### TE1 — sigma-observatory (System Monitor)

```bash
sigma-observatory                    # full TUI dashboard

┌─ CPU ──────────────┐┌─ Memory ─────────────┐
│ Core 0: ████░  42% ││ Used:    892 MB       │
│ Core 1: ██░░░  28% ││ Free:    6.9 GB       │
│ Core 2: █████  61% ││ Slab:    234 MB       │
│ Core 3: ███░░  38% ││ Cache:   1.2 GB       │
└────────────────────┘└──────────────────────┘
┌─ Network ──────────┐┌─ Disk ───────────────┐
│ eth0  RX: 12 Mbps  ││ nvme0: 450 IOPS      │
│ eth0  TX:  4 Mbps  ││ Read:  230 MB/s      │
│ WiFi: Connected    ││ Write: 180 MB/s      │
└────────────────────┘└──────────────────────┘
┌─ Top Processes ────────────────────────────┐
│ PID   COMM          CPU%  MEM   CAP         │
│ 1234  sigma-ca       2.1  45MB  india.gstn  │
│ 5678  sigma-ai       8.3  2.1G  compute     │
└────────────────────────────────────────────┘
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| `/proc/sigma/` procfs stats | `kernel/vfs/sigma_procfs.cpp` | `kernel-exp` | cpu/mem/net/io/sched stats as VFS files | 
| TUI rendering (VT100) | `userland/tools/sigma_observatory.cpp` | `performance-optimized` | 1 s refresh, cursor positioning | 
| `--json` streaming mode | `userland/tools/sigma_observatory.cpp` | `performance-optimized` | Machine-readable for Grafana | 
| Prometheus endpoint | `userland/tools/sigma_observatory.cpp` | `release/cloud` | `sigma-observatory --prometheus :9090` | 
| Process capability display | `userland/tools/sigma_observatory.cpp` | `release/standalone` | Show each process's sigma-bus capabilities | 

### TE2 — sigma-doctor (Diagnostic Tool)

```bash
sigma-doctor
# Scanning SigmaOS health..
✓ Kernel: booted successfully (slot A)
✓ Boot time: 1.8s (target: < 2s)
✓ Memory: 892MB / 8GB used
✓ Network: eth0 up, IP 10.0.2.15, DNS reachable
⚠ Wi-Fi: driver sigma-drv-iwlwifi not loaded
  Fix: sigma-drv load sigma-drv-iwlwifi
⚠ PQC: ML-KEM using PRNG (not real NTT)
  Fix: sigma-pkg install sigma-pqc-native
✗ GSTN API: timeout (last checked 2h ago)
  Fix: sigma-net check --host gstn.gov.in
✓ sigma-ca: v1.0 installed and healthy
✓ sigma-agri: v1.0 installed and healthy
✓ Audit chain: 1,247 entries, chain intact

Summary: 2 warnings, 1 error
Run 'sigma-doctor --fix' to auto-resolve warnings
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| sigma-doctor daemon checks | `userland/tools/sigma_doctor_cli.cpp` | `tools-dev` | 15 health checks covering all subsystems | 
| `--fix` auto-remediation | `userland/tools/sigma_doctor_cli.cpp` | `tools-dev` | Run the fix command for each warning | 
| `--json` output | `userland/tools/sigma_doctor_cli.cpp` | `tools-dev` | Machine-readable for CI integration | 
| sigma-doctor on OOBE | `userland/installer/sigma_oobe.cpp` | `release/standalone` | Run after first boot, show results | 

---

## 11. Roadmap Index — All 9 Documents

| Document | Primary dimensions | Approx lines | 
| ---------- | -------------------- | ------------- | 
| [Quality-Stability-Performance-Roadmap](Quality-Stability-Performance-Roadmap) | Stability, Performance, Quality, UX, Security, Accessibility, DX | ~1,000 | 
| [Stability-Performance-Extended](Stability-Performance-Extended) | Energy, Reliability, Observability, Release, Network QA, India QA, Hardware, Rust | ~900 | 
| [Compatibility-Automation-Personalisation-Roadmap](Compatibility-Automation-Personalisation-Roadmap) | Linux/Win32/POSIX compat, Automation, Customisation, Personalisation | ~700 | 
| [Advanced-Quality-Roadmap](Advanced-Quality-Roadmap) | PQC depth, Network stack, Enterprise, AI/ML, i18n, Education, Rural, Community | ~700 | 
| [Systems-Excellence-Roadmap](Systems-Excellence-Roadmap) | Gaming, IoT, Dev tools, Packages, Updates, Multi-platform, Sprint plan | ~700 | 
| [Engineering-Principles-Roadmap](Engineering-Principles-Roadmap) | SOLID/OOP, Design patterns, CLI design, Optimisation, Refactoring | ~700 | 
| [Modularisation-Architecture-Roadmap](Modularisation-Architecture-Roadmap) | Shard system, Build modularity, Plugin API, Automation depth | ~700 | 
| [Sovereignty-UserDefined-Roadmap](Sovereignty-UserDefined-Roadmap) | Foreign dep reduction, User extensions, India-first, DID identity | ~700 | 
| [Continuous-Improvement-Roadmap](Continuous-Improvement-Roadmap) | Versioning, Code review, Testing, Docs, Automation pipelines, sigma-nanolib, ZDL, UI arch | ~800 | 

**Total: 9 documents, ~7,000 lines of actionable engineering roadmap.**

---

*See also: [Sovereignty UserDefined Roadmap](Sovereignty-UserDefined-Roadmap) · [Engineering Principles Roadmap](Engineering-Principles-Roadmap) · [Modularisation Architecture Roadmap](Modularisation-Architecture-Roadmap) · [Branch Development Roadmap](Branch-Development-Roadmap) · [Development Roadmap](Development-Roadmap)*
