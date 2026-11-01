# SigmaOS Kernel Test Taxonomy

## Overview

The Kernel Test Taxonomy defines 7 distinct testing tiers required to validate the safety, performance, resilience, and integration of the SigmaOS kernel codebase.

## 7 Tiers of Testing

| Test Tier | Purpose | Required For |
|---|---|---|
| **Unit** | Pure logic and data structures validation | All kernel modules |
| **Property** | Invariants under randomly generated inputs | Allocators, VFS, syscall argument parsing |
| **Integration** | Cross-subsystem interaction and state consistency | Scheduler, memory, VFS, syscall handlers |
| **Boot Smoke** | Firmware-to-shell end-to-end boot validation | All boot-affecting changes |
| **Fuzzing** | Malformed input resilience and security auditing | Parsers, ELF loaders, drivers, syscall inputs |
| **Stress / Race** | Resource exhaustion and concurrency lock ordering | Memory allocators, IPC, networking, scheduler |
| **Fault Injection** | Failure recovery behavior under synthetic errors | Storage, swap, installer, drivers |
