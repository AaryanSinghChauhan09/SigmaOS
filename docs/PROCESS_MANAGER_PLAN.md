# SigmaOS Process Manager Master Architecture & Support Matrix Plan

## Design and Safety Rules
- Separate kernel lifecycle from service policy
- Reap every child
- Avoid PID-reuse races in management
- Bound service recovery
- Keep service dependencies declarative and cycle-checked

## Proposed PR Sequence
Sequence of pull requests establishing process management.

## Recommended First Milestone
Stabilize process table and reaping semantics.

## Periodic Maintenance PR Cadence
Scheduled process subsystem audits.

## Component-Specific Rules
Lifecycle isolation, signal propagation boundaries.

## PR Template and Merge Gates
Automated process state verification gates.

## Completion Gates
Validation of process reaping and service restart boundaries.
