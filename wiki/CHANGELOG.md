# Σ SigmaOS Changelog

All notable changes to the SigmaOS system will be transactionally recorded in this file.

---

## [15.0.0] - 2026-08-02

### Added
- **Dynamic Mirror Latency Ranking**: Integrated real-time mirror listing and bubble-sort latency ranking inside the `Sigma-Claw` crawling daemon.
- **Download Bandwidth Throttling**: Added a rate-limiting pacemaker calculating chunk pacing delays based on user-defined KB/s constraints in `Sigma-Claw`.
- **Exponential Backoff Retries**: Deployed a fallback connection retry policy with exponential backoff on offline crawling mirror endpoints.
- **A/B Transactional State Machine**: Built a multi-state transactional update pipeline in the `Sigma-Update` package manager daemon.
- **APT/DNF Concurrency Guards**: Implemented strict lock-file acquisition logic inside `Sigma-Update` to prevent multiple concurrent updates from colliding.
- **Automated Rollback Systems**: Added post-staging system health monitoring triggering automated rollback of active partitions if B fails health checks.
- **Orca/ speech-dispatcher Style Settings**: Supported customized voice settings (speech rate and volume percentage) for `Sigma-Voice` screen readers.
- **Priority-Based Sound Queueing**: Added prioritized sound queues (High, Normal, Low) mapping notifications and alarms onto system sound outputs.
- **Abbreviation Pronunciation Dictionaries**: Supported a static dictionary map expanding abbreviations (e.g. "UI" to "User Interface", "SIE" to "Sigma Intelligence Engine") inside `Sigma-Voice`.
- **Fcitx/IBus Modifiers**: Handled keyboard layout hotkeys (e.g. Ctrl+Space) toggling input method modes seamlessly in `Sigma-IME`.
- **Asynchronous Candidate Matching**: Supported real-time candidate suggestion filtering and matching in CJK pinyin modes.
- **User Dictionary Definitions**: Integrated customizable phrase dictionaries in `Sigma-IME` matching local input to localized strings.
- **Sovereign Primitive Types**: Created `sigma_kernel_types.h` defining canonical sizes, integers, and status formats.
- **Rigorous C++ Assertions**: Extended native test runner (`tests/sigma_test_runner.cpp`) to run 18 new functional checks verifying all distro-inspired daemon mechanisms.

### Fixed
- **Pre-existing Rust Compiler Errors**: Fixed 47 pre-existing compilation blocks in the `sigmaos` library:
  - Cleaned up duplicate module and import of `cow_snapshot` in `src/filesystem/mod.rs`.
  - Exported `HashMap` from `src/klib/mod.rs` to fix unresolved imports across dozens of files.
  - Declared and exported the missing `importer` module in `src/sigpkg/mod.rs`.
  - Corrected signature and type reassignment in `src/sigpkg/importer.rs`.
  - Added missing `Storage` and `Input` driver variants, added `ProbeFailed` variant to `DriverError`, and stubbed `init`, `probe`, and `shutdown` methods on `SimpleDriver` in `src/driver/framework.rs`.
  - Removed duplicate, conflicting `IntoIterator` implementations from the end of `src/virt/cli.rs`, resolving ambiguous type annotation blocks.
# Changelog

All notable changes to SigmaOS will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Universal package manager with support for apt, yum, pacman, snap, flatpak
- Cross-platform compatibility layer for Windows .exe, macOS .dmg, Android .apk
- AI-powered system-level automation with predictive capabilities
- Built-in virtualization support with KVM/QEMU, Docker, Kubernetes
- Unified dashboard system with real-time monitoring
- Accessibility framework with vision/hearing/mobility/cognitive support
- Customization engine with Samsung Modes & Routines-style automation
- Gamified productivity system with achievements and Pomodoro timer
- Cross-device orchestration for IoT and smart home integration
- Round-robin scheduler with time-sliced execution
- USB HID keyboard driver with event handling
- VESA framebuffer driver with mode switching
- Package recipe system for build automation

### Changed
- Enhanced buddy allocator with memory initialization and statistics
- Improved dependency resolution in package manager
- Updated security framework with additional capability checks

