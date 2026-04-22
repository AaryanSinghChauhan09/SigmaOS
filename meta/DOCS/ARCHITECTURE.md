# SigmaOS Architecture, Contributing & Roadmap

## Architecture

SigmaOS is organized into the following top-level modules:

```
SigmaOS/
├── core/orchestrator/   # Rust — shard registry, event bus, C FFI
├── shards/              # Independent Rust/C shard suites
│   ├── virtualization/  # WASM JIT + VirtIO (Rust)
│   ├── security/        # Zero-trust, crypto, firewall (Rust)
│   ├── wasm/            # Cross-language WASM bridge (Rust)
│   ├── sync/            # GitHub + local sync (Rust)
│   └── automation/      # CI triggers, daemon (Rust)
├── kernel/suites/       # C11 bare-metal suites S01–S33
├── cli/ (sigmactl.py)   # Unified CLI — build/sync/shard/profile
├── gui/ (web_ui/)       # Zenith Dashboard — mirrors CLI
├── automation/          # CI/CD workflows and build scripts
└── profiles/            # Personalization JSON templates
```

## Roadmap

### v1.1 (Next)
- [ ] Full Ed25519 in security shard
- [ ] WASM JIT pipeline (Phase 2)
- [ ] GUI plugin marketplace UI

### v1.2
- [ ] Real UEFI boot via S04_HAL
- [ ] VirtIO block + net drivers
- [ ] cargo install sigmaos (binary release)

### v2.0
- [ ] Distributed lattice nodes (mesh sync)
- [ ] Neural predictive scheduler
- [ ] Full WASI capability enforcement

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for coding standards and PR workflow.
