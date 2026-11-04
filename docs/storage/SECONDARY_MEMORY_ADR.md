# Secondary Memory Lifecycle Subsystem ADR

## Context
Architecture decision record for Secondary Memory Lifecycle Subsystem.

## Decision
Adopt modular block, cache, and mount pipeline with bounded allocations.

## Consequences
Deterministic lifecycle, race-free unmount and teardown, explicit error recovery.
