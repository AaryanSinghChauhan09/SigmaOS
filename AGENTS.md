# SigmaOS AI Agent Architectural Guide & Directives (`AGENTS.md`)

Welcome, AI Agent! This document provides the authoritative architectural specification, conventions, testing protocols, and guidelines for autonomous engineering agents working on the **SigmaOS** codebase.

---

## 1. System Overview & Core Philosophy

**SigmaOS** is a sovereign, zero-dependency, `#![no_std]` compliant operating system written in Rust. It incorporates paradigms, security models, and performance innovations absorbed from 21+ open-source OS ecosystems (Arch, Debian, Fedora, NixOS, CachyOS, Void, Alpine, Gentoo, Pop!_OS, FreeBSD, OpenBSD, NetBSD, DragonFly BSD, Illumos/Solaris, SmartOS, Haiku, Plan 9, Minix 3, and Contiki-OS).

### Key Architectural Directives
1. **Zero External Dependencies**: All core production dependencies under `[dependencies]` in `Cargo.toml` must remain zero. No external third-party crates.
2. **Strict `#![no_std]` Compatibility**: Subsystems in `src/` must maintain strict `#![no_std]` compatibility using `alloc::` primitives (`alloc::format`, `alloc::string::String`, `alloc::vec::Vec`, `alloc::collections::BTreeMap`) rather than `std` imports for memory operations.
3. **Multi-Architecture Support**: SigmaOS supports 7 CPU architectures (`X86_32`, `X86_64`, `ARM64`, `RISCV64`, `LoongArch64`, `PowerPC64`, `S390x`) via `src/arch/portability.rs` and `src/arch/hal.rs`.
4. **Proactive Testing & Verification**: Every change must be verified using standalone unit test binaries (`rustc --edition=2021 --test ...`) and the global test runner `./run_sigma_tests.sh`.

---

## 2. Directory Layout & Module Hierarchy

```
.
├── src/                                  # Core operating system source
│   ├── arch/                             # CPU architecture support & HAL
│   │   ├── portability.rs                # Multi-arch register contexts & context switching
│   │   ├── hal.rs                        # Hardware abstraction layer
│   │   ├── cpu_features.rs               # CPU ISA feature detection (x86-64-v1..v4, AVX-512)
│   │   └── comprehensive.rs              # Paging, trap frames, and multi-arch MMU
│   ├── distro/                           # Linux & BSD distribution parity and leapfrog engines
│   │   ├── sovereign_nextgen_distro_leap.rs # SchedExt BPF scheduler, Landlock v5, CAS store, HAMMER2 dedup
│   │   ├── open_source_distro_innovations.rs # NuttX RTOS, OpenBSD vmm/bhyve, DTrace, Gentoo EAPI 8
│   │   ├── linux_bsd_inspirations.rs     # Universal 21-distro cross-subsystem bridge
│   │   └── sovereign_distro_dominance.rs # Zero-copy store, BORE scheduler, PQC VPN
│   ├── kernel/                           # Core kernel scheduling, paging, IPC
│   │   ├── scheduler.rs                  # EEVDF & ULE process scheduler
│   │   ├── bore.rs                       # CachyOS BORE & NuttX RT preemption scheduler
│   │   ├── paging.rs                     # Demand paging & page fault handling
│   │   └── process.rs                    # Process lifecycle & zero-copy IPC
│   ├── package/ & sigpkg/                # Universal multi-format package manager & AUR
│   │   ├── universal.rs                  # Multi-format package parsing & adapter pipeline
│   │   ├── aur_integration.rs            # Arch User Repository (AUR) client & auditing
│   │   └── arch_pacman_engine.rs         # Pacman repo management & PGP keyring
│   ├── compatibility/                    # Clean-room OS compatibility layers
│   │   ├── fedora.rs                     # Fedora/RHEL DNF, SELinux, Bodhi, Ignition, status.fpo
│   │   ├── bsd.rs                        # FreeBSD Jails & OpenBSD PF firewall
│   │   ├── linux_standards.rs            # LSB, FHS, PAM, Cgroup v2 governor
│   │   └── india_stack.rs                # UPI, GST, MICR cheque processing
│   ├── drivers/                          # Dynamic hardware manager & multi-OS driver stack
│   ├── security/                         # Hardened security (Pledge, Unveil, Capsicum, Landlock, PQC)
│   └── klib/                             # Zero-dependency kernel library, CSPRNG, allocators
├── .github/workflows/                    # CI/CD pipelines
│   ├── ci.yml                            # Main CI build & distro test suite
│   ├── linux-cgroup-eevdf-sched-ci.yml   # Scheduler & SchedExt BPF verification
│   └── 06_Documentation_Pages_Sync.yml   # Rustdoc HTML generation & Pages artifact staging
├── tests/                                # Rust inspection tests & pytest suite
└── run_sigma_tests.sh                    # Global test suite runner script
```

