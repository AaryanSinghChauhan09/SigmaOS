# Σ tools/loader — Sovereign Module Loader

Handles **dynamic loading, verification, and lifecycle management** of SigmaOS
kernel modules and user-space shards at runtime.

## Source Files

| File | Description |
|---|---|
| `module_loader.rs` | Core loader: ELF/WASM parse, verify, link, execute |

## Loading Pipeline

```
Request: load("sigma-net.shard")
   │
   ├─ 1. Fetch from Sovereign Package Registry (or local path)
   │
   ├─ 2. Verify Ed25519 signature against Trust Root
   │
   ├─ 3. Parse ELF / WASM headers
   │
   ├─ 4. Resolve capability requirements
   │       └─ Query kernel: does caller hold CAP_MODULE_LOAD?
   │
   ├─ 5. Map code into isolated memory domain (IOMMU-backed)
   │
   ├─ 6. Patch relocations + link against sovereign libc
   │
   └─ 7. Call module entry point with capability token
```

## API Interface

```c
// Load and start a module from a path
module_handle_t sigma_module_load(const char *path, cap_token_t caller_caps);

// Unload a module cleanly (calls shutdown hook)
int sigma_module_unload(module_handle_t handle);

// Query a loaded module's exported symbols
void *sigma_module_sym(module_handle_t handle, const char *symbol);

// List all currently loaded modules
int sigma_module_list(module_info_t *out, size_t max_count);

// Verify a module archive (without loading)
int sigma_module_verify(const char *path);

// Initialise the loader subsystem
void init_tools_loader(void);
```

## Module Manifest

Every loadable module ships with `module.json`:

```json
{
  "name": "sigma-net",
  "version": "1.2.0",
  "entry_point": "sigma_net_init",
  "capabilities_required": ["CAP_NET_BIND", "CAP_IRQ_BIND"],
  "capabilities_provided": ["CAP_SOCKET_API"],
  "signature": "<Ed25519 over (name|version|sha256(code))>"
}
```

## Hot-Reload

The loader supports **hot-module replacement** for non-critical modules:

1. Load new version into a shadow domain.
2. Quiesce the old version (drain in-flight requests).
3. Atomically swap the dispatch table pointer.
4. Unload the old version.

## Roadmap

- [x] ELF loader + relocation (`module_loader.rs`)
- [ ] WASM capsule loader (Cranelift JIT)
- [ ] Signature verification integration
- [ ] Hot-reload (shadow domain swap)
- [ ] Dependency graph resolver (topological sort)
- [ ] Module version compatibility checks (semver)

## Related Modules

- [`modules/ext/plugins`](../../ext/plugins/README.md) — Plugin capsule format
- [`modules/security/isolation`](../../security/isolation/README.md) — Module sandboxing
