# SigmaOS Architecture Decisions Records (ADR)

## ADR-001: Separation of Desktop Edition from Experimental Microkernel Research
- **Context**: The repository contained parallel ambitions for an all-in-one microkernel, POSIX clone, hypervisor, cloud container runtime, and desktop OS.
- **Decision**: Prioritize **SigmaOS Desktop Edition** as the primary product vehicle. Speculative and research modules (such as distributed cluster replication and legacy hardware drivers) are classified as research/prototype and decoupled from the main bootable path.
- **Consequence**: Focuses testing and implementation on the boot → desktop → package → update loop.

## ADR-002: Zero External Crates Policy in Cargo.toml
- **Context**: Relying on thousands of transitive crates creates supply-chain risk and binary bloat.
- **Decision**: Keep `[dependencies]` in `Cargo.toml` completely empty. Implement data structures, crypto, protocols, and solvers in-tree.
- **Consequence**: All core components remain 100% sovereign, auditable, and self-contained.

## ADR-003: Hybrid `std` Userspace with Freestanding `#![no_std]` Core
- **Context**: Strict `#![no_std]` across all desktop GUI tools and package managers creates unnecessary reinvented wheel friction, whereas a full `std` dependency breaks microkernel bare-metal targets.
- **Decision**:
  - Low-level kernel primitives (`src/kernel/`, `src/klib/`) remain `#![no_std]` compatible.
  - Higher-level desktop components (`src/compositor/`, `src/sigpkg/`, `src/distro/`) leverage standard library collections (`BTreeMap`, `Vec`, `String`).
- **Consequence**: Enables fast desktop iteration while preserving the clean kernel abstraction boundary.

## ADR-004: Declarative Atomic State Management
- **Context**: Imperative package managers (like apt) suffer from broken dependencies and partial upgrade crashes.
- **Decision**: Adopt a content-addressed Merkle Store (`src/sigpkg/merkle_store.rs`) paired with declarative generation pinning.
- **Consequence**: Every update generates a distinct generation hash, enabling instantaneous and fail-safe rollbacks.
