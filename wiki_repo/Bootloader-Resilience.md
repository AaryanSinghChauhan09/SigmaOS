# Bootloader & Resilience

Immutable-style recovery inspired by Fedora CoreOS / Flatcar, with a sovereign twist.

## Boot flow

1. `kernel/core/boot/sigma_boot.c` — `boot_sequence()`
2. Rollback gate — `sigma_rollback_check_fallback()`
3. Safe Mode — `load_safe_mode()` + **Fix it** menu

## Fix it menu

`kernel/core/boot/sigma_boot_recovery_menu.c`

Options: minimal boot, rollback update, reload net driver, rescue shell, reboot.

## Resilience shards

- `kernel/resilience/sigma_rollback.cpp`
- `kernel/resilience/sigma_micro_fallback.cpp`

## Automation check

```bash
./scripts/sigma_automation.sh recovery-check
```

## QEMU test

```bash
qemu-system-x86_64 -cdrom build/sigmaos.iso -serial stdio
```
