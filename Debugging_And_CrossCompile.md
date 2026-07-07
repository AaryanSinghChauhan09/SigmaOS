# SigmaOS Native Debugging Suite

> GDB + perf + LTTng unified into one SigmaOS tooling experience.

## Overview

The SigmaOS debugging suite integrates three industrial-strength debugging tools — GDB, perf, and LTTng — into a unified interface with AI-assisted stack trace analysis.

## Components

### GDB Integration
- Create debug sessions targeting binaries or running PIDs
- Conditional breakpoints with hit counting
- AI-suggested breakpoint placement around crash sites

### Perf Profiling
- CPU cycle, cache miss, instruction, and branch miss counters
- Configurable sample rate (default: 99 Hz)
- Automatic **flamegraph generation** (SVG output)

### LTTng Kernel Tracing
- Enable kernel tracepoints: `sched_switch`, `irq_handler_entry`, etc.
- Session-based recording with persistent trace storage
- Output to `/var/log/sigma/traces/`

### AI-Assisted Analysis
- **Stack trace analysis**: Detects SIGSEGV, SIGABRT, deadlock patterns
- **Breakpoint suggestions**: Recommends breakpoint locations around crash sites
- Powered by the local `sigma_ai_engine`

## Implementation

- **Source**: `devtools/sigma_debugger.nim`
- **Language**: Nim
- **Key APIs**:
  - `createGdbSession(binary, pid)` — start a debug session
  - `startPerfProfile(pid, duration, sample_rate)` — launch perf recording
  - `createLttngSession(name)` — start kernel tracing
  - `analyzeStackTrace(trace)` — AI root-cause analysis

---

# SigmaOS Cross-Compile Toolchain

> Buildroot/Yocto-style reproducible embedded builds.

## Supported Architectures

| Architecture | Toolchain | Sysroot                                    |
|-------------|-----------|---------------------------------------------|
| x86_64      | GCC       | `/opt/sigma-toolchain/x86_64-linux-musl`    |
| AArch64     | Clang     | `/opt/sigma-toolchain/aarch64-linux-musl`   |
| RISC-V 64   | GCC       | `/opt/sigma-toolchain/riscv64-linux-musl`   |

## Build Profiles

| Profile  | Flags                                            | Use Case        |
|----------|--------------------------------------------------|-----------------|
| Debug    | `-g -O0 -DDEBUG`                                | Development     |
| Release  | `-O2 -DNDEBUG`                                  | Production      |
| MinSize  | `-Os -s -DNDEBUG`                               | Embedded/IoT    |
| Hardened | `-O2 -fstack-protector-all -fPIE -D_FORTIFY_SOURCE=2` | Security |

## Reproducible Builds

- `SOURCE_DATE_EPOCH` override for deterministic timestamps
- Environment clearing to prevent build pollution
- SBOM generation in SPDX 2.3 format

## Implementation

- **Source**: `devtools/sigma_crosscompile.nim`
- **Language**: Nim
