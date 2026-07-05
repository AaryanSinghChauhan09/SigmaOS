# SigmaOS — Modularisation & Architecture Roadmap

## Shard System · Module Boundaries · Dependency Management

## Build Modularity · Runtime Modularity · Feature Flags · Plugin System

---

## Current Modularisation State

| Area | State | Gap |
|------|-------|-----|
| 600-shard lattice design | ✅ Documented | No runtime shard loader |
| SDF Ring-3 driver isolation | ✅ Framework | No actual Ring-3 launch mechanism |
| sigma-bus IPC between modules | ⚠️ Header only | No capability token passing |
| Build profiles (standalone/cloud/microkernel) | ✅ CMake | Some subsystems not properly gated |
| `#ifdef SIGMA_MICROKERNEL_PROFILE` guards | ⚠️ Partial | Inconsistent across files |
| Dynamic module load/unload | ❌ Missing | All modules are statically linked |
| Plugin API for profession apps | ❌ Missing | Apps compiled into binary directly |
| Dependency graph enforcement | ❌ Missing | No `SIGMA_DEPENDS_ON()` at compile time |

---

## 1. Shard Lattice Architecture

### SL1 — Shard Definition Standard

Every shard is an independently loadable, versioned, capability-bounded unit.
This is the core architectural commitment of SigmaOS — never violated.

```cpp
// include/sigma_shard.h — every module must declare this:
struct SigmaShardManifest {
    const char*   name;           // "sigma-net-tcp"
    uint32_t      version;        // SemVer encoded: major<<16|minor<<8|patch
    const char*   description;
    const char*   author;
    const char*   license;        // SPDX identifier

    // What this shard needs from the kernel:
    const char**  required_caps;  // {"sigma.cap.net.tx", "sigma.cap.net.rx", nullptr}
    const char**  optional_caps;  // {"sigma.cap.crypto.pqc", nullptr}

    // What sigma-bus topics this shard publishes/subscribes:
    const char**  publishes;      // {"sigma.net.packet.rx", nullptr}
    const char**  subscribes;     // {"sigma.net.packet.tx", nullptr}

    // Lifecycle callbacks:
    sigma_err_t (*init)(void);
    sigma_err_t (*shutdown)(void);
    sigma_err_t (*health_check)(void);  // watchdog calls this every 1 s

    // Recovery handler — called by sigma-heal on crash:
    sigma_err_t (*recover)(const char* crash_reason);
};

// Registration macro — called at module link time:
#define SIGMA_DECLARE_SHARD(manifest_var) \
    __attribute__((used, section(".sigma_shards"))) \
    static const SigmaShardManifest* _shard_##manifest_var = &manifest_var;
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| `sigma_shard.h` header | `include/sigma_shard.h` | `kernel-exp` | Defines `SigmaShardManifest` struct |
| `.sigma_shards` ELF section | `linker.ld` | `kernel-exp` | Collect all shard manifests at link time |
| Shard loader at boot | `kernel/core/sigma_shard_loader.cpp` | `kernel-exp` | Walk `.sigma_shards` section, call `init()` in topo order |
| Shard dependency resolver | `kernel/core/sigma_shard_loader.cpp` | `kernel-exp` | Topological sort by `required_caps` |
| Shard health watchdog | `kernel/core/sigma_shard_watchdog.cpp` | `kernel-exp` | Call each shard's `health_check()` every 1 s |
| Shard crash → sigma-heal | `kernel/diagnostics/sigma_crash_reporter.cpp` | all | On shard segfault → call `recover()`, log, restart |

### SL2 — Shard Categories & Hierarchy

```
Level 0 — Microkernel core (always present, cannot be unloaded):
  sigma-sched   sigma-mm      sigma-irq    sigma-timer
  sigma-bus     sigma-caps    sigma-audit  sigma-boot

Level 1 — Hardware abstraction (loaded by sigma-dna hardware probe):
  sigma-drv-e1000    sigma-drv-nvme    sigma-drv-vga
  sigma-drv-i915     sigma-drv-iwlwifi sigma-drv-hda

Level 2 — System services (loaded by boot profile):
  sigma-vfs      sigma-net-tcp   sigma-net-udp   sigma-net-dns
  sigma-crypto   sigma-trust     sigma-mac       sigma-cgroup

Level 3 — Userland services (loaded on demand):
  sigma-ca       sigma-health    sigma-agri      sigma-zenith
  sigma-wine     sigma-ai        sigma-fleet     sigma-cron

