# SigmaOS Capability Policy & Token Specification

SigmaOS replaces traditional UNIX superuser (`root`) privilege checks with fine-grained, cryptographically validated capability tokens (`CapabilityToken`).

---

## Capability Bitmask Layout

Capability tokens are 64-bit bitmasks passed in process security contexts and syscall arguments.

```text
[63..............................32][31...............................0]
       Reserved / Extension Bits             Core Security Capabilities
```

| Bit | Capability Identifier | Description |
|---|---|---|
| `0` | `CAP_SYS_ADMIN` | Override system resource limits and debug kernel state |
| `1` | `CAP_NET_RAW` | Open raw network sockets and bind to low-numbered ports |
| `2` | `CAP_SYS_RAWIO` | Perform raw MMIO / port IO hardware operations |
| `3` | `CAP_STORAGE_MOUNT` | Perform filesystem mount/unmount and partition operations |
| `4` | `CAP_KILL_PROCESS` | Send termination signals to processes owned by other users |
| `5` | `CAP_SET_SECURITY` | Modify process security context and capability tokens |
| `6` | `CAP_PACKAGE_INSTALL` | Execute package operations and kernel module loading |

---

## Capability Enforcement Rules

1. Every syscall handler that performs privileged operations MUST invoke `verify_capability(caller_ctx, CAP_REQUIRED)`.
2. If capability check fails, syscall MUST return `-EPERM` immediately without mutating kernel state.
3. Capabilities CANNOT be escalated without a set-capability token signed by a trusted system authority.
