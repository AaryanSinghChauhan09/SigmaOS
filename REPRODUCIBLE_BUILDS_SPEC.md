# Reproducible Builds & CI/CD Pipeline

## 1. The Build Philosophy
SigmaOS guarantees that building the same source code twice on different machines will result in bit-for-bit identical binaries. This is achieved by containerizing the build environment, freezing timestamps, and stripping non-deterministic metadata.

## 2. GitHub Actions Skeleton (`.github/workflows/ci.yml`)
The CI pipeline is the ultimate source of truth.
1. **Compilation Check:** Uses `cargo xbuild` or Rust nightly `no_std` targets to compile the bare-metal kernel.
2. **SBOM Generation:** Automatically generates a Software Bill of Materials (SBOM) using the latest SPDX standard. This provides an exhaustive inventory of every crate and dependency used in the kernel.
3. **Reproducibility Test (Future Phase):** The CI will build the kernel twice in completely disparate environments (different paths, different timezones) and verify the SHA-256 hashes match.
4. **Cryptographic Signing:** The resultant kernel and `sigpkg` artifacts are signed via `sbsign` (mocked in CI currently) with the SigmaOS Secure Boot key.

## 3. Package Build Farm
Similar to Nix/Guix, the SigmaOS package build farm utilizes isolated MicroVMs to compile packages.
- No network access during the build phase (preventing rogue `curl | bash` scripts in build files).
- Predictable paths (`/build`).
- Fixed system time (e.g., `SOURCE_DATE_EPOCH`).
