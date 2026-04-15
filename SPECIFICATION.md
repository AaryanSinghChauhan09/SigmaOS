# S SIGMAOS: THE SOVEREIGN SPECIFICATION (v2.0)

This document defines the formal technical requirements for any code unit ("Shard") within the SigmaOS Zenith Supreme architecture.

## 1. Zero-Dependency Mandate
- Shards MUST NOT include any headers outside of the `include/` directory.
- Shards MUST NOT link against external libraries (libc, libm, etc.).
- All primitives MUST be provided by the Sovereign LibC.

## 2. Registry-Based Orchestration
- Every Shard MUST be registered via a dedicated Sector Registry (Arch, Memory, FS, etc.).
- Initialization MUST be performed via the Registry callback, not direct calls from PID 0.

## 3. Modular Interface (v2.0)
- Shards MUST implement the `sovereign_shard_interface_t`.
- `initialize()`: Return `SIGMA_OK` upon successful sector seating.
- `self_test()`: Perform internal state audit and return validation status.

## 4. Architectural Purity
- No hardcoded personal or environment-specific paths.
- No global state without atomic protection (TicketLocks).
- PII (Personal Identifiable Information) is strictly forbidden in code and comments.

## 5. Industrial Performance
- Memory allocation MUST be O(1) via the Magazine Slab system.
- Synchronizaton MUST use non-starving Ticket Spinlocks.

---
*By adhering to this specification, SigmaOS remains the world's only freestanding, sharded, and sovereign ecosystem.*
