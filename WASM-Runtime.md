# WASM Runtime Integration

> **Component**: `kernel/wasm/` | **Status**: ACTIVE

SigmaOS integrates a native WebAssembly (WASM) runtime directly into the kernel, allowing for ultra-fast, securely sandboxed execution of user-provided code without the overhead of a full virtual machine or container.

---

## Architecture

The SigmaOS WASM runtime is built on a `no_std` compatible interpreter (and eventual JIT compiler) that runs within its own Sovereign Shard (`S07_WASM_Runtime`). 

```
┌─────────────────────────────────────────────────────────────┐
│                    SIGMA WASM RUNTIME                       │
│                                                             │
│  ┌───────────────┐  ┌────────────────┐  ┌──────────────┐  │
│  │ WASM Module   │  │ Sandboxed Mem  │  │ WASI Compat  │  │
│  │ Loader        │  │ Allocator      │  │ Layer        │  │
│  └───────┬───────┘  └────────┬───────┘  └──────┬───────┘  │
│          │                   │                  │          │
│  ┌───────┴───────────────────┴──────────────────┴───────┐  │
│  │              SIGMA-BUS DISPATCH TABLE                │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

## Features

1. **Near-Native Performance**: By compiling WASM ahead-of-time (AOT) during package installation, SigmaOS achieves near-native execution speeds for WASM shards.
2. **Strict Sandboxing**: WASM modules execute in a linearly addressed memory space. Buffer overflows or illegal memory accesses are mathematically impossible to escape into the kernel.
3. **WASI Compatibility**: The runtime implements a subset of the WebAssembly System Interface (WASI), mapped directly to `sigma-bus` IPC calls, allowing standard WASM modules (compiled from Rust, C/C++, Go) to run unmodified.
4. **Hot-Swappable Capabilities**: Capabilities (network access, disk access) are injected at runtime via capability handles, adhering to the Sovereign Principles of least-privilege.

## Usage

Running a WASM binary natively from the shell:

```bash
# Execute a WASM module
sigma wasm run /path/to/module.wasm

# Execute with specific capabilities
sigma wasm run --cap-net --cap-fs-read=/tmp /path/to/module.wasm
```

## Integration with Shards

Future iterations of the Sovereign Registry will allow WASM modules to register themselves as full system Shards, enabling developers to write core system services in any WASM-targetable language while retaining the security guarantees of the Sovereign Sandbox.
