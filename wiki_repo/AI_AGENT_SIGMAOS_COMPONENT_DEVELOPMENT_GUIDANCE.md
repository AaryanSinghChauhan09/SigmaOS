# AI Agent SigmaOS Component Development Guidance & Engineering Standards

## Executive Summary
This document serves as the **Master Wiki Portal & Standard Operating Procedure (SOP)** for AI Agents (including Jules, Autonomous Coding Agents, and AI-driven Subsystem Maintainers) tasked with developing, refactoring, maintaining, or expanding components within **SigmaOS**.

SigmaOS is an omnipresent, zero-dependency, quantum-safe operating system engineered in 100% Safe Rust (`klib`). To maintain absolute architectural superiority over legacy Linux and BSD distributions while leveraging their finest engineering practices, AI Agents must strictly adhere to the guidelines set forth in this guide.

---

## 1. Distro-Inspired Development Principles for AI Agents

AI Agents developing SigmaOS components must incorporate the best architectural patterns from major Linux and BSD operating systems:

### 1.1 Arch Linux Parity & ALPM Mechanics
- **Simplicity & Bleeding-Edge Standards**: Ensure code paths are clean, minimal, and transparent. Avoid unnecessary abstractions.
- **ALPM Package Translation**: When modifying `src/sigpkg/` or package resolution engines, maintain full compatibility with Arch PKGBUILD manifests, `.PKGINFO` metadata, and `vercmp` version comparison logic (Epoch:PkgVer-PkgRel).
- **Isolated Clean-Chroot Build Patterns**: AI Agents must ensure that all component builds and tests run in hermetic environments without assuming pre-existing host dependencies.

### 1.2 Debian Policy & Reproducibility
- **Strict Dependency & Trigger Declarations**: Component dependencies must be explicitly mapped to canonical names via `UniversalDependencyMapper`.
- **Reproducible Build Assurance**: All generated artifacts, manifests, and binaries must be deterministic. Avoid non-deterministic timestamps or random seeds in generated code.
- **Post-Installation Trigger Hooks**: Emulate Debian `dpkg-triggers` and scriptlet lifecycle hooks (`preinst`, `postinst`, `prerm`, `postrm`) within `UniversalScriptletConverter`.

### 1.3 Fedora / RedHat Enterprise Readiness
- **Atomic Generation Updates & Rollbacks**: Any system configuration or system component change must support atomic generation staging and instant rollback capabilities via `ConfigManager` and `SovereignAtomicUpdateEngineTool`.
- **Systemd Service & Socket Parity**: Subsystems must interface seamlessly with systemd-style socket activation, D-Bus service interfaces, and `systemd-oomd`/PSI memory pressure monitoring.
- **Crypto-Policies Centralization**: Hardcode zero insecure ciphers. All network, SSH, or IPC components must default to Post-Quantum Cryptography (Dilithium5 / Kyber1024) and TLS 1.3+.

### 1.4 Gentoo Portage & Granular Modularity
- **USE-Flag Conditional Feature Toggles**: Features within SigmaOS modules must be configurable via modular flags (`USE` flag equivalents) resolved via `GentooUseFlagResolverEngine`.
- **Subslot & ABI Stability Guarantees**: Prevent breaking changes across kernel-userland boundaries (`KABI`). Any structural layout modification must preserve backward compatibility or increment slot versioning.

### 1.5 NixOS & Guix Deterministic Store Closures
- **Content-Addressed Store Paradigm**: Prefer immutable path references (`/sigma/store/<hash>-<name>-<version>`) over mutable global paths.
- **Declarative Configuration Schema**: Configuration changes must be validated against `LinuxBsdDeclarativeConfigEngine` schema before applying.

### 1.6 FreeBSD & OpenBSD Security & Virtualization Primitives
- **Capsicum & Pledge/Unveil Process Isolation**: Every newly created service or userland daemon must declare its required permissions and restrict its execution sandbox using `pledge` (system call restriction) and `unveil` (filesystem path restriction) capabilities.
- **Bhyve & vmm(4) Hardware Hypervisor Acceleration**: Virtualization components in `src/virtualization/` must maintain zero-overhead hardware virtualization primitives matching FreeBSD Bhyve and OpenBSD `vmm(4)`.
- **Signify Cryptographic Verification**: Signed manifests must be validated using OpenBSD-style `signify` Ed25519 signatures.

