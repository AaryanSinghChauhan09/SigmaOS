# SigmaOS Boot Matrix & Runtime Evidence Log

This matrix tracks boot validation status across target architectures, execution environments, and firmware interfaces.

## Target Platform Status

| Target Architecture | Environment / Emulator | Boot Target | Status | Latest Evidence | Timeout Limit | Failure Policy |
|---|---|---|---|---|---|---|
| x86_64 | QEMU System i386/x86_64 | `shell_ready` | Verified | `build/boot/boot_smoke_result.json` | 5.0s | Panic + Diagnostic Artifact |
| x86_64 | Bare Metal (UEFI) | `shell_ready` | Model/Untested | Manual log required | 10.0s | Recovery Console |
| AArch64 | QEMU virt | `userspace_start` | Partial | Simulated unit test | 5.0s | Fallback Serial |
| RISC-V 64 | QEMU virt | `kernel_entry` | Partial | Simulated unit test | 5.0s | Fallback Serial |

## Boot Validation Requirements

1. **Boot Smoke Test:** `scripts/ci/qemu_boot_smoke.sh` must pass on every push to `main` and in PR CI.
2. **Deterministic Shell Marker:** Boot sequence must produce `[BOOT_STAGE_OK] shell_ready` or `DETERMINISTIC SHELL MARKER DETECTED`.
3. **Artifact Retention:** Boot diagnostic JSON artifacts must be stored in `build/boot/` for CI upload.
4. **Timeout Enforcement:** Boot duration exceeding 5.0 seconds in emulator environment fails CI.
