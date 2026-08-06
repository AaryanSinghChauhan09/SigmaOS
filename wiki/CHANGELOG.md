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
