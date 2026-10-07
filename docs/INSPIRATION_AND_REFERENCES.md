# Inspiration & References

SigmaOS draws inspiration from these outstanding projects. Each entry maps to
concrete components in this repository — not just ideas.

| Project | Inspiration | SigmaOS components |
|---|---|---|
| **Linux Kernel** | Kernel architecture, VFS, scheduler design, driver model, input subsystem | `src/kernel/scheduler.rs` (CFS/EEVDF model), `src/vfs/`, `src/drivers/`, `src/filesystem/btrfs.rs` (fail-closed metadata model), `src/input/` (Linux input event / evdev interface) |
| **Omarchy** | Desktop experience, modular installer, theme system | `zenith_desktop/` (dock ARIA states, command palette), `web_ui/index.html` (installation preview flow), `src/theming/omarchy_theme_suite.rs`, `src/desktop/launcher.rs` (keyboard-first launcher), `src/desktop/screenshot_capture.rs` |
| **os-tutorial** | Bootloader patterns, interrupt handling, educational approach | `src/arch/x86_64/` (GDT/IDT/paging stubs), `tools/sigma_bootloader_compat.rs`, `scripts/build_iso.sh` (fails closed until boot is validated) |
| **Linux Mint** | User experience, polish, documentation standards | `src/desktop/software_manager.rs` (mintinstall-inspired), `src/mintinstall/`, `src/mintupdate/`, `src/desktop/mint_backup_tool.rs`, `src/desktop/mint_update_manager.rs`, `src/tools/mint_system_report.rs` (honest host inspection), `docs/INSPIRATION_AND_REFERENCES.md` (this file) |
| **Redox OS** | Rust-based microkernel design patterns | `src/ipc/helenos_async.rs` (async IPC), `src/kernel/xdp_engine_sovereign.rs` (validated packet parsing), capability-gated module layout in `src/lib.rs` |
| **xv6** | Clean educational OS design | Small focused modules with standalone `rustc --test` suites (see `run_sigma_tests.sh`), `src/desktop/` manager models with explicit unavailable-backend behavior |

## Component Development Status

| Component | Status | Inspiration | Next Milestone |
|---|---|---|---|
| **Linux Input Event (evdev) Interface** | Prototype | Linux evdev | QEMU input device integration |
| **Power Management** | Prototype | Linux ACPI, BSD powerd | ACPI table parsing |
| **Package Management** | Prototype | APT/DNF/PKG | Multi-distro support |
| **Kernel Subsystem** | Prototype | Full Linux kernel | QEMU boot validation |
| **Desktop Subsystem** | Prototype | Omarchy, Mint, Cinnamon | Verified boot-to-graphical-session |

## Notes

- Prototype status is tracked honestly: `FEATURE_STATUS.toml` records
  module-level test evidence only, not bootability or hardware support.
- The installer (`web_ui/index.html`) is a **preview**: it detects no disks
  and installs nothing until a verified bootable image and storage backend
  exist.
- System reports (`src/tools/mint_system_report.rs`) read only from host
  procfs/os-release sources and mark missing data unavailable instead of
  fabricating values.

## Development Plans

For detailed development plans for each component, see:
- [Development Plans](Development-Standalone.md) - Comprehensive plans for each SigmaOS component
- [Roadmap](Roadmap.md) - Overall development roadmap
- [Component Index](Component-Index.md) - Index of all SigmaOS components with status

## Inspiration Sources

SigmaOS uses inspiration from multiple sources to build a unique operating system:

1. **Linux Kernel** - Core kernel architecture, VFS, scheduler design
2. **Omarchy** - Desktop experience, modular installer, theme system
3. **Linux Mint** - User experience, polish, documentation standards
4. **Redox OS** - Rust-based microkernel design patterns
5. **xv6** - Clean educational OS design for reference implementation

The inspiration system allows SigmaOS to adopt proven designs while
building a unique operating system with its own goals and architecture.
