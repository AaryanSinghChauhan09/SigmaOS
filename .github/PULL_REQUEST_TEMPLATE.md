# SigmaOS Pull Request Template

## Component
- Target subsystem: (kernel / hal / drivers / fs / net / crypto / zenith_desktop / userland)

## Upstream Study & Reference Study
- Upstream study: Overview of reference implementation (e.g. Linux 6.14+, FreeBSD 15.0+, OpenBSD 7.7+)
- What SigmaOS adopts: Architectural synthesis and zero-dependency absorption strategy

## Safety and rollback
- Risk level: (Low / Medium / High / Critical)
- Safety and rollback: Capability ring invariants, atomic state transitions, and fallback procedures

## Kernel safety review
- Concurrency & locking implications: Mutex, Spinlock, and Lockless atomic invariants
- Capability ring invariants: Zero-trust ring boundaries and sandboxing verification
