# SigmaOS Developer SDK Specification

## Overview

The SigmaOS Developer SDK provides a complete set of systems debugging and performance profiling utilities integrated natively with the OS runtime. GDB, `perf`, and LTTng are pre-packaged, along with a reproducible cross-compilation toolchain resembling Buildroot/Yocto, allowing developers to target SigmaOS from any build host.

### Key Features

- **Cross-Compilation Toolchain**: GCC/Clang cross-compilers for SigmaOS
- **Debugging Tools**: GDB, LLDB integration with kernel debugging
- **Performance Profiling**: perf, LTTng for performance analysis
- **Reproducible Builds**: Yocto-style build farm integration
- **Native API Bindings**: Rust, C, Nim bindings for SigmaOS APIs
- **IDE Integration**: VS Code, IntelliJ plugin support
- **Documentation**: Comprehensive API documentation and examples

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

## SDK Components

### Toolchain

**Cross-Compiler**:
- GCC 12.2.0 with SigmaOS target
- Clang 16.0 with LLVM backend
- Binutils 2.40
- Glibc 2.35
- Kernel headers 6.1.0

**Target Triple**:
- `x86_64-sigmaos-elf`: Bare metal target
- `x86_64-sigmaos-linux-gnu`: Userspace target

### Debugging Tools

**GDB**:
- GDB server for remote debugging
- Kernel debugging support
- Capability-aware debugging
- Thread debugging

**LLDB**:
- LLDB server for remote debugging
- Rust debugging support
- Python scripting

**perf**:
- CPU profiling
- Memory profiling
- Tracepoint support

**LTTng**:
- Kernel tracing
- Userspace tracing
- Event streaming

### Profiling Tools

**perf**:
- CPU cycle profiling
- Cache miss profiling
- Branch prediction profiling

**LTTng**:
- System call tracing
- Function tracing
- Custom tracepoints

**Valgrind**:
- Memory leak detection
- Thread error detection
- Cache profiling

## Configuration

### SDK Configuration

**File**: `/etc/sigma/sdk.conf`

```toml
[toolchain]
sysroot = "/usr/share/sigma-sdk/sysroot"
target = "x86_64-sigmaos-elf"
optimization = "O2"
debug_symbols = true
strip = false

[build]
parallel_jobs = 4
ccache = true
distcc = false

[profiling]
lttng_daemon_port = 5342
enable_perf_events = true
perf_buffer_size = "64MB"

[debugging]
gdb_port = 1234
lldb_port = 1235
enable_kernel_debug = true
```

### Cross-Compilation Setup

**Environment Variables**:
```bash
export SIGMA_SDK_ROOT=/opt/sigmaos-sdk
export PATH=$SIGMA_SDK_ROOT/bin:$PATH
export CC=x86_64-sigmaos-elf-gcc
export CXX=x86_64-sigmaos-elf-g++
export AR=x86_64-sigmaos-elf-ar
export RANLIB=x86_64-sigmaos-elf-ranlib
export STRIP=x86_64-sigmaos-elf-strip
export SYSROOT=$SIGMA_SDK_ROOT/sysroot
```

## Technical Implementation

### Debugging Hooks

```rust
// userland/sigpkg/sigpkg_core.rs (simulated SDK helper)
use libc::{ptrace, PTRACE_ATTACH};
use std::ptr;

pub fn configure_debugging_session(target_pid: u32) -> Result<(), io::Error> {
    // Assert cap_debug capabilities before attaching ptrace
    validate_capability(get_current_task_caps(), CAP_DEBUG)?;
    unsafe {
        ptrace(PTRACE_ATTACH, target_pid as i32, ptr::null_mut(), ptr::null_mut());
    }
    Ok(())
}

pub fn validate_capability(current_mask: u64, required_cap: u64) -> Result<(), CapabilityError> {
    if (current_mask & required_cap) == 0 {
        return Err(CapabilityError::PermissionDenied);
    }
    Ok(())
}
```

### Cross-Compilation Toolchain

