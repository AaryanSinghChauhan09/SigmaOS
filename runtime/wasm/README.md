# WebAssembly Runtime

A sovereign, libc-free WebAssembly (WASM) interpreter/JIT that allows
cross-platform apps to run sandboxed on SigmaOS.

## Why WASM?
WASM gives SigmaOS a portable app format that is:
1. **Architecture-neutral** — one binary runs on x86_64, ARM64, and RISC-V
2. **Capability-sandboxed** — WASM modules can only access resources they
   have been explicitly granted
3. **POSIX-free** — no libc needed

## Execution Model
```
.wasm module → SigmaWASM validator → JIT compiler → native shard
```

## Roadmap
- [ ] WASM binary validator (MVP spec)
- [ ] Interpreter (for boot, no JIT)
- [ ] Cranelift-based JIT backend
- [ ] WASI (WebAssembly System Interface) sovereign mapping
