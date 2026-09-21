# 🌙💤 SIGMAOS OMARCHY SYSTEM SLEEP, POWER PROFILES & HIBERNATION BLUEPRINT
## Comprehensive Architecture, AC/Battery Profile Memory Matrix, and 4-Phase Execution Roadmap for Suspend, Hibernation & Power Management for https://github.com/AaryanSinghChauhan09/SigmaOS

---

## EXECUTIVE SUMMARY & CONCEPTUAL VISION

Operating system power management balances performance and energy conservation on laptops, desktops, and edge workstations. Inspired by **Omarchy Linux** (the Arch Linux + Hyprland + Quickshell distribution designed by DHH & 37signals), **SigmaOS** implements a dedicated **System Sleep, Power Profiles & Hibernation Subsystem** (`src/power/omarchy_power.rs`).

This subsystem manages separate power profiles for AC power and battery states, CLI commands (`omarchy powerprofiles`), UI suspend toggles (`omarchy toggle suspend`), Btrfs `/swap` subvolume allocations matching physical RAM sizes (e.g. 32GB swap for 32GB RAM), and Limine bootloader `resume` kernel parameters.

---

## PART 1: OMARCHY POWER PROFILES, SUSPEND & HIBERNATION MATRIX

### 1. Dual Power Profile Memory (AC vs. Battery)
- **AC Plugged In**: Default `Performance` power profile.
- **Battery Unplugged**: Default `Balanced` power profile (or user-selected `PowerSaver`).
- **State Memory**: Automatically remembers and restores user profile choices separately for AC vs. Battery when plugging or unplugging power chargers.
- **CLI Commands**:
  - `omarchy powerprofiles list`: Displays available profiles (`performance`, `balanced`, `power-saver`) and current AC/battery bindings.
  - `omarchy powerprofiles set autodetect power-saver`: Sets the profile for the current power state.
  - `omarchy powerprofiles set battery power-saver`: Explicitly sets the profile for the battery state regardless of current charger status.

### 2. Suspend Toggle (`omarchy toggle suspend`)
- Toggles visibility of the Suspend option in the System menu (`Super + Esc`).
- Enables users on hardware with buggy S3 sleep implementations to easily hide/disable or reveal/test suspend functionality without system instability.

### 3. Hibernation Setup & Subvolume Removal (`omarchy hibernation setup` / `remove`)
- **Setup (`omarchy hibernation setup`)**:
  - Verifies free disk space on the primary boot drive.
  - Creates a dedicated `/swap` Btrfs subvolume sized 1:1 with physical RAM allocation (e.g. 32GB swap file for 32GB physical RAM).
  - Configures Limine bootloader `resume=UUID=...` and `resume_offset=...` kernel command line parameters.
  - Reveals the Hibernate option under System menu (`Super + Esc`).
- **Remove (`omarchy hibernation remove`)**:
  - Unmounts and deletes the `/swap` subvolume, reclaiming disk space.
  - Strips `resume` parameters from Limine bootloader config.
  - Hides the Hibernate option from System menu (`Super + Esc`).

---

## PART 2: CORE ARCHITECTURAL PILLARS FOR SIGMAOS

```
                 +-------------------------------------------------+
                 |   SIGMAOS OMARCHY POWER & SLEEP ENGINE          |
                 +-------------------------------------------------+
                                          |
      +-------------------+---------------+---------------+-------------------+
      |                   |               |               |                   |
      v                   v               v               v                   v
⚡ DUAL POWER PROFILES   🌙 SUSPEND TOGGLE  💾 HIBERNATION SETUP  🚀 LIMINE BOOTLOADER 🔋 ACPI / S3 / S4
  AC vs. Battery Memory   omarchy toggle    Btrfs /swap Subvol    Kernel Cmdline      POWER STATE
  • AC: Performance       suspend           • 1:1 RAM Allocation  • resume=UUID=...   MACHINE
  • Battery: Balanced     • Super+Esc Menu  • omarchy hibernation • resume_offset=... • S0 Idle / S3 Suspend
  • Auto-Detect Switch    Visibility        setup / remove        • Config Generator  • S4 Hibernate
```

---

## PART 3: 4-PHASE DEVELOPMENT ROADMAP

### PHASE 1: AC/Battery Dual Power Profiles & State Memory Engine
- Implement `OmarchyPowerEngine` in `src/power/omarchy_power.rs` tracking separate `ac_profile` and `battery_profile` settings.
- Support `omarchy powerprofiles list` and `set` CLI command dispatching.

### PHASE 2: System Suspend Toggle & UI Menu Visibility
- Implement `toggle_suspend_ui_visibility()` controlling Suspend menu visibility under System launcher (`Super + Esc`).
- Provide persistable configuration flags saved in `/etc/sigmaos/power.toml`.

### PHASE 3: Btrfs `/swap` Subvolume Allocation & Limine Resume Parameters
- Implement `setup_hibernation(ram_gb)` creating Btrfs `/swap` subvolume equal to physical RAM size.
- Automatically append `resume` and `resume_offset` kernel parameters to Limine bootloader configuration (`limine.conf`).

### PHASE 4: ACPI / S3 Suspend / S4 Hibernation Power State Machine
- Integrate ACPI power state transitions (S0 Working, S3 Suspend-to-RAM, S4 Hibernate-to-Disk, S5 Soft Off).
- Ensure memory scrubbing and hardware register state saving across suspend/resume cycles.

---

## PART 4: VERIFICATION BENCHMARK & TEST CRITERIA

1. **Power Profile Memory Unit Tests**: Verify that changing battery profile to `power-saver` retains `performance` for AC profile and restores correctly on power state toggles.
2. **Suspend Toggle Unit Tests**: Confirm `omarchy toggle suspend` flips the System menu suspend visibility boolean cleanly.
3. **Hibernation Setup Unit Tests**: Validate `/swap` subvolume size calculations (32GB RAM -> 32GB swap) and Limine resume string formatting.
4. **CLI Command Parsing Unit Tests**: Test `omarchy powerprofiles set battery power-saver` string dispatching.

---
*End of SigmaOS Omarchy System Sleep, Power Profiles & Hibernation Blueprint Specification.*
