# Secondary Memory Lifecycle Subsystem ADR

## Context
Secondary memory lifecycle management for SigmaOS.

## Decision
Implement decoupled block, cache, and mount managers.

## Consequences
Guarantees race-free I/O and zero lost writebacks.
