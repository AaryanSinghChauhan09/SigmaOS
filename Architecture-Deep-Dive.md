# Architecture Deep Dive

SigmaOS is a microkernel-based operating system with a strict capability isolation model, post-quantum cryptography throughout, and a shard-based process model.

## Key Topics

- **Kernel Ring Model** — Rings 0–3, with only the kernel and IRQ handlers in Ring 0
- **Shard Isolation** — Every process is a shard with a capability token, sigma_pledge, and sigma_unveil
- **sigma-bus IPC** — Typed message passing with O(1) capability verification on every message
- **Memory Layout** — Kernel space vs user/shard space, zero-copy shared regions
- **IPC Performance** — Target: < 500 ns round-trip (local), < 2 µs (cross-CPU)
- **Security Chain** — pledge → unveil → AVC O(1) cache → PQC attestation → TPM2

## Full Document

[docs/Architecture_Deep_Dive.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/Architecture_Deep_Dive.md)

## See Also

- [Kernel Internals](Kernel)
- [Security Model Deep Dive](Security-Model-Deep-Dive)
- [Shard Development Guide](Shard-Development-Guide)
