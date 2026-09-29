# Architecture Decision Records (ADR)

## ADR-001: Zero External Crate Dependency Philosophy
- **Status**: Accepted
- **Context**: External dependencies bring security supply chain risks, build complexity, and version churn.
- **Decision**: All core operating system components, kernel modules, driver stacks, and core utilities must be implemented using zero external third-party Rust crates (`#![no_std]` / `std` core Rust standard library primitives).

## ADR-002: Tri-Agent Autonomous Continuous Engineering Framework
- **Status**: Accepted
- **Context**: Scaling OS development requires specialized domain focus.
- **Decision**: Utilize Bolt ⚡ (Performance), Palette 🎨 (UX & A11y), and Sentinel 🛡️ (Security) agents with strict boundaries and automated pre-commit review procedures.

## ADR-003: Multi-Distro Universal Package Manager (SigmaPkg)
- **Status**: Accepted
- **Context**: Linux and BSD ecosystems are fragmented across format managers (deb, rpm, apk, pkg, pacman, ebuild).
- **Decision**: Provide transbuilder adapters and SAT DPLL solvers in `SigmaPkg` to convert, transpile, and sandbox foreign distro packages into native `.sigpkg` objects seamlessly.
