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
1. Kernel wait/wakeup primitives
2. Kernel locks and completions
3. Userspace synchronization APIs
4. Process-shared IPC synchronization
5. Concurrency diagnostics

## Periodic Maintenance PR Cadence
Scheduled synchronization audits.

## Component-Specific Rules
Detailed rules for futex, semaphore, and wait queues.

## PR Review Template and Merge Gates
Authoritative merge gates.

## Completion Gates
- Prototype
- Integrated
- Supported
