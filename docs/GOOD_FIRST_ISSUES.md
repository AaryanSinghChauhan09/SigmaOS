# Good First Issues for SigmaOS

Welcome to SigmaOS! This page lists issues that are good starting points for new contributors. These issues are well-scoped, have clear acceptance criteria, and include guidance for implementation.

## How to Contribute

1. Check the [Contributing Guide](../CONTRIBUTING.md) for our contribution process
2. Look for issues labeled `good-first-issue` in the repository
3. Comment on the issue to claim it (we'll assign it to you)
4. Ask questions in the issue if you need clarification
5. Submit a pull request with your changes

## Current Good First Issues

### Phase 0: Core & Trust

#### [Implement Reproducible Builds Pipeline](../.github/ISSUE_TEMPLATE/reproducible-builds-pipeline.md)
- **Difficulty**: Medium
- **Time Estimate**: 1–3 weeks
- **Skills Needed**: CI/CD, CMake, Rust
- **Description**: Create a CI job that produces byte-for-byte reproducible images and records build provenance metadata
- **Labels**: `Phase 0`, `security`, `ci`, `medium-priority`

### Phase 1: Hardware Parity

#### [Add RISC-V and aarch64 CI Images](../.github/ISSUE_TEMPLATE/multi-arch-ci-images.md)
- **Difficulty**: Medium
- **Time Estimate**: 1–3 weeks
- **Skills Needed**: Cross-compilation, QEMU, CMake, Rust
- **Description**: Implement multi-architecture CI support for RISC-V and aarch64 using QEMU test images
- **Labels**: `Phase 1`, `hardware`, `ci`, `medium-priority`

### Phase 2: App Ecosystem

#### [Implement WASI/WASM Runtime](../.github/ISSUE_TEMPLATE/wasm-wasi-runtime.md)
- **Difficulty**: Medium
- **Time Estimate**: 2–6 weeks
- **Skills Needed**: Rust, WASM, Wasmtime, systems programming
- **Description**: Implement a WASI/WASM runtime for sandboxed applications using Wasmtime
- **Labels**: `Phase 2`, `security`, `runtime`, `medium-priority`

## Easy Wins (Quick Start)

These are smaller tasks that can be completed in a few hours to a few days:

### Documentation
- Add inline documentation to kernel modules
- Write tutorials for building custom images
- Create driver authoring guide
- Improve README with getting started examples

### Testing
- Add unit tests for core utilities
- Create smoke tests for basic shell commands
- Add property-based tests for data structures
- Set up fuzzing for specific components

### Tooling
- Improve error messages in sigma-sh
- Add color output to CLI tools
- Create shell completion scripts
- Add progress indicators to long-running operations

## Skill-Based Categories

### For Rust Developers
- Kernel scheduler improvements
- WASM runtime implementation
- Driver development (Rust-based)
- IPC system enhancements

### For C/C++ Developers
- Low-level kernel components
- Bootloader integration
- Hardware abstraction layer
- Performance-critical paths

### For Web/JavaScript Developers
- Package store web UI
- Developer dashboard
- Documentation site improvements
- Management interface

### For Systems/OS Developers
- Filesystem implementation
- Driver porting from Linux
- Virtualization support
- Power management

## Getting Help

- **Discord/IRC**: Join our community chat for real-time help
- **GitHub Discussions**: Use discussions for questions and ideas
- **Email**: Contact maintainers for sensitive topics
- **Office Hours**: Weekly contributor calls (check calendar)

## Contributor Recognition

We recognize contributors through:
- Hall of Fame in our documentation
- Contributor badges in GitHub
- Annual contributor appreciation post
- Opportunity to become a maintainer

## Next Steps

1. Pick an issue from the list above
2. Read the issue template for detailed requirements
3. Set up your development environment
4. Start hacking and ask questions if stuck
5. Submit your pull request

---

**Note**: If you don't find an issue that matches your skills or interests, feel free to propose a new issue! We're always open to new ideas and improvements.
