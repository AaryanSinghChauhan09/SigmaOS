# OSS Absorption: Wasmtime & WebAssembly — Universal Compute

> **Status**: 🔄 Active | **Source Project**: Wasmtime (Bytecode Alliance) | **Target Shard**: `SigmaOS WebAssembly Runtime`

---

## 1. Executive Summary

WebAssembly (WASM) is a binary instruction format for a stack-based virtual machine, designed as a portable compilation target for programming languages. Wasmtime is a standalone, fast, and secure runtime for WebAssembly built in Rust.

SigmaOS absorbs Wasmtime to provide `sigma-wasm`, a subsystem capable of running sandboxed plugins, serverless functions, and architecture-independent binaries at near-native speeds.

---

## 2. Key Features Absorbed

### 2.1 WASI Integration (WebAssembly System Interface)

Instead of relying purely on POSIX, SigmaOS supports executing WASI-compliant modules natively. This allows developers to write code once (in Rust, C, Go, etc.) and run it securely on any SigmaOS node, regardless of CPU architecture (x86_64, ARM64, RISC-V).

```bash
# Compile a rust program to WASI
$ cargo build --target wasm32-wasi

# Run it natively via sigma-wasm
$ sigma run app.wasm
```

### 2.2 Sandboxed Plugins

Because WASM enforces strict memory isolation, SigmaOS utilizes it for application plugins and kernel extensions (via eBPF-to-WASM bridges) without risking system stability.

```rust
// kernel/plugin/wasm_host.rs
// SPDX-License-Identifier: MIT

pub fn run_untrusted_plugin(wasm_bytes: &[u8]) -> Result<()> {
    let engine = Engine::default();
    let module = Module::from_binary(&engine, wasm_bytes)?;
    
    // Store encapsulates the sandbox state
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[])?;
    
    // Invoke the entry point safely
    let start_func = instance.get_typed_func::<(), ()>(&mut store, "_start")?;
    start_func.call(&mut store, ())?;
    
    Ok(())
}
```

---

## 3. References & Standards

- Wasmtime — `wasmtime.dev` (Apache-2.0)
- WebAssembly System Interface (WASI) — `wasi.dev`
