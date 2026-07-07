# SigmaOS Driver Auto-Updater

## Overview
SigmaOS implements a signed driver package auto-updater framework (`sigdriver`) to maintain hardware compatibility matrices without manual compilation or untrusted binaries. The system scans the PCI, USB, and ACPI buses, matches hardware IDs against a cryptographically signed database, and stages updates securely. This framework absorbs driver testing, staging, and automated delivery infrastructure models from `https://github.com/fedora-infra` (such as the Bodhi updates feedback cycle and Koji packaging hooks) to ensure stable driver release gating.

## Driver Lifecycle and Signature Verification
Drivers are packaged as signed kernel modules (`.sko`). Each driver undergoes hardware compatibility checks before load.

```
 [Hardware Probe] ──► [Vendor/Device IDs] ──► [Driver Registry]
                                                     │
                                                     ▼
 [Signature Verified?] ◄── [Verify Keys] ◄── [Download .sko]
         │
         ├──► Yes ──► Load Module into Kernel
         └──► No  ──► Quarantine and Log Alert
```

## Database and Policy Schema
Policies are defined in `/etc/sigdriver/policy.sigma`:
```toml
[updater]
auto_check = true
allow_experimental_drivers = false
require_cryptographic_signatures = true

[trusted_certificates]
kernel_signing_key = "/etc/keys/sigma-driver-ca.pem"
```

## Technical Implementation
The kernel module loader verifies signatures using cryptographic primitives implemented in Rust/Nim.

```rust
// boot/sigma_secureboot.rs
pub fn verify_module_signature(module_data: &[u8], signature: &[u8]) -> bool {
    // Cryptographic signature checking logic
    let public_key = load_system_signing_key();
    public_key.verify(module_data, signature).is_ok()
}
```

## Roadmap & Milestones
- **Phase 1 (Months 0-3)**: Automatic PCI/USB hardware discovery scan.
- **Phase 2 (Months 3-6)**: Cryptographic signature validation for `.sko` modules.
- **Phase 3 (Months 6-9)**: Rolling/fallback system that loads safe VGA/Ethernet drivers if updates fail.
- **Phase 4 (Months 9-12)**: Upstream vendor hardware certification portal and driver matrix database.
