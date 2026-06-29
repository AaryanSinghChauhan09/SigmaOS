# SigmaOS — Engineering Principles Roadmap
## OOP Architecture · CLI Design · Optimisation · Code Quality
## Codebase Standards · Design Patterns · Refactoring Plan

---

## 1. Object-Oriented Programming Principles

SigmaOS is written in C++17. Every subsystem must apply OOP principles
consistently — not just use `class` as a namespace. This section audits
what exists and defines the standard every new piece of code must meet.

### OOP1 — SOLID Principles Audit & Enforcement

#### Single Responsibility Principle (SRP)

**Current violations:**

| File | Violation | Fix | 
| ------ | ----------- | ----- | 
| `zenith_desktop/compositor/sigma_compositor.cpp` | `Compositor` class: owns window list, renders frames, polls input, handles self-healing, and maintains cursor — 5 responsibilities | Split into `WindowManager`, `Renderer`, `InputPoller`, `SelfHealMonitor`, `CursorLayer` | 
| `kernel/core/sigma_kernel_main.c` | One function does: serial init, VGA, IDT, PIC, slab, PIT, keyboard, scheduler, tasks, interrupts | Each init → its own `sigma_<subsystem>_init()` called from a boot sequence table | 
| `userland/tools/sigma_pod_cli.cpp` | `main()` parses args, creates containers, lists them, destroys them | Split into `PodArgParser`, `PodCreateCommand`, `PodListCommand`, `PodDestroyCommand` | 
| `crypto/SovereignDilithium5.cpp` | `SovereignDilithium::sign()` does: NTT, commitment hash, response polynomial, hint bits, serialisation | Each step → private method with a single purpose | 

**Enforcement rule:** Every `class` or `struct` answers one question:
*"What is this responsible for?"* — the answer must fit in 10 words.

#### Open/Closed Principle (OCP)

New functionality should be added by extension, not modification.

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| SDF driver registration macro | `hal/sigma_hal_driver.h` | `drivers-dev` | `SIGMA_SDF_REGISTER_DRIVER(Class, name, vid, pid)` — no changes to HAL core | 
| sigma-bus service registry | `kernel/ipc/sigma_bus.cpp` | `kernel-exp` | New services register without touching bus core | 
| Profession app plugin interface | `userland/indiastack/sigma_profession_base.h` | `tools-dev` | `class SigmaProfessionApp : public ISigmaApp` — new apps extend, not modify | 
| Syscall dispatch via table | `kernel/core/sigma_syscall_dispatch.cpp` | `kernel-exp` | Table of function pointers — add syscall by adding row, not switch case | 
| Theme extensibility | `zenith_desktop/theme/sigma_theme_engine.cpp` | `release/standalone` | `IThemeProvider` interface — new themes implement, not modify engine | 

#### Liskov Substitution Principle (LSP)

Derived classes must be substitutable for their base class.

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| `SovereignDriverBase` interface | `hal/sigma_hal_driver.h` | `drivers-dev` | All SDF drivers: `probe()`, `init()`, `shutdown()`, `ioctl()` — NVMe/NIC/GPU all substitutable | 
| `ISigmaApp` profession base | `userland/indiastack/sigma_profession_base.h` | `tools-dev` | `start()`, `stop()`, `status()` — sigma-ca/sigma-health/sigma-agri substitutable | 
| `ISigmaFilesystem` VFS interface | `kernel/vfs/sigma_vfs_interface.h` | `fs-dev` | SigmaFS/ext4/fat32 all implement same `open/read/write/close` | 
| `ICryptoProvider` PQC interface | `include/crypto/sigma_crypto_provider.h` | `performance-optimized` | Kyber and Dilithium implement same `keygen/sign/verify/encap/decap` | 

#### Interface Segregation Principle (ISP)

No class should be forced to depend on methods it does not use.

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| Split `sigma_vfs.h` into `IReadable`, `IWritable`, `ISeekable` | `kernel/vfs/` | `fs-dev` | Read-only filesystems implement only `IReadable` | 
| Split driver interface into `INetDriver`, `IBlockDriver`, `IDisplayDriver` | `hal/sigma_hal_driver.h` | `drivers-dev` | NIC doesn't implement display methods | 
| Split `ISigmaApp` into `ICLIApp`, `IGUIApp`, `IBackgroundService` | `userland/indiastack/sigma_profession_base.h` | `tools-dev` | sigma-agri is CLI; sigma-health has GUI; sigma-cron is background | 
| sigma-bus: separate `IPublisher` from `ISubscriber` | `kernel/ipc/sigma_bus.cpp` | `kernel-exp` | Publishers don't need to subscribe and vice versa | 

