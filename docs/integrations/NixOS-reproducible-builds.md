# NixOS-Inspired Reproducible Build Patterns

## Overview

SigmaOS adopts **content-addressed, reproducible builds** inspired by Nix (LGPL) — without using Nix code. Every `sigma-pkg build` produces a deterministic output hash. Binary artifacts can be substituted from a cache if the hash matches, avoiding redundant compilation.

---

## Core Principles

1. **Content-addressed outputs** — build output is stored at `/sigma/store/<hash>-<name>-<version>/`
2. **SOURCE_DATE_EPOCH** — all timestamps clamped to the last Git commit timestamp
3. **strip-deterministic** — debug symbols stripped with stable section ordering
4. **Normalized ar archives** — `ar` archives have deterministic member order
5. **Locale + timezone isolation** — builds run in `LANG=C TZ=UTC` environments

---

## sigma-pkg build: Hash Path Layout

```
/sigma/store/
  sha256:abc123def456-sigma-edit-1.2.0/
    bin/sigma-edit
    lib/libsigma-edit.so
    share/doc/sigma-edit/
  sha256:789abc012def-sigma-curl-0.9.1/
    bin/sigma-curl
```

The hash is a SHA-256 of the **build inputs**: source tarball hash + dependency closure hashes + build flags. This mirrors the Nix store design.

---

## Build Environment Setup

```bash
# In every sigma-pkg build() function:
export SOURCE_DATE_EPOCH=$(git log -1 --format=%ct)
export LANG=C
export TZ=UTC
export LC_ALL=C
export RUSTFLAGS="-C strip=symbols"

# Normalise ar archives:
find . -name '*.a' -exec strip --strip-debug --enable-deterministic-archives {} \;
```

---

## sigma-pkg PKGBUILD Format

```toml
# packages/sigma-edit/SIGPKG
[package]
name    = "sigma-edit"
version = "1.2.0"
source  = "https://registry.sigmaos.dev/src/sigma-edit-1.2.0.tar.zst"
sha256  = "abc123...def456"

[build]
command = "cargo build --release --locked"
install = "install -Dm755 target/release/sigma-edit $out/bin/sigma-edit"

[dependencies]
sigma-gui   = ">=1.0.0"
sigma-fonts = ">=0.5.0"
```

---

## Binary Substituter Protocol

When `sigma-pkg install <pkg>` is invoked:

1. Compute expected store path hash from SIGPKG
2. Query substituter: `GET https://cache.sigmaos.dev/<hash>.narinfo`
3. If found: download + verify + install (skip build)
4. If not found: build locally, upload to cache

```
narinfo format:
  StorePath: /sigma/store/sha256:abc123-sigma-edit-1.2.0
  URL: nar/sha256:abc123.nar.zst
  Compression: zstd
  FileHash: sha256:...
  FileSize: 1234567
  Sig: sigma-cache-1:base64-dilithium5-sig
```

---

## sigma-rebuild: Reproducibility Checker

```bash
# Rebuild sigma-edit and compare hash with cached artifact
sigma-rebuild sigma-edit

# Output:
# Building sigma-edit-1.2.0...
# Local hash:  sha256:abc123def456...
# Cache hash:  sha256:abc123def456...
# ✓ REPRODUCIBLE
```

If hashes differ, `sigma-rebuild` prints a diff of the first differing byte offset and flags for investigation.

---

## Exit Criteria

- `sigma-pkg rebuild sigma-edit` produces **identical hash** on two independent builds.
- `sigma-pkg install sigma-edit` succeeds via binary substituter (no local build triggered).
- CI job `reproducible-build-check` fails if any artifact's hash changes between runs.
