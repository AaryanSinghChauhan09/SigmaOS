# SigmaOS Boot Matrix

## QEMU / Hardware Boot Validation Status

| Environment | Architecture | Firmware | Status | Last Validated | Evidence / Artifact |
|---|---|---|---|---|---|
| QEMU x86_64 | x86_64 | SeaBIOS / OVMF | Validated (Boot to Shell) | Monthly CI | `build/boot/boot_smoke_result.json` |
| QEMU aarch64 | aarch64 | EDK2 UEFI | Partial Model | Roadmap | Pending hardware testing |
| Bare Metal x86 | x86_64 | UEFI | Untested | Roadmap | Pending physical hardware test log |
