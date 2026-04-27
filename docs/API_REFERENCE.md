# Σ SIGMAOS SOVEREIGN API REFERENCE (v1.0)

This document provides a comprehensive technical reference for the **SigmaOS Micro-Shard API**. All functions listed here are zero-dependency primitives engineered for bare-metal silicon control.

---

## 🧩 CORE KERNEL API

### `kprintf(const char* fmt, ...)`
Industrial-grade formatted output to the primary VGA buffer and serial UART.
- **Parameters**: `fmt` (Format string), `...` (Arguments).
- **Mode**: Sovereign Sync.

### `cpu_get_id()`
Retrieves the unique identifier for the current processing core.
- **Returns**: `u32` (CPU ID).

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
- **name**: Shard name.
- **buffer**: Memory destination.
- **size**: Shard binary size.

---

## 🔒 SECURITY & TRUST

### `secure_boot_verify(void* shard_data, u32 size, shard_sig_t* sig)`
Performs a cryptographic audit of a shard binary before activation.
- **Returns**: `bool_t` (TRUE if verified).

### `rbac_check(u64 shard_id, bool_t net_req, bool_t fs_req)`
Enforces zero-trust capability grants at the shard level.
- **Returns**: `bool_t` (TRUE if permitted).
