# SigmaOS Maintenance Policy

This policy outlines the expectations for maintaining shards and infrastructure within the Sovereign Lattice.

## ðŸ Quality Standards

1. **Code Style**: All C++ code must pass `clang-format` and `clang-tidy` checks.
2. **Documentation**: Every new shard must be registered in `SHARDS.manifest` and the Wiki.
3. **Security**: Shards must operate within the appropriate `SovereignSandbox` level.

## ðŸ› ï¸ Review Process

- All changes must be submitted via Pull Requests.
- PRs require at least one approval from a designated subsystem owner (see `CODEOWNERS`).
- CI/CD quality gates must pass (Build, Test, Scan).

## ðŸš€ Release Cadence

- **Nightly**: Automated builds from the `develop` branch.
- **Stable**: Monthly milestones from the `main` branch.
- **Critical**: Security patches are prioritized and released immediately upon verification.

---

### Questions? Contact the project lead via GitHub Discussions
