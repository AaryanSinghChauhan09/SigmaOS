# Runbook: QEMU Boot Validation

## Agent Role
Boot and Runtime Validator (`boot-validator`)

## Workflow Steps
1. Build ISO or kernel image artifact in clean container environment.
2. Launch QEMU with x86_64 target configuration and serial redirected to log file.
3. Monitor serial output stream for early kernel banner, interrupt setup, VFS mount, and shell prompt.
4. Execute smoke-test commands (`uname`, `help`, `exit`).
5. Save serial console log to `reports/boot/qemu-x86_64.log` and structured outcome to `reports/boot/qemu-x86_64.json`.
6. Update boot matrix with QEMU version, artifact SHA256, and execution timestamp.
