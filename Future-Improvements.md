# SigmaOS: Repository Future Improvements

This document tracks meta-development goals, process refinements, and engineering quality gates beyond the feature-specific roadmap.

## ⚙️ Kernel & Platform Engineering

* **Declarative Manifests**: Transition to automated shard discovery (e.g., CMake/Ninja) to eliminate Makefile drift.
* **Panic Lifecycle**: Standardize uniform panic macros and serial minidumps for debugging.
* **SMP Maturity**: Formalize CPU bring-up and inter-processor interrupt (IPI) protocols.
* **Freestanding Audit**: Systematic removal of remaining STL headers from the core kernel.

## 🛡️ Security & Supply Chain

* **Formal Threat Modeling**: Document assets and adversaries in the Official Wiki.
* **Signed Releases**: Integrity-verified release artifacts via `sigstore` or checksums.
* **SBOM Generation**: Comprehensive Software Bill of Materials for all lattice dependencies.

## 🧪 Testing & Observability

* **QEMU Integration CI**: Automated boot tests on every PR to ensure kernel integrity.
* **Coverage Metrics**: Host-side unit test coverage tracking (gcov/llvm-cov).
* **Zenith E2E Smoke Tests**: Automated UI validation for critical dashboard paths.

## 🛠️ Developer Experience (DX)

* **Reproducible Environments**: Implementation of Dev Containers or Nix Flakes for 1-click build setups.
* **ADR Registry**: Architecture Decision Records to document major design choices.
* **Doxygen Integration**: Automatically generated API documentation from kernel source.

## 📊 Honest Progress Tracking

* **Milestone Dashboards**: Regular updates to the build status, test counts, and backlog progress on the GitHub Wiki.

---

### Σ Sovereignty is the Continuous Pursuit of Perfection
