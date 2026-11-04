# SigmaOS Process Manager Master Architecture & Support Matrix Plan

## Design and Safety Rules
- Separate kernel lifecycle from service policy
- Reap every child
- Avoid PID-reuse races in management
- Bound service recovery
- Keep service dependencies declarative and cycle-checked

## Proposed PR Sequence
19-PR structured sequence roadmap.

## Recommended First Milestone
Delivery of baseline PID allocation and child reaping.

## Periodic Maintenance PR Cadence
Scheduled maintenance and regression runs.

## Component-Specific Rules
Process table bounds, signal delivery safety, and service manager supervision.

## PR Template and Merge Gates
Review checklist, tests, and evidence proof.

## Completion Gates
- Prototype
- Integrated
- Supported
