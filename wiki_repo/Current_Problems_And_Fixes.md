# Current Problems & Fixes Tracking

This document formalizes the tracking of critical bugs and gaps across the SigmaOS ecosystem, dictating immediate action plans and architectural fixes.

## 1. Kernel & Drivers
- **Problem**: Limited hardware compatibility, unstable kernel modules, memory leaks, and driver crashes.
- **Fix**:
  - Track a Linux LTS kernel branch for stability.
  - Publish and maintain a definitive Hardware Compatibility List (HCL).
  - Upstream drivers wherever possible; fund bounties for Broadcom/NVIDIA support.
  - Audit current kernel tree and run rigorous regression tests.

## 2. Package Manager (sigpkg)
- **Problem**: Dependency resolution errors, no rollback logic, unsigned packages.
- **Fix**:
  - Implement atomic updates with transaction state and rollback logic.
  - Add GPG/Ed25519 signing for all packages and metadata.
  - Build reproducible package metadata (SBOMs).

## 3. Installer
- **Problem**: No polished GUI installer, partitioning failures, secure-boot integration gaps, dual-boot conflicts.
- **Fix**:
  - Develop a Calamares-style GUI installer alpha.
  - Add secure boot and encrypted home defaults anchored to TPM.
  - Rigorously test and patch dual-boot integrations with Windows/Linux.

## 4. Desktop (Zenith)
- **Problem**: Compositor crashes, input lag, missing accessibility options.
- **Fix**:
  - Switch to a Wayland-first compositor with XWayland fallback.
  - Add a comprehensive accessibility suite (screen reader, high contrast, Indic language packs).
  - Optimize rendering pipeline for minimal latency and strict Vulkan/DRM rendering.

## 5. Security
- **Problem**: Incomplete sandboxing, weak privilege separation, privilege escalation paths.
- **Fix**:
  - Integrate Firecracker/custom microVMs for strict app isolation.
  - Add TPM attestation and signed kernel enforcement (measured boot).
  - Implement strict MAC policies (AppArmor/SELinux paradigm) across all userland daemons.

## 6. Build & CI
- **Problem**: Lack of reproducible builds and automated CI/CD checks.
- **Fix**:
  - Deploy a containerized build farm with deterministic timestamps.
  - Build GitHub Actions pipeline enforcing kernel + packages integrity.
  - Enable automated CVE scanning in the CI pipeline.

## 7. Documentation
- **Problem**: Missing contributor guides, outdated roadmap files, inconsistent Wiki sync.
- **Fix**:
  - Create and maintain `.md` specs for each subsystem.
  - Automatically sync `.md` files with the GitHub Wiki before merging PRs.
  - Add issue templates and enforce triage labels (kernel, security, UI, AI).
