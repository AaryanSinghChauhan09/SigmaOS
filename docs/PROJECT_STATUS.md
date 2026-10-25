# SigmaOS Project Capability & Status Inventory

## Status Promotion Levels

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | Runtime path plus integration tests |
| Hardened | Fuzzing, negative tests, security review |
| Supported | Reproducible boot/install/update evidence |

## Component Readiness Matrix

| Component | Target Level | Current Level | Evidence Path |
|---|---|---|---|
| Kernel / Boot | Integrated | Integrated | `reports/boot/qemu-x86_64.log` |
| Memory Subsystem | Integrated | Integrated | `tests/memory/physical.rs` |
| VFS / Filesystems | Integrated | Integrated | `tests/filesystem/core.rs` |
| IPC & Signals | Integrated | Integrated | `tests/signal/signal_test.rs` |
| Security & Sandboxing | Hardened | Hardened | `tests/security/capability_test.rs` |
| Drivers & VirtIO | Integrated | Integrated | `tests/storage/block.rs` |
| Package Management | Integrated | Prototype | `tests/sigpkg_client_tests.rs` |
| Installer & Update | Integrated | Integrated | `installer/tests/install_dry_run.rs` |
| Zenith Desktop | Integrated | Prototype | `zenith_desktop/` |
