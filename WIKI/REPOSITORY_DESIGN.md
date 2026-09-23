# SigmaOS Package Repository Infrastructure Architecture

## 1. Executive Summary

The SigmaOS Package Repository Infrastructure defines the hosting, indexing, mirroring, and distribution topology for official and third-party `.sigpkg` software repositories. It guarantees high availability, cryptographic tamper detection, delta package generation, and global CDN delivery.

## 2. Repository Tiers & Hierarchy

SigmaOS repositories are organized into distinct channels:

```
+-----------------------------------------------------------------+
|                       Repository Hierarchy                      |
|                                                                 |
|  +------------------+  +-------------------+  +---------------+ |
|  | Core             |  | Extra             |  | Multilib      | |
|  | (Kernel, init,   |  | (Desktop, GUI apps|  | (32-bit compat| |
|  | core libraries)  |  | developer tools)  |  | drivers)      | |
|  +------------------+  +-------------------+  +---------------+ |
|                                                                 |
|  +------------------------------------------------------------+ |
|  | Community & User Repository (AUR-style user submissions)  | |
|  +------------------------------------------------------------+ |
+-----------------------------------------------------------------+
```

### 2.1 Official Repository Branches
- **`core`**: Mission-critical system components, bootloader, kernel, toolchains, base libraries. High security audit requirement.
- **`extra`**: Desktop environments, web browsers, productivity apps, multimedia frameworks.
- **`multilib`**: 32-bit execution support libraries for legacy software and gaming runtimes.
- **`staging` & `testing`**: QA validation channels before package promotion to stable.

## 3. Repository Directory Structure & Metadata

A standard SigmaOS repository mirror layout:

```
https://repo.sigmaos.org/sigpkg/
├── x86_64/
│   ├── core/
│   │   ├── core.db.tar.zst          # Index database of all core packages
│   │   ├── core.db.tar.zst.sig      # Detached cryptographic signature
│   │   ├── linux-kernel-6.10-1-x86_64.sigpkg
│   │   └── sigma-init-1.0-1-x86_64.sigpkg
│   ├── extra/
│   │   ├── extra.db.tar.zst
│   │   └── zenith-terminal-1.2-1-x86_64.sigpkg
```

### 3.1 Repository Metadata Database (`core.db`)
Contains compact index files describing every package:
- Package metadata (`depends`, `version`, `size`, `installed_size`).
- File list manifests (`files`) for fast `sigpkg search-file` lookups.
- Cryptographic hash trees (BLAKE3 / SHA-256) for verifying package payloads before installation.

## 4. Mirror Synchronization & CDN Distribution

- **Mirror Brain Daemon (`sigmirror`)**: Geo-IP routing engine directing clients to the nearest synchronized HTTP/HTTPS/Rsync mirror node.
- **Atomic Repository Sync**: Repository index database updates use atomic symlink swaps, preventing partial read inconsistencies while packages sync.
