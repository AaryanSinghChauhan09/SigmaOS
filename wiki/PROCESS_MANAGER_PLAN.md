# SigmaOS Process Manager Master Architecture & Support Matrix Plan

## Design and Safety Rules
- Separate kernel lifecycle from service policy
- Reap every child
- Avoid PID-reuse races in management
- Bound service recovery
- Keep service dependencies declarative and cycle-checked

## Proposed PR Sequence
1. Core process lifecycle state machine
2. PID allocator isolation
3. Reparenting and orphan adoption
4. Service supervisor engine

## Recommended First Milestone
- Basic fork/exec and waitpid test suites

## Periodic Maintenance PR Cadence
- Monthly process supervisor stress tests

## Component-Specific Rules
- Deterministic process table bounds

## PR Template and Merge Gates
- CI pipeline gating

## Completion Gates
- Prototype: Basic fork/exec
- Integrated: Service supervisor functioning
- Supported: Full production tier verification
