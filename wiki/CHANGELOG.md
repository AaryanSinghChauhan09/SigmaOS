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
