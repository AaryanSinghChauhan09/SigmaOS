# SigmaOS Developer SDK Specification

## Overview
The SigmaOS Developer SDK provides a complete set of systems debugging and performance profiling utilities integrated natively with the OS runtime. GDB, `perf`, and LTTng are pre-packaged, along with a reproducible cross-compilation toolchain resembling Buildroot/Yocto, allowing developers to target SigmaOS from any build host.

## Development Workflow
```
 [Build Host (Linux/macOS)] ──► [sig-sdk Cross Toolchain]
                                         │
                                         ▼
   [Staged ISO Image] ◄──────────────────┘
         │
         ▼ (Boot in QEMU)
 [Debugger (GDB) Session] ◄──► [LTTng Telemetry Port]
```

## SDK Properties & Configuration
SDK profiles are declared in `/etc/sigma/sdk.conf`:
```toml
[toolchain]
sysroot = "/usr/share/sigma-sdk/sysroot"
target = "x86_64-sigmaos-elf"
optimization = "O2"

[profiling]
lttng_daemon_port = 5342
enable_perf_events = true
```

## Technical Implementation
Debugging hooks link GDB server directly to the kernel capability interfaces for trace control.

```rust
// userland/sigpkg/sigpkg_core.rs (simulated SDK helper)
pub fn configure_debugging_session(target_pid: u32) -> Result<(), io::Error> {
    // Assert cap_debug capabilities before attaching ptrace
    validate_capability(get_current_task_caps(), CAP_DEBUG)?;
    unsafe {
        ptrace(PTRACE_ATTACH, target_pid, ptr::null_mut(), ptr::null_mut());
    }
    Ok(())
}
```

## Roadmap & Milestones
- **Phase 1 (Months 0-3)**: GCC/Clang and LLVM cross-compilers target configuration.
- **Phase 2 (Months 3-6)**: Porting GDB server onto the SigmaOS userland runtime.
- **Phase 3 (Months 6-9)**: LTTng integration for tracepoint execution monitoring.
- **Phase 4 (Months 9-12)**: Fully automated Yocto-style build farm configuration to produce signed release artifacts.
