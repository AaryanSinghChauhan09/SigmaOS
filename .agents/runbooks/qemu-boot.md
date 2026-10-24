# Runbook: Boot and Runtime Validator Execution (.agents/runbooks/qemu-boot.md)

1. Build ISO image in clean build environment.
2. Launch QEMU x86_64 with fixed settings:
   - Fixed RAM, CPU cores, display, and serial pipe output.
3. Wait for serial prompt and capture boot logs to `reports/boot/qemu_x86_64.log`.
4. Run basic smoke commands and verify clean exit.
5. Record execution metrics and SHA256 hashes in `reports/boot/qemu_x86_64.json`.
