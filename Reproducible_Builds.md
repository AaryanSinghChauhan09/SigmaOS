# Reproducible Builds Roadmap

## 1. Containerized Build Farm
SigmaOS guarantees that building the same source twice results in bit-for-bit identical binary output.
- **Build runner**: Isolated builder containers containing standardized toolchains and libraries.
- **Sealed environments**: Builders run offline without internet connectivity to prevent unauthorized package downloads during compilation.
- **Nix/Guix Adaptations**: System compilers map outputs to address-based content trees.

## 2. Determinism Policies
- **Frozen Timestamps**: Timestamps are overwritten using a fixed epoch (`SOURCE_DATE_EPOCH`).
- **Path Stripping**: All binary paths are stripped of compiler host directories.
- **Deterministic ordering**: Compilers sort source trees before parsing blocks.

## 3. SBOM Generation & Attestation
- **SPDX/CycloneDX Integration**: The CI build farm generates software bills of materials (SBOM) documenting exact version hashes of dependencies.
- **Cryptographic Attestations**: Build systems sign output hashes, attaching provenance attestations to verify that compilation occurred on certified runners.

## 4. Roadmap Phases
- **Phase 1 (0–3m)**: Standardize build tooling versioning and enforce compiler environment settings.
- **Phase 2 (3–6m)**: Script build pipelines to verify matching binary checksums on separate machines.
- **Phase 3 (6–9m)**: Automate SPDX SBOM generator integrations for every `.sigpkg` compilation.
- **Phase 4 (9–12m)**: Setup the distributed build farm using signed builder verification.

## 5. Contributor Guidelines
- Do not utilize compilation-time variables (like current system clock checks or environment overrides).
- Ensure all library dependencies are explicitly pinned inside `Cargo.toml` or dependency profiles.