### 1.7 CachyOS Performance & eBPF Scheduling
- **eBPF `sched_ext` Policy Governors**: CPU and GPU scheduling tasks in `src/system/` and `src/ai/` must leverage eBPF-guided dynamic scheduling policies (`scx_bpfland`, `scx_rusty`, `scx_lavd`).
- **Low-Latency Memory Tuning**: ZRAM zstd compression, THP madvise, and swappiness parameters must be optimized for real-time responsiveness.

---

## 2. Core Architectural Mandates for AI Agents

### Mandate 1: Zero External Dependencies (`klib` Mandate)
- AI Agents must **NEVER** introduce external C/C++ libraries, Python scripts, or unvetted third-party crates into core kernel and system shards.
- All algorithms, parsers, codecs, protocols, and data structures must be implemented natively in Safe Rust within `src/klib/` or specialized native modules.

### Mandate 2: Safe Rust First
- Avoid `unsafe` blocks unless interfacing directly with bare-metal hardware registers or MMIO.
- When `unsafe` is strictly required, wrap it in a safe, memory-checked abstraction and document the exact safety invariants.

### Mandate 3: Standard Prelude Compatibility & Compilation
- **Do NOT** import `use std::format;` or duplicate prelude macros. Use `format!` directly as standard in Rust.
- Ensure all code compiles cleanly under `cargo check --lib` and passes both standalone and full test suites.

### Mandate 4: Multi-Distro Universal Package Interoperability
- Package management modifications in `src/sigpkg/` or `src/package/` must maintain seamless translation across `.deb`, `.rpm`, `.pkg.tar.zst`, `.apk`, `.ebuild`, `.hpkg`, and `.pkg` formats via `UniversalPackageAdapter` and `UniversalPmCommandDispatcher`.

---

## 3. Step-by-Step AI Agent Component Development Workflow

AI Agents developing or modifying components in SigmaOS must follow this 6-step lifecycle:

```
[1. Diagnostic & Scope Analysis] ---> [2. Safe Rust Blueprint Design] ---> [3. Implementation in src/]
                                                                                 |
[6. Documentation & Portal Sync] <--- [5. Full Test Verification] <--- [4. Local Unit Testing]
```

1. **Diagnostic & Scope Analysis**:
   - Inspect existing module definitions in `src/` and `WHAT_IS_WORKING_AND_NOT_WORKING.md`.
   - Identify the exact System Shard (`S-SHARD-01` through `S-SHARD-12`) or subsystem responsible for the component.

2. **Safe Rust Blueprint Design**:
   - Design data structures, trait interfaces, and error types without external dependencies.
   - Map security permissions to native Capability Gate Permissions (`Permission::FileRead`, `Permission::NetworkTcp`, etc.).

3. **Implementation**:
   - Write code in the designated source file under `src/`.
   - Export new public structs/functions in the parent `mod.rs`.
   - Ensure clean imports without duplicate or non-existent symbols.

4. **Local Unit Testing**:
   - Include inline `#[cfg(test)] mod tests` blocks covering nominal execution, boundary conditions, and invalid inputs.

5. **Full Test Suite Verification**:
   - Execute `./run_sigma_tests.sh` in the sandbox environment.
   - Verify that all test suites pass with 0 failures and 0 compilation errors.

6. **Documentation & Portal Sync**:
   - Update `Home.md` and `Table-of-contents.md` across all 5 mirror locations (`./`, `docs/`, `wiki/`, `wiki_repo/`, `WIKI/`, `wiki_content/`).
   - Replicate new documentation files across all wiki mirror paths to ensure 100% SHA-256 hash parity.

---

## 4. Verification & QA Checklist for AI Agents

Before submitting changes, AI Agents must verify:

- [x] Code compiles without errors under `cargo check --lib`.
- [x] All unit tests pass cleanly via `./run_sigma_tests.sh`.
- [x] No `use std::format;` or invalid prelude imports exist.
- [x] No external binary or C library dependencies introduced.
- [x] New documentation files created under `docs/` are synchronized across `wiki/`, `wiki_repo/`, `WIKI/`, and `wiki_content/`.
- [x] SHA-256 checksums match 100% across all documentation mirror locations.

---

*SigmaOS AI Agent Component Development Guidance — Version 1.0 (2026)*
