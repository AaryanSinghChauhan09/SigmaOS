# Changelog

All notable changes to the SigmaOS project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **SovereignEventBus** (`include/ipc/SovereignEventBus.h`): Zero-heap publish/subscribe bus inspired by Linux kernel notifier chains. Decouples all kernel shards.
- **SovereignScheduler** (`include/sched/SovereignScheduler.h`): CFS-inspired fair scheduler with EDF real-time path and AI-hint integration (ghOSt-inspired).
- **SovereignClawGateway** (`kernel/core/ai/SovereignClawGateway.cpp`): Entry-point for all AI-driven automation workflows (OpenClaw-inspired).
- **SovereignAgentCore** (`kernel/core/ai/SovereignAgentCore.cpp`): Lifecycle management for autonomous AI agents.
- **SovereignKernelBridge** (`kernel/core/ai/SovereignKernelBridge.cpp`): Secure, read-only telemetry bridge for AI agents.
- **Dependabot** (`.github/dependabot.yml`): Weekly automated dependency updates for npm and GitHub Actions.
- **CodeQL Config** (`.github/codeql-config.yml`): Extended security + quality queries with path exclusions.
- **Issue Templates**: Structured `bug_report.md`, `feature_request.md`, `security_report.md`.
- **PR Template** (`.github/PULL_REQUEST_TEMPLATE.md`): Full quality/security checklist.
- **Governance Docs**: `MAINTAINERS.md`, `MAINTENANCE_POLICY.md`, `RELEASE_PROCESS.md`.
- **MIT License**: Added `LICENSE` file for legal clarity.

### Fixed
- **SovereignResearchMatrix**: Replaced `void*` containers with `std::vector` + `std::map`, fixed invalid `const const char*` signature, converted blocking `sleep_for` to async detached threads.
- **SovereignScholasticRepo**: Replaced `void* m_repo` with `std::map<std::string, std::unique_ptr<IScholasticShard>>`, fixed iteration with structured bindings.
- **SovereignOmniTool**: Eliminated all raw machine opcode execution (FPU, VMCALL, AVX2, hot-patch JMP) — replaced with safe portable C++ equivalents.
- **SovereignSpotlight**: Made `reindexLattice()` async via `std::thread::detach()` to prevent startup blocking.
- **zenith_desktop.js search**: Added debounce utility (150ms) — eliminates per-keystroke O(N) rescanning and UI jank.
- **sigma_log / sigma_printf**: Resolved mixed stream/printf usage across research shards.
- **SHARDS.manifest**: Rebuilt with correct paths and all new industrial shards registered.

### Changed
- **CI/CD** (`.github/workflows/ci.yml`): Upgraded to actions v4, added arm64 cross-compile matrix, QEMU boot smoke test, SBOM generation (SPDX), npm audit job, and automated release publishing.
- **README.md**: Added Build Status, Maintainability badges and Governance section.
- **CODEOWNERS**: Expanded to cover all subsystem boundaries.
- **1000-Shard Modularization Roadmap** published to `docs/MODULARIZATION_ROADMAP.md` and GitHub Wiki.

## [v100.2_Futuristic] - 2026-05-08

### Added
- **Sovereign AI Stack**: Integrated `ClawGateway`, `WorkflowEngine`, and `PredictiveUX` shards.
- **Agentic Infrastructure**: Added `SovereignAgentCore` and `SovereignChain` for autonomous task orchestration.
- **Industrial Infrastructure**: Added `.clang-format`, `.clang-tidy`, and `Doxyfile`.
- **Reproducible Environments**: Added DevContainer and root `Dockerfile`.
- **New Documentation**: Added `ARCHITECTURE.md`, `ROADMAP.md`, `GETTING_STARTED.md`, `HARDWARE.md`, and `MODULARIZATION_ROADMAP.md`.
- **Infrastructure**: Added `SovereignDAL` for cross-distro package management abstraction.
- **Strategy**: Published the **1000-Shard Modularization Roadmap** for long-term lattice evolution.
- **Sovereign Packaging Specification**: `ecosystem/packaging-spec.md` defining `.spkg` archive format.
- **Marketplace UI**: Glassmorphic shard marketplace injected into Zenith `zenith.html`.
- **MAC Engine**: `validateMACPolicy` stubs in `SovereignSandboxEngine`.
- **LogD**: Centralized logging daemon `userland/logd.cpp`.
- **Update Agent**: Atomic PQC-signed update engine `userland/update-agent.cpp`.
- **Installer**: `iso_root/installer/install.sh` prototype.
- **Verified Boot**: `docs/security/verified-boot.md` with Dilithium/TPM chain-of-trust.
- **Accessibility Guide**: `docs/a11y.md`.

### Fixed
- Improved header normalization across all 600 shards.
- Resolved compilation errors in the `sigma_log` and `sigma_hal` interfaces.
- Fixed glassmorphic rendering bugs in `zenith_desktop.css`.
