# Sovereign SDKs

Comprehensive SDKs tailored for IoT, HPC, Cloud, and general SigmaOS development.

## Overview

The SigmaOS SDK ecosystem provides language-specific bindings, development tools, and libraries for building applications on SigmaOS across different deployment profiles and use cases.

## Available SDKs

### 1. Rust SDK (`sigma-sdk-rust`)

The primary SDK for kernel and userland development in Rust:

- **Kernel Development**: Zero-allocation kernel module development with `no_std` support
- **Syscall Bindings**: Type-safe syscall wrappers with capability-based access control
- **Driver Development**: OOP-based driver framework with trait abstractions
- **Async Runtime**: Sovereign async runtime compatible with SigmaOS scheduler
- **Memory Management**: Custom allocators respecting SigmaOS memory policies

```rust
use sigma_sdk::kernel::{Syscall, Capability};
use sigma_sdk::driver::{Device, Driver};

#[derive(Device)]
struct MyDevice {
    capability: Capability,
}

impl Driver for MyDevice {
    fn init(&mut self) -> Result<(), SigmaError> {
        // Driver initialization
        Ok(())
    }
}
```

### 2. C/C++ SDK (`sigma-sdk-c`)

C and C++ development kit for legacy codebases and performance-critical applications:

- **Freestanding Headers**: Headers designed for `-ffreestanding` compilation
- **POSIX Compatibility**: Optional POSIX compatibility layer for porting
- **Kernel Headers**: C-compatible kernel headers for low-level development
- **Build Integration**: CMake and Makefile integration with SigmaCC toolchain
- **Runtime Library**: Minimal runtime library for C applications

```c
#include <sigma/kernel.h>
#include <sigma/syscall.h>

int main() {
    sigma_capability_t cap = sigma_acquire_capability(SIGMA_CAP_FS_READ);
    sigma_fd_t fd = sigma_open("/etc/config", O_RDONLY, cap);
    // ... application code
    sigma_release_capability(cap);
    return 0;
}
```

### 3. Python SDK (`sigma-sdk-python`)

Python bindings for rapid application development and scripting:

- **Syscall Bindings**: Python wrappers for SigmaOS syscalls
- **AI Integration**: Native integration with SigmaAI for ML applications
- **Package Management**: Python package manager integration with sigma-pkg
- **Data Science**: NumPy/Pandas-compatible data science libraries
- **Automation**: Workflow automation and scripting support

```python
from sigma import kernel, ai, workflow

@workflow.schedule(daily="06:00")
def backup_task():
    kernel.fs.backup("/home", "/backup")
    ai.log_completion("Backup completed")

if __name__ == "__main__":
    backup_task()
```

### 4. Nim SDK (`sigma-sdk-nim`)

Nim SDK for high-performance systems programming:

- **Kernel Modules**: Nim kernel module development with zero-cost abstractions
- **Agent Development**: SDK for sigma-agent and AI assistant development
- **Metaprogramming**: Compile-time metaprogramming for code generation
- **FFI Integration**: Seamless FFI with C and Rust components
- **Performance**: Optimized for performance-critical applications

```nim
import sigma/kernel
import sigma/agent

proc myTask*(ctx: Context): Result[void] =
  let cap = ctx.acquireCapability(CapFSRead)
  defer: ctx.releaseCapability(cap)
  ctx.fs.open("/etc/config", O_RDONLY, cap)
  return ok()
```

### 5. WebAssembly SDK (`sigma-sdk-wasm`)

WASM runtime and SDK for web and cross-platform applications:

- **WASI Compatibility**: WASI-compatible syscall layer
- **Sandboxing**: Capability-based sandboxing for WASM modules
- **Runtime**: Lightweight WASM runtime integrated with SigmaOS
- **Toolchain**: wasm32-sigma target for SigmaOS WASM compilation
- **Interop**: Seamless interoperability with native SigmaOS components

### 6. IoT SDK (`sigma-sdk-iot`)

Specialized SDK for embedded and IoT devices:

- **Microcontroller Support**: ARM Cortex-M, RISC-V, ESP32 support
- **Resource Constrained**: Optimized for memory and power-constrained devices
- **Sensor Integration**: Common sensor libraries and drivers
- **Connectivity**: LoRaWAN, BLE, WiFi, and cellular connectivity
- **OTA Updates**: Over-the-air update mechanism for IoT devices

### 7. HPC SDK (`sigma-sdk-hpc`)

High-performance computing SDK for scientific and technical computing:

- **MPI Implementation**: SigmaOS-native MPI implementation
- **GPU Computing**: CUDA and ROCm integration for GPU acceleration
- **Numerical Libraries**: Optimized BLAS, LAPACK, and FFT libraries
- **Job Scheduling**: HPC job scheduler integration with SigmaOS scheduler
- **Cluster Management**: Tools for managing SigmaOS HPC clusters

