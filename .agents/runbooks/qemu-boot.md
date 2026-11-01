# Runbook: QEMU Boot and Runtime Validator (`qemu-boot.md`)

## Objective
Build the bootable artifact, launch in QEMU x86_64 simulation, capture serial console output, verify reachability of shell prompt, and validate basic command execution.

## Execution Sequence
1. **Clean Workspace & Build:**
   ```bash
   make clean
   python3 tools/boot_matrix.py
   ```
2. **Serial Console Log Analysis:**
   - Verify BIOS/UEFI firmware handoff.
   - Confirm PCI MMIO configuration and kernel memory manager initialization.
   - Verify root VFS mount and userspace init spawn.
   - Confirm shell prompt is reached without kernel panics.
3. **Artifact Recording:**
   - Serial log saved to `reports/boot/qemu_x86_64.log`.
   - Structured JSON report saved to `reports/boot/qemu_x86_64.json`.
   - Update matrix in `reports/boot/boot_matrix.json`.

## Failure Classification
- `bootloader`: Fails prior to kernel entry point.
- `kernel_panic`: Unhandled fault or assert during kernel early boot.
- `driver`: PCI, serial, or timer interrupt failure.
- `init`: Root VFS mount or process #1 launch error.
- `shell`: Shell daemon crash or failure to present prompt.
