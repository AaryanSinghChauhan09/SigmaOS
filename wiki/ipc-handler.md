# SigmaOS IPC Handler Master Architecture, Support Matrix, and Program Plan

## Repository Context and Scope
Core inter-process communication mechanisms for SigmaOS.

## Scope and Boundaries
Pipes, Unix domain sockets, message queues, shared memory, and eventfds.

## Periodic Task Cadence
- Regular review cadences.

## Development Phases
- Phase 1: Local primitives
- Phase 2: Descriptor passing
- Phase 3: Zero-copy shared memory

## Missing Components Matrix
- Tracking IPC feature parity with Linux/POSIX.

## Industry-Standard Rules
## Correctness Rules
## Security Rules
## Rust-Specific Rules
## Testing Rules

## First 10 IPC PR Sequence
1. PR 1: Pipes
2. PR 2: Eventfd
3. PR 3: Unix domain sockets
4. PR 4: Message queues
5. PR 5: Shared memory
6. PR 6: Descriptor passing
7. PR 7: Credential passing
8. PR 8: Non-blocking I/O
9. PR 9: Polling and epoll integration
10. PR 10: Security auditing

## Governance and Merge Policy
All IPC changes require verification with `./run_sigma_tests.sh`.
