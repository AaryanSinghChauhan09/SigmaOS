# SigmaOS Changelog

All notable changes to the SigmaOS sovereign microkernel and system services are documented here.

## [Unreleased]
### Added
- Comprehensive branch merge including 10+ feature branches
- India Stack integration with mock UPI payments and GST tax calculation
- ReactOS-inspired Windows NT compatibility layer with PE parsing
- Local LLM orchestrator with device-aware model scheduling
- Context window pruner for sliding dialogue history
- Security enhancements with Kali Stack and Nemoclaw frameworks
- Multilingual support for Hindi, Tamil, and Sanskrit languages
- Memory leak fixes in custom Vec implementation with proper Drop trait
- Linux-grade Virtual Filesystem enhancements
- Phase 1 core components: VMM paging, enhanced round-robin scheduler, USB HID keyboard, VESA framebuffer
- OOP peripheral architecture with fixed build errors
- Enhanced accessibility features (keyboard, magnifier, screen reader)
- AI next-generation capabilities and wandr AI assistant
- Audio driver and editor enhancements
- Authentication and identity management improvements
- Expanded compatibility layer with multiple OS support (absorb_tools, apache_ossie, chimera_linux, gentoo, historic_linux, mint_linux, relay_nexus, solid_kernel, sovereign_suite, tiny_core, wasm_sandbox)
- Device management and driver framework improvements
- Filesystem with COW snapshot and VFS enhancements
- systemd init compatibility layer
- Kernel scheduler and memory management enhancements
- Custom memory management (buddy allocator, paging, custom Vec)
- ML inference capabilities
- Network stack with TCP/UDP improvements
- Process management and spawn capabilities
- Security framework with hardening and vulnerability management
- Package management with RPM compatibility and universal adapter
- Virtualization with hypervisor, microVM, and OCI pod support
- Stateful firewall with conntrack and netfilter hooks
- Indian compliance integration architecture
- Production Readiness Roadmap
- Btrfs filesystem improvements with Linux distro-inspired features
- Daemon improvements inspired by Linux distributions
- Driver improvements with Linux-inspired features
- Toolchain GCC adapter fixes
- Atomic kernel headers for Linux compatibility
- Comprehensive documentation updates

## [1.1.0] - 2026-08-02
### Added
- SteamOS-style GPU driver recovery and reset in `drivers/graphics/sigma_kms.cpp`.
- Clear Linux-inspired dynamic power/performance scaling profiles in the graphics driver.
- Polymorphic universal peripheral matching and USB speed negotiation state machine in `drivers/usb/sigma_usb_hcd.cpp`.
- Zero-allocation DAG Topological Sorter (Kahn's Algorithm) in `kernel/drivers/sigma_driver_manager.cpp` to sequence loading dependencies.
- Native Hardware and Drivers test suite in `tests/sigma_test_runner.cpp` with 46 passing assertions.

## [1.0.0] - 2026-07-15
### Added
- First public release of SigmaOS sovereign system core.
- Capability-Based Sandboxing and Pledge/Unveil permission checks.