Level 4 — Optional extensions (user-installed via sigma-pkg):
  sigma-posix-compat   sigma-wine-dxvk   sigma-bhashini-ta
  sigma-ros2           sigma-gamemode    sigma-drv-bt
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Level 0 shard manifests | `kernel/core/shards/` | `kernel-exp` | Declare manifests for all 8 microkernel shards |
| Level 1 auto-detect via sigma-dna | `hal/sigma_dna.cpp` | `drivers-dev` | CPUID/PCI probe → load correct driver shard |
| Level 2 by profile | `init/sigma_profile_selector.cpp` | all | Profile manifest lists required Level 2 shards |
| Level 3 on-demand load | `kernel/core/sigma_shard_loader.cpp` | `kernel-exp` | Load when first sigma-bus request arrives |
| Level 4 via sigma-pkg | `userland/sigma-pkg/sigma_pkg_cli.cpp` | `tools-dev` | Install = copy to `/sigma/shards/` + register |

### SL3 — Capability-Based Shard Isolation

Every shard declares what capabilities it needs. The kernel grants only those.

```
Capability token system:
  sigma.cap.net.tx          — transmit packets
  sigma.cap.net.rx          — receive packets
  sigma.cap.fs.read         — read from VFS
  sigma.cap.fs.write        — write to VFS
  sigma.cap.crypto.pqc      — access Kyber/Dilithium
  sigma.cap.display.kms     — write to framebuffer
  sigma.cap.audio.pcm       — write to audio PCM
  sigma.cap.india.gstn      — call GSTN API
  sigma.cap.india.abdm      — call ABDM API
  sigma.cap.india.upi       — call UPI API
  sigma.cap.pii.aadhaar     — read Aadhaar numbers (highly restricted)
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Capability token definitions | `include/sigma_caps.h` | `kernel-exp` | 32-bit bitmask per capability category |
| Capability grant at shard init | `kernel/security/sigma_caps.cpp` | `kernel-exp` | Kernel grants declared caps, denies others |
| Capability check on sigma-bus call | `kernel/ipc/sigma_bus.cpp` | `kernel-exp` | Every IPC: verify caller has required cap |
| PII capability (Aadhaar) extra gate | `kernel/security/sigma_caps.cpp` | `kernel-exp` | `sigma.cap.pii.aadhaar` requires sigma-trustd DID proof |
| Capability audit log | `kernel/security/sigma_immutable_audit_trail.cpp` | all | Every cap use logged with timestamp + DID |

---

## 2. Build Modularity

### BM1 — CMake Profile System

**Current:** `CMakeLists.txt` has SIGMA_PROFILE variable. Some guards missing.

```cmake

# CMakeLists.txt — target profiles:

# microkernel  : Level 0 + 1 only, < 512 KB kernel

# standalone   : All levels, Zenith desktop

# cloud        : Level 0-2, container-optimised, no GUI

# rtos         : Level 0-1 + sigma-sched-edf, no network

# mobile       : Level 0-1 (ARM64), sigma-ultra

# forensic     : Level 0-2 + sigma-forensics, read-only root

# gaming       : Level 0-3 + sigma-dxvk, GameMode

option(SIGMA_PROFILE "Build profile" "standalone")
option(SIGMA_ENABLE_ZENITH "Build Zenith desktop" ON)
option(SIGMA_ENABLE_INDIA_STACK "Build India Stack apps" ON)
option(SIGMA_ENABLE_WINE "Build sigma-wine compat layer" OFF)
option(SIGMA_ENABLE_GAMING "Build gaming stack" OFF)
option(SIGMA_ENABLE_RTOS "Build RT scheduler" OFF)
option(SIGMA_ENABLE_PQC_AVX512 "AVX-512 Kyber/Dilithium" OFF)
option(SIGMA_ENABLE_RUST "Enable Rust components" OFF)
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Profile CMake guards for every subsystem | `CMakeLists.txt` | all | `if(SIGMA_ENABLE_ZENITH)` around all Zenith source |
| `make check-profile` target | `Makefile` | all | Verify binary only contains expected shards |
| Profile size budget CI gate | `.github/workflows/sigma_ci.yml` | all | microkernel image < 512 KB enforced |
| CMake component targets | `CMakeLists.txt` | all | `cmake --install --component kernel` for partial install |
| Reproducible profile hash | `.github/workflows/sigma_ci.yml` | all | Per-profile SHA256 in release manifest |