#### Dependency Inversion Principle (DIP)

High-level modules must not depend on low-level modules. Both depend on abstractions.

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| sigma-ca depends on `IGSTClient`, not GSTN HTTP client | `userland/apps/sigma-ca/sigma_ca.cpp` | `release/standalone` | Inject `IGSTClient` — swap sandbox/production without touching CA logic | 
| sigma-health depends on `IABDMClient`, not HTTP | `userland/apps/sigma-health/sigma_health.cpp` | `release/standalone` | Testable: inject mock ABDM client in CI | 
| sigma-compositor depends on `IFramebuffer`, not VirtIO-GPU | `zenith_desktop/compositor/sigma_compositor.cpp` | `release/standalone` | Swap VESA/VirtIO/i915 without touching compositor | 
| sigma-shell depends on `IVFSProvider`, not VFS impl | `userland/shell/sigma_shell.cpp` | `tools-dev` | Shell testable with mock filesystem | 

### OOP2 — Design Patterns Applied

#### Singleton — where it is right and where it is wrong

**Current:** `SovereignGPU`, `SovereignNICEngine`, `SovereignNVMeEngine`, `SovereignKyber`, `SovereignDilithium` all use Meyer's singleton. This is correct for hardware drivers and crypto engines — there is physically one GPU, one NIC.

**Wrong singletons to fix:**

| Class | Problem | Fix | 
| ------- | --------- | ----- | 
| `SovereignDilithium::getInstance()` | Forces global state; makes unit testing impossible | Use `ICryptoProvider` DI instead; singleton only as default factory | 
| `Zenith::Compositor::getInstance()` | Cannot test compositor without global state | Constructor injection; singleton only at boot, passed as reference | 
| `LinuxCompatLayer::getInstance()` | Multiple Wine prefixes need multiple instances | Remove singleton; use per-process `LinuxCompatLayer` object | 

#### Observer Pattern — sigma-bus events

**New files:** `kernel/ipc/sigma_bus_observer.h`

```cpp
// Every sigma-bus subscriber is an IObserver:
class ISignaBusObserver {
public:
    virtual void on_event(const sigma_bus_event_t& event) = 0;
    virtual sigma_topic_t subscribed_topic() const = 0;
    virtual ~ISignaBusObserver() = default;
};

// Example: sigma-ca observes GST invoice events from sigma-accounts
class CAInvoiceObserver : public ISignaBusObserver {
    void on_event(const sigma_bus_event_t& e) override {
        if (e.topic == TOPIC_GST_INVOICE_POSTED)
            sigma_ca_update_dashboard(e.payload);
    }
};
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| `ISignaBusObserver` base class | `kernel/ipc/sigma_bus_observer.h` | `kernel-exp` | Standard subscribe/notify interface | 
| Profession app observers | `userland/apps/sigma-ca/sigma_ca.cpp` | `release/standalone` | sigma-ca subscribes to invoice/payroll events | 
| Driver event notifications | `hal/sigma_hal_driver.h` | `drivers-dev` | NIC UP/DOWN events → sigma-netd observer | 
| Security event observers | `kernel/security/sigma_ids.cpp` | all | sigma-ids observes every syscall outcome | 

#### Factory Pattern — driver and app creation

```cpp
// sigma-drv factory: no switch-case, only registration
class SigmaDriverFactory {
    using Creator = std::function<SovereignDriverBase*()>;
    std::unordered_map<sigma_u32, Creator> m_registry;
public:
    void register_driver(sigma_u32 pci_id, Creator c) {
        m_registry[pci_id] = c;
    }
    SovereignDriverBase* create(sigma_u32 pci_id) {
        auto it = m_registry.find(pci_id);
        return (it != m_registry.end()) ? it->second() : nullptr;
    }
};
// Each driver registers itself at link time via SIGMA_SDF_REGISTER_DRIVER macro
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| `SigmaDriverFactory` | `hal/sigma_driver_factory.cpp` | `drivers-dev` | PCI ID → driver instance, no switch-case | 
| `SigmaAppFactory` | `userland/indiastack/sigma_app_factory.cpp` | `tools-dev` | DID profession → app instance | 
| `SigmaFSFactory` | `kernel/vfs/sigma_fs_factory.cpp` | `fs-dev` | Mount type string → filesystem implementation | 
| `SigmaCryptoFactory` | `crypto/sigma_crypto_factory.cpp` | `performance-optimized` | Algorithm name → `ICryptoProvider` implementation | 

