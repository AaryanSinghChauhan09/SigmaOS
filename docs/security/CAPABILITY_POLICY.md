# SigmaOS Capability Policy & Privileged Paths

This policy defines capability tokens and authorization rules for privilege-sensitive kernel mechanisms.

## Capability Token Structure

Capability tokens are 64-bit cryptographically verifiable capability descriptors (`CapabilityToken`):

```rust
pub struct CapabilityToken {
    pub rights: u64,
    pub domain_id: u32,
    pub token_hash: u64,
}
```

## Capability Rights Bitfield

- `CAP_SYS_ADMIN` (`0x0001`): Global system administration (mount, reboot, driver load).
- `CAP_NET_RAW` (`0x0002`): Raw network packet transmission and socket binding.
- `CAP_SETUID` (`0x0004`): Process UID/GID manipulation.
- `CAP_SYS_PTRACE` (`0x0008`): Process debugging and memory inspection.
- `CAP_DEV_IO` (`0x0010`): Direct hardware port and DMA register access.

## Enforcement Points

- **Syscall Gate:** `dispatch_syscall` checks process capability bitmask before invoking privileged handlers.
- **VFS Gate:** Mount and device node creation require `CAP_SYS_ADMIN` or `CAP_DEV_IO`.
- **Network Gate:** Binding sockets below port 1024 requires `CAP_NET_RAW`.