```rust
// sdk/toolchain/src/compiler.rs
use std::process::Command;

pub struct CrossCompiler {
    target: String,
    sysroot: String,
}

impl CrossCompiler {
    pub fn new(target: &str, sysroot: &str) -> Self {
        CrossCompiler {
            target: target.to_string(),
            sysroot: sysroot.to_string(),
        }
    }
    
    pub fn compile(&self, source: &Path, output: &Path) -> Result<(), CompileError> {
        let mut cmd = Command::new(format!("{}-gcc", self.target));
        
        cmd.arg("--sysroot").arg(&self.sysroot);
        cmd.arg("-O2");
        cmd.arg("-g"); // Debug symbols
        cmd.arg("-o").arg(output);
        cmd.arg(source);
        
        let output = cmd.output()?;
        
        if !output.status.success() {
            return Err(CompileError::CompilationFailed(
                String::from_utf8_lossy(&output.stderr).to_string()
            ));
        }
        
        Ok(())
    }
}
```

### GDB Server Integration

```rust
// sdk/debugging/src/gdb_server.rs
use std::net::TcpListener;

pub struct GDBServer {
    port: u16,
}

impl GDBServer {
    pub fn new(port: u16) -> Self {
        GDBServer { port }
    }
    
    pub fn start(&self) -> Result<(), GDBError> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.port))?;
        
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.handle_connection(stream)?;
                }
                Err(e) => {
                    eprintln!("Connection error: {}", e);
                }
            }
        }
        
        Ok(())
    }
    
    fn handle_connection(&self, stream: TcpStream) -> Result<(), GDBError> {
        // Handle GDB remote protocol
        let mut reader = BufReader::new(stream.try_clone()?);
        let mut writer = stream;
        
        loop {
            let mut buffer = String::new();
            reader.read_line(&mut buffer)?;
            
            if buffer.starts_with('$') {
                let response = self.process_command(&buffer)?;
                writer.write_all(response.as_bytes())?;
            }
        }
    }
}
```

### LTTng Integration

```rust
// sdk/profiling/src/lttng.rs
use lttng::{Session, Channel, Event};

pub struct LTTngTracer {
    session: Session,
}

impl LTTngTracer {
    pub fn new(name: &str) -> Result<Self, LTTngError> {
        let session = Session::create(name)?;
        Ok(Self { session })
    }
    
    pub fn enable_kernel_events(&mut self, events: &[&str]) -> Result<(), LTTngError> {
        for event in events {
            self.session.enable_kernel_event(event)?;
        }
        Ok(())
    }
    
    pub fn start(&mut self) -> Result<(), LTTngError> {
        self.session.start()?;
        Ok(())
    }
    
    pub fn stop(&mut self) -> Result<(), LTTngError> {
        self.session.stop()?;
        Ok(())
    }
}
```

## API Bindings

### Rust Bindings

```rust
// sdk/bindings/rust/sigmaos-sys/src/lib.rs
use libc::{c_int, c_void};

#[repr(C)]
pub struct SigmaTask {
    pub id: u64,
    pub state: TaskState,
    pub priority: u32,
}

#[repr(C)]
pub enum TaskState {
    Running,
    Ready,
    Blocked,
    Terminated,
}

extern "C" {
    pub fn sigma_create_task() -> *mut SigmaTask;
    pub fn sigma_destroy_task(task: *mut SigmaTask);
    pub fn sigma_schedule_next() -> *mut SigmaTask;
}

pub struct Task {
    inner: *mut SigmaTask,
}

impl Task {
    pub fn new() -> Result<Self, TaskError> {
        let inner = unsafe { sigma_create_task() };
        if inner.is_null() {
            return Err(TaskError::CreationFailed);
        }
        Ok(Task { inner })
    }
    
    pub fn id(&self) -> u64 {
        unsafe { (*self.inner).id }
    }
}

impl Drop for Task {
    fn drop(&mut self) {
        unsafe { sigma_destroy_task(self.inner) };
    }
}
```

### C Bindings

```c
// sdk/bindings/c/sigmaos.h
#ifndef SIGMAOS_H
#define SIGMAOS_H

#include <stdint.h>

typedef enum {
    TASK_RUNNING,
    TASK_READY,
    TASK_BLOCKED,
    TASK_TERMINATED
} TaskState;

typedef struct {
    uint64_t id;
    TaskState state;
    uint32_t priority;
} SigmaTask;

SigmaTask* sigma_create_task(void);
void sigma_destroy_task(SigmaTask* task);
SigmaTask* sigma_schedule_next(void);

#endif // SIGMAOS_H
```