### BM2 — Feature Flag System

```cpp
// include/sigma_features.h — compile-time feature flags:
// (also stored in sigma_features.json for runtime introspection)

#ifdef SIGMA_FEATURE_PQC
  // Post-quantum cryptography available
#endif

#ifdef SIGMA_FEATURE_ZENITH
  // Zenith desktop compositor available
#endif

#ifdef SIGMA_FEATURE_INDIA_STACK
  // India Stack API clients available
#endif

#ifdef SIGMA_FEATURE_WINE_COMPAT
  // Windows EXE compatibility layer
#endif

#ifdef SIGMA_FEATURE_RTOS_EDF
  // EDF real-time scheduler
#endif

// Runtime query:
sigma_bool sigma_feature_available(const char* feature_name);
// Example:
if (sigma_feature_available("INDIA_STACK")) {
    sigma_gst_compute(&data);
}
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| `sigma_features.h` compile-time flags | `include/sigma_features.h` | all | Mirror of `sigma_features.json` as `#define` |
| `sigma_feature_available()` runtime | `kernel/core/sigma_features.cpp` | `kernel-exp` | Read feature bitmask set at boot by profile selector |
| `sigma-cli features list` command | `userland/tools/sigma_cli.cpp` | `tools-dev` | Show all features, which are enabled |
| Feature flag CI matrix | `.github/workflows/sigma_ci.yml` | all | Build with each feature off — verify no compile error |

---

## 3. Runtime Modularity

### RM1 — Dynamic Shard Loading

```bash

# Runtime shard management:

sigma-drv load sigma-drv-iwlwifi   # load Wi-Fi driver at runtime

sigma-drv unload sigma-drv-i915    # unload GPU driver (sigma-heal takes over)

sigma-drv reload sigma-drv-e1000   # hot-reload NIC driver

sigma-shard list                   # all loaded shards + version + health

sigma-shard status sigma-net-tcp   # health + stats for one shard

sigma-shard load /sigma/shards/sigma-gamemode.shard
sigma-shard unload sigma-gamemode
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| `.shard` ELF shared object format | `include/sigma_shard.h` | `kernel-exp` | PIC ELF + `SigmaShardManifest` in `.sigma_shards` section |
| Shard loader (dlopen equivalent) | `kernel/core/sigma_shard_loader.cpp` | `kernel-exp` | Map shard ELF into kernel address space |
| Shard unloader (safe) | `kernel/core/sigma_shard_loader.cpp` | `kernel-exp` | Quiesce shard: drain sigma-bus queue, call `shutdown()` |
| Shard version check | `kernel/core/sigma_shard_loader.cpp` | `kernel-exp` | Reject shard if ABI version mismatch |
| Shard signature verify | `kernel/core/sigma_shard_loader.cpp` | `kernel-exp` | ML-DSA-87 verify manifest before load |
| `sigma-shard` CLI | `userland/tools/sigma_shard_cli.cpp` | `tools-dev` | list/status/load/unload/reload |

### RM2 — Kernel Live Patching (sigma-kpatch)

```bash
sigma-perf kpatch status           # list active patches

sigma-perf kpatch apply CVE-2026-1234.kpatch  # apply live patch

sigma-perf kpatch verify <patch>   # verify ML-DSA sig before apply

sigma-perf kpatch rollback <id>    # revert a patch

# No reboot required for kernel security patches

```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| kpatch function redirect | `kernel/kpatch/sigma_kpatch.cpp` | `performance-optimized` | Write JMP instruction at function entry |
| kpatch signature verify | `kernel/kpatch/sigma_kpatch.cpp` | `performance-optimized` | ML-DSA-87: reject unsigned patches |
| kpatch apply CI test | `tests/integration/test_kpatch.sh` | `performance-optimized` | Apply patch → verify function redirected → rollback |
| kpatch audit log | `kernel/kpatch/sigma_kpatch.cpp` | `performance-optimized` | Every patch logged to sigma-audit with DID |
| Unsigned patch rejection CI | `.github/workflows/sigma_ci.yml` | `prepare-sigmaos-launch` | `sigma_ci.yml` kpatch_unsigned_reject scenario |

---

## 4. Module Dependency Graph

### DG1 — Dependency Declarations

```cpp
// Every module declares dependencies via linker section:
// kernel/net/sigma_net_tcp.cpp:
SIGMA_DEPENDS_ON("sigma-vfs");      // needs filesystem for socket files
SIGMA_DEPENDS_ON("sigma-caps");     // needs capability system
SIGMA_DEPENDS_ON("sigma-crypto");   // needs TLS
SIGMA_OPTIONAL_DEP("sigma-ipv6");   // optional IPv6

