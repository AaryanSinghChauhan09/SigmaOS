# SigmaOS Formal Release Cadence & Errata Process

This document defines the release cadence, branching model, and errata patch management process for SigmaOS, adopting proven practices from Debian (Stable/Testing/Unstable) and OpenBSD (fixed 6-month cycle + signed errata).

---

## Release Cadence

1. **Fixed 6-Month Release Cycle:**
   - **March Release (X.3):** Spring major feature release.
   - **September Release (X.9):** Autumn long-term support (LTS) release.
2. **Branching Model:**
   - `main`: Active development branch (Rawhide / Rolling).
   - `release/X.Y`: Stable release branch cut at feature freeze (e.g. `release/1.0`).
   - `tags/vX.Y.Z`: Cryptographically signed release tags.

---

## Release Verification & Signing

- **GPG & Dilithium-5 Dual Signing:** All release commits, git tags, and ISO boot media manifests must be dual-signed using GPG and Dilithium-5 PQC signatures.
- **Reproducible Build Manifests:** Every release publishes SHA256 build manifests and `diffoscope` binary equivalence proofs backing up the zero-drift reproducible build guarantee.

---

## Errata & Security Patching Process

- **Security Advisory (SSA):** Critical security fixes receive an official Sigma Security Advisory number (e.g., `SSA-2026-001`).
- **Errata Patches:** Applied directly to `release/X.Y` branches and backported to active LTS releases with minimal regression risk.