#### Command Pattern — sigma-cli

```cpp
// Every CLI command is a Command object — undo/redo/logging for free
class ICommand {
public:
    virtual sigma_err_t execute(const CmdContext& ctx) = 0;
    virtual sigma_err_t undo(const CmdContext& ctx) { return SIGMA_ERR_NOT_SUPPORTED; }
    virtual const char* name() const = 0;
    virtual const char* description() const = 0;
    virtual ~ICommand() = default;
};

class PkgInstallCommand : public ICommand {
    sigma_err_t execute(const CmdContext& ctx) override; // install
    sigma_err_t undo(const CmdContext& ctx) override;    // rollback
    const char* name() const override { return "pkg install"; }
};
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| `ICommand` base class | `userland/tools/sigma_cli_command.h` | `tools-dev` | execute/undo/name/description interface | 
| Command registry | `userland/tools/sigma_cli_registry.cpp` | `tools-dev` | Map `"pkg install"` → `PkgInstallCommand` instance | 
| Undo stack | `userland/tools/sigma_cli_registry.cpp` | `tools-dev` | `sigma-cli undo` reverses last command | 
| Command history log | `userland/tools/sigma_cli_registry.cpp` | `tools-dev` | Every command execution → sigma-audit | 
| Batch command file | `userland/tools/sigma_cli_registry.cpp` | `tools-dev` | `sigma-cli run script.sigma` | 

#### Strategy Pattern — scheduler, filesystem, crypto

```cpp
// Scheduler: different algorithms for different profiles
class ISchedulerStrategy {
public:
    virtual void enqueue(sigma_task_t* task) = 0;
    virtual sigma_task_t* dequeue() = 0;
    virtual const char* name() const = 0;
};
class RoundRobinStrategy  : public ISchedulerStrategy { ... };
class MLFQStrategy        : public ISchedulerStrategy { ... };
class EDFStrategy         : public ISchedulerStrategy { ... };
class CFSStrategy         : public ISchedulerStrategy { ... };

// Boot profile selects strategy — no #ifdef
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| `ISchedulerStrategy` | `kernel/sched/sigma_sched_strategy.h` | `kernel-exp` | Plug in RR/MLFQ/EDF/CFS without touching kernel core | 
| `IFilesystemStrategy` | `kernel/vfs/sigma_fs_strategy.h` | `fs-dev` | SigmaFS/ext4/tmpfs as interchangeable strategies | 
| `ICryptoStrategy` | `include/crypto/sigma_crypto_strategy.h` | `performance-optimized` | Kyber-512/768/1024 as strategies; select per security level | 
| `INetworkStrategy` | `kernel/net/sigma_net_strategy.h` | `drivers-dev` | WiFi/Ethernet/USSD as interchangeable strategies | 

### OOP3 — Code Quality Standards

```cpp
// Every public API in SigmaOS must follow this pattern:

class SigmaExample {
public:
    // 1. Factory method instead of complex constructor
    static SigmaExample* create(const SigmaExampleConfig& cfg);

    // 2. RAII — resources acquired in constructor, released in destructor
    ~SigmaExample();

    // 3. No raw pointers in public API — use sigma_unique_ptr or reference
    sigma_err_t process(sigma_span<const sigma_u8> data);

    // 4. const-correct: const on every method that doesn't mutate state
    sigma_u32 version() const;

    // 5. Explicit error handling — no exceptions (kernel code), no silent failure
    sigma_err_t init();   // returns error code, never throws

    // 6. Move semantics for expensive objects
    SigmaExample(SigmaExample&&) noexcept = default;
    SigmaExample& operator=(SigmaExample&&) noexcept = default;

    // 7. No implicit conversions
    explicit SigmaExample(sigma_u32 id);

    // 8. No mutable global state outside of singleton hardware drivers
private:
    sigma_u32 m_id;           // m_ prefix for member variables
    sigma_u32 m_state;
    static sigma_u32 s_count; // s_ prefix for static members
};
```

