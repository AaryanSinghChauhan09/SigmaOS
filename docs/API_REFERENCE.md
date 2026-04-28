# Σ SIGMAOS SOVEREIGN API REFERENCE (v20.0)

This document provides a comprehensive technical reference for the **SigmaOS Micro-Shard API**. All functions listed here are zero-dependency primitives engineered for bare-metal silicon control.

---

## 🧩 SOVEREIGN LIBC (ZENITH)

The Sovereign LibC is a modular, zero-dependency implementation of essential C primitives, optimized for direct hardware interaction and memory sharding.

### 📡 I/O PRIMITIVES
*   `sigma_printf(const char* format, ...)`: Industrial-grade formatted output with support for `%s`, `%d`, `%u`, `%llu`, `%llx`, `%p`, and improved `%f` (bare-metal float handling).
*   `sigma_print(const char* str)`: Direct syscall-backed string print.
*   `sigma_log(const char* msg)`: Labeled system logging shard.

### 🧵 STRING PRIMITIVES
*   `sigma_strlen(const char* s)`: ASM-optimized string length calculation.
*   `sigma_streq(const char* s1, const char* s2)`: Sovereign string equality check.
*   `sigma_atoi(const char* s)`: String to integer conversion shard.

### 🧠 MEMORY MANAGEMENT
*   `sigma_malloc(sigma_size_t size)`: Bump-pointer slab allocator (128 MB default shard).
*   `sigma_free(void* ptr)`: Per-process shard cleanup (no-op by design for speed).

---

## 🧩 CORE KERNEL API

### `kprintf(const char* fmt, ...)`
Industrial-grade formatted output to the primary VGA buffer and serial UART.
- **Parameters**: `fmt` (Format string), `...` (Arguments).
- **Mode**: Sovereign Sync.

---

## 📡 INTER-PROCESS COMMUNICATION (IPC)

### `ipc_send(u64 receiver, u32 type, void* data, u32 size)`
Sends an asynchronous message to a target micro-shard.
- **receiver**: Target shard ID.
- **type**: Message type identifier.
- **data**: Pointer to message payload.
- **size**: Payload size (max 256 bytes).

---

## 💾 STORAGE & FILESYSTEM

### `summon_shard(const char* name, void* buffer, u32 size)`
Linguistically summons a dormant shard from the filesystem into the active lattice.

---

## 🔒 SECURITY & TRUST

### `secure_boot_verify(void* shard_data, u32 size, shard_sig_t* sig)`
Performs a cryptographic audit of a shard binary before activation.

### `rbac_check(u64 shard_id, bool_t net_req, bool_t fs_req)`
Enforces zero-trust capability grants at the shard level.
