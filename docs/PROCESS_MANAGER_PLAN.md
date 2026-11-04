# SigmaOS Process Manager Master Architecture & Support Matrix Plan

## Design and Safety Rules
- Separate kernel lifecycle from service policy
- Reap every child
- Avoid PID-reuse races in management
- Bound service recovery
- Keep service dependencies declarative and cycle-checked

## Proposed PR Sequence
Roadmap of process management PRs.

## Recommended First Milestone
Core process supervisor and orphan adoption.

## Periodic Maintenance PR Cadence
Scheduled process management reviews.

## Component-Specific Rules
Detailed rules per component.

## PR Template and Merge Gates
Authoritative merge gates.

## Completion Gates
- Prototype
- Integrated
- Supported
