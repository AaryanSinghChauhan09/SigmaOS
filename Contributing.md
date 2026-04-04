# CONTRIBUTING TO Σ SIGMAOS: SOVEREIGN ZENITH 🛡️

First of all, thank you for considering contributing to SigmaOS. We are building the world's most performant, zero-dependency, and industrial-grade operating system monolith. Your help is mission-critical!

When contributing to this repository, please first discuss the change you wish to make via an issue, email, or any other method with the owners of this repository before making a change.

Please note we have a [Code of Conduct](./CODE_OF_CONDUCT.md), please follow it in all your interactions with the project.

## Pull Request Process 🌌

1. Ensure any install or build dependencies are removed before the end of the layer when doing a build.
2. Update the `README.md` with details of changes to the interface, this includes new environment variables, exposed ports, useful file locations and container parameters.
3. Increase the version numbers in any relevant files and the `README.md` to the new version that this Pull Request would represent.
4. You may merge the Pull Request after it has been reviewed and signed off by at least two other maintainers. If you do not have permission to do so, a maintainer will merge it for you.

## Development Standards 🛡️

*   **Zero-Dependency**: No external headers or libraries. Use `sigma_kernel_types.h` and the defined silicon-direct primitives.
*   **Pure x86_64**: Ensure all assembly is for 64-bit Long Mode.
*   **Safety First**: Use **B6 Sovereign Stack Canaries** and **NMA Isolation** for all new shards.
*   **Performance Purity**: Use the provided benchmarking shards to ensure your code is at least 25% more efficient than standard OS counterparts.

## Bug Reports 🛡️

We use GitHub Issues to track public bugs. Report a bug by [opening a new issue](https://github.com/AaryanSinghChauhan09/SigmaOS/issues/new).

**A great bug report includes:**
* A quick summary and/or background
* Steps to reproduce
* What you expected would happen
* What actually happens
* Notes (possibly including why you think this might be happening, or stuff you tried that didn't work)

## License 🌌

By contributing to SigmaOS, you agree that your contributions will be licensed under its license.

---
**SigmaOS Zenith. Pure Performance. Absolute Sovereignty.**
