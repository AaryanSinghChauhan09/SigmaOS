# AGENTS DIRECTIVE: ZERO-DEPENDENCY, PRE-DEFINED FUNCTION & LIBRARY ELIMINATION MANAGEMENT

This document governs AI agent operations regarding the strict reduction and elimination of dependencies on pre-defined functions, pre-defined standard/third-party libraries, external package manager frameworks, and third-party projects in **SigmaOS**.

---

## 1. CORE PHILOSOPHY & OBJECTIVES

SigmaOS is an autonomous, bare-metal, zero-dependency operating system built from scratch. To maintain complete architectural purity, transparency, and self-hosting capability:
- **No Pre-Defined Standard Libraries (`#![no_std]`):** Operating system code must run directly on bare metal without depending on high-level language runtime standard libraries (e.g., Rust `std::`, C standard library `libc`, C++ `std::`, Zig `std`, or Nim system runtimes).
- **User-Defined Functions (UDFs) & Primitives:** Every data structure (hash maps, vectors, dynamic strings, B-trees, atomic locks), math primitive, memory allocator, string formatter, and algorithm must be constructed from first principles using custom, user-defined bare-metal primitives (`src/klib/`).
- **Bare-Metal Object-Oriented Principles (OOP):** Core OS subsystems and driver layers must be designed using structured object-oriented paradigms tailored for bare metal:
  - **Encapsulation:** Hardware registers, memory ranges, and MMIO structures wrapped inside strict state-guard objects.
  - **Factory Pattern:** Dynamic driver and package adapter instantiation based on hardware IDs or format signatures.
  - **Adapter Pattern:** Wrapper interfaces converting external/foreign formats into native SigmaOS primitives without invoking foreign runtimes.
  - **Observer Pattern:** Thread-safe, event-driven kernel notifications and async event dispatching.
  - **Singleton Pattern:** Centralized system managers (VMM, Scheduler, Driver Manager) coordinating global state.

---

## 2. ELIMINATION & REPLACEMENT SPECIFICATION

| Category | Pre-Defined / Foreign Dependency | Native SigmaOS Replacement Strategy | Location |
| :--- | :--- | :--- | :--- |
| **Language Runtime** | `std::`, `libc`, `glibc`, `musl` | Custom `#![no_std]` kernel primitives (`src/klib/`) & bare MMIO | `src/klib/`, `src/kernel/` |
| **Data Structures** | `std::collections` (`HashMap`, `BTreeMap`, `Vec`) | Custom `SigmaHashMap`, `SigmaBTreeMap`, `SigmaVec`, `SigmaString` | `src/klib/hashmap.rs`, `src/klib/vec.rs` |
| **Math Primitives** | `libm`, `std::f64::powf`, libc `sin`/`cos` | Zero-dependency bare-metal bitwise & polynomial math ops | `src/klib/math_ops.rs` |
| **Memory Allocation** | `malloc`/`free`, `jemalloc`, `mimalloc` | Custom SLUB object allocator, FreeBSD-style UMA zone allocator, Page Folio manager | `src/klib/slab.rs`, `src/memory/zone.rs` |
| **Package Management** | `apt`, `pacman`, `dnf`, `nix`, `snap`, `flatpak` | Native `SigmaPkg` format + Universal Package Translator shims | `src/package/universal.rs` |
| **Desktop / Compositor**| `X11`, `Wayland`, `Hyprland`, `Weston` | Custom Zenith Compositor rendering directly to framebuffer / DRM-KMS | `zenith_desktop/`, `src/ui/` |
| **Toolchains & Scripts**| Python scripts (`.py`), external C++ binaries (`.cpp`) | Native Rust binaries (`src/tools/`) & POSIX shell shims (`scripts/`) | `src/tools/`, `scripts/` |

---

## 3. AI AGENT IMPLEMENTATION DIRECTIVES

1. **Strict `#![no_std]` Enforceability:**
   - Any new kernel module, device driver, filesystem layer, or networking stack added to SigmaOS MUST include `#![no_std]` and strictly rely on `core::` or custom `src/klib/` abstractions.
   - When writing standalone test runners (`rustc --test`), conditionally isolate standard library imports using `#[cfg(any(feature = "standalone_test", test))]` shims.

2. **User-Defined Function (UDF) Supremacy:**
   - Always prefer custom, inline user-defined functions and zero-dependency algorithms over importing external crates or platform functions.
   - All string manipulations, hashing functions (e.g., FNV-1a, SipHash, CRC32C), and cryptographic routines (Kyber-1024, Dilithium-5) must be self-contained in native Rust code.

3. **OOP Design Pattern Integration:**
   - **Factory:** Use factory constructors (`PackageFactory::get_strategy()`, `DriverFactory::create()`) for dynamic resource allocation.
   - **Adapter:** Wrap legacy/external interfaces (`UniversalPackageTranslator`, `ForeignDistroAdapter`) in custom shims so SigmaOS core code remains pure.
   - **Observer:** Maintain thread-safe subscriber arrays (`PackageTriggerRegistry`, `KernelEventObserver`) for non-blocking notifications.

4. **Continuous Elimination Auditing:**
   - Periodically execute `./scripts/no_std_check.sh` to ensure no prohibited standard library symbols or foreign package dependencies leak into core bare-metal modules.

---

## 4. VERIFICATION WORKFLOW

Before committing changes that touch core primitives, run:
```bash
# 1. Run no_std compliance checker
./scripts/no_std_check.sh

# 2. Execute full SigmaOS test suite
./run_sigma_tests.sh

# 3. Synchronize documentation with Wiki targets
./scripts/sync_wiki.sh
```
