# Capability Enforcement Policy

## Fine-Grained Capability Tokens

In SigmaOS, access to privileged system operations is controlled via granular capabilities:

| Capability Flag | Operation Permitted |
|---|---|
| `CAP_SYS_ADMIN` | System administration, mounting filesystems, sysctl configuration |
| `CAP_NET_RAW` | Opening raw sockets, packet injection, network interface configuration |
| `CAP_SYS_RAWIO` | Direct I/O port access (`inb`/`outb`), MMIO mapping |
| `CAP_SYS_PTRACE` | Process tracing, memory inspection of other processes |
| `CAP_DAC_OVERRIDE` | Bypassing file permission checks |
| `CAP_KILL` | Sending signals to processes owned by other users |

---

## Syscall Entry Enforcement Rule

Every syscall dispatcher entrypoint that modifies system state, reads restricted data, or performs hardware I/O MUST invoke:

```rust
pub fn check_capability(task: &Task, cap: Capability) -> Result<(), SyscallError> {
    if task.capabilities.contains(cap) {
        Ok(())
    } else {
        Err(SyscallError::PermissionDenied)
    }
}
```

Failure to verify capabilities on privileged syscall paths is considered a **Critical Security Bug**.
