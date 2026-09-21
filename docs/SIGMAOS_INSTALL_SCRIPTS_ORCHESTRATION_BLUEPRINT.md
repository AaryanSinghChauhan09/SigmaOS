# 🛠️📦 SIGMAOS OMARCHY INSTALLATION ORCHESTRATION & SETUP LEAF BLUEPRINT
## Comprehensive Architecture, Target-Side Command Orchestration, and 4-Phase Execution Roadmap for ISO Finalization, Idempotent Hardware Setup, and Per-User Config Resync for https://github.com/AaryanSinghChauhan09/SigmaOS

---

## EXECUTIVE SUMMARY & CONCEPTUAL VISION

Operating system installation requires a clear division of responsibility between ISO image boot media and target-side system configuration. Inspired by **Omarchy Linux** (the opinionated Arch Linux + Hyprland distribution designed by DHH & 37signals), **SigmaOS** implements a target-side **Installation Orchestration & Reusable Setup Leaf Subsystem** (`src/installer/omarchy_installer.rs`).

The live ISO owns installation orchestration (disk partitioning, filesystem creation, base package bootstrapping). The repository ships target-side setup commands (`bin/omarchy-apply-system`, `bin/omarchy-apply-hardware`, `bin/omarchy-finalize-user`, `bin/omarchy-reinstall-configs`) and modular, sourced leaf setup scripts under `install/`.

---

## PART 1: OMARCHY INSTALLATION COMMANDS & LEAF DIRECTIVES SPECIFICATION

### 1. Target-Side Command Suite (`bin/`)
- **`bin/omarchy-apply-system`**: Runs root-owned system setup during ISO finalization. Orchestrates system-wide configuration, package initialization, and calls `omarchy-apply-hardware`.
- **`bin/omarchy-apply-hardware`**: Runs idempotent, hardware-specific setup (GPU drivers, Wi-Fi firmware, audio codecs, touchpad gestures). Called by `omarchy-apply-system`. Supplies matching kernel headers for DKMS driver compilation before invoking hardware installers.
- **`bin/omarchy-finalize-user`**: Runs per-user runtime finalization (creating skill symlinks, setting `xdg-user-dirs`, applying MIME defaults, and executing `install/user/all.sh`). Seeded by `/etc/skel` from `omarchy-settings`.
- **`bin/omarchy-reinstall-configs`**: Explicit, destructive resync restoring factory default user configurations into an existing user's `$HOME`.

### 2. Sourced Setup Leaves (`install/`) Directives
- **No Shebangs & `run_logged`**: Leaf scripts under `install/` are sourced via `run_logged $OMARCHY_INSTALL/path/to/script.sh` and intentionally omit `#!/bin/sh` or `#!/bin/bash` shebangs.
- **No `exit`**: Sourced setup scripts avoid calling `exit` to prevent premature termination of parent installer loops, unless intentionally aborting setup on fatal errors.
- **Dynamic Variable Resolution**: Always use `$OMARCHY_INSTALL` (e.g. `/usr/share/omarchy/install`) and `$OMARCHY_PATH` (e.g. `/usr/share/omarchy`) instead of hardcoded paths.
- **Root Hardware Scoping (`install/hardware/`)**: Keep root-scoped hardware setup under `install/hardware/` and orchestrate execution through `install/hardware/all.sh`.
- **Per-User Scoping (`install/user/`)**: Keep every per-user setup leaf under `install/user/` (including `install/user/hardware/` and `install/user/first-run/`) so user-scoped actions are explicitly isolated.
- **DKMS Kernel Header Supply**: Base installation supplies matching kernel headers before hardware setup starts. DKMS drivers assume headers exist.
- **Package & Command Checks**: Prefer helper commands (`omarchy-pkg-check`, `omarchy-cmd-check`) for dependency verification. Direct `command -v`, `pacman`, or `pacman-key` are allowed only in low-level package-helper contexts.

---

## PART 2: CORE ARCHITECTURAL PILLARS FOR SIGMAOS

```
                 +-------------------------------------------------+
                 |  SIGMAOS TARGET-SIDE INSTALLATION ORCHESTRATOR |
                 +-------------------------------------------------+
                                          |
      +-------------------+---------------+---------------+-------------------+
      |                   |               |               |                   |
      v                   v               v               v                   v
⚙️ APPLY SYSTEM        🔌 APPLY HARDWARE   👤 FINALIZE USER    🔄 REINSTALL CONFIGS📜 SOURCED LEAVES
  bin/omarchy-apply-   bin/omarchy-apply-  bin/omarchy-        bin/omarchy-        install/
  system               hardware            finalize-user       reinstall-configs   • Sourced via run_logged
  • Root-Owned Setup   • Idempotent Setup  • Skill Symlinks    • Destructive Reset • No Shebangs & No exit
  • ISO Finalization   • DKMS Headers First• xdg-user-dirs     • Seeded from       • $OMARCHY_INSTALL
  • Calls Hardware     • Calls hardware/   • MIME Defaults     • /etc/skel         • $OMARCHY_PATH
```

---

## PART 3: 4-PHASE DEVELOPMENT ROADMAP

### PHASE 1: System-Side Setup Command Orchestrator
- Implement `OmarchyInstallerEngine` in `src/installer/omarchy_installer.rs` for executing `omarchy-apply-system` and `omarchy-apply-hardware`.
- Ensure hardware setup enforces DKMS kernel header supply prior to driver module compilation.

### PHASE 2: Per-User Finalization & Config Resync Engine
- Implement `omarchy-finalize-user` creating `xdg-user-dirs`, skill symlinks, and default MIME associations.
- Provide `omarchy-reinstall-configs` for factory `/etc/skel` resetting of user `$HOME` configuration files.

### PHASE 3: Idempotent Hardware Setup & DKMS Header Supply
- Implement `install/hardware/all.sh` orchestrator with idempotency checks (skipping already installed GPU drivers / firmware).
- Supply matching `linux-headers` package dependencies before running DKMS build tasks.

### PHASE 4: Sourced Leaf Script Runner & Variable Resolution
- Implement `run_logged` script executor enforcing `$OMARCHY_INSTALL` and `$OMARCHY_PATH` variable expansion.
- Lint leaf scripts ensuring zero shebangs (`#!/bin/sh`) and zero raw `exit` calls in sourced sub-scripts.

---

## PART 4: VERIFICATION BENCHMARK & TEST CRITERIA

1. **Target Command Execution Unit Tests**: Verify that `omarchy-apply-system` correctly triggers `omarchy-apply-hardware` and logs execution steps.
2. **Environment Variable Resolution Unit Tests**: Confirm `$OMARCHY_INSTALL` and `$OMARCHY_PATH` expand properly without falling back to hardcoded `/usr/share/omarchy` strings.
3. **DKMS Kernel Header Pre-Check Unit Tests**: Validate that hardware setup validates kernel header presence before executing DKMS driver installations.
4. **Destructive Config Resync Unit Tests**: Ensure `omarchy-reinstall-configs` backs up existing `$HOME` configs before overwriting from `/etc/skel`.

---
*End of SigmaOS Omarchy Installation Orchestration Blueprint Specification.*
