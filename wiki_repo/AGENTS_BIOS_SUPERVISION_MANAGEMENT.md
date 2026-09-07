# AGENTS_BIOS_SUPERVISION_MANAGEMENT.md — AI Agent BIOS Supervision Guidelines for SigmaOS

Welcome, AI Agent! This document defines the standards, architectural models, hardware watchdog protocols, and verification routines for managing, developing, and extending **BIOS Supervision, POST Hardware Monitoring, Firmware Attestation, and Boot Guard Systems** in **SigmaOS**.

---

## 1. SigmaOS BIOS Supervision System Architecture Overview

The BIOS Supervision System in SigmaOS continuously supervises early boot execution, hardware POST (Power-On Self-Test) sequences, ACPI/SMBIOS firmware sensors, and secure boot key chains to guarantee system integrity prior to full kernel execution.

### Core BIOS Supervision Modules (`src/boot/`)
* **POST Execution & Hardware Probe Supervisor (`src/boot/post.rs`, `src/boot/firmware_bridge.rs`)**:
  - Power-On Self-Test (POST) code step supervision (CPU register check, RAM memory pattern test, PCI bus enumeration).
  - Diagnostic POST beep code & LED error code dispatching.
* **Firmware Bridge & Hardware Monitor Supervisor (`src/boot/firmware_bridge.rs`)**:
  - Real-time supervision of SMBIOS/ACPI thermal zone sensors (`hw.acpi.thermal.tz0`), CPU fan RPMs, and power rail voltage levels.
  - Hardware watchdog timer initialization and heartbeat keep-alive polling.
* **Boot Guard & Secure Boot Supervisor (`src/boot/secure_boot.rs`, `src/boot/verified.rs`)**:
  - Continuous signature verification of boot components against the UEFI database (`PK`, `KEK`, `db`).
  - Fallback kernel image recovery triggering when POST or boot signature verification fails (`sigma_boot.rs`).
* **Early Memory & Descriptor Table Supervisor (`src/boot/bootloader.rs`)**:
  - Early GDT (Global Descriptor Table), IDT (Interrupt Descriptor Table), and Page Table PML4 self-referential mapping supervision prior to long-mode transition.

---

## 2. BIOS Supervision Guidelines for AI Agents

When modifying or extending BIOS supervision, POST, or firmware bridge logic:

### 1. Fail-Safe POST Error Recovery
* **Non-Blocking POST Reporting**: POST hardware test routines must set explicit status codes rather than looping infinitely on hardware warnings.
* **Automatic Fallback Triggering**: If a critical POST failure occurs (e.g. corrupted primary kernel image or invalid GDT), the supervisor must trigger automated fallback bootloader recovery (`sigma_boot.rs` A/B slot rollback).

### 2. Hardware Watchdog Heartbeat Protocol
* **Watchdog Service Intervals**: Ensure firmware hardware watchdogs are kicked at regular intervals during early boot initialization to prevent unexpected hardware resets before the kernel timer driver (`src/timer/`) takes ownership.

### 3. Non-Volatile Firmware State Supervision
* **Tamper Detection**: Monitor UEFI NVRAM variables for unauthorized modifications to boot orders or Secure Boot database entries (`db` / `dbx`).

---

## 3. Verification & Testing Protocols

1. **Standalone Firmware Test Execution**:
   ```bash
   rustc --test --edition=2021 --cfg 'feature="standalone_test"' src/boot/sigma_boot.rs -o /tmp/test_boot
   /tmp/test_boot
   ```
2. **QEMU POST & BIOS Supervision Simulation**:
   - Test legacy POST and UEFI boot supervision in QEMU:
     ```bash
     qemu-system-x86_64 -bios /usr/share/ovmf/OVMF.fd -drive file=build/sigmaos.iso,format=raw -m 2048 -d guest_errors,unimp
     ```
3. **Core Test Runner Execution**:
   ```bash
   ./run_sigma_tests.sh
   ```

---

## 4. Pre-Commit Checklist for BIOS Supervision Changes

Before submitting BIOS supervision or POST changes:
- [ ] Confirmed POST failure paths trigger automated fallback recovery.
- [ ] Verified hardware watchdog heartbeat reset intervals in early boot routines.
- [ ] Verified non-volatile NVRAM tamper detection assertions.
- [ ] Executed standalone boot tests (`src/boot/sigma_boot.rs`).
- [ ] Executed `./run_sigma_tests.sh` with 100% test pass rate.
- [ ] Requested automated code review using `request_code_review`.
- [ ] Recorded BIOS supervision learnings using `initiate_memory_recording`.
