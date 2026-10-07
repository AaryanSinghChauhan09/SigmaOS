# Inspiration & References

SigmaOS draws inspiration from these outstanding projects. Each entry maps to
concrete components in this repository — not just ideas.

| Project | Inspiration | SigmaOS components |
|---|---|---|
| Linux Kernel | Kernel architecture, VFS, scheduler design, driver model | `src/kernel/scheduler.rs` (CFS/EEVDF model), `src/vfs/`, `src/drivers/`, `src/filesystem/btrfs.rs` (fail-closed metadata model) |
| Omarchy | Desktop experience, modular installer, theme system | `zenith_desktop/` (dock ARIA states, command palette), `web_ui/index.html` (installation preview flow), `src/theming/omarchy_theme_suite.rs`, `src/desktop/launcher.rs` (keyboard-first launcher), `src/desktop/screenshot_capture.rs` |
| os-tutorial | Bootloader patterns, interrupt handling, educational approach | `src/arch/x86_64/` (GDT/IDT/paging stubs), `tools/sigma_bootloader_compat.rs`, `scripts/build_iso.sh` (fails closed until boot is validated) |
| Linux Mint | User experience, polish, documentation standards | `src/desktop/software_manager.rs` (mintinstall-inspired), `src/mintinstall/`, `src/mintupdate/`, `src/desktop/mint_backup_tool.rs`, `src/desktop/mint_update_manager.rs`, `src/tools/mint_system_report.rs` (honest host inspection), `docs/INSPIRATION_AND_REFERENCES.md` (this file) |
| Redox OS | Rust-based microkernel design patterns | `src/ipc/helenos_async.rs` (async IPC), `src/kernel/xdp_engine_sovereign.rs` (validated packet parsing), capability-gated module layout in `src/lib.rs` |
| xv6 | Clean educational OS design | Small focused modules with standalone `rustc --test` suites (see `run_sigma_tests.sh`), `src/desktop/` manager models with explicit unavailable-backend behavior |

## Notes

- Prototype status is tracked honestly: `FEATURE_STATUS.toml` records
  module-level test evidence only, not bootability or hardware support.
- The installer (`web_ui/index.html`) is a **preview**: it detects no disks
  and installs nothing until a verified bootable image and storage backend
  exist.
- System reports (`src/tools/mint_system_report.rs`) read only from host
  procfs/os-release sources and mark missing data unavailable instead of
  fabricating values.
