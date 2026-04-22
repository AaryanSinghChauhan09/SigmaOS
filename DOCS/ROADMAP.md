# SigmaOS Roadmap

## Current: v1.0 — Sovereign Lattice Foundation

### ✅ Completed
- 33-suite C11 kernel lattice (S01–S33)
- Rust memory manager (buddy allocator, C FFI)
- Rust security shard (sandbox, crypto, firewall)
- Rust WASM runtime (MVP opcode interpreter)
- Rust sync shard (fetch/push/pull_rebase)
- Rust automation daemon (`sigma-daemon` binary)
- Core orchestrator (heapless shard registry, event bus)
- IPC ring buffer (lock-free SPSC)
- Custom config parser (zero-dep key=value + JSON)
- Lightweight C logger (syscall-direct, serial MMIO)
- `sigmactl` Python CLI (build/shard/sync/profile/status)
- `sigmactl` Rust native binary (via `cli/main.rs`)
- Zenith GUI dashboard (38 JS modules, glassmorphic)
- Settings panel with live CLI reference
- Plugin loader (dynamic JS plugin system)
- Config bridge (GUI ↔ CLI shared state)
- Shard status dashboard (live CPU/mem, restart/kill)
- Rust HTTP backend (zero-dep `std::net` only)
- Frontend components (ShardManager, SyncPanel, Profiles, Status)
- Shared C utility library (`sigma_utils.h`)
- WASM FFI bridge (C ↔ Rust ↔ WASM shared memory)
- Profiles: developer, secure, lightweight, default
- Devcontainer + GitHub Actions smart CI
- Install scripts (bash + PowerShell)
- Integration tests (pytest, 6 test classes)

---

## v1.1 — Security Hardening (Next 4 weeks)

- [ ] Full Ed25519 signature verification in `shards/security`
- [ ] Capability enforcement on all IPC messages
- [ ] Neural Firewall integration into Event Bus listener
- [ ] Encrypted profile storage (XOR cipher from `crypto.c`)
- [ ] Audit log persistence to disk via `sigma_utils` config

## v1.2 — Hardware Finalization (6-8 weeks)

- [ ] Complete `UefiSupport.c` — UEFI GOP framebuffer init
- [ ] VirtIO block device driver (`virtio_blk.c`)
- [ ] VirtIO network driver (`virtio_net.c`)
- [ ] QEMU boot pipeline passing full CI gate
- [ ] Real physical memory map from UEFI `GetMemoryMap`

## v1.3 — Developer Experience (8-10 weeks)

- [ ] `cargo install sigma-cli` one-command Rust binary install
- [ ] Plugin marketplace UI in Zenith dashboard
- [ ] `sigmactl plugin search` fetching from registry
- [ ] GUI build log streaming (SSE from backend)
- [ ] VS Code extension for shard highlighting

## v2.0 — Distributed Lattice (12+ weeks)

- [ ] Mesh sync: distributed SigmaOS nodes via gossip protocol
- [ ] Neural predictive scheduler (ML-guided process priority)
- [ ] Full WASI capability enforcement layer
- [ ] WebAssembly system interface (WASI) compliance
- [ ] Multi-architecture support (ARM64, RISC-V)
- [ ] Self-evolving shard registry (hot-reload without reboot)
