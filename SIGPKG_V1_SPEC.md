# Sovereign Package Manager (sigpkg) v1.0 Specification

## 1. Overview
`sigpkg` is the canonical package manager for SigmaOS. It diverges from traditional package managers (apt, pacman, dnf) by enforcing strict reproducibility, cryptographic provenance, atomic updates, and deterministic rollbacks, adopting the best concepts from Nix and Guix without abandoning the traditional imperative workflow familiar to most developers.

## 2. Core Tenets
1. **Cryptographic Provenance:** No package can be installed without a valid Ed25519 signature from a trusted repository key.
2. **Atomic Transactions:** Updates are staged into a separate BTRFS/SigmaFS subvolume or snapshot. The live system is never mutated in-place.
3. **Instant Rollback:** If an update causes boot failure or kernel panic, the system automatically pivots back to the previous snapshot.
4. **Delta Updates:** Network bandwidth is preserved by fetching binary deltas (bsdiff/courgette style) rather than full tarballs.

## 3. Package Structure (.sigpkg)
A `.sigpkg` file is an uncompressed archive containing:
- `meta.toml`: Name, version, dependencies, architecture, epoch.
- `payload.tar.zst`: Zstandard compressed binary payload.
- `manifest.json`: SHA-256 hashes of every file in the payload.
- `sigpkg.sig`: Detached Ed25519 signature of the `meta.toml` + `manifest.json`.

## 4. Transaction State Machine
The `sigpkg` daemon uses a rigorous state machine (implemented in `userland/sigpkg/src/lib.rs`):
- `IDLE`: Daemon ready.
- `DOWNLOADING`: Fetching payloads and deltas.
- `VERIFYING`: Checking Ed25519 signatures and payload SHA-256 hashes against the manifest.
- `STAGING`: Extracting payload to the passive snapshot `/mnt/sigma_staging`.
- `COMMITTING`: Swapping the active bootloader entry to the staged snapshot.
- `SUCCESS / FAILED / ROLLEDBACK`: Final outcome tracking.

## 5. Security Posture
- Packages do not have install scripts (`postinst`, `preinst`). All configuration must be declarative via the system service manager (SigmaInit) or declarative triggers.
- This entirely eliminates the risk of malicious arbitrary code execution during the `dpkg --configure` phase seen in traditional distros.
