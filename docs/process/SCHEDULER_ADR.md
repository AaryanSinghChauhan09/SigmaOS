# Architectural Decision Record: Scheduler Subsystem Lifecycle State Machine

## Decision

SigmaOS models task lifecycle transitions explicitly and keeps queue ownership,
CPU affinity, preemption, and teardown as separate contracts.

## Context

A scheduler combines latency, fairness, wakeup, migration, and cleanup behavior.
Implicit transitions make starvation and use-after-teardown failures difficult
to reproduce.

## Consequences

The lifecycle state machine is the authoritative source for transition validity.
Each policy change requires focused transition tests, queue evidence, and a
documented rollback or downgrade path.

