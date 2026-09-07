name: "Supply Chain: Reproducible Builds, SBOM Generation & Cosign Signing"
description: "Ensure bit-for-bit build determinism, generate SPDX/CycloneDX SBOMs, and sign release binaries using Cosign/Sigstore."
title: "[SUPPLY-CHAIN] Reproducible Builds, SBOM Generation, and Cosign Signing"
labels: ["supply-chain", "security", "reproducible-builds", "sigstore"]
assignees: []
body:
  - type: markdown
    attributes:
      value: |
        ## Overview
        Harden the SigmaOS supply chain by verifying bit-for-bit build reproducibility across architectures, generating automated Software Bill of Materials (SBOMs), and signing all release artifacts and container images using Cosign/Sigstore keyless signing.

  - type: textarea
    id: implementation-tasks
    attributes:
      label: Implementation Tasks
      placeholder: |
        - [ ] Strip non-deterministic timestamps, paths, and metadata from build outputs
        - [ ] Add SPDX and CycloneDX SBOM generation steps to release workflow
        - [ ] Integrate Cosign keyless OIDC signing for release binaries and ISO images
        - [ ] Add dual-runner CI job verifying bit-for-bit output identity
        - [ ] Publish public verification documentation and verification keys

  - type: textarea
    id: success-metrics
    attributes:
      label: Success Metrics & Acceptance Criteria
      value: |
        - 100% build reproducibility verified across dual independent runners.
        - Validated SBOM signatures and Cosign attestations published for every release.
