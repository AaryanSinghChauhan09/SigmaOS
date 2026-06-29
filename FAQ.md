# Frequently Asked Questions

---

## General

### What is SigmaOS in one sentence?

SigmaOS is a browser-first operating system where Chromium is the desktop shell, web apps get access to real Unix primitives, and a custom freestanding microkernel handles security and process isolation.

### Is this a Linux distribution?

Partly. The `release/browser` and `release/standalone` profiles run on top of a minimal Buildroot Linux base. The `release/microkernel` profile runs the custom SigmaOS kernel directly on bare hardware with no Linux underneath. All profiles share the same userland code.

### Does it run existing Linux apps?

The POSIX compatibility layer (`userland/compat/`) lets you run existing ELF binaries with minimal modification. Full compatibility is Phase 5 of the roadmap. For now, apps compiled against the sigma SDK run natively; everything else needs the compat shim.

### Can I install it on real hardware today?

Yes — build the ISO and `dd` it to a USB drive. Boot from USB. It works on most x86_64 hardware. WiFi driver support depends on your hardware. See [Building from Source](Building-from-Source) for the full write-to-USB instructions.

---

## Security

### What does `sigma_pledge` actually do?

After a process calls `sigma_pledge(promises)`, the kernel records a bitmask of allowed syscall classes. On every subsequent syscall entry, `sigma_pledge_check()` verifies the syscall is in the allowed set. If not, SIGABRT is sent immediately and the event is logged to the audit ring. The restriction is **irreversible** — a process cannot expand its own pledge promises.

Example: a process that pledged only `stdio | rpath` cannot call `socket()`. Trying to do so triggers SIGABRT before the syscall executes at all.

### What does `sigma_unveil` actually do?

After calling `sigma_unveil_lock()`, any VFS operation on a path NOT in the unveil table returns `-ENOENT`. The file appears not to exist — not "permission denied" but completely invisible. This is stronger than chroot: chroot restricts the root, unveil restricts individual paths.

Example: a process that unveiled only `/tmp:rw` tries to `open("/etc/passwd", O_RDONLY)`. The kernel's VFS layer returns `-ENOENT` before any filesystem driver is consulted.

### Why Kyber-1024 for key exchange and Dilithium3 for signatures?

These are different mathematical problems. Kyber-1024 is a Key Encapsulation Mechanism (KEM) — it lets two parties agree on a shared secret. It cannot produce signatures. Dilithium3 is a signature scheme — it lets you prove a message came from the holder of a private key.

Mixing them up was a real bug in the hypervisor code (fixed in Round 7 / Improvements-Overview). The correct separation:
- **Key exchange**: Kyber-1024 (FIPS 203 / ML-KEM-1024)
- **Signatures**: Dilithium3 (FIPS 204 / ML-DSA-65)

### Is the file encryption (CryptFS) working?

