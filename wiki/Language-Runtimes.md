# Σ ext/runtimes — Sovereign Language Runtimes

Hosts sovereign runtimes for languages other than C/Rust that can safely
execute inside SigmaOS user space without requiring the Linux ABI.

## Supported Runtimes

| Runtime | Language | Format | Status |
|---|---|---|---|
| `sigma-wasm` | WebAssembly | `.wasm` | 🔧 In-progress |
| `sigma-luajit` | Lua 5.4 | `.lua` | 📋 Planned |
| `sigma-python` | Python 3.x | `.py` | 📋 Planned |
| `sigma-zig` | Zig | `.zig` ELF | ✅ Native (no shim needed) |

## WebAssembly Runtime (`sigma-wasm`)

SigmaOS's primary portable app format — a sovereign, libc-free WASM interpreter
and JIT that sandboxes apps with hardware capability tokens.

### Execution Model

```
.wasm module
  └─ sigma-wasm validator   (type checking, memory bounds)
        └─ Interpreter      (boot-time, no JIT dependency)
              └─ Cranelift JIT (post-boot, hot path optimisation)
                    └─ Native shard (capability-gated)
```

### WASI Sovereign Mapping

| WASI Syscall | SigmaOS Translation |
|---|---|
| `fd_read` | `sigma_vfs_read()` |
| `fd_write` | `sigma_vfs_write()` |
| `sock_send` | `sigma_net_send()` |
| `clock_time_get` | `hal_get_timestamp_ns()` |

## API Interface

```c
// Load and validate a WASM module
wasm_module_t *sigma_wasm_load(const uint8_t *bytes, size_t len);

// Instantiate with a capability token
wasm_instance_t *sigma_wasm_instantiate(wasm_module_t *m, cap_token_t cap);

// Call an exported function
int64_t sigma_wasm_call(wasm_instance_t *inst, const char *fn, ...);

// Destroy an instance (frees memory)
void sigma_wasm_destroy(wasm_instance_t *inst);
```

## Roadmap

- [ ] WASM binary validator (MVP spec)

- [ ] Stack-machine interpreter (for early boot)

- [ ] Cranelift JIT backend integration

- [ ] WASI → SigmaOS syscall mapping table

- [ ] Lua 5.4 interpreter port (no C stdlib)

- [ ] Python 3 minimal port (for scripting tools)

- [ ] Runtime hot-swap (update runtime without reboot)

## Related Modules

- [`modules/ext/plugins`](../plugins/README.md) — WASM capsule packaging

- [`modules/tools/sandbox`](../../tools/sandbox/README.md) — Runtime sandboxing
