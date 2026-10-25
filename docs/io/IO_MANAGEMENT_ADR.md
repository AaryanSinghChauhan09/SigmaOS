# ADR: SigmaOS Subsystem I/O Management Architecture

## Context and Problem Statement
SigmaOS requires a unified, safe, recoverable, and observable I/O subsystem. Previous ad-hoc I/O experiments (e.g. `io_uring`, `eventfd`, `fd_table`, `ioctl`, `pipe`, `socket`) lacked an integrated lifecycle manager enforcing strict contracts across interrupts, DMA transfers, buffer caching, and multi-queue scheduling.

## Decision Drivers
- Need for zero-leak, zero-race, zero-corruption I/O handling under stress.
- Requirement for observable OOM metrics and bounded resource allocation.
- Support for modern Linux/BSD async patterns (`io_uring_v2`, VirtIO, NVMe).

## Considered Options
1. Model-only decoupled I/O modules without unified lifecycle tracking.
2. Authoritative 10-stage lifecycle coordinator (`IoScheduler`, `InterruptManager`, `DmaManager`, `CallbackManager`, `BufferHead`).

## Decision Outcome
Chosen Option 2: Implement an authoritative 10-stage lifecycle contract:
```text
submit -> validate -> enqueue -> execute -> interrupt/DMA ->
complete -> callback -> reclaim -> free -> error/recovery
```

### Consequences
- All I/O operations execute with bounded metadata, typed errors, and explicit state transitions.
- High memory safety with zero-allocation guarantees in critical interrupt contexts where required.
- Full compliance with `IO_RULES.toml` and automated rehearsal/audit verification.
