# SigmaOS Package Management (sigpkg)

## Overview
SigmaOS implements a declarative, reproducible package management ecosystem (`sigpkg`) inspired by NixOS and Guix. Every package installation is deterministic and transaction-safe, using a Content-Addressed Store. System upgrades utilize Fedora Silverblue-style immutable image staging, enabling instant rollback to previous system states on boot failure.

## Declarative Package Architecture
```
 [declarative.sigma configuration]
                 │
                 ▼
     [Generate Dependency Graph]
                 │
                 ▼
 [Fetch Content-Addressed Packages] ──► [Verify GPG Repository Signature]
                 │
                 ▼
     [Link files into /run/current-system/sw/bin]
```

## Configuration Specification
System profiles are declared globally in `/etc/sigpkg/system.sigma`:
```toml
[profile]
name = "sovereign-desktop"
immutable_root = true

[packages]
include = [
    "core-utils",
    "zenith-desktop",
    "sigma-ai-runtime",
    "scilab"
]

[repository]
url = "https://pkg.sigmaos.org/stable"
gpg_key = "/etc/keys/sigma-repo-pub.gpg"
```

## Technical Implementation
The package builder verifies cryptographic hashes of all files in the package manifest before staging them.

```rust
// userland/sigpkg/sigpkg_core.rs
pub fn verify_package_integrity(package_path: &Path, expected_hash: &str) -> bool {
    let mut file = File::open(package_path).unwrap();
    let mut hasher = Sha256::new();
    io::copy(&mut file, &mut hasher).unwrap();
    let result = hasher.finalize();
    format!("{:x}", result) == expected_hash
}
```

## Roadmap & Milestones
- **Phase 1 (Months 0-3)**: Declarative profile parser and local Content-Addressed symlinking engine.
- **Phase 2 (Months 3-6)**: Immutable root file system mounting and A/B boot partition switching.
- **Phase 3 (Months 6-9)**: Signed repository manifest validation using GPG signatures.
- **Phase 4 (Months 9-12)**: Automated rollbacks triggered by daemon health-check failures on startup.
