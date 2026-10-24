# SigmaOS Kernel Test Taxonomy

## Overview
SigmaOS enforces a 7-tier test taxonomy to ensure kernel maintenance, performance, and stability across unit models, property invariants, cross-subsystem integration, boot smoke tests, fuzzing, stress scenarios, and fault injection.

## Test Tier Definitions

| Tier | Test Type | Purpose | Scope & Required Target |
|---|---|---|---|
| Tier 1 | `Unit` | Verifies pure logic, data structure invariants, and error conversions | All kernel structs, allocators, and algorithms |
| Tier 2 | `Property` | Tests invariants under randomly generated valid and invalid inputs | Page tables, path normalization, syscall arg parsing |
| Tier 3 | `Integration` | Verifies cross-subsystem interaction and state transitions | Scheduler + Memory, VFS + Block Cache, Syscalls + Capabilities |
| Tier 4 | `Boot` | Validates firmware-to-shell state machine execution in QEMU | Entire kernel boot path and shell marker verification |
| Tier 5 | `Fuzz` | Tests resilience against untrusted or malformed external inputs | ELF parser, VFS path parser, Network packets, IPC, Syscalls |
| Tier 6 | `Stress` | Verifies behavior under resource exhaustion, high concurrency, and lock contention | Multi-threaded scheduler, OOM allocator stress, lock races |
| Tier 7 | `Fault Injection` | Tests error recovery paths when hardware or memory calls fail | Disk write drop, allocation failure injection, power loss |