Yes — fixed in Round 13 (Issue #44). The old `derive_key()` was a stub that wrote 32 zero bytes as the AES key (not encrypted at all). The fix uses TPM2 PCR unsealing + HKDF-SHA256 to derive a real AES-256-GCM key from a TPM-sealed master secret. If the boot chain is tampered (PCR values change), the key cannot be unsealed.

Run `sigmactl health` to confirm — `sigma-cryptfs: ok` means real encryption is active.

---

## Networking

### How is the DNS resolver different from using the system resolver?

The SigmaOS DNS resolver (`net/dns/sigma_dns.cpp`) is a full in-process implementation — it doesn't call `getaddrinfo()` or `/etc/resolv.conf`. Benefits:
- DoH (DNS-over-HTTPS) by default — no passive DNS surveillance
- DNSSEC validation with root KSK 2017 trust anchor
- LRU cache with TTL respect
- NXDOMAIN injection for sinkholed malware domains

### Does WPA3 work on real hardware?

The WPA3/SAE code (`net/wifi/sigma_wpa3.cpp`) implements the full dragonfly key exchange (RFC 7664) including hunting-and-pecking, commit/confirm exchange, and EAPOL key processing. The EC P-256 operations use stub implementations pending liboqs / OpenSSL integration. Real hardware support requires driver-level integration with the WiFi chipset firmware.

### What is the hybrid TLS key exchange?

Instead of just X25519 (classical) for the TLS key exchange, SigmaOS uses X25519 **and** Kyber-1024 in parallel. The final shared secret is derived from both. Breaking the connection requires breaking **both** algorithms — classical computer attacks are blocked by X25519; quantum computer attacks are blocked by Kyber-1024.

---

## Development

### How do I add a new daemon?

1. Create `sigmad/mydaemon/main.go`
2. Listen on `/run/sigma/mydaemon.sock` (Unix HTTP)
3. Register with sigma-watchdog (`POST /watchdog/register`)
4. Send heartbeats every 30s (`POST /watchdog/heartbeat`)
5. Emit `/metrics` endpoint for Prometheus scraping
6. Add a dinit service file at `sigma-etc/services/sigma-mydaemon.d`

See `sigmad/healthd/main.go` as a reference implementation.

### How do I add a new kernel subsystem?

1. Create headers in `kernel/<subsystem>/` with SPDX license
2. Add `SIGMA_DTRACE_PROBE` points at key entry/exit paths (zero-cost when disabled)
3. Add compile-time assertions with `SIGMA_BUILD_ASSERT` for struct size contracts
4. Add to `Makefile` stub tracker if any functions are placeholders
5. Add regression tests in `tests/kernel/<subsystem>/`
6. Document in the wiki

### What coding standards does the project use?

- C/C++: `.clang-format` enforces style. `SPDX-License-Identifier: GPL-2.0-or-later` on every file.
- Go: `gofmt` enforced by pre-commit.
- Commits: Conventional Commits format (`feat:`, `fix:`, `docs:`) enforced by `.conform.yaml`.
- No hosted stdlib headers (`<stdio.h>`, `<string.h>`) in kernel code — use `klib/` equivalents.

### How do I run the tests?

```bash
# Check for stubs
make check-stubs

# C++ unit tests (host mode, Google Test)
cd tests/cpp_host && cmake -B build && cmake --build build && cd build && ctest

# Kernel regression tests (OpenBSD regress style)
make -C tests regress

# Memory tests
bash tests/kernel/test_mm.sh

# Network tests
cd tests/net && cmake -B build && cmake --build build && ./build/test_net_stack

# POSIX compliance
bash tests/posix/run_posix_tests.sh

# Visual regression (openQA)
python3 tests/openqa/sigma_visual_test.py
```

---

## Architecture

### Why Go for daemons instead of C/Rust?

Go gives safe memory management (no use-after-free, no buffer overflows in daemon code), built-in goroutine scheduling for concurrent request handling, and easy Unix socket HTTP servers with the standard library. The daemons aren't in the hot path for performance — they handle management operations. C is used where it matters for performance (kernel, drivers, crypto).

### Why not use D-Bus for IPC?

D-Bus has a complex, monolithic implementation with many attack surfaces. sigma-bus (`userland/ipc/sigma_bus.h`) is a capability-gated IPC layer where each message route requires an explicit capability token — a process can't accidentally (or maliciously) reach a daemon it wasn't granted access to.

### What is the sigma-healthd for?

It's the system's conscience. Every subsystem reports its status to healthd. At any time, `sigmactl health` shows exactly which parts are real implementations and which are stubs. This prevents the situation where a security feature appears to work in logs but is actually a no-op (the original CryptFS bug).

---

## Contributing

### I found a stub. How do I report it?

Open an issue labelled `stub` on GitHub. Include the file path and function name. The CI pipeline (`make check-stubs`) maintains a list of known stubs that must be acknowledged before a release build succeeds.

### Can I contribute without knowing kernel programming?

Absolutely. Many high-value contributions don't touch the kernel:
- Go daemon improvements (`sigmad/`)
- Web shell / Zenith Desktop features
- Wiki improvements
- Test coverage
- India-specific professional apps (`userland/apps/sigma-*/`)
- Documentation and examples

### Where do I start?

Read [Building from Source](Building-from-Source), get a build working, run `make check-stubs`, and pick any known stub to implement. The Contributor Roadmap has a prioritised list of open work.

---

*See also: [Architecture Overview](Architecture-Overview) · [Security Model](Security-Model) · [Building from Source](Building-from-Source) · [Contributor Roadmap](Contributor-Roadmap)*
