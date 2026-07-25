# Developer SDK Guide

SigmaOS includes the `sigma-sdk` CLI tool and debugger hooks for building apps and drivers.

## 1. Project Scaffolding
Use the SDK to scaffold projects of various types:
- `KernelModule`
- `UserApp`
- `StaticLibrary`
- `SharedLibrary`

## 2. Debugging & Telemetry
- Supports remote GDB target attach.
- Profiling traces collect CPU, memory, system calls, context switches, and block I/O statistics.
