# Universal Package Translator & Cross-Distro Manual

## Architecture Overview

The Universal Package Translator (`UniversalPackageTranslator`) and Repository Synchronizer (`DistroRepoSyncEngine`) in `src/package/universal.rs` allow SigmaOS to ingest, translate, and install packages from any major Linux or BSD distribution into native `SigmaPkg` format.

## Key Features Inspired by Linux & BSD Distros

### 1. Multi-Format Ingestion

Supports reading package metadata from:

*   `.deb` / `.udeb` (Debian, Ubuntu, Linux Mint)
*   `.rpm` (Fedora, RHEL, openSUSE)
*   `.pkg.tar.zst` / `.pkg.tar.xz` (Arch Linux, Manjaro)
*   `.apk` (Alpine Linux)
*   `.xbps` (Void Linux)
*   `.pkg` / `.ports` (FreeBSD)
*   `.ebuild` (Gentoo Linux)
*   `.nixpkg` (NixOS)

### 2. Dependency Normalization (`normalize_dependency_name`)

Inspired by Bedrock Linux `brl` interop, package dependencies across Linux distros are mapped to unified `SigmaPkg` virtual tokens:

*   `libc6` / `glibc` / `musl` -> `sovereign-libc`
*   `gcc` / `g++` / `clang` -> `sovereign-build-essential`
*   `libssl-dev` / `openssl-devel` -> `sovereign-openssl`
*   `python3` / `python3-base` -> `sovereign-python3`
*   `nodejs` / `node` -> `sovereign-nodejs`

### 3. External Repository Synchronizer (`DistroRepoSyncEngine`)

Automated indexing and metadata caching for Debian APT, Arch ALPM, Fedora DNF, Alpine APKINDEX, and FreeBSD Ports repositories.

### 4. Direct Foreign Installation (`install_foreign_distro_package`)

Allows `UniversalPackageManager` to ingest foreign package manifests and install them directly as native `SigmaPkg` objects (`sigpkg-<package>`).
