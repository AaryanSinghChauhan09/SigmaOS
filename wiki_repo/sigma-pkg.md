# sigma-pkg — SigmaOS Package Manager Specification

**Status:** Draft · Target: v0.1 (local mode) / v1.0 (online mode)
**Owner:** ecosystem/pkg team
**Canonical source:** `userland/sigma-pkg/`

---

## Overview

sigma-pkg is the sovereign package manager for SigmaOS. v0.1 ships a fully local build-and-install flow with no network dependency. v1.0 adds a public registry, online search, and cryptographically verified downloads. Zero telemetry in both modes.

## Goals

- Reproducible installs: content-addressed store keyed by `BLAKE3(archive)`

- Atomic transactions: every install/remove is a single rename(2) swap

- Rollback: one-level undo for any operation

- Privacy: no analytics, no phone-home, no version check pings

- PQC signatures: every package signed with Dilithium-5; verified before extraction

---

## CLI Commands

```
sigma-pkg install   <pkg>[@<ver>] [--arch <arch>] [--root <dir>]
sigma-pkg remove    <pkg> [--purge]
sigma-pkg update    [<pkg>]           # fetch latest metadata + upgrade

sigma-pkg search    <query>           # full-text search in registry index

sigma-pkg info      <pkg>[@<ver>]     # show metadata, deps, files

sigma-pkg verify    <pkg>             # re-check installed package signatures

sigma-pkg build     <PKGBUILD>        # build .sigpkg from PKGBUILD recipe

sigma-pkg list      [--installed | --available]
sigma-pkg rollback  [<pkg>]           # undo last install/remove

sigma-pkg clean     [--cache]         # remove downloaded archives from cache

```

---

## .sigpkg Format

A `.sigpkg` file is a `zstd`-compressed tar archive with the following structure:

```
<name>-<version>-<arch>.sigpkg
├── META/
│   ├── manifest.toml      # name, version, arch, deps[], license, description

│   ├── checksums.b3       # BLAKE3 of every file in payload

│   └── signature.dil5     # Dilithium-5 signature over checksums.b3

└── payload/               # files to install, rooted at /

    └── usr/...
```

### manifest.toml fields

```toml
name        = "sigma-sh"
version     = "0.2.0"
arch        = "x86_64"           # or "aarch64" / "riscv64" / "any"

depends     = ["libc-sigma>=1.0", "sigma-bus>=0.5"]
optional    = ["sigma-ai"]
license     = "Apache-2.0"
description = "SigmaOS default shell"
install_sh  = "META/install.sh"  # optional post-install hook

```

---

## Registry Protocol (v1.0)

Base URL: `https://registry.sigmaos.dev/v1/`

| Endpoint | Method | Response |
|----------|--------|----------|
| `/v1/index` | GET | `index.zst` — full package list (TOML array) |
| `/v1/pkg/{name}/{version}/{arch}` | GET | `.sigpkg` binary stream |
| `/v1/search?q=<query>` | GET | JSON array of matching manifest.toml objects |
| `/v1/info/{name}/{version}` | GET | Single manifest.toml as JSON |

Index is signed by the registry key (Dilithium-5); client verifies before caching. Index format: TOML, one `[[package]]` stanza per entry. Updated on every registry push.

---

## Dependency Resolver

Algorithm: topological sort (Kahn's algorithm) on the dependency DAG.

1. Build DAG from `manifest.toml` `depends` fields

2. Detect cycles → error with cycle description

3. Compute install order (leaves first)

4. Conflict detection: two packages providing same virtual target → error unless `replaces` declared

5. Version range evaluation: `>=`, `<=`, `==`, `~=` (compatible release)

---

## Atomic Transaction Model

```
install flow:
  download → verify sig → verify checksums → extract to /var/sigma-pkg/staging/<name>-<ver>/
  → build overlay: for each file: rename staging → final path (atomic)
  → write /var/sigma-pkg/db/<name>.toml (installed record)
  → cleanup staging

rollback flow:
  read /var/sigma-pkg/db/<name>.toml.bak (saved before install)
  → rename current files back to staging
  → restore backup files from /var/sigma-pkg/backup/<name>-<prev-ver>/
```

On failure at any step: staging directory left for inspection; no partial state in live tree.

---

## Privacy

- No network requests unless `sigma-pkg update` or `sigma-pkg install` with remote source

- No telemetry, usage stats, or version pings ever

- `--offline` flag: disables all network access for this invocation

- All registry TLS uses TLS 1.3 + Kyber-1024 hybrid

---

## Implementation Plan

- [ ] 1. manifest.toml parser + validator

- [ ] 2. .sigpkg tar-zstd writer + reader

- [ ] 3. BLAKE3 checksum engine (`crypto/blake3.c`)

- [ ] 4. Dilithium-5 signature verify wrapper (`crypto/dilithium.c`)

- [ ] 5. Local install/remove/list commands

- [ ] 6. Package DB (`/var/sigma-pkg/db/`)

- [ ] 7. Atomic rename-based install

- [ ] 8. Rollback (backup + restore)

- [ ] 9. Dependency resolver (Kahn's topo-sort)

- [ ] 10. Registry HTTP client (v1.0) with TLS + Kyber

- [ ] 11. `sigma-pkg build` from PKGBUILD

- [ ] 12. `sigma-pkg search` full-text index

- [ ] 13. `sigma-pkg verify` re-check installed

- [ ] 14. Test suite: install/remove/rollback, dep resolution, sig verification

---

## Status

| Feature | State |
|---------|-------|
| Local install (v0.1) | ⬜ Not started |
| Dep resolver | ⬜ Not started |
| Atomic transactions | ⬜ Not started |
| Registry protocol (v1.0) | ⬜ Not started |
| Dilithium-5 sig verify | ⬜ Not started |
| sigma-pkg build | ⬜ Not started |
