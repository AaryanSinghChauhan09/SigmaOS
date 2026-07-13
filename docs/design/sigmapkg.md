# SigmaPkg (`sigpkg`) Design Specification

## 1. Overview
SigmaPkg (`sigpkg`) is the native package manager for SigmaOS. Inspired by Nix, APK (Alpine), and XBPS (Void), it is designed for speed, atomicity, and reproducibility.

## 2. Core Principles
1. **Atomic Transactions**: Installs and upgrades are all-or-nothing. If a package fails to extract or verify, the transaction is completely rolled back.
2. **Content-Addressed Storage**: Package assets are stored in a content-addressed store (e.g., `/sigma/store`), allowing multiple versions of libraries to coexist if needed.
3. **Cryptographic Verification**: Every `.spkg` archive must be signed with Dilithium5 (Post-Quantum Cryptography).
4. **Zero-Unsafe Policy**: The package manager is written entirely in safe Rust.

## 3. Architecture
- **Resolver**: Calculates the dependency graph and ensures no conflicts.
- **Fetcher**: Downloads `.spkg` archives from mirrored repositories using `sigma-net`.
- **Verifier**: Checks the Dilithium5 signature of the downloaded package against the trusted keystore.
- **Extractor**: Atomically unpacks the payload to the store and updates symlinks in `/usr/bin` (or equivalent).

## 4. Interfaces
```bash
$ sigpkg install helix
$ sigpkg remove helix
$ sigpkg update
$ sigpkg upgrade
```

## 5. Security Considerations
- **Supply Chain Attacks**: Mitigated by strict post-quantum cryptographic signatures.
- **Incomplete Updates**: Mitigated by writing to a temporary overlay and performing an atomic rename/swap.
- **Capabilities**: `sigpkg` runs with `CAP_NET_RAW` for fetching and `CAP_SYS_ADMIN` for unpacking, but drops capabilities when running pre/post install scripts (which are executed in a sandboxed compartment).
