# Upstream Study: Arch Linux Pacman & Packaging

1. **Upstream Problem**: Transparent package archives, fast dependency resolution, and ALPM database hooks.
2. **Data Structure & Control Flow**: PKGINFO metadata parsing, dependency graph SAT solver, tar.zst extraction, hook triggers.
3. **Security & Failure Assumptions**: GPG signature verification (`pacman-key`); file collision protection.
4. **Direct Adoption**: PKGINFO stanza format, tar.zst container packaging, and pre/post transaction hooks.
5. **Rust/SigmaOS Adaptation**: `SigmaPkg` dependency solver in safe Rust with sandboxed hook execution.
6. **Exit Test Criteria**: Dry-run package installation resolves complex graph and detects file collision.
7. **Licensing & Attribution**: GPL-2.0 License.
