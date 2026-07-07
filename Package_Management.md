# Package Management Roadmap (sigpkg)

## 1. sigpkg v1 Architecture & Format
The sovereign package manager (`sigpkg`) enforces strict reproducibility, cryptographic provenance, atomic updates, and deterministic rollbacks.

### Package Format Spec (`.sigpkg`)
A `.sigpkg` file is an uncompressed tape archive (`tar`) structure comprising:
1. `meta.toml`: Declares package metadata (name, version, dependencies, epoch, build target).
2. `payload.tar.zst`: Zstandard compressed binary directories (`/bin`, `/lib`, `/usr`).
3. `manifest.json`: JSON payload storing SHA-256 hashes of every file.
4. `sigpkg.sig`: Detached Ed25519 cryptographic signature of the `meta.toml` and `manifest.json`.

## 2. Signing & Repository Security
- **Key Infrastructure**: Repositories require Ed25519 signature pairs. Packages cannot be staged if verification against the public keyring fails.
- **Delta Updates**: Employs Courgette/bsdiff adaptations to ship small chunk differences instead of complete binary payloads.

## 3. Transaction State Machine & Rollback
Updates are non-destructive and staged onto passive subvolumes:
- `STAGING` -> `VERIFYING` -> `COMMITTING` -> `SUCCESS` (or `ROLLEDBACK` if boot fails).
- Anomaly monitors trigger automatic bootloader reversion if systemd-boot/grub detects kernel panic or critical daemon exit loops.

## 4. Roadmap Phases
- **Phase 1 (0–3m)**: CLI builder utility, metadata parser, and basic verification stubs.
- **Phase 2 (3–6m)**: Integration of Zstandard decompression and signature validation in boot targets.
- **Phase 3 (6–9m)**: Delta calculation engine and Btrfs/SigmaFS snapshot manager.
- **Phase 4 (9–12m)**: Multi-repo signed registry index hosting and mirrors.

## 5. Contributor Guidelines
- Packages must be defined in a clean, single declarative `recipe.yaml`.
- Absolutely no arbitrary post-install shell scripting is permitted.
