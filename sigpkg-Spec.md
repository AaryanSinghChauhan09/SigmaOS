# sigpkg — Sovereign Package Manager Specification

> The native SigmaOS package format and manager. PQC-signed, reproducible, atomic.

---

## Overview

`sigma-pkg` is the SigmaOS package manager. It handles `.sigpkg` archives — a sovereign
format that combines a binary payload, metadata, dependency graph, and Dilithium-5 signature.

```
sigma-pkg install sigma-edit        # install
sigma-pkg remove  sigma-edit        # remove
sigma-pkg update                    # update all
sigma-pkg search  editor            # search registry
sigma-pkg info    sigma-edit        # show metadata
sigma-pkg verify  sigma-edit.sigpkg # verify signature
sigma-pkg build   PKGBUILD          # build from source
```

---

## .sigpkg Format

A `.sigpkg` file is a gzip-compressed tar archive with the following layout:

```
sigma-edit-1.2.0-x86_64.sigpkg
├── META
│   ├── MANIFEST.toml      ← package metadata
│   ├── DEPS.toml          ← dependency declarations
│   ├── FILES.sha256       ← SHA-256 checksums for all payload files
│   └── SIGNATURE.d5sig    ← Dilithium-5 signature over FILES.sha256
└── PAYLOAD/
    ├── usr/bin/sigma-edit
    ├── usr/share/sigma-edit/
    └── usr/share/man/man1/sigma-edit.1.gz
```

### MANIFEST.toml Schema

```toml
[package]
name        = "sigma-edit"
version     = "1.2.0"
description = "Sovereign text and code editor"
author      = "SigmaOS Project"
license     = "GPL-2.0-or-later"
arch        = "x86_64"           # x86_64 | arm64 | riscv64 | any | wasm
profile     = ["standalone", "minimal", "cloud"]
size_bytes  = 892340
installed_size_bytes = 2100000

[build]
reproducible = true
source_url   = "https://github.com/AaryanSinghChauhan09/SigmaOS"
build_date   = "2026-07-01T00:00:00Z"
build_hash   = "sha256:abc123..."

[signing]
algorithm   = "Dilithium-5"
public_key  = "sigmaos-official-2026.d5pub"
signature   = "SIGNATURE.d5sig"
```

### DEPS.toml Schema

```toml
[dependencies]
sigma-coreutils = ">=1.0.0"
sigma-libc      = ">=0.5.0"

[optional]
sigma-spellcheck = ">=1.0.0"    # pulled in if available

[conflicts]
nano = "*"                       # conflicts with plain nano

[provides]
editor = "1.2.0"                 # virtual package
```

---

## Registry Protocol

`sigma-pkg` fetches packages from a registry server over HTTPS + PQC TLS.

### Local Mode (v0.1 — available now)
```
/var/sigma/pkg/
├── repo/
│   ├── INDEX.toml          ← package index
│   └── *.sigpkg            ← local packages
├── installed/
│   └── sigma-edit/MANIFEST.toml
└── cache/
    └── *.sigpkg
```

### Online Mode (v1.0 target)
```
Registry URL: https://pkg.sigmaos.app/v1/
GET /v1/index                        → package index (signed)
GET /v1/pkg/{name}/{version}/{arch}  → download .sigpkg
GET /v1/search?q={query}             → search results
POST /v1/publish                     → upload package (auth required)
```

Registry responses are signed with the SigmaOS root Dilithium-5 key.
`sigma-pkg` verifies the registry signature before trusting any index data.

---

## PKGBUILD — Build Recipe Format

```toml
# PKGBUILD for sigma-edit
[package]
name    = "sigma-edit"
version = "1.2.0"
source  = "https://github.com/AaryanSinghChauhan09/SigmaOS/archive/v1.2.0.tar.gz"
sha256  = "abc123..."

[build]
steps = [
  "make PROFILE=standalone sigma-edit -j$(nproc)",
  "make DESTDIR=$PKG install-sigma-edit",
]

[check]
steps = [
  "make test-sigma-edit",
]
```

Build a package:
```bash
sigma-pkg build PKGBUILD
# Output: sigma-edit-1.2.0-x86_64.sigpkg
```

---

## Reproducible Builds

Every official SigmaOS package must be reproducible:

1. Fixed build timestamp (from `SOURCE_DATE_EPOCH`).
2. Deterministic linker flags (`-Wl,--build-id=none`).
3. Sorted file lists in archives.
4. Documented build environment (compiler version, flags).

Verify reproducibility:
```bash
sigma-pkg rebuild sigma-edit-1.2.0-x86_64.sigpkg
sigma-pkg diff   sigma-edit-1.2.0-x86_64.sigpkg sigma-edit-1.2.0-x86_64.rebuild.sigpkg
# Output: "Identical" or diff of differing bytes
```

---

## Multi-Format Output

`sigma-pkg build` can emit multiple formats from one PKGBUILD:

```bash
sigma-pkg build --format sigpkg   PKGBUILD   # native
sigma-pkg build --format appimage PKGBUILD   # Linux AppImage
sigma-pkg build --format flatpak  PKGBUILD   # Flatpak bundle
sigma-pkg build --format apk      PKGBUILD   # Android APK
sigma-pkg build --format wasm     PKGBUILD   # WASM bundle
sigma-pkg build --format jar      PKGBUILD   # Java JAR
sigma-pkg build --format nupkg    PKGBUILD   # NuGet package
```

This is the foundation of SigmaOS's multi-format distribution promise.

---

## Privacy Policy

`sigma-pkg` sends **zero telemetry by default**. The only network requests are:

- Package index fetch (anonymous GET, no cookies, no session tracking).
- Package download (anonymous GET).
- Signature verification against the public key (local, no network).

Opt-in analytics (disabled by default):
```toml
# /etc/sigma/pkg.toml
[analytics]
enabled = false          # default
report_installs = false  # default
```

---

*See also: [Professional-Tools-And-Apps](Professional-Tools-And-Apps.md) · [Reproducibility-Guide](Reproducibility-Guide.md) · [Sovereign-Packaging-Specification](Sovereign-Packaging-Specification.md)*
