# SigmaOS Process Manager Master Architecture & Support Matrix Plan

## Design and Safety Rules
- Separate kernel lifecycle from service policy
- Reap every child
- Avoid PID-reuse races in management
- Bound service recovery
- Keep service dependencies declarative and cycle-checked

## Proposed PR Sequence
1. Core process lifecycle
2. Child reaping
3. Orphan adoption
4. Service dependencies
5. Restart bounds

## Recommended First Milestone
- Milestone 1: Basic supervisor and daemon management.

## Periodic Maintenance PR Cadence
- Regular audit cycles.

## Component-Specific Rules
- Strict DAG checks for services.

## PR Template and Merge Gates
- Verified execution tests.

## Completion Gates
- Prototype, Integrated, Supported.
