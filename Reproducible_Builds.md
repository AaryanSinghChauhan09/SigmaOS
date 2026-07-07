# Reproducible Builds Roadmap

## Philosophy
Every package and kernel built for SigmaOS must be bit-for-bit reproducible.

## CI Build Farm
- **Phase 1:** GitHub Actions skeleton generating SBOMs and verifying compilation targets.
- **Phase 2:** Containerized offline build runners mimicking Nix/Guix strict isolation.
- **Phase 3:** Deterministic timestamps (`SOURCE_DATE_EPOCH`) and path-stripping enforcement across the entire `sigpkg` ecosystem.

## SBOM and Attestation
Cryptographic signing of both the artifact and its accompanying SBOM guarantees full supply chain transparency.
