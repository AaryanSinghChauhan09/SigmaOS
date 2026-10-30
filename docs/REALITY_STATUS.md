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
| User mode and Linux ABI | No end-to-end unmodified Linux workload evidence | Not verified |
| Virtio block/network | Models and partial implementations exist; interrupt-driven path is not proven | Partial |
| Dynamic ELF loading | No verified end-to-end loader path | Not verified |
| GPU/Wayland desktop | Prototype and architectural code; no real GPU session evidence | Prototype |
| Package transactions | Dependency preparation, conflict checks, and scoped rollback are unit-tested | Partial |

## Product wedge

The near-term target is a small, secure microVM/virtualization host. It uses a
narrow virtio-focused hardware surface and gives SigmaOS measurable goals:

1. Boot a real x86_64 image in QEMU on every relevant change.
2. Reach a serial shell and run a real statically linked program.
3. Add virtio-block and virtio-net with interrupt-driven I/O.
4. Publish reproducible boot, memory, syscall, and network measurements.

Desktop breadth, multi-distro parity, creative applications, and physical GPU
support remain longer-term work until this wedge is demonstrably reliable.
