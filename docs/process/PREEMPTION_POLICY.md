# Preemption Policy Specification

Preemption is permitted only at safe scheduler boundaries with interrupts and
execution context in a valid state. Hard-interrupt paths request or record
preemption without performing unbounded scheduler work.

Latency targets, disabled-preemption sections, nested requests, and rollback
behavior must be measured or explicitly marked partial. No unsupported
real-time claim is promoted without repeatable evidence.