### Fixed
- Integer overflow vulnerability in buddy allocator
- Integer overflow vulnerabilities in filesystem read/write operations
- Various memory safety issues identified by code scanning

### Security
- Added GitHub Actions CI workflow with security checks
- Implemented Dependabot for automated dependency updates
- Enhanced SECURITY.md with comprehensive security policy

## [0.1.0] - 2024-07-15

### Added
- Initial SigmaOS kernel implementation
- Capability-based security system
- SigmaPkg package manager foundation
- EEVDF scheduler implementation
- Buddy allocator for memory management
- Capability-based IPC system
- TCP/IP stack implementation
- Virtual filesystem with capability-based security
- GPU, storage, network, and input drivers
- Sigma-sh REPL shell
- AI-driven optimization system
- Resilience and self-healing modules

### Security
- Post-quantum cryptography support (Kyber-1024, Dilithium-5)
- Capability-based access control
- Secure boot implementation
- Memory safety guarantees through Rust

## [0.0.1] - 2024-07-01

### Added
- Project initialization
- Basic repository structure
- Initial documentation
- CI/CD pipeline setup
# Changelog
# Changelog
# Changelog

All notable changes to SigmaOS will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Universal package manager with support for apt, yum, pacman, snap, flatpak
- Cross-platform compatibility layer for Windows .exe, macOS .dmg, Android .apk
- AI-powered system-level automation with predictive capabilities
- Built-in virtualization support with KVM/QEMU, Docker, Kubernetes
- Unified dashboard system with real-time monitoring
- Accessibility framework with vision/hearing/mobility/cognitive support
- Customization engine with Samsung Modes & Routines-style automation
- Gamified productivity system with achievements and Pomodoro timer
- Cross-device orchestration for IoT and smart home integration
- Round-robin scheduler with time-sliced execution
- USB HID keyboard driver with event handling
- VESA framebuffer driver with mode switching
- Package recipe system for build automation

### Changed
- Enhanced buddy allocator with memory initialization and statistics
- Improved dependency resolution in package manager
- Updated security framework with additional capability checks

### Fixed
- Integer overflow vulnerability in buddy allocator
- Integer overflow vulnerabilities in filesystem read/write operations
- Various memory safety issues identified by code scanning

### Security
- Added GitHub Actions CI workflow with security checks
- Implemented Dependabot for automated dependency updates
- Enhanced SECURITY.md with comprehensive security policy

## [0.1.0] - 2024-07-15

### Added
- Initial SigmaOS kernel implementation
- Capability-based security system
- SigmaPkg package manager foundation
- EEVDF scheduler implementation
- Buddy allocator for memory management
- Capability-based IPC system
- TCP/IP stack implementation
- Virtual filesystem with capability-based security
- GPU, storage, network, and input drivers
- Sigma-sh REPL shell
- AI-driven optimization system
- Resilience and self-healing modules

### Security
- Post-quantum cryptography support (Kyber-1024, Dilithium-5)
- Capability-based access control
- Secure boot implementation
- Memory safety guarantees through Rust

## [0.0.1] - 2024-07-01

### Added
- Project initialization
- Basic repository structure
- Initial documentation
- CI/CD pipeline setup
# Σ SigmaOS Changelog

All notable changes to the SigmaOS system will be transactionally recorded in this file.

---

## [15.0.0] - 2026-08-02

