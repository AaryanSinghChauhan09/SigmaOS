# SigmaOS Process Synchronization Master Architecture & PR Proposal Plan

## Core Design Rules
- Name the synchronization domain
- Separate lock from event
- Never sleep in atomic context
- Specify ownership and failure semantics
- Wait on predicates, not notifications
- Prevent lost wakeups
- Use explicit memory ordering
- Address priority inversion and starvation
- Treat user-memory objects as untrusted
- Test races systematically

## Proposed PR Sequence Roadmap
Priority order:
1. Primitives
2. Locks

## Periodic Maintenance PR Cadence
- Regular audit cycles.

## Component-Specific Rules
- Bounded locks.

## PR Review Template and Merge Gates
- Verified execution tests.

## Completion Gates
- Prototype, Integrated, Supported.
