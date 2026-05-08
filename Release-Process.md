# SigmaOS Release Process

This document describes how official SigmaOS releases are prepared, signed, and distributed.

## 📦 1. Preparation
- Finalize the `CHANGELOG.md` with all new features and fixes.
- Tag the release in Git (e.g., `v100.2_Futuristic`).
- Trigger the **Release Pipeline** in GitHub Actions.

## 🖋️ 2. Cryptographic Signing
- Release artifacts (`sigmaos.iso`, `shards.tar.gz`) are signed using the Sovereign PQC key.
- Hashes (SHA-256) are generated for all binaries.

## 🚀 3. Distribution
- Signed binaries are uploaded to GitHub Releases.
- The `SHARDS.manifest` is updated to reflect the new lattice state.
- Documentation and Wiki are updated to match the release version.

## 🛡️ 4. Verification
Users can verify the integrity of a release using the following command:
```bash
sigma-verify --artifact sigmaos.iso --signature sigmaos.iso.sig
```

---
*For security reporting during the release cycle, see [Security Policy](Security-Policy.md).*