### Nim Bindings

```nim
# sdk/bindings/nim/sigmaos.nim
type
  TaskState = enum
    Running, Ready, Blocked, Terminated
  
  SigmaTask = object
    id: uint64
    state: TaskState
    priority: uint32

proc sigma_create_task(): ptr SigmaTask {.importc, cdecl.}
proc sigma_destroy_task(task: ptr SigmaTask) {.importc, cdecl.}
proc sigma_schedule_next(): ptr SigmaTask {.importc, cdecl.}

type Task = ref object
  inner: ptr SigmaTask

proc newTask(): Task =
  let inner = sigma_create_task()
  if inner == nil:
    raise newException(Exception, "Failed to create task")
  Task(inner: inner)

proc id(task: Task): uint64 =
  task.inner.id
```

## IDE Integration

### VS Code Extension

**Features**:
- Syntax highlighting for SigmaOS config files
- IntelliSense for SigmaOS APIs
- Debug configuration for GDB
- Build task integration

**Configuration**:
```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "name": "SigmaOS Debug",
      "type": "gdb",
      "request": "launch",
      "target": "./target/x86_64-sigmaos-elf/debug/app",
      "gdbpath": "/opt/sigmaos-sdk/bin/x86_64-sigmaos-elf-gdb",
      "remote": true,
      "remoteAddress": "localhost",
      "remotePort": 1234
    }
  ]
}
```

### IntelliJ Plugin

**Features**:
- Project templates for SigmaOS
- Run configurations
- Debugger integration
- Build system integration

## Build Farm Integration

### Yocto-Style Build

**Bitbake Recipe**:
```bitbake
# recipes-kernel/sigmaos-kernel/sigmaos-kernel_6.1.bb
require recipes-kernel/linux/linux-yocto.inc

SRC_URI = "git://github.com/AaryanSinghChauhan09/SigmaOS;branch=main;protocol=https"
SRCREV = "${AUTOREV}"

LINUX_VERSION = "6.1.0"
LINUX_VERSION_EXTENSION = "-sigmaos"

COMPATIBLE_MACHINE = "sigmaos"
```

**Build Configuration**:
```bash
# Initialize build environment
source sigmaos-sdk-init-build-env

# Build kernel
bitbake sigmaos-kernel

# Build SDK
bitbake sigmaos-sdk

# Build image
bitbake sigmaos-image
```

## Best Practices

### Development

1. **Use Cross-Compilation**: Always use the SDK toolchain
2. **Enable Debug Symbols**: Keep debug symbols during development
3. **Test on Target**: Test on actual hardware when possible
4. **Version Control**: Use version control for all code

### Debugging

1. **Use GDB**: Use GDB for interactive debugging
2. **Enable Tracing**: Use LTTng for system-level tracing
3. **Profile Performance**: Use perf for performance profiling
4. **Check Capabilities**: Verify capability permissions

### Building

1. **Reproducible Builds**: Use deterministic build settings
2. **Parallel Builds**: Use parallel compilation
3. **Cache Builds**: Use ccache for faster builds
4. **Clean Builds**: Periodically clean build artifacts

## Roadmap & Milestones

### Phase 1 (Months 0-3)
- GCC/Clang cross-compilers target configuration
- Basic SDK structure
- C and Rust bindings
- GDB integration

### Phase 2 (Months 3-6)
- GDB server porting
- LLDB integration
- perf integration
- Basic profiling support

### Phase 3 (Months 6-9)
- LTTng integration
- Advanced tracing
- IDE plugins
- Build farm integration

### Phase 4 (Months 9-12)
- Yocto-style build farm
- Automated release artifacts
- Advanced debugging features
- Performance optimization tools

## References

- [GDB Documentation](https://www.gnu.org/software/gdb/documentation/)
- [LLDB Documentation](https://lldb.llvm.org/)
- [perf Documentation](https://perf.wiki.kernel.org/)
- [LTTng Documentation](https://lttng.org/docs/)
- [Yocto Project](https://www.yoctoproject.org/)
