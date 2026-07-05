# Σ ext/plugins — Sovereign Plugin & Extension Framework

Allows third-party capabilities to be injected into SigmaOS as **cryptographically
verified, capability-gated capsules** without rebuilding the kernel.

## Source Files

| File | Description |
|---|---|
| `extension_api.rs` | Public trait definitions for all plugin types |
| `capsule.rs` | Capsule packaging: sign, verify, load, unload |
| `auto_driver_builder.rs` | AI-assisted driver scaffolding from hardware IDs |
| `policy_modules.rs` | Runtime policy injection (MAC rules, firewall) |

## Plugin Capsule Format

A capsule is a signed tar-like archive:

```
capsule.shard
├── manifest.json   # name, version, capabilities, author, signature
├── code.wasm       # or code.elf (sovereign ELF)
└── policy.sigma    # optional MAC policy additions
```

## Extension API

```rust
pub trait SigmaPlugin: Send + Sync {
    fn name(&self) -> &'static str;
    fn init(&mut self, ctx: &mut PluginContext) -> SigmaResult<()>;
    fn shutdown(&mut self) -> SigmaResult<()>;
}

// Register a plugin at runtime
pub fn plugin_register(capsule_path: &str) -> PluginId;

// Query loaded plugins
pub fn plugin_list() -> &[PluginInfo];

// Unload a plugin cleanly
pub fn plugin_unload(id: PluginId) -> SigmaResult<()>;
```

## Security Model

1. Every capsule must carry an **Ed25519 signature** from a key in the Sovereign
   Trust Root.
2. The kernel validates the signature before mapping any code page.
3. Capabilities are enforced at IPC call boundaries — a plugin cannot exceed its
   declared capability set.

## Roadmap

- [x] Extension API trait (`extension_api.rs`)
- [x] Capsule signing & verification (`capsule.rs`)
- [x] AI driver scaffolding stub (`auto_driver_builder.rs`)
- [x] Policy module injection (`policy_modules.rs`)
- [ ] WASM capsule JIT execution (Cranelift backend)
- [ ] Hot-reload (swap capsule version without reboot)
- [ ] Capsule sandboxing via `modules/tools/sandbox`
- [ ] GUI app-store frontend for capsule discovery

## Related Modules

- [`modules/tools/sandbox`](../../tools/sandbox/README.md) — Capsule isolation
- [`modules/security/access_control`](../../security/access_control/README.md) — Capability enforcement
