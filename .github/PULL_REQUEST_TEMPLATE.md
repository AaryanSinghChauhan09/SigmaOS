<!--
## I/O Subsystem Review Checklist
- I/O component: Description of modified I/O component
- I/O policy: Conformance with async I/O policy and bounded latency
- Interrupt manager: Top-half / bottom-half handler registration
- DMA manager / DMA queue: Scatter-gather buffer safety and translation
- Buffer manager: LRU cache and dirty block eviction
- I/O scheduler: Deadline and priority QoS guarantee
- I/O safety review: Failure modes, timeout handling, and rollback plan
-->

## Summary of Changes
Provide a succinct description of the pull request changes.

## Verification
- Standalone tests passing
- Integration tests passing
- CI build checks verified
