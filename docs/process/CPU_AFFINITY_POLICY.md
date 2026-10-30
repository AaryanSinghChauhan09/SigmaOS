# CPU Affinity Policy Specification

Affinity is an admission and migration constraint, not a replacement for task
lifecycle state. A task may run only on an allowed CPU, and migration updates
queue ownership atomically with the scheduler state.

Invalid masks, offline CPUs, load balancing, and affinity changes during
execution require explicit results. Tests cover rejection, routing, migration,
and SMP synchronization before promotion.

