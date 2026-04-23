# SigmaOS Module Loader, Diagnostics & Sandboxing

## Dynamic Module Loader
Located at `modules/tools/loader/module_loader.c`.

Allows drivers and services to be **hot-swapped without rebooting** the kernel — one of SigmaOS's key USPs over legacy monolithic kernels.

- `module_register()` — Declare a module and its init/cleanup hooks at startup.
- `module_load(name)` — Dynamically initialize and activate a module at runtime.
- `module_unload(name)` — Cleanly tear down a module, freeing all resources.

## Tamper-Proof Audit Logger
Located at `modules/tools/diag/logger.c`.

Every log entry is protected by an **FNV-1a integrity hash**, ensuring any tampering is immediately detectable — surpassing typical Linux `syslog` mechanisms.

- Supports `LOG_DEBUG`, `LOG_INFO`, `LOG_WARN`, `LOG_ERROR`, and `LOG_AUDIT` levels.
- `verify_log_integrity()` — Scans all buffered entries for hash mismatches.
- `trace_syscall()` — Emits an audit-level trace event for every system call.

## Process Sandboxing
Located at `modules/security/isolation/sandbox.c`.

Provides BSD-jail style and Docker-inspired **container isolation** for untrusted processes.

- Configurable policy bitmasks: `SANDBOX_NO_NETWORK`, `SANDBOX_NO_DISK`, `SANDBOX_READ_ONLY_FS`.
- `sandbox_check_syscall()` — Enforced at the syscall dispatcher level to intercept blocked calls.
- `sandbox_check_memory()` — Hard memory page cap per-sandbox; prevents denial-of-service attacks.

## Encrypted Sovereign Network Stack
Located at `modules/core/net/sovereign_net.c`.

Implements **encrypted-by-default packet transmission** — every packet leaving or entering a SigmaOS node is encrypted at the network layer.

- `sovereign_packet_t` — Custom protocol header including nonce + AEAD authentication tag.
- `sovereign_send()` — Encrypts payload before transmission via the underlying socket layer.
- In production: upgrade cipher to **ChaCha20-Poly1305** or **AES-256-GCM**.