// Build system validates: if sigma-net-tcp is included,
// sigma-vfs/sigma-caps/sigma-crypto must also be included.
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| `SIGMA_DEPENDS_ON` macro | `include/sigma_module_deps.h` | all | Emit dependency record in `.sigma_deps` ELF section |
| Dependency validator script | `scripts/check_deps.py` | all | Walk `.sigma_deps`, verify all deps satisfied for profile |
| Circular dependency detection | `scripts/check_deps.py` | all | Topological sort — fail if cycle detected |
| Missing dependency CI gate | `.github/workflows/sigma_ci.yml` | all | Run `check_deps.py` on every build |
| Dependency graph visualisation | `scripts/gen_arch_diagram.sh` | `docs-update` | Graphviz DOT from dependency records |

### DG2 — API Stability Contracts

```cpp
// SIGMA_STABLE marks an API as frozen — never changes:
SIGMA_STABLE sigma_err_t sigma_sys_read(uint32_t fd, void* buf, size_t n);
SIGMA_STABLE sigma_err_t sigma_sys_write(uint32_t fd, const void* buf, size_t n);

// SIGMA_EXPERIMENTAL marks work-in-progress:
SIGMA_EXPERIMENTAL sigma_err_t sigma_sys_io_uring_submit(sigma_uring_sqe_t* sqe);

// SIGMA_DEPRECATED marks for removal in next major version:
SIGMA_DEPRECATED sigma_err_t sigma_sys_old_socket(int domain, int type);
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| `SIGMA_STABLE` / `SIGMA_EXPERIMENTAL` / `SIGMA_DEPRECATED` macros | `include/sigma_abi.h` | `tools-dev` | Attribute macros + ABI version embedding |
| `make check-abi` — symbol diff | `Makefile` | `tools-dev` | `nm` diff: fail if SIGMA_STABLE symbol changed signature |
| ABI stability CI gate | `.github/workflows/sigma_ci.yml` | all | Block merge if ABI broken |
| Deprecation warning in CI | `.github/workflows/sigma_ci.yml` | all | `grep SIGMA_DEPRECATED` → warn in PR comment |

---

## 5. Plugin System for Profession Apps

### PS1 — ISigmaApp Plugin Interface

```cpp
// include/sigma_app_plugin.h
// Every profession app implements this interface:

class ISigmaApp {
public:
    // Lifecycle
    virtual sigma_err_t start()    = 0;
    virtual sigma_err_t stop()     = 0;
    virtual sigma_err_t suspend()  = 0;  // background
    virtual sigma_err_t resume()   = 0;  // foreground

    // Identity
    virtual const char* app_id()      const = 0;  // "sigma-ca"
    virtual const char* display_name()const = 0;  // "Sigma CA"
    virtual const char* version()     const = 0;  // "1.0.0"
    virtual const char* profession()  const = 0;  // "chartered_accountant"

    // CLI entrypoint (argc, argv)
    virtual sigma_err_t run_cli(int argc, const char** argv) = 0;

    // GUI entrypoint (return surface handle)
    virtual sigma_u32   run_gui(sigma_u32 parent_surface) { return 0; }

    // sigma-bus event handler
    virtual void on_event(const sigma_bus_event_t& e) {}

    // sigma-heal recovery hook
    virtual sigma_err_t recover(const char* reason) { return SIGMA_OK; }

    virtual ~ISigmaApp() = default;
};

