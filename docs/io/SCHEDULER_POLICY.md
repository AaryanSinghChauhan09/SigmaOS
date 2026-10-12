# I/O Scheduler Policy Specification

## Lifecycle
- Submit: Validates request parameters and assigns monotonic ID.
- Queue: Priority ordering (RealTime > BestEffort > Idle).
- Dispatch: Moves request to Active table in Execute stage.
- Completion: Traverses completion stages and clears active descriptor.