### 8. Cloud SDK (`sigma-sdk-cloud`)

Cloud-native development SDK:

- **Container Runtime**: SDK for developing SigmaOS containers
- **Orchestration**: Kubernetes-compatible orchestration tools
- **Service Mesh**: Service mesh implementation for microservices
- **Serverless**: Serverless computing framework on SigmaOS
- **Storage**: Cloud storage integration and abstraction layer

## Installation

### Rust SDK

```bash
# Install via sigma-pkg
sigma-pkg install sigma-sdk-rust

# Or build from source
cd sdk/rust
cargo build --release
cargo install --path .
```

### C/C++ SDK

```bash
# Install via sigma-pkg
sigma-pkg install sigma-sdk-c

# Or build from source
cd sdk/c
mkdir build && cd build
cmake ..
make install
```

### Python SDK

```bash
# Install via sigma-pkg
sigma-pkg install sigma-sdk-python

# Or via pip (with SigmaOS Python runtime)
pip install sigma-sdk
```

### Other SDKs

```bash
# Install any SDK
sigma-pkg install sigma-sdk-<language>

# List available SDKs
sigma-pkg search sdk
```

## Documentation

### Rust SDK
- [Getting Started](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Rust-SDK-Getting-Started)
- [Kernel Development](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Rust-Kernel-Development)
- [Driver Development](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Rust-Driver-Development)
- [API Reference](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Rust-API-Reference)

### C/C++ SDK
- [Getting Started](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/C-SDK-Getting-Started)
- [Freestanding Development](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Freestanding-C-Development)
- [POSIX Compatibility](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/C-POSIX-Compatibility)
- [API Reference](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/C-API-Reference)

### Other SDKs
- [Python SDK](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Python-SDK)
- [Nim SDK](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Nim-SDK)
- [WASM SDK](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/WASM-SDK)
- [IoT SDK](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/IoT-SDK)
- [HPC SDK](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/HPC-SDK)
- [Cloud SDK](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Cloud-SDK)

## Examples

### Kernel Module (Rust)

```rust
// examples/kernel_module.rs
use sigma_sdk::kernel::{Module, Syscall, Capability};

#[derive(Module)]
struct MyKernelModule {
    capability: Capability,
}

impl MyKernelModule {
    pub fn new() -> Result<Self, SigmaError> {
        let cap = Syscall::acquire_capability(SIGMA_CAP_DRIVER)?;
        Ok(Self { capability: cap })
    }
}
```

### Userland Application (C)

```c
// examples/user_app.c
#include <sigma/userland.h>
#include <sigma/fs.h>

int main() {
    sigma_init();
    sigma_capability_t cap = sigma_acquire_capability(SIGMA_CAP_FS_READ);
    sigma_file_t *file = sigma_fs_open("/etc/config", O_RDONLY, cap);
    // ... application logic
    sigma_fs_close(file);
    sigma_release_capability(cap);
    return 0;
}
```

### AI Application (Python)

```python
# examples/ai_app.py
from sigma import ai, kernel

def main():
    model = ai.load_model("phi-3")
    result = model.inference("Hello, SigmaOS!")
    kernel.log.info(f"Result: {result}")

if __name__ == "__main__":
    main()
```

## Tooling

### sigma-build

Unified build tool for all SDKs:

```bash
# Build project with automatic SDK detection
sigma-build

# Cross-compile for different architectures
sigma-build --target aarch64-sigma

# Release build with optimizations
sigma-build --release
```

### sigma-test

Testing framework for SDK applications:

```bash
# Run tests
sigma-test

# Run with coverage
sigma-test --coverage

# Run specific test
sigma-test --test my_test
```

### sigma-package

Package management for SDK applications:

```bash
# Create package
sigma-package create

# Build package
sigma-package build

# Publish to registry
sigma-package publish
```

## Roadmap

- [ ] Go SDK for cloud-native applications
- [ ] Julia SDK for scientific computing
- [ ] Swift SDK for Apple ecosystem integration
- [ ] Kotlin SDK for Android compatibility
- [ ] Zig SDK for systems programming
- [ ] Unified SDK documentation portal
- [ ] SDK versioning and compatibility matrix
- [ ] Automated SDK testing across all profiles
- [ ] SDK migration guides from other OSes
- [ ] Performance benchmarking suite for all SDKs

## Contributing

When contributing to SDKs:

1. Follow the SigmaOS coding standards
2. Ensure zero-allocation where applicable
3. Add comprehensive documentation
4. Include examples for all features
5. Test across all supported profiles
6. Maintain backward compatibility

## Support

- [SDK Issues](https://github.com/AaryanSinghChauhan09/SigmaOS/issues)
- [SDK Discussions](https://github.com/AaryanSinghChauhan09/SigmaOS/discussions)
- [Documentation](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki)
