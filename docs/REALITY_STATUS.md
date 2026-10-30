# SigmaOS Reality Status

Last reviewed: 2026-10-30

This page separates verified behavior from design intent. Unit tests and data
structures are useful evidence, but they do not prove hardware or QEMU
behavior. A capability is promoted only when its execution evidence is
repeatable.

| Area | Current evidence | Status |
| --- | --- | --- |
| Rust library build | `CARGO_BUILD_JOBS=1 cargo check --lib` | Verified locally |
| Multiboot kernel | C source and linker script exist; real boot is CI-gated | In progress |
| QEMU boot-to-shell | Real ISO/QEMU harness added; first CI run is required | In progress |
| ISO release artifact | Builder now requires a SigmaOS kernel and a real ISO9660 tool; host-kernel and tar fallbacks removed | Build-verified locally; boot/install integration remains |
| Installer safety | Explicit target and destructive-action confirmation, dry-run mode, NVMe partition naming, and fail-fast package/bootloader checks | Script-verified locally; real disk installation not exercised |
| Desktop session entry point | Single `sigmaos-session` launcher with explicit Hyprland/UWSM or Zenith backend selection; shell service has a persistent daemon mode | Script-verified locally; live compositor session not exercised |
| Component repositories | Twelve component repositories are published with initial contracts; network-manager creation is pending GitHub rate limiting | Published scaffolds; code extraction, ownership, CI, and release integration remain |
| Ring-3 proof workload | 32-bit Multiboot kernel now configures GDT/TSS/IDT and enters a test workload through `iret`; hardware/QEMU execution still needs CI evidence | In progress |
| ELF load plan | Bounds-checked ELF64/x86_64 `PT_LOAD` parser with W^X, entry validation, zero-initialized image materialization, and page-level mapping plan | Formatting/static validation passes; page-table mapping and execution are not integrated |
| x86_64 syscall MSRs | Validated `STAR`/`LSTAR`/`FMASK` configuration abstraction added; privileged install is not yet wired into boot | Partial |
| User mode and Linux ABI | No end-to-end unmodified Linux workload evidence | Not verified |
| Virtio block/network | Models and partial implementations exist; interrupt-driven path is not proven | Partial |
| Dynamic ELF loading | Relocation engine supports checked `RELATIVE` entries, indexed symbol resolution, and fails closed for unresolved symbols | Not verified end-to-end |
| GPU/Wayland desktop | Prototype and architectural code; no real GPU session evidence | Prototype |
| Package transactions | Dependency preparation, conflict checks, and scoped rollback are unit-tested | Partial |
| Distribution release lifecycle | Signed architecture-specific manifests with edge → RC → stable promotion gates | Implemented as policy; repository publishing and installer integration remain |

## Product wedge

The near-term target is a small, secure microVM/virtualization host. It uses a
narrow virtio-focused hardware surface and gives SigmaOS measurable goals:

1. Boot a real x86_64 image in QEMU on every relevant change.
2. Reach a serial shell and run a real statically linked program.
3. Add virtio-block and virtio-net with interrupt-driven I/O.
4. Publish reproducible boot, memory, syscall, and network measurements.

Desktop breadth, multi-distro parity, creative applications, and physical GPU
support remain longer-term work until this wedge is demonstrably reliable.