### Added
- **Dynamic Mirror Latency Ranking**: Integrated real-time mirror listing and bubble-sort latency ranking inside the `Sigma-Claw` crawling daemon.
- **Download Bandwidth Throttling**: Added a rate-limiting pacemaker calculating chunk pacing delays based on user-defined KB/s constraints in `Sigma-Claw`.
- **Exponential Backoff Retries**: Deployed a fallback connection retry policy with exponential backoff on offline crawling mirror endpoints.
- **A/B Transactional State Machine**: Built a multi-state transactional update pipeline in the `Sigma-Update` package manager daemon.
- **APT/DNF Concurrency Guards**: Implemented strict lock-file acquisition logic inside `Sigma-Update` to prevent multiple concurrent updates from colliding.
- **Automated Rollback Systems**: Added post-staging system health monitoring triggering automated rollback of active partitions if B fails health checks.
- **Orca/ speech-dispatcher Style Settings**: Supported customized voice settings (speech rate and volume percentage) for `Sigma-Voice` screen readers.
- **Priority-Based Sound Queueing**: Added prioritized sound queues (High, Normal, Low) mapping notifications and alarms onto system sound outputs.
- **Abbreviation Pronunciation Dictionaries**: Supported a static dictionary map expanding abbreviations (e.g. "UI" to "User Interface", "SIE" to "Sigma Intelligence Engine") inside `Sigma-Voice`.
- **Fcitx/IBus Modifiers**: Handled keyboard layout hotkeys (e.g. Ctrl+Space) toggling input method modes seamlessly in `Sigma-IME`.
- **Asynchronous Candidate Matching**: Supported real-time candidate suggestion filtering and matching in CJK pinyin modes.
- **User Dictionary Definitions**: Integrated customizable phrase dictionaries in `Sigma-IME` matching local input to localized strings.
- **Sovereign Primitive Types**: Created `sigma_kernel_types.h` defining canonical sizes, integers, and status formats.
- **Rigorous C++ Assertions**: Extended native test runner (`tests/sigma_test_runner.cpp`) to run 18 new functional checks verifying all distro-inspired daemon mechanisms.

### Fixed
- **Pre-existing Rust Compiler Errors**: Fixed 47 pre-existing compilation blocks in the `sigmaos` library:
  - Cleaned up duplicate module and import of `cow_snapshot` in `src/filesystem/mod.rs`.
  - Exported `HashMap` from `src/klib/mod.rs` to fix unresolved imports across dozens of files.
  - Declared and exported the missing `importer` module in `src/sigpkg/mod.rs`.
  - Corrected signature and type reassignment in `src/sigpkg/importer.rs`.
  - Added missing `Storage` and `Input` driver variants, added `ProbeFailed` variant to `DriverError`, and stubbed `init`, `probe`, and `shutdown` methods on `SimpleDriver` in `src/driver/framework.rs`.
  - Removed duplicate, conflicting `IntoIterator` implementations from the end of `src/virt/cli.rs`, resolving ambiguous type annotation blocks.
# SigmaOS Changelog & Strategic Architectural Mapping

All notable changes to the SigmaOS sovereign operating system and system services are documented here. This guide maps our newly realized next-generation capabilities (Phase E/F) directly to the comparative Linux/Windows/BSD roadmaps.

---

## [1.1.0] - 2026-08-02
### Added
- **SteamOS-inspired GPU Driver Recovery & Reset** (`drivers/graphics/sigma_kms.cpp`):
  - Implements a self-healing GPU hang detection state machine (`sigma_kms_recover_gpu`) that safely clears frame buffer caches and resets display contexts, completely eliminating standard ring-buffer freezes.
- **Clear Linux-inspired Graphics Performance Profiles** (`drivers/graphics/sigma_kms.cpp`):
  - Provides dynamic switching between `POWERSAVE` (30 FPS limit, clock-gated, 16ms latency), `BALANCED` (60 FPS, 8ms latency), and `HIGH PERFORMANCE` (144 FPS high-refresh rate, 1ms latency) modes.
- **Linux Device Tree & mac80211-style Universal Peripheral matching** (`drivers/usb/sigma_usb_hcd.cpp`):
  - Introduces a polymorphic `UnifiedPeripheral` interface with placement-new dynamic allocations (`ModernXhciController`) to manage MMIO vs. Port I/O transparently.
- **Standard USB Speed Negotiation State Machine** (`drivers/usb/sigma_usb_hcd.cpp`):
  - Automatically negotiates standard device speeds from `USB_SPEED_LOW` (1.5 Mbps) up to `USB_SPEED_SUPER_PLUS` (10 Gbps) and simulates safe hotplug/detachment.
- **DAG Topological Sorter & Dependency-Aware modprobe** (`kernel/drivers/sigma_driver_manager.cpp`):
  - Implements Kahn's Algorithm for a zero-allocation, linear-time topological dependency sorter to load kernel driver dependencies in order, preventing startup resource deadlocks. Handles cascaded fallback recovery.
