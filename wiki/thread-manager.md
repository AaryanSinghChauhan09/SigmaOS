# SigmaOS Thread Manager Master Architecture, Support Matrix, and Program Plan

## Thread Lifecycle States
Threads transition through explicit lifecycle states:
- `New`
- `Ready`
- `Running`
- `Blocked`
- `Suspended`
- `Zombie`
- `Terminated`

## Engineering Cadence
- Daily: triage and unit verification
- Weekly: integration tests and PR sync
- Biweekly: component review
- Monthly: milestone audit
- Quarterly: architecture roadmap

## Development Phases
- Phase 1: Define the Contract
- Phase 2: Core Thread Object
- Phase 3: Scheduler Integration
- Phase 4: POSIX-Facing Behavior

## Missing Components Deliverable Matrix
Tracking thread manager, priority inheritance, TLS, and scheduling classes.

## First 10 Suggested PR Sequence
1. Thread control block definition
2. Ready queue ring buffer
3. Context switch state save/restore
4. Priority scheduling policy
5. Mutex priority inheritance
6. Sleep and wakeup waitqueues
7. POSIX pthread interface compatibility
8. Thread local storage support
9. SMP CPU affinity mapping
10. Stress test suite and scheduler invariants
