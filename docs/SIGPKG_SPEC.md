# sigpkg — SigmaOS Package Specification

**Version:** 1.0  
**Format:** `sigma.toml` manifest + `.sigpkg` archive

---

## Package Format

A `.sigpkg` file is a zstd-compressed tar archive:

```
my-app-1.0.0.sigpkg
├── sigma.toml          # Manifest
├── files/              # Package files (installed to /)
│   ├── usr/bin/my-app
│   └── usr/share/my-app/
├── scripts/
│   ├── pre-install.sh
│   ├── post-install.sh
│   └── pre-remove.sh
└── sigma.sig           # Dilithium-5 signature over sha256(archive minus .sig)
```

---

## sigma.toml Manifest

```toml
[package]
name        = "my-app"
version     = "1.0.0"
description = "My application for SigmaOS"
authors     = ["Developer <dev@example.com>"]
license     = "MIT"
homepage    = "https://example.com/my-app"
arch        = "x86_64"           # or arm64, riscv64, any
profile     = ["standalone", "cloud", "desktop"]  # compatible profiles

[build]
reproducible = true              # deterministic build required for Certified
source_url   = "https://github.com/example/my-app"
commit       = "abc123def456"    # exact commit for reproducibility

[dependencies]
sigma-libc   = ">=1.0.0"
sigma-ssl    = ">=3.0.0"

[optional-dependencies]
sigma-gui    = ">=2.0.0"         # only for desktop profile

[security]
pledge       = ["stdio", "rpath", "inet"]  # process capabilities at runtime
unveil       = ["/usr/share/my-app:r", "/tmp:rw"]
sandbox      = "ring3"           # ring3 = isolated process

[signing]
algorithm    = "dilithium5"
fingerprint  = "sha256:abc123..."   # public key fingerprint
certified    = true              # SigmaOS Certified badge

[install]
prefix       = "/usr"
bin          = ["bin/my-app"]
data         = ["share/my-app/"]
config       = ["etc/my-app.conf"]
```

---

## Repository Layout

```
sigma-repo/
├── index.json          # Signed package index
├── pool/
│   ├── my-app-1.0.0-x86_64.sigpkg
│   ├── my-app-1.0.0-x86_64.sigpkg.sha256
│   └── my-app-1.0.0-x86_64.sigpkg.sig
└── keys/
    └── sigma-repo.pub  # Repository signing public key
```

---

## Client Commands

```bash
# Install
sigma-pkg install my-app
sigma-pkg install my-app==1.0.0

# Remove
sigma-pkg remove my-app

# Update
sigma-pkg update              # update all
sigma-pkg update my-app       # update specific package

# Search
sigma-pkg search nginx
sigma-pkg info my-app

# List installed
sigma-pkg list

# Verify
sigma-pkg verify my-app       # check signatures + checksums

# Build
sigma-pkg build sigma.toml    # build from manifest
sigma-pkg publish             # publish to registry
```

---

## Transactional Updates (A/B)

sigpkg uses atomic A/B partitions for safe updates:

```
Slot A (active):  /sigma/system-a/  ← running
Slot B (inactive): /sigma/system-b/  ← update target

1. sigma-pkg downloads + verifies new packages to slot B
2. Signature + dm-verity hash computed for slot B
3. Boot pointer updated to slot B
4. Reboot
5. If boot fails → automatic rollback to slot A
```

---

## Security

Every package must:
1. Be signed with the author's Dilithium-5 key
2. Have a `sha256` checksum that matches the downloaded archive
3. Declare `pledge` + `unveil` permissions in `sigma.toml`
4. For "Certified" status: pass SigmaOS team security review

Verification at install time:
```
sigma-pkg verify nginx-1.24.0-x86_64.sigpkg
  ✓ sha256 checksum
  ✓ Dilithium-5 signature (author key)
  ✓ Sigma Certified (team signature)
  ✓ pledge constraints declared
  ✓ reproducible build attestation
```

---

## Building a Package

```bash
# Initialize a new package
sigma-pkg new my-app

# Edit sigma.toml
# Build
sigma-pkg build

# Test in QEMU
sigma-pkg test --qemu

# Publish
sigma-pkg publish --registry https://pkg.sigmaos.io
```

---

*See also: [Package Manager Spec](../wiki_repo/Package-Manager-Spec.md) · [DOWNLOAD.md](../DOWNLOAD.md)*