| Standard | Enforcement | Branch | Detail | 
| ---------- | ------------ | -------- | -------- | 
| `m_` member prefix, `s_` static prefix | clang-tidy `readability-identifier-naming` | all | `check-identifier-naming` CI job | 
| `const` on all non-mutating methods | clang-tidy `misc-const-correctness` | all | Block merge if const missing | 
| No raw `new`/`delete` in public APIs | clang-tidy `cppcoreguidelines-no-malloc` | all | Use `sigma_unique_ptr` / slab API | 
| `[[nodiscard]]` on error-returning functions | clang-tidy `modernize-use-nodiscard` | all | Prevent silently discarded errors | 
| No implicit conversions (`explicit`) | clang-tidy | all | Block implicit single-arg constructors | 
| Move semantics for PQC key objects | Manual review | `performance-optimized` | PQC keys: move-only, never copy | 
| Zero raw pointers in profession app APIs | Code review | `release/standalone` | All APIs use `sigma_span<>` or references | 

---

## 2. CLI Architecture Roadmap

### CLI1 — Unified CLI Surface Design

Every sigma command follows the same grammar:

```
sigma-<tool> <noun> <verb> [--flag value] [--bool-flag] [positional]

Examples:
  sigma-cli profile use desktop
  sigma-pkg install sigma-ca --version 1.0
  sigma-net status --json
  sigma-zenith layout bsp
  sigma-agri msp --crop wheat --year 2026
  sigma-ca gst file --gstin 27ABCDE1234F1Z5 --period 2026-06
  sigma-perf bench pqc --iterations 10000
  sigma-fleet device list --filter "status=healthy" --json
```

**Rules every tool must follow:**

| Rule | Implementation | CI gate | 
| ------ | --------------- | --------- | 
| `--help` on every verb | Print usage + examples | `sigma-<tool> <verb> --help` exits 0 | 
| `--json` on every list/status command | Structured JSON to stdout | Parse output with `jq` in CI | 
| `--dry-run` on every mutating command | Show what would happen | No side-effects when flag set | 
| `--quiet` / `-q` | Suppress informational output | Only errors to stderr | 
| `--verbose` / `-v` | Extra detail | Debug output | 
| Exit code 0 = success, 1 = user error, 2 = system error | `sigma_err_t` → exit code map | CI asserts exit codes | 
| Colour in terminal, plain in pipe | `isatty(1)` check | `sigma-pkg list | grep` works | 
| Progress bar for long operations | `[████░░░░] 45%` | No progress in `--json` mode | 

### CLI2 — sigma-cli Completions & Discovery

```bash
# Tab completion (fish-style, already partially real in sigma-sh):
sigma-cli <TAB>                    # shows: profile alias pkg pod wine ...
sigma-cli profile <TAB>            # shows: list show use create edit export
sigma-cli profile use <TAB>        # shows: desktop minimal cloud forensic gaming

# Fuzzy search (fzf-style):
sigma-cli search pkg               # fuzzy: sigma-pkg, sigma-cli pkg, ...
sigma-cli --interactive            # TUI picker for all commands
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| Completion data JSON | `userland/tools/sigma_cli_completions.json` | `tools-dev` | Machine-readable tree: command → subcommands → flags | 
| sigma-sh completion engine | `userland/shell/sigma_shell.cpp` | `tools-dev` | On TAB: read completions JSON, filter by prefix | 
| Fish-style abbreviations | `userland/shell/sigma_shell.cpp` | `tools-dev` | `sca` expands to `sigma-ca`, `sag` to `sigma-agri` | 
| `sigma-cli --interactive` TUI | `userland/tools/sigma_cli.cpp` | `release/standalone` | ncurses-style picker showing all commands | 
| Inline examples on error | `userland/tools/sigma_cli.cpp` | `tools-dev` | Wrong usage → "Did you mean: sigma-cli profile use <name>?" | 
| Man page for every command | `docs/man/` | `docs-update` | Auto-generated from `--help` output + examples | 

### CLI3 — sigma-sh Advanced Features

**Current:** Parser complete, history/aliases/env real, no TTY.

```bash
# Features to implement:

# Scripting
if sigma-net status --json | jq -r '.connected' | grep -q true; then
    sigma-ai ask "Today's GST filing status"
fi

# Pipelines (already tokenised — just missing exec):
sigma-agri msp --list | sort -k3 -n | head -5

# Process substitution
sigma-ca gst compute <(sigma-digilocker fetch --gstin 27ABCDE1234F1Z5)

