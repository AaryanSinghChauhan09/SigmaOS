# Context Switch Policy Specification

A context switch saves the outgoing execution context, validates the incoming
task state, records CPU ownership, and restores the selected task. Switches
must occur at defined preemption or blocking boundaries and must not bypass
lifecycle validation.

Architecture-specific frame details remain separate from generic scheduler
policy. Tests verify ownership, state transitions, and failure recovery.

