# SigmaOS Installer & Recovery Runbook

This document defines the operational procedures for installer VM rehearsals, dry-run validations, failure injection scenarios, and recovery path execution for SigmaOS deployments.

## Core Recovery Objectives
1. **Zero Data Loss on Failure**: Disk partition mutations must be non-destructive until user confirmation gate is explicitly granted.
2. **Deterministic Rollback**: Interrupted installation or update transactions must restore the system to the prior known-good state.
3. **Emergency Diagnostic Shell**: Failed installations or corrupt boot sectors must fail closed into a minimal rescue environment.

---

## Installer Rehearsal Workflows

### 1. Dry-Run Validation Mode
The installer dry-run mode validates target disk geometry, package manifest dependencies, and partition table specifications without writing to disk.

```bash
# Execute standalone dry-run integration test
cargo test --test dry_run
```

Verification assertions:
- No partition table write calls issued to `BlockDeviceManager`.
- Disk space availability check succeeds.
- Installation payload manifest checksums pass.

### 2. Failure Injection Testing
Failure injection simulates sudden power loss, disk write errors, or corrupt payload downloads at critical installation stages.

```bash
# Execute failure injection test suite
cargo test --test failure_injection
```

Injected scenarios:
- **Pre-partitioning failure**: Verifies disk partition table remains untouched.
- **Mid-extraction failure**: Verifies temp staging directory cleanup and rollback.
- **Bootloader install failure**: Verifies legacy boot sector preservation and rescue prompt creation.

---

## VM Rehearsal Procedure

1. **Build Installer ISO Image**:
   ```bash
   ./scripts/release/prepare_release.sh --tier test_distribution
   ```
2. **Launch QEMU Target VM**:
   ```bash
   qemu-system-x86_64 -m 2048 -drive file=target_disk.img,format=raw -cdrom build/sigmaos_installer.iso -boot d
   ```
3. **Execute Interactive Installation**:
   - Select target disk `/dev/sda`.
   - Confirm layout (`ext4` root `/`, `swap` 2GB, `boot` 512MB).
   - Verify completion marker `SIGMAOS_INSTALL_SUCCESS`.
4. **Simulate Crash & Rescue Boot**:
   - Interrupt payload extraction midway.
   - Reboot VM and verify entry into rescue console.
