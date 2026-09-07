# AI Agent System State Management & Update Governance in SigmaOS

## Overview

SigmaOS system state architecture (`src/update/distro_update_parity.rs`, `src/distro/clear_linux.rs`, `src/innovation.rs`, `src/boot/sigma_boot.rs`) provides atomic A/B system updates, Intel Clear Linux inspired stateless `/etc` & `/usr` separation, post-quantum signed update verification, and point-in-time state rollbacks.

AI agents (such as Jules, Herdr agentic tasks, and automated system update daemons) must follow these system state management rules to guarantee system bootability and zero downtime.

---

## System State Architecture & Boot Stages

```
UEFI / UKI Boot Loader (`sigma_boot`)
                │
                ▼
   Initramfs & Kernel Handoff (`BootStageKind`)
                │
                ▼
   Stateless Config Overlay (`IntelClearLinuxStatelessEngine`)
    - Factory Defaults: `/usr/share/defaults`
    - User Overrides:   `/etc`
                │
                ▼
   Active A/B Partition Switch (`SovereignSystemUpdateAndTestingEngine`)
    - Primary Slot A / Fallback Slot B
                │
                ▼
   Post-Quantum Dilithium Signature Verification
```

---

## 1. Stateless Configuration Management (`IntelClearLinuxStatelessEngine`)

AI agents modifying system configurations must distinguish between immutable factory defaults and local user overrides:

```rust
use sigmaos::kernel::IntelClearLinuxStatelessEngine;

let mut stateless = IntelClearLinuxStatelessEngine::new();

// Register default system configuration
stateless.register_default_config("/etc/hostname", "sigma-default");

// Set local user override
stateless.set_user_override("/etc/hostname", "sigma-custom-agent-node");

// Query resolved config (returns user override if present)
let config = stateless.resolve_config("/etc/hostname").unwrap();

// Reset system state to factory defaults (removes /etc overrides)
stateless.reset_etc_to_stateless();
```

---

## 2. Atomic A/B System Updates & Self-Diagnostics

When applying OS updates, AI agents invoke `SovereignSystemUpdateAndTestingEngine`:

```rust
use sigmaos::update::distro_update_parity::SovereignSystemUpdateAndTestingEngine;

let mut update_engine = SovereignSystemUpdateAndTestingEngine::new();

// 1. Stage update payload to secondary slot
update_engine.stage_update_payload(update_bytes, pqc_signature)?;

// 2. Run post-quantum Dilithium5 signature check & self-diagnostics
let report = update_engine.run_system_self_diagnostics()?;
if report.is_healthy {
    // 3. Switch active boot slot atomically
    update_engine.commit_active_boot_slot()?;
} else {
    // 4. Auto-rollback to fallback slot
    update_engine.rollback_boot_slot()?;
}
```

---

## 3. ZFS & Btrfs Boot Environment Activation

AI agents managing boot environments use `BsdBootEnvironmentManager`:

```rust
use sigmaos::boot::BsdBootEnvironmentManager;

let mut be_mgr = BsdBootEnvironmentManager::new();

// Create new boot environment snapshot
be_mgr.create_boot_environment("be-sigmaos-0.2.0")?;

// Activate target boot environment for next reboot
be_mgr.activate_boot_environment("be-sigmaos-0.2.0")?;
```

---

## Directives for AI Agents Managing System State

1. **Never Modify `/usr/share/defaults` Directly**: System defaults are read-only; place overrides in `/etc`.
2. **Always Run Self-Diagnostics Before Committing**: Run `run_system_self_diagnostics()` prior to completing boot slot switches.
3. **Log State Transitions**: Log all state changes in the immutable audit trail for cryptographic chain-of-custody.
