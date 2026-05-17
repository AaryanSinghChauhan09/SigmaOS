# SigmaOS API Documentation

SigmaOS exposes a robust, zero-dependency ABI (Application Binary Interface) called `Z-SYSCALL` for userland applications and dynamically loaded shards to interact with the microkernel.

## Standard Syscalls

The following standard POSIX-like system calls are available:

### `sys_read`

```c
sigma_isize sys_read(sigma_u32 fd, void* buffer, sigma_size_t count);
```

Reads `count` bytes from file descriptor `fd` into `buffer`. Returns bytes read or error.

### `sys_write`

```c
sigma_isize sys_write(sigma_u32 fd, const void* buffer, sigma_size_t count);
```

Writes `count` bytes from `buffer` to file descriptor `fd`. Returns bytes written or error.

### `sys_mmap`

```c
void* sys_mmap(void* addr, sigma_size_t length, int prot, int flags, int fd, sigma_u64 offset);
```

Maps memory directly to the calling shard. Crucial for custom memory allocators.

---

## Shard Extensibility APIs

SigmaOS is designed around modular "Shards". You can load custom functionality into the kernel dynamically.

### `load_shard_module`

```c
sigma_status load_shard_module(sigma_module_t* module);
```

Dynamically links and loads a `sigma_module_t` into the kernel lattice.

#### Parameters

- `module`: A pointer to a valid `sigma_module_t` struct containing the `init_hook` and `exit_hook`.

### `register_device`

```c
sigma_status register_device(const char* device_name, void* operations);
```

Registers a new hardware device with the Sovereign HAL.

---

## Post-Quantum APIs

### `pq_encrypt`

```c
sigma_status pq_encrypt(const void* data, sigma_size_t size, void* out_buffer);
```

Encrypts data using the hardware-accelerated Post-Quantum cryptographic engine.
 