# SigmaOS Process Synchronization Master Architecture & PR Proposal Plan

## Repository Context and Scope
Synchronization primitives, wait queues, futexes, and mutexes.

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

## Periodic Maintenance PR Cadence
Cadence.

## Component-Specific Rules
Component rules.

## PR Review Template and Merge Gates
Review template.

## Completion Gates
Completion criteria.
