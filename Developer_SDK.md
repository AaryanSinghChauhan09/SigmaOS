# Developer SDK Roadmap

## 1. Developer Sandboxes
To keep the host system pristine, SigmaOS provides isolated developer sandboxes.
- **MicroVM Shell**: Spawns isolated builder containers (`sigma_sandbox.rs`) pre-loaded with developer packages.
- **Resource Limits**: Configures CPU/Memory quotas for builds to prevent compiler resource starvation.
- **LSP Bridging**: Redirects Neovim, VSCode, and Helix LSP requests through a secure socket into the container environment.

## 2. Packaging Templates
Official templates simplify compilation and testing setup for multiple frameworks:
- **Rust Template**: Automates cross-compilation with standard `no_std` compiler profiles.
- **C/C++ Template**: Enforces reproducible header checks and static compilation directives.
- **Node.js/Python Templates**: Freezes dependency states using lockfiles.

## 3. CI/CD Skeleton
Provides GitHub Actions workflows for:
- Deterministic checks and SBOM validations.
- Automated code formatting and linting.
- Ed25519 signature signing for build releases.

## 4. Roadmap Phases
- **Phase 1 (0–3m)**: Standardize cross-compilation targets and publish basic template recipes.
- **Phase 2 (3–6m)**: Implement container isolation controls and LSP socket redirections.
- **Phase 3 (6–9m)**: Launch the verification CLI tools and SBOM validator integrations.
- **Phase 4 (9–12m)**: Expand IDE extension suites and debugging tool wrappers.

## 5. Contributor Guidelines
- Add tests for every new SDK template in standard QEMU images.
- Keep templates clean, documenting variable overrides.
