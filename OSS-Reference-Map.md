# SigmaOS OSS Reference Map

> Canonical source: [docs/OSS_Reference_Map.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/OSS_Reference_Map.md)

This page summarises the open-source projects SigmaOS draws **architectural inspiration**
from — cleanroom reimplementation only, no GPL code copied.

---

## Key References by Subsystem

| Subsystem | Study | What |
|---|---|---|
| Capability security | seL4, Fuchsia/Zircon | Capability model, IPC fastpath |
| Package manager | Nix/Guix | Content-addressed store, reproducible builds |
| Compositor | Smithay (Rust, MIT) | Wayland-inspired compositor framework |
| TCP/IP | smoltcp (Rust, MIT) | `no_std` TCP/IP stack |
| WASM isolation | Wasmtime, WasmEdge | Component model, WASI capability boundary |
| PQC correctness | liboqs (MIT) | Test vectors for Kyber-1024/Dilithium-5 |
| MicroVM | Firecracker (Apache 2) | Jailer + minimal device model |
| Installer | Alpine setup-alpine | Minimal question-answer installer |
| CoW filesystem | btrfs/ZFS | Snapshot/subvolume semantics |
| AI inference | llama.cpp, ONNX RT | GGUF format, execution provider abstraction |

See the [full reference map](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/OSS_Reference_Map.md)
for every subsystem with detailed notes on cleanroom compliance.

---

*See also: [CANONICAL_CLEANROOM_ABSORPTION](CANONICAL_CLEANROOM_ABSORPTION) · [Open-Source-Drivers](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/Open_Source_Drivers.md)*
