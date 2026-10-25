# Platform Boot Matrix & Acceptance Criteria

## Overview
This matrix establishes the reproducible firmware-to-shell boot paths, required verification signals, and recorded acceptance criteria for SigmaOS across target architectures and execution environments.

---

## Boot Matrix

| Architecture | Platform / Machine | Firmware | Target Artifact | Boot Signal / Marker | Acceptance Status |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **x86_64** | QEMU `q35` / `pc` | SeaBIOS / OVMF UEFI | ISO / Standalone Image | `SIGMAOS BOOT SUCCESS - SHELL READY` | **Verified in CI** |
| **x86_64** | Bare Metal PC | UEFI v2.8+ | USB Live ISO | GRUB / Direct Kernel Entry | Untested |
| **aarch64** | QEMU `virt` | EDK2 / UEFI | Raw Image | `SIGMAOS BOOT SUCCESS` | Experimental |
| **riscv64** | QEMU `virt` | OpenSBI | ELF Kernel | `SIGMAOS BOOT SUCCESS` | Experimental |

---

## Minimum Acceptance Criteria (QEMU x86_64)

1. **Boot Initialization**:
   - Firmware hands off execution to Multiboot2/UEFI loader.
   - Physical memory manager initializes frame allocator.
   - Paging manager constructs 4-level page table identity/kernel mappings.

2. **Kernel Entry & Subsystem Init**:
   - Red-Black scheduler starts init process (PID 1).
   - Core drivers (Serial 16550A, RTC, VirtIO block) probe and attach cleanly.

3. **Userland Handoff**:
   - Init process launches interactive shell or smoke test harness.
   - Shell outputs marker `SIGMAOS BOOT SUCCESS - SHELL READY`.

4. **CI Integration**:
   - `scripts/ci/qemu_boot_smoke.sh` executes QEMU in headless mode.
   - Boot log is captured and verified by `tests/boot/boot_to_shell_test.py`.
