# Architecture Decision Records (ADR)

## ADR 001: Pure Safe Rust Kernel & Subsystem Architecture
- **Status:** Accepted
- **Context:** SigmaOS aims for 100% zero-dependency, memory-safe, `#![no_std]` core OS operation.
- **Decision:** All core subsystems are implemented in clean, safe Rust with explicit contracts, zero-allocation UDF bytecodes, and PQC cryptographic assurance.
