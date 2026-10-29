# SigmaOS Installer Safety & System Recovery Runbook

## Overview

SigmaOS prioritizes installer safety and system recoverability. Destructive operations (partitioning, formatting, package extraction, bootloader installation, and system updates) must provide dry-run execution modes, explicit confirmation prompts, structured logging, and verified rollback/recovery guidance.

---

## Installer Dry-Run Rehearsal

### Purpose
Validates the installer workflow, target disk detection, partitioning scheme, and package manifest calculations without mutating storage devices or writing to disk.

### Rehearsal Steps
1. Execute the installer dry-run test suite:
   ```bash
   cargo test --test dry_run
   ```
2. Verify output log markers:
   - Target disk inspection log recorded
   - Partition table scheme validated (GPT / MBR)
   - Zero write operations performed on physical or virtual block devices
   - Expected transaction dry-run summary generated

---

## Failure Injection & Interrupted Update Rehearsal

### Purpose
Ensures that if an installation or system update is interrupted (power failure, process panic, corrupted payload, or write error), the system state remains recoverable without data corruption or unbootable state.

### Rehearsal Steps
1. Execute failure injection harness:
   ```bash
   cargo test --test failure_injection
   ```
2. Test scenarios:
   - **Pre-commit interrupt:** Interruption during package download / payload staging. Expect clean purge of temporary staging directories.
   - **Mid-write interrupt:** Interruption during package extraction to A/B staging root. Expect system to retain active boot partition unchanged and fail safe.
   - **Post-write bootloader switch interrupt:** Interruption during EFI boot variable update. Expect fallback to secondary valid boot target.

---

## Step-by-Step Recovery Procedures

### Scenario 1: Interrupted System Update Recovery
1. Boot into the SigmaOS Diagnostic / Recovery Prompt from the QEMU / GRUB boot menu.
2. Select **System Recovery & Rollback**.
3. Verify current active partition slot and staging slot:
   ```bash
   sigma-update-control --status
   ```
4. Perform atomic rollback to the previous known-good partition slot:
   ```bash
   sigma-update-control --rollback
   ```
5. Reboot the system and confirm shell marker verification.

### Scenario 2: Corrupted Boot Loader Configuration
1. Boot system using the official SigmaOS Live Rescue ISO image.
2. Mount the EFI System Partition (ESP) in diagnostic mode:
   ```bash
   mount /dev/sda1 /mnt/boot
   ```
3. Restore canonical backup boot config:
   ```bash
   cp /mnt/boot/EFI/SigmaOS/boot.cfg.bak /mnt/boot/EFI/SigmaOS/boot.cfg
   ```
4. Run recovery verification diagnostic:
   ```bash
   sigma-diag --verify-boot
   ```

---

## Verification & Monthly Rehearsal Logging

Every monthly rehearsal must record logs and results into `reports/installer_rehearsal_<YYYY_MM>.log` with evidence for:
- Dry-run mode pass status
- Failure injection clean recovery status
- A/B partition atomic update/rollback verification