# Here documents
sigma-accounts voucher <<EOF
{"type":"sales","amount":10000,"gstin":"27ABCDE1234F1Z5"}
EOF

# Background jobs
sigma-ai ask "analyse this report" &
sigma-agri enam prices --mandi Azadpur &
jobs         # list running background jobs
wait %1      # wait for job 1
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| Fork + exec external commands | `userland/shell/sigma_shell.cpp` | `tools-dev` | `sigma_sys_fork()` + `sigma_sys_execve()` | 
| Pipe implementation | `userland/shell/sigma_shell.cpp` | `tools-dev` | `sigma_sys_pipe()` + dup2 for stdin/stdout | 
| `>`, `>>`, `<`, `2>` redirect | `userland/shell/sigma_shell.cpp` | `tools-dev` | `sigma_sys_open()` + dup2 | 
| Background `&` + `jobs` builtin | `userland/shell/sigma_shell.cpp` | `tools-dev` | Track bg PIDs in `g_jobs[64]` table | 
| `if/else/fi` scripting | `userland/shell/sigma_shell.cpp` | `tools-dev` | Parse control flow tokens | 
| `for/while/do/done` loops | `userland/shell/sigma_shell.cpp` | `tools-dev` | Loop execution with break/continue | 
| Function definitions | `userland/shell/sigma_shell.cpp` | `tools-dev` | `my_func() { commands; }` | 
| `$()` command substitution | `userland/shell/sigma_shell.cpp` | `tools-dev` | Capture stdout of subcommand | 
| Here-doc `<<EOF` | `userland/shell/sigma_shell.cpp` | `tools-dev` | Multi-line stdin input | 
| `set -e` / `set -x` mode | `userland/shell/sigma_shell.cpp` | `tools-dev` | Exit on error / xtrace | 
| `.sigma_profile` source on login | `userland/shell/sigma_shell.cpp` | `tools-dev` | Read aliases/env from profile at startup | 
| Glob expansion (`*`, `?`, `[a-z]`) | `userland/shell/sigma_shell.cpp` | `tools-dev` | `ls *.cpp` → VFS readdir + filter | 
| Ctrl+R history search | `userland/shell/sigma_shell.cpp` | `tools-dev` | Reverse-search through `g_history[]` | 
| Ctrl+L clear screen | `userland/shell/sigma_shell.cpp` | `tools-dev` | `\033[2J\033[H` VT100 | 

### CLI4 — India-Specific CLI Features

