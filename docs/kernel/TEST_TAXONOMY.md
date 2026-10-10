# Kernel Test Taxonomy

| Test type | Purpose | Required for |
|---|---|---|
| Unit | Pure logic and data structures | All kernel modules |
| Property | Invariants under generated inputs | Allocators, VFS, syscall parsing |
| Integration | Cross-subsystem behavior | Scheduler, memory, VFS, syscalls |
| Boot | Firmware-to-shell behavior | All boot-affecting changes |
| Fuzz | Malformed input resilience | Parsers, loaders, drivers, syscalls |
| Stress | Resource exhaustion and races | Memory, IPC, networking, scheduler |
| Fault injection | Failure and recovery behavior | Storage, swap, installer, drivers |
