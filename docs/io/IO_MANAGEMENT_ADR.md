# I/O Management Architecture Decision Record (ADR)

## Context
SigmaOS requires standardized subsystem I/O dispatching and resource limits.

## Decision
Adopt a structured 10-stage lifecycle with bounded interrupt and DMA dispatching.