```bash
# Rupee currency output (₹, not $)
sigma-agri msp --crop wheat        # Output: "₹2,425 per quintal"
sigma-ca gst compute --gstin ...   # Output: "CGST: ₹9,000 | SGST: ₹9,000"

# Indian date format
sigma-cal gst-due 2026-07          # Output: "31 July 2026"
sigma-agri pmkisan status          # Output: "Last credit: 01 April 2026"

# Number formatting (Indian system: lakhs/crores)
sigma-accounts balance --account Sales  # Output: "₹12,45,67,890" (not 124,567,890)

# Regional language output
SIGMA_LANG=hi sigma-agri msp --crop wheat
# Output: "गेहूं का MSP: ₹2,425 प्रति क्विंटल"

# Aadhaar / PAN masking in output
sigma-digilocker list              # Output: "PAN: ABCDE****F" (masked)
sigma-abdm patient search --name Ramesh  # Output: "ABHA: ****-****-1234"
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| Indian number formatter | `userland/locales/sigma_l10n.cpp` | `release/standalone` | Lakh/crore format with correct comma placement | 
| ₹ rupee symbol output | `userland/locales/sigma_l10n.cpp` | `release/standalone` | UTF-8 `₹` (U+20B9) in all monetary output | 
| Indian date format | `userland/locales/sigma_l10n.cpp` | `release/standalone` | `dd Month YYYY` by default for Indian locale | 
| PAN/Aadhaar masking | `userland/tools/sigma_pii_mask.cpp` | `tools-dev` | Regex mask PAN/Aadhaar in all CLI output | 
| `SIGMA_LANG=hi` env var | `userland/locales/sigma_l10n.cpp` | `release/standalone` | All CLI tools read `SIGMA_LANG` env | 
| Diacritics in column alignment | `userland/locales/sigma_l10n.cpp` | `release/standalone` | `printf("%-20s", hindi_string)` — Unicode-aware column width | 

---

## 3. Optimisation Roadmap

### OPT1 — Compiler Optimisation

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| LTO (Link-Time Optimisation) | `Makefile` | `performance-optimized` | `-flto=thin` for kernel + userland | 
| PGO (Profile-Guided Optimisation) | `Makefile` | `performance-optimized` | `make PROFILE=pgo iso` → profile run → `make PROFILE=pgo-use iso` | 
| `-O3` for hot paths, `-Os` for cold | `Makefile` | `performance-optimized` | Per-subsystem optimisation level | 
| AVX-512 vectorisation | `Makefile` | `performance-optimized` | `-march=native` for AVX-512 builds | 
| Dead code elimination | `Makefile` | all | `-fdata-sections -ffunction-sections --gc-sections` | 
| String literal deduplication | `Makefile` | all | `-fmerge-constants` | 
| Reproducible builds with opt | `Makefile` | all | `SOURCE_DATE_EPOCH` + `-fmacro-prefix-map` | 

### OPT2 — Runtime Optimisation

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| Hot path branch prediction hints | `kernel/core/sigma_syscall_dispatch.cpp` | `kernel-exp` | `__builtin_expect(cond, 1)` on common syscalls | 
| Likely/unlikely macros | `include/sigma_kernel_types.h` | all | `#define SIGMA_LIKELY(x) __builtin_expect(!!(x),1)` | 
| Cache line alignment | `kernel/sched/sigma_runqueue.cpp` | `performance-optimized` | `alignas(64)` on hot data structures | 
| False sharing elimination | `kernel/sched/sigma_runqueue.cpp` | `performance-optimized` | Per-CPU runqueue on separate cache lines | 
| Prefetch hints | `kernel/fs/sigma_readahead.cpp` | `performance-optimized` | `__builtin_prefetch` for sequential reads | 
| Inline critical functions | `klib/sigma_lockfree.h` | `performance-optimized` | `__attribute__((always_inline))` on CAS loops | 
| Avoid unnecessary copies | All hot paths | all | Pass `const&` or `sigma_span<>` instead of value | 

### OPT3 — Memory Optimisation

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| Small buffer optimisation (SBO) | `klib/sigma_string.cpp` | `tools-dev` | Strings ≤ 15 chars stored inline, no heap | 
| Pool allocator for IPC messages | `kernel/ipc/sigma_bus.cpp` | `kernel-exp` | Fixed-size message pool, no `sigma_malloc` per message | 
| Zero-copy IPC via shared memory | `kernel/ipc/sigma_bus.cpp` | `kernel-exp` | Large payloads: share physical page, not copy | 
| Compact enum types | All headers | all | `typedef enum : uint8_t` where range < 256 | 
| Bitfield packing for flags | All headers | all | `struct Flags { uint32_t readable:1; writable:1; ... }` | 
| NUMA-local allocation | `kernel/mm/sigma_numa.cpp` | `performance-optimized` | `sigma_malloc_on_node(size, cpu_node)` | 
| Slab pre-warming | `klib/sigma_slab_lockfree.cpp` | `performance-optimized` | Pre-alloc hot slab sizes at boot | 

---

## 4. Codebase Refactoring Plan

### RF1 — Namespace Standardisation

**Current:** Mix of `SigmaOS::`, `sigma::`, no namespace, C-style names.

| Standard | Rule | Files to update | 
| ---------- | ------ | ---------------- | 
| Kernel C code | No namespace (C linkage) | `kernel/core/*.c`, `kernel/vfs/*.c` | 
| Kernel C++ code | `namespace sigma::kernel::` | `kernel/core/*.cpp`, `kernel/sched/*.cpp` | 
| Userland tools | `namespace sigma::tools::` | `userland/tools/*.cpp` | 
| Profession apps | `namespace sigma::apps::<appname>::` | `userland/apps/**/*.cpp` | 
| Crypto | `namespace sigma::crypto::` | `crypto/*.cpp` | 
| Networking | `namespace sigma::net::` | `net/**/*.cpp` | 
| Compat layer | `namespace sigma::compat::` | `runtime/compat/**/*.cpp` | 
| Public C API | `extern "C"` with `sigma_` prefix | All `extern "C"` blocks | 

### RF2 — Error Handling Standardisation

**Current:** Mix of `K_OK`/`SIGMA_SUCCESS`/`-1`/`false`/`nullptr` — no single standard.