---

## 3. Subsystem Breakdown for AI Agents

### A. Multi-Architecture HAL (`src/arch/`)
- Enums: `Architecture` (`X86_32`, `X86_64`, `ARM64`, `RISCV64`, `LoongArch64`, `PowerPC64`, `S390x`).
- Context Structures: `X86Context`, `X64Context`, `Arm64Context`, `Riscv64Context`, `LoongArch64Context`, `Ppc64Context`, `S390xContext`.
- Context Switch Engine: `SovereignContextSwitchEngine` handles context saving/restoration and kernel trap simulation.

### B. Distro Leapfrog & Parity Subsystem (`src/distro/`)
- `SovereignSchedExtEngine`: Linux 6.12+ `sched_ext` extensible BPF scheduler supporting dynamic policy switching (`ScxBpfland`, `ScxLavd`, `ScxCachyBore`, `ScxCentral`).
- `SovereignLandlockV5Guard`: Linux Landlock v5 path & TCP access control fused with FreeBSD Capsicum & OpenBSD Pledge/Unveil.
- `SovereignHermeticCasStoreEngine`: Content-Addressed Storage package store with Merkle closure tree verification and zero-downtime generation switching.
- `SovereignMicroarchJitEngine`: CPU ISA level detection (`X86_64V1..V4`, `Arm64Neoverse`, `RiscvVector1_0`) and dynamic SIMD JIT dispatching.
- `SovereignHammer2DeduplicationEngine`: DragonFly BSD HAMMER2 multi-master CoW FNV-1a block deduplication and emergency read-only snapshots.

### C. Universal Package Manager (`src/package/` & `src/sigpkg/`)
- Supports 50+ package formats (DEB, RPM, Pacman, APK, Flatpak, Snap, AppImage, XBPS, Ebuild, Ports, PKG, etc.).
- Auto-converts foreign package manifests into native `SigmaPkg` format with dependency resolution, scriptlet translation, and sandboxing.

---

## 4. Testing & Verification Protocols for AI Agents

When editing or adding code in SigmaOS, AI agents MUST follow these verification procedures:

### Step 1: Run Subsystem Standalone Unit Test
Compile and execute the specific standalone unit test binary:
```bash
# Example for distro leapfrog engine
rustc --edition=2021 --test src/distro/sovereign_nextgen_distro_leap.rs -o build/test_nextgen_leap && ./build/test_nextgen_leap

# Example for multi-arch portability
rustc --edition=2021 --test src/arch/portability.rs -o build/test_arch_portability && ./build/test_arch_portability

# Example for open-source distro innovations
rustc --test --edition 2021 src/distro/open_source_distro_innovations.rs -o build/test_open_source_distro_innovations && ./build/test_open_source_distro_innovations
```

### Step 2: Run Full System Test Suite
Before committing changes, execute the main test runner:
```bash
./run_sigma_tests.sh
```

### Step 3: Check Code Compilation
Verify library compilation:
```bash
cargo check
```

---

## 5. Git & Submission Guidelines for AI Agents

1. **Branch Naming**: All git branches created by agents MUST begin with `jules-` prefix (e.g., `jules-add-arch-support-123`).
2. **Pre-Commit Procedure**: Always execute pre-commit instructions before submitting.
3. **Commit Messages**: Follow conventional commit syntax (e.g. `feat(arch): add LoongArch64 and PPC64 support`, `fix(distro): resolve duplicate method definitions`).

---
*End of AGENTS.md*
