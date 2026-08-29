# Changelog

All notable changes to SigmaOS will be documented in this file.

## [Unreleased]

### Added
- **V13 Absolute Omnipresent Self-Sufficiency Ultra Encyclopedia**: Extensive documentation mapping 500+ legacy applications to Twelve native Sovereign System Shards.
- **Sentinel enhancements**: Added rigorous path traversal protection preventing bypasses using `..` with `:` delimiters.
- **sigpkg improvements**: Enhanced universal multiformat package management system capabilities, bringing closer parity with Linux/BSD distro packaging semantics.
- **Palette Desktop**: Improved installer wizard focus management, button state ARIA labels, and inline validation for robust accessibility.
- **Wiki Documentation**: Comprehensive wiki pages added encompassing Architecture, Component Matrices, Roadmap, Security Models, and Distro Inspirations.

### Changed
- **Linux & BSD Distro Interoperability**: Major improvements to ensure subsystems function cohesively across compatibility layers.
- **Core Kernel**: Cleaned up syntax comments and enum variant duplications, stabilizing C++ harness and Rust inspection test suites.
- **Documentation**: Transferred implementation status documents to the centralized WIKI for better visibility.
- **Project Structure**: Added standard `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, and `ARCHITECTURE.md` to the `docs/` folder.

### Fixed
- Fixed critical security vulnerabilities in GitHub Actions token permissions.
- Resolved various syntax and enum duplication issues in kernel core files.

### Performance
- **Bolt Audio**: Cached explicit name lengths in `SimpleAudioDevice` for improved low-latency audio processing.
