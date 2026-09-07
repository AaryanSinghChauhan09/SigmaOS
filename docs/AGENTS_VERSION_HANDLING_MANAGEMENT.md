# SigmaOS AI Agent Version Handling Management Guidelines

## 1. Overview
SigmaOS implements declarative versioning, generation profile management, and atomic version rollbacks operated by AI system agents (such as `VersionHandlingManager`, `NixProfileStore`, `SteamOsVersionUpdater`, and `DeclarativeAppVersionParser`). These guidelines define Semantic Versioning (`Version` struct), NixOS-style profile generation rollbacks, SteamOS atomic A/B partition updates, and `.sigma-app` app shard versioning.

## 2. Core Version Handling Management Principles

### 2.1 Semantic Versioning (`Version::parse`)
- **SemVer Structure**: Versions are parsed into structured `Version` tuples (`major`, `minor`, `patch`) handling build metadata and pre-release tags (`7.0.11-r1` -> `Version { major: 7, minor: 0, patch: 11 }`).
- **Version Constraint Matching**: Dependency resolvers evaluate version constraints (`Exact`, `GreaterThan`, `LessThan`, `GreaterOrEqual`, `LessOrEqual`, `Any`).

### 2.2 NixOS Declarative Profile Generations
- **Store Path Hash Verification**: Packages are installed to content-addressed store paths (`/nix/store/<hash>-<name>-<version>`).
- **Atomic Generation Rollbacks**: `NixProfileStore` (`src/sigpkg/nixos.rs`) maintains profile generations with package set snapshots. Switching or rolling back generations (`sigma rollback gen-<id>`) is an atomic symlink swap operation.

### 2.3 SteamOS Atomic A/B System Partition Updates
- **A/B Partition Updates**: `SteamOsAtomicAbImageUpdateEngine` applies system partition updates to inactive A/B storage slots with SHA256 image checksum verification.
- **Fail-Safe Fallback**: Boot failures automatically revert `BootOrder` to the active slot, preventing unbootable system states.

### 2.4 Declarative App Shards (`.sigma-app`) Versioning
- **Shards Marketplace**: `.sigma-app` manifests specify app version numbers, API capability levels, and minimum OS kernel version requirements (`src/package/declarative_app.rs`).

---
*Maintained by the SigmaOS Release, Packaging & SIG-Apps Steering Committee.*