// Plugin export — every .spkg app exports this:
extern "C" ISigmaApp* sigma_app_create();
extern "C" void       sigma_app_destroy(ISigmaApp* app);
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| `sigma_app_plugin.h` | `include/sigma_app_plugin.h` | `tools-dev` | Full `ISigmaApp` interface |
| App loader (dlopen equivalent) | `userland/daemons/sigma_appd.cpp` | `release/standalone` | Load `.spkg` app shared object, call `sigma_app_create()` |
| App registry daemon | `userland/daemons/sigma_appd.cpp` | `release/standalone` | Maintain list of loaded apps, route CLI/GUI requests |
| Migrate sigma-ca to plugin | `userland/apps/sigma-ca/sigma_ca.cpp` | `release/standalone` | Implement `ISigmaApp`, export `sigma_app_create()` |
| Migrate sigma-agri to plugin | `userland/apps/sigma-agri/sigma_agri.cpp` | `release/standalone` | Already has `main()` — wrap in `ISigmaApp` |
| Auto-discover apps in `/sigma/apps/` | `userland/daemons/sigma_appd.cpp` | `release/standalone` | Scan directory, load all `.spkg` apps |
| App sandbox via sigma-pod | `userland/daemons/sigma_appd.cpp` | `release/cloud` | Each app in its own sigma-pod container |

### PS2 — sigma-bus Plugin Messaging

```cpp
// Apps communicate only via sigma-bus — never direct function calls:

// sigma-ca publishes invoice posted:
sigma_bus_publish(
    "sigma.gst.invoice.posted",
    &invoice_data,
    sizeof(invoice_data),
    caller_caps   // capability token
);

// sigma-accounts subscribes and receives:
sigma_bus_subscribe(
    "sigma.gst.invoice.posted",
    &my_observer,
    sigma.cap.india.gstn   // must have this cap to subscribe
);
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Capability-gated subscribe | `kernel/ipc/sigma_bus.cpp` | `kernel-exp` | Reject subscribe if caller lacks required cap |
| Message schema validation | `kernel/ipc/sigma_bus.cpp` | `kernel-exp` | Validate payload against topic schema |
| Topic schema registry | `include/sigma_bus_topics.h` | `tools-dev` | All topic names + payload structs in one header |
| Dead-letter queue | `userland/daemons/sigma_queue.cpp` | `release/cloud` | Undeliverable messages → retry queue |

---

## 6. Cross-Cutting Concerns — Modular Logging

### ML1 — sigma-log (Structured Logging)

**Current:** `sigma_log_info/warn/err` macros exist. No structured output.

```cpp
// include/sigma_log.h — structured logging:

typedef enum {
    SIGMA_LOG_TRACE = 0,
    SIGMA_LOG_DEBUG = 1,
    SIGMA_LOG_INFO  = 2,
    SIGMA_LOG_WARN  = 3,
    SIGMA_LOG_ERROR = 4,
    SIGMA_LOG_FATAL = 5,
} sigma_log_level_t;

