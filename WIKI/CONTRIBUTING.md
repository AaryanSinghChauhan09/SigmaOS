# Contributing to SigmaOS

Thank you for your interest in contributing to SigmaOS!

## Development Process
1. **Fork the Repository**: Create a personal fork of the repository.
2. **Branch**: Create a feature branch (`feat/your-feature` or `fix/your-fix`).
3. **Write Code**: Ensure your code is entirely written in Rust (unless strictly necessary for bootloader assembly). Avoid `unsafe` blocks whenever possible.
4. **Format and Lint**: Run `cargo fmt` and `cargo clippy`.
5. **Test**: Run the test suite (`cargo test` or `make test`).
6. **Submit a PR**: Create a Pull Request against the `main` branch.

## Coding Standards
- Strict adherence to Rust conventions.
- Document all public modules, functions, and structs using `///` doc comments.
- Keep the microkernel minimal. Put features in user-space servers.

## Communication
- Open an Issue before starting major feature work to discuss design.
- Be respectful and follow our [Code of Conduct](CODE_OF_CONDUCT.md).
