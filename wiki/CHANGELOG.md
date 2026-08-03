# SigmaOS Changelog

All notable changes to the SigmaOS sovereign microkernel and system services are documented here.

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