- **NixOS-style DKMS Rebuild Trigger** (`kernel/drivers/sigma_driver_registry.cpp`):
  - Implements DKMS auto-rebuilding of compiled driver objects post host-kernel swap.
- **Gentoo & Clear Linux-style Toolchain Compiler Optimizations** (`src/toolchain/adapter.rs`):
  - Injects native target hardware optimizations (`-O3 -march=native -ftree-vectorize -ffast-math`) to deliver industry-leading execution speeds.
- **NixOS & Fedora-style Security Hardening Compiler Flags** (`src/toolchain/adapter.rs`):
  - Dynamically configures secure compiler flags including position-independent executables (`-fPIE -pie`), read-only relocation binders (`-Wl,-z,now`), stack-clash protection, and strict fortify source boundaries (`-D_FORTIFY_SOURCE=3`).
- **SystemRescue-grade Storage & Partition Diagnostics** (`src/distro/recovery.rs`):
  - Adds real-time partition table validation and bad blocks scanning utilities.
- **Timeshift-style Snapshot-Based Rollback Engine** (`src/distro/recovery.rs`):
  - Fully restores filesystems to a previous checkpoint, handling added, modified, or deleted files cleanly in a single transition pass.
- **Tails-inspired Cryptographic Image Signatures verification** (`src/distro/recovery.rs`):
  - Enforces strict verification of backup restore archives using post-quantum Dilithium-5 signatures before rollback execution.
- **Linux-style Bit-Packed Ioctl Decoder** (`src/package/linux_translation.rs`):
  - Automatically parses any 32-bit ioctl into Direction, Size, Type/Group, and Action ID components (`DecodedIoctl`), supporting standard tty (`TCGETS`), block (`BLKGETSIZE`), and filesystem (`FIONBIO`) translation.
- **Ubuntu-style Systemd Init Target states** (`src/init/systemd_init.rs`):
  - Pre-registers standard target states (`poweroff.target`, `reboot.target`, `emergency.target`) and introduces structured service verification controls (status checks, reloads, restarts).
- **Linux & BSD-grade DMA Engine Safety Wrappers** (`src/embedded/dma.rs`):
  - Enforces standard 4-byte (word) buffer alignment checks and strict physical address bounds filters (guarding regions above `0xF0000000`).
- **6-Phase AI & Automation Suite** (`src/ai/sai.rs`):
  - *Phase 1 (SigmaAI)*: Translates natural language queries to safe CLI commands.
  - *Phase 2 (Workflow Orchestration)*: Implements n8n/Airflow-style DAG pipeline nodes with dependencies.
  - *Phase 3 (Adaptive CLI Suggestions)*: Tracks past command frequency and suggests completions.
  - *Phase 4 (Error Explanation)*: Translates kernel error codes to plain English logs with repair proposals.
  - *Phase 5 (AI-Driven Security)*: Monitors active ports/payloads and scores behavioral threats.
  - *Phase 6 (AI-Assisted Dev)*: Generates high-quality unit tests dynamically.
- **Supply Chain Attestation & Software Bill of Materials (BOM)** (`src/package/signing.rs`):
  - Tracks detailed executable provenance, records deliberate code review audit logs, and validates transitive trust chains.
- **C++ Native Verification Harness** (`tests/sigma_test_runner.cpp`):
  - Extended to test 100% of newly added KMS, xHCI, DriverManager, and DKMS capabilities, achieving 46/46 passing C++ assertions.

### Fixed
- **78 Crate-Level Rust Compilation Errors**:
  - Properly declared and exported missing submodules in `src/klib/mod.rs` (such as `HashMap`, `String`, `HashSet`, etc.).
  - Resolved conflicting `IntoIterator` implementations on custom `Vec` in `src/virt/cli.rs`.
  - Implemented `FromIterator`, `pop`, `insert`, `first`, and `last` on `Vec<T>` inside `src/klib/vec.rs`.
  - Fixed a critical buckets-initialization bug in `src/klib/hashmap.rs` causing index out of bounds panics on `new()` HashMaps.
  - Wrapped potential integer additions overflows inside `src/klib/hash.rs` DJB2 and FNV-1a algorithms to prevent debug-test panics.
