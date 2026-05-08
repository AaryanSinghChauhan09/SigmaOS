# Changelog
All notable changes to the SigmaOS project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v100.2_Futuristic] - 2026-05-08

### Added
- **Sovereign AI Stack**: Integrated `ClawGateway`, `WorkflowEngine`, and `PredictiveUX` shards.
- **Agentic Infrastructure**: Added `SovereignAgentCore` and `SovereignChain` for autonomous task orchestration.
- **Industrial Infrastructure**: Added `.clang-format`, `.clang-tidy`, and `Doxyfile`.
- **Reproducible Environments**: Added DevContainer and root `Dockerfile`.
- **New Documentation**: Added `ARCHITECTURE.md`, `ROADMAP.md`, `GETTING_STARTED.md`, and `HARDWARE.md`.
- **Infrastructure**: Added `SovereignDAL` for cross-distro package management abstraction.

### Fixed
- Improved header normalization across all 600 shards.
- Resolved compilation errors in the `sigma_log` and `sigma_hal` interfaces.
- Fixed glassmorphic rendering bugs in `zenith_desktop.css`.

### Changed
- Refactored `SHARDS.manifest` to follow the industrial 6-layer architecture model.
- Updated `ci.yml` with linting jobs and CodeQL security scanning.