// Structured log macro — zero overhead when level disabled:
#define sigma_log_structured(level, component, msg, ...) \
    sigma_log_emit(level, component, __FILE__, __LINE__, msg, ##__VA_ARGS__)

// Usage:
sigma_log_structured(SIGMA_LOG_INFO, "sigma-ca",
    "GST return filed gstin=%s period=%s arn=%s",
    data->gstin, data->period, data->arn);
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Structured log format | `include/sigma_log.h` | `tools-dev` | JSON lines: `{"ts":1234,"level":"info","comp":"sigma-ca","msg":"..."}` |
| Log level filtering per component | `kernel/core/sigma_log.cpp` | `tools-dev` | `SIGMA_LOG_LEVEL_sigma-ca=debug` env var |
| Log routing to sigma-audit | `kernel/core/sigma_log.cpp` | all | ERROR/FATAL → DID-signed audit entry |
| Log rotation | `userland/daemons/sigma_logd.cpp` | `tools-dev` | Rotate at 10 MB, keep 5 rotations |
| `sigma-log tail` CLI | `userland/tools/sigma_log_cli.cpp` | `tools-dev` | `sigma-log tail --component sigma-ca --level warn` |
| Compile-time log level strip | `Makefile` | `performance-optimized` | `-DSIGMA_LOG_MIN_LEVEL=WARN` strips TRACE/DEBUG |
| OpenTelemetry log export | `userland/sigma_otel_export.cpp` | `release/cloud` | Forward structured logs to Grafana/Elastic |

---

## 7. Modularisation Quality Gates

### MQ1 — Per-Module Test Coverage

Every module must have its own test directory:

```
tests/
  kernel/
    test_sched.cpp       # sigma-sched tests

    test_mm.cpp          # sigma-mm tests

    test_irq.cpp         # sigma-irq tests

    test_bus.cpp         # sigma-bus tests

  net/
    test_tcp.cpp         # sigma-net-tcp tests

    test_udp.cpp         # sigma-net-udp tests

  crypto/
    test_kyber.cpp       # sigma-crypto Kyber tests

    test_dilithium.cpp   # sigma-crypto Dilithium tests

  apps/
    test_sigma_ca.cpp    # sigma-ca profession app tests

    test_sigma_agri.cpp  # sigma-agri tests

  compat/
    test_linux_elf.cpp   # sigma-linux-compat tests

    test_pe_loader.cpp   # sigma-wine PE loader tests

```

| Gate | CI check | Branch | Target |
|------|---------|--------|--------|
| Every module has test file | `scripts/check_test_coverage.sh` | all | No module directory without matching `test_*.cpp` |
| Module test run in isolation | `tests/Makefile` | all | Each test links only against its module |
| Mock injection for dependencies | Test pattern | all | Use `ICryptoProvider` mock in CA tests |
| Code coverage per module | gcov/llvm-cov | `kernel-exp` | Minimum 70% line coverage per module |
| No inter-module test dependency | `scripts/check_test_coverage.sh` | all | Tests must not call functions from other modules directly |

### MQ2 — Module Size Budget

| Module | Max binary size | Current | Target |
|--------|----------------|---------|--------|
| sigma-sched (microkernel) | 32 KB | Unknown | < 32 KB |
| sigma-mm (microkernel) | 48 KB | Unknown | < 48 KB |
| sigma-net-tcp | 64 KB | Unknown | < 64 KB |
| sigma-crypto (Kyber+Dilithium) | 128 KB | Unknown | < 128 KB |
| sigma-ca profession app | 256 KB | Unknown | < 256 KB |
| sigma-zenith compositor | 512 KB | Unknown | < 512 KB |
| sigma-wine loader | 1 MB | Unknown | < 1 MB |
| Full microkernel image | 512 KB | Unknown | < 512 KB |
| Full standalone ISO | 500 MB | Unknown | < 500 MB |

```bash

# Size budget CI gate:

sigma_automation.sh size-check      # new command — check all module sizes

```

---

## 8. Ease of Use — Deeper Coverage

### EU1 — Error Recovery UX

### Every error must be a teaching moment:

```bash

# Bad (current behavior):

$ sigma-pkg install sigma-foo
Error: -5

# Good (target behavior):

$ sigma-pkg install sigma-foo
✗ Package not found: sigma-foo

  Did you mean?
    sigma-fssai    (food safety tools)
    sigma-forest   (forest officer tools)

  Search for packages:
    sigma-pkg search foo

  If you know the exact name:
    sigma-pkg search --exact sigma-foo
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Fuzzy package name suggestion | `userland/sigma-pkg/sigma_pkg_cli.cpp` | `tools-dev` | Levenshtein distance 1–2 from known packages |
| "Did you mean?" for all CLI | `userland/tools/sigma_cli.cpp` | `tools-dev` | Apply to every "not found" error |
| Contextual help on error | `userland/tools/sigma_cli.cpp` | `tools-dev` | Every error code → human message + next steps |
| `sigma-doctor` — health check tool | `userland/tools/sigma_doctor_cli.cpp` | `tools-dev` | Scan for common misconfigurations, suggest fixes |

### EU2 — Progressive Disclosure

```bash

# Level 1 — simple user (hide complexity):

sigma-agri msp                     # shows 5 most common crops

sigma-ca gst                       # shows current month only

# Level 2 — power user (more options visible):

sigma-agri msp --list              # shows all 26 crops

sigma-ca gst --period 2025-04      # specify period

# Level 3 — developer (everything):

sigma-agri msp --list --json --year 2026
sigma-ca gst --debug --trace --all-returns

# Discovery via --help levels:

sigma-ca --help           # basic usage

sigma-ca --help --verbose # all options

sigma-ca --help --expert  # internal flags too

```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Default "simple" mode | All CLI tools | `release/standalone` | Only show 5 most common options by default |
| `--verbose` unlocks more options | All CLI tools | `release/standalone` | `--help --verbose` shows all flags |
| Sensible defaults everywhere | All CLI tools | `tools-dev` | Assume current year, current GSTIN from profile |
| `sigma-doctor` auto-diagnose | `userland/tools/sigma_doctor_cli.cpp` | `tools-dev` | "Your Config.sigma has no india.state_code set" |

### EU3 — Onboarding Flow Quality

```bash

# 5-minute onboarding (verified by CI timer):

1. Boot SigmaOS → language selection (30 s)

2. Scan DID QR → ABHA linked (60 s)

3. Profile auto-suggested by profession (30 s)

4. sigma-pkg install sigma-ca (60 s)

5. sigma-ca dashboard opens (30 s)
Total: < 4 minutes to first profession app running
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Onboarding timer CI | `tests/ui/test_oobe_time.sh` | `prepare-sigmaos-launch` | Assert OOBE completes in < 4 minutes |
| Profile suggestion accuracy | `userland/installer/sigma_oobe.cpp` | `release/standalone` | 90% correct profession suggestion from DigiLocker |
| First-run app auto-launch | `zenith_desktop/personalization/sigma_profile_engine.cpp` | `release/standalone` | After OOBE: sigma-ca opens automatically for CA |
| India-first defaults | `userland/installer/sigma_oobe.cpp` | `release/standalone` | Default timezone IST, locale hi_IN, timezone Asia/Kolkata |

---

## 9. Automation — Deeper Coverage

### AU1 — sigma-cron Implementation

**New daemon:** `userland/daemons/sigma_cron.cpp`

```cpp
// Cron job format (sigma-cron.conf):
// @reboot   sigma_automation.sh quality-check
// @daily    sigma_automation.sh india-sync
// @weekly   sigma_automation.sh perf-bench
// 0 2 * * * sigma_automation.sh backup
// 0 6 * * * sigma-pkg update
// 30 9 * * 1-5 sigma-ca gst --remind    # weekdays 9:30am GST reminder

struct SigmaCronJob {
    char   schedule[64];      // cron expression or @daily/@weekly/@reboot
    char   command[256];      // command to execute
    char   user[32];          // run as this user's DID profile
    bool   run_on_battery;    // skip if on battery (mobile)
    bool   require_network;   // skip if offline
    sigma_u64 last_run;       // epoch timestamp
    sigma_u64 next_run;       // computed next run time
};
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Cron expression parser | `userland/daemons/sigma_cron.cpp` | `tools-dev` | Parse `* * * * *` + @special tokens |
| APIC timer wakeup | `userland/daemons/sigma_cron.cpp` | `kernel-exp` | Sleep until `next_run`, wake via sigma-bus timer event |
| Job execution via sigma-sh | `userland/daemons/sigma_cron.cpp` | `tools-dev` | Fork sigma-sh, exec command, capture stdout/stderr |
| Job result → sigma-audit | `userland/daemons/sigma_cron.cpp` | `tools-dev` | Log exit code + output to sigma-audit |
| Battery/network awareness | `userland/daemons/sigma_cron.cpp` | `release/mobile` | Skip wifi-requiring jobs when offline |
| `sigma-cron list/add/remove/run` | `userland/tools/sigma_cron_cli.cpp` | `tools-dev` | Full CLI management |
| Persistent jobs in SigmaFS | `userland/daemons/sigma_cron.cpp` | `fs-dev` | Write to `/sigma/etc/sigma-cron.conf` |

### AU2 — sigma-hook Event Automation

**New daemon:** `userland/daemons/sigma_hook.cpp`

```toml

# /sigma/etc/sigma-hooks.conf

[[hook]]
event   = "network.connected"
command = "sigma_automation.sh india-sync"
timeout = 30

[[hook]]
event   = "package.installed"
command = "sigma-sec verify --pkg %PKG_NAME"
timeout = 10

[[hook]]
event   = "boot.success"
command = "sigma_automation.sh quality-check"
timeout = 60

[[hook]]
event   = "sigma.ids.alert"
command = "sigma-sec audit log --last 100"
timeout = 5

[[hook]]
event   = "profession.ca.login"
command = "sigma-ca dashboard --startup"
timeout = 10
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Hook config TOML parser | `userland/daemons/sigma_hook.cpp` | `tools-dev` | Parse `sigma-hooks.conf`, register sigma-bus subscriptions |
| sigma-bus event subscription | `userland/daemons/sigma_hook.cpp` | `kernel-exp` | Subscribe to each event topic |
| Command execution with timeout | `userland/daemons/sigma_hook.cpp` | `tools-dev` | Fork + exec + kill if exceeds timeout |
| `%VAR` interpolation | `userland/daemons/sigma_hook.cpp` | `tools-dev` | Replace `%PKG_NAME` with event payload fields |
| `sigma-hook list/add/remove/test` | `userland/tools/sigma_hook_cli.cpp` | `tools-dev` | Full CLI |
| Profession login hooks | `userland/daemons/sigma_hook.cpp` | `release/standalone` | DID credential detected → launch profession app |

---

## 10. Optimisation — Code-Level Patterns

### OP1 — Hot Path Identification & Optimisation

### Tools to measure:

```bash
sigma-perf record --pid $(pgrep sigma-net-tcp) --duration 30
sigma-perf report --top 20        # top 20 hot functions

sigma-perf flame ./sigma-ca       # flamegraph

```

### Known hot paths that need optimisation:

| Hot path | Current cost | Target | Optimisation |
|----------|-------------|--------|-------------|
| sigma-bus message dispatch | Unknown | < 100 ns | Lock-free MPSC queue |
| VFS path lookup | Unknown | < 200 ns | Directory entry cache |
| `sigma_accounts_post()` | Unknown | < 5 ms / 10K invoices | Batch insert, single transaction |
| `sigma_agri_msp()` | Unknown | < 1 ms | Linear scan → hash table |
| Kyber-1024 keygen | ~200 µs (PRNG) | < 150 µs (real NTT) | AVX-512 butterfly |
| Compositor `renderFrame()` | Unknown | < 8.3 ms @ 120 Hz | Pre-computed command buffers |
| GSTN API HTTP call | ~500 ms | < 200 ms | PQC-TLS session reuse |

### OP2 — Memory Layout Optimisation

```cpp
// Before (poor cache locality — 72 bytes, many cache lines):
struct sigma_task_t {
    char     name[64];      // 64 bytes — rarely accessed
    uint32_t pid;           // 4 bytes — hot
    uint32_t state;         // 4 bytes — hot
};

// After (hot fields first — fits in one cache line):
struct sigma_task_t {
    uint32_t pid;           // offset 0  — hot
    uint32_t state;         // offset 4  — hot
    uint32_t priority;      // offset 8  — hot
    uint32_t cpu_affinity;  // offset 12 — hot
    // cold fields after:
    char     name[64];      // offset 64 — cold
} __attribute__((packed));
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| `sigma_task_t` hot-fields-first | `kernel/sched/sigma_sched.h` | `performance-optimized` | Pid/state/priority in first cache line |
| `sigma_account_t` hot path | `userland/apps/sigma-accounts/sigma_accounts.h` | `release/standalone` | Balance/type in first 8 bytes |
| sigma-bus message hot fields | `kernel/ipc/sigma_bus.h` | `kernel-exp` | Topic hash + payload ptr in first cache line |
| VFS inode hot fields | `kernel/vfs/sigma_vfs.h` | `fs-dev` | Size/type/permissions in first cache line |
| Static analyser for struct layout | `scripts/check_struct_layout.py` | `performance-optimized` | Flag structs where hot fields aren't first |

---

## Summary: Comprehensive Roadmap Index

All roadmap documents with their primary dimensions:

| Doc | Primary dimensions | Lines |
|-----|--------------------|-------|
| Quality-Stability-Performance-Roadmap | Stability, Performance, Quality, UX, Security, Accessibility, DX | ~1,000 |
| Stability-Performance-Extended | Energy, Reliability, Observability, Release, Network QA, India QA, Hardware | ~900 |
| Compatibility-Automation-Personalisation-Roadmap | Linux/Win32/POSIX compat, Automation, Customisation, Personalisation | ~700 |
| Advanced-Quality-Roadmap | PQC depth, Network stack, Enterprise, AI/ML, i18n, Education, Rural, Community | ~700 |
| Systems-Excellence-Roadmap | Gaming, IoT, Dev tools, Packages, Updates, Multi-platform, Sprint plan | ~700 |
| Engineering-Principles-Roadmap | SOLID/OOP, Design patterns, CLI design, Optimisation, Refactoring | ~700 |
| Modularisation-Architecture-Roadmap | Shard system, Build modularity, Runtime loading, Plugin API, Automation depth | ~700 |

### Total: 7 documents, ~5,400 lines of actionable engineering roadmap.

---

*See also: [Engineering Principles Roadmap](Engineering-Principles-Roadmap) · [Systems Excellence Roadmap](Systems-Excellence-Roadmap) · [Feature Branch Roadmap](Feature-Branch-Roadmap) · [CLI Commands Roadmap](CLI-Commands-Roadmap) · [Branch Development Roadmap](Branch-Development-Roadmap)*