```cpp
// Target: every function returns sigma_err_t
typedef int32_t sigma_err_t;

// Standard error codes (include/sigma_error_codes.h):
#define SIGMA_OK           0
#define SIGMA_ERR_INVAL   -1   // Invalid argument
#define SIGMA_ERR_NOMEM   -2   // Out of memory
#define SIGMA_ERR_IO      -3   // I/O error
#define SIGMA_ERR_PERM    -4   // Permission denied
#define SIGMA_ERR_NOENT   -5   // No such file/resource
#define SIGMA_ERR_BUSY    -6   // Resource busy
#define SIGMA_ERR_TIMEOUT -7   // Operation timed out
#define SIGMA_ERR_NOSYS   -8   // Not implemented
#define SIGMA_ERR_NETWORK -9   // Network error
#define SIGMA_ERR_CRYPTO  -10  // Cryptographic error
#define SIGMA_ERR_INDIA   -11  // India Stack API error

// Every error-returning function is [[nodiscard]]:
[[nodiscard]] sigma_err_t sigma_pkg_install(const char* name);

// Macro for propagation (similar to Rust's ?):
#define SIGMA_TRY(expr) do { sigma_err_t _e = (expr); if (_e != SIGMA_OK) return _e; } while(0)
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| Define `sigma_error_codes.h` | `include/sigma_error_codes.h` | `tools-dev` | All error codes, `sigma_err_to_string()` | 
| `SIGMA_TRY` propagation macro | `include/sigma_error_codes.h` | `tools-dev` | Clean error propagation without exceptions | 
| Migrate kernel to `sigma_err_t` | `kernel/core/`, `kernel/vfs/` | `kernel-exp` | Replace `-1`/`false` return codes | 
| Migrate userland to `sigma_err_t` | `userland/tools/`, `userland/apps/` | `tools-dev` | Replace `int`/`bool` returns | 
| `[[nodiscard]]` on all API functions | All public headers | all | clang-tidy CI enforcement | 
| `sigma_err_to_string()` for CLI | `include/sigma_error_codes.h` | `tools-dev` | Human-readable error messages | 

### RF3 — Header Hygiene

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| SPDX header on every file | All files | all | CI gate already defined — enforce | 
| Include guards → `#pragma once` | All `.h` files | all | `sed -i` migration script | 
| Forward declarations over includes | All headers | all | Reduce compile time: declare instead of include | 
| Separate interface from implementation | All `.h`/`.cpp` pairs | all | No implementation in headers (except templates) | 
| Remove `using namespace std` | All files | all | Explicit `std::` — no namespace pollution | 
| Consistent include order | `.clang-format` | all | System → sigma headers, alphabetical within group | 
| `compile_commands.json` always fresh | `Makefile` | all | `make compile_commands` as first target | 

---

## Summary of New Dimensions Added

| Document | New dimensions | Status | 
| ---------- | --------------- | -------- | 
| [Quality-Stability-Performance-Roadmap](Quality-Stability-Performance-Roadmap) | Stability, Performance, Quality, UX, Security, Accessibility, DX | ✅ Done | 
| [Stability-Performance-Extended](Stability-Performance-Extended) | Energy, Reliability, Observability, Release, Network QA, India QA, Hardware | ✅ Done | 
| [Compatibility-Automation-Personalisation-Roadmap](Compatibility-Automation-Personalisation-Roadmap) | Linux/Win32/POSIX compat, Automation, Customisation, Personalisation | ✅ Done | 
| [Advanced-Quality-Roadmap](Advanced-Quality-Roadmap) | PQC depth, Network stack, Enterprise, AI/ML, i18n, Education, Rural, Community | ✅ Done | 
| [Systems-Excellence-Roadmap](Systems-Excellence-Roadmap) | Gaming, IoT, Dev tools, Packages, Updates, Multi-platform, Sprint plan | ✅ Done | 
| [Engineering-Principles-Roadmap](Engineering-Principles-Roadmap) | SOLID/OOP principles, Design patterns, CLI architecture, Optimisation, Refactoring | ✅ This doc | 

**Total: 6 documents, ~4,700 lines of actionable engineering roadmap.**

---

*See also: [Systems Excellence Roadmap](Systems-Excellence-Roadmap) · [CLI Commands Roadmap](CLI-Commands-Roadmap) · [Feature Branch Roadmap](Feature-Branch-Roadmap) · [Branch Development Roadmap](Branch-Development-Roadmap)*
