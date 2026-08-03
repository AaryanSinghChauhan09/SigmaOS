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
