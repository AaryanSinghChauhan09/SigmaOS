# Upstream Study: NixOS Declarative Configuration & Immutability

1. **Upstream Problem**: Reproducible system builds, generation-based rollbacks, and store path immutability.
2. **Data Structure & Control Flow**: Declarative AST evaluation -> Content-addressed store derivation -> Generation switch.
3. **Security & Failure Assumptions**: Read-only store path prevents configuration drift and unauthorized mutation.
4. **Direct Adoption**: Declarative system configuration schema and generation-based atomic rollbacks.
5. **Rust/SigmaOS Adaptation**: `system.toml` strongly typed configuration schema with generation snapshot state.
6. **Exit Test Criteria**: System configuration switch builds new generation and rolls back cleanly on error.
7. **Licensing & Attribution**: MIT License.
