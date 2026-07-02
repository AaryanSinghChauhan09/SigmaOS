# OSTree-Inspired Atomic System Updates

## Overview

SigmaOS implements **atomic, rollback-safe system updates** inspired by [OSTree](https://github.com/ostreedev/ostree) (LGPL-2.0). The root filesystem is **immutable** during runtime. Updates are staged to an alternate partition (A/B model) and activated on reboot. Failed boots trigger automatic rollback.

---

## A/B Partition Model

```
Disk layout:
  /dev/sda1  EFI System Partition (ESP)
  /dev/sda2  sigma-rootfs-A  (current active)
  /dev/sda3  sigma-rootfs-B  (next update target)
  /dev/sda4  sigma-data      (persistent user data, mounted /data)

Boot flow:
  sigma-boot.efi reads /boot/sigma-deployment → activates A or B
```

---

## sigma-update: Update Lifecycle

```
sigma-update apply <update-url>
  1. fetch    — download update bundle (.sigupd)
  2. verify   — cosign signature + sha256 manifest
  3. stage    — write to inactive partition (B if A is active)
  4. mark     — write deployment descriptor to EFI var SigmaBootNext=B
  5. reboot   — reboot into B
  6. activate — on successful boot, mark B as permanent active
  7. rollback — if B fails 3 consecutive boots, revert to A
```

---

## File Layout

```
userland/update/
├── README.md
└── sigma_update.rs
```

---

## sigma_update.rs

```rust
//! sigma-update: atomic system update agent for SigmaOS.

use std::path::Path;

const INACTIVE_PART: &str = "/dev/sda3";
const BOOT_EFI_VAR: &str = "/sys/firmware/efi/efivars/SigmaBootNext";

pub struct UpdateAgent {
    update_url: String,
    bundle_path: String,
}

impl UpdateAgent {
    pub fn new(update_url: &str) -> Self {
        Self {
            update_url: update_url.to_string(),
            bundle_path: "/tmp/sigma-update.sigupd".to_string(),
        }
    }

    /// Step 1: Fetch update bundle.
    pub fn fetch(&self) -> Result<(), UpdateError> {
        println!("Fetching update from {}", self.update_url);
        sigma_curl::download(&self.update_url, &self.bundle_path)
            .map_err(|_| UpdateError::FetchFailed)?;
        Ok(())
    }

    /// Step 2: Verify bundle signature.
    pub fn verify(&self) -> Result<(), UpdateError> {
        let sig_path = format!("{}.sig", self.bundle_path);
        let status = std::process::Command::new("cosign")
            .args(["verify-blob",
                "--key", "/etc/sigma/update-verify.pem",
                "--bundle", &sig_path,
                &self.bundle_path])
            .status()
            .map_err(|_| UpdateError::VerifyFailed)?;
        if status.success() { Ok(()) } else { Err(UpdateError::InvalidSignature) }
    }

    /// Step 3: Stage to inactive partition.
    pub fn stage(&self) -> Result<(), UpdateError> {
        println!("Staging update to {}", INACTIVE_PART);
        // Decompress + write sigupd bundle to inactive partition
        let status = std::process::Command::new("sigma-update-writer")
            .args([&self.bundle_path, INACTIVE_PART])
            .status()
            .map_err(|_| UpdateError::StageFailed)?;
        if status.success() { Ok(()) } else { Err(UpdateError::StageFailed) }
    }

    /// Step 4: Set EFI boot variable to activate inactive partition.
    pub fn mark_next_boot(&self) -> Result<(), UpdateError> {
        std::fs::write(BOOT_EFI_VAR, b"B")
            .map_err(|_| UpdateError::EfiVarFailed)?;
        println!("Next boot will activate partition B");
        Ok(())
    }

    /// Run full update cycle (without reboot).
    pub fn apply(&self) -> Result<(), UpdateError> {
        self.fetch()?;
        self.verify()?;
        self.stage()?;
        self.mark_next_boot()?;
        println!("Update staged. Reboot to activate.");
        Ok(())
    }
}

/// Rollback: called by sigma-init if boot fail counter >= 3.
pub fn rollback() -> Result<(), UpdateError> {
    println!("Rolling back to partition A");
    std::fs::write(BOOT_EFI_VAR, b"A")
        .map_err(|_| UpdateError::EfiVarFailed)?;
    Ok(())
}

#[derive(Debug)]
pub enum UpdateError {
    FetchFailed,
    VerifyFailed,
    InvalidSignature,
    StageFailed,
    EfiVarFailed,
}
```

---

## Rollback Logic in sigma-init

```rust
// userland/sigma_init/src/boot_counter.rs

const MAX_BOOT_FAILURES: u32 = 3;

pub fn check_boot_health() {
    let count = read_efi_var("SigmaBootFailCount").unwrap_or(0);
    if count >= MAX_BOOT_FAILURES {
        eprintln!("sigma-init: {} consecutive boot failures, rolling back", count);
        sigma_update::rollback().expect("Rollback failed");
        std::process::Command::new("reboot").status().unwrap();
    }
    // Reset counter on successful boot
    write_efi_var("SigmaBootFailCount", 0);
}
```

---

## Exit Criteria

- `sigma-update apply https://updates.sigmaos.dev/latest` stages update and sets next boot target.
- System boots into new partition B after reboot.
- Corrupt update triggers rollback to A after 3 failed boots.
