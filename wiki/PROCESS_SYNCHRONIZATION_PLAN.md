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
PR 1: docs(sync): inventory primitives and define subsystem boundaries
PR 2: docs(sync): define synchronization contracts and lock-context rules

## Periodic Maintenance PR Cadence
- Regular race condition stress tests

## Component-Specific Rules
- Atomic boundaries

## PR Review Template and Merge Gates
- CI verification

## Completion Gates
- Prototype: Basic locks
- Integrated: Futex and eventfd
- Supported: Full production tier verification
