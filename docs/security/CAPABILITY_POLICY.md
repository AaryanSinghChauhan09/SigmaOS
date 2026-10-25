# SigmaOS Capability Token Policy

This document defines the capability token policy, privilege elevation rules, capability masks, and handle restriction models in SigmaOS.

## Capability Token Classification

Capabilities in SigmaOS are unforgeable, 64-bit bitmasks associated with process control blocks.

```rust
pub struct CapabilityToken {
    pub token_id: u64,
    pub permissions: CapabilityMask,
}

pub flags CapabilityMask: u32 {
    const CAP_READ          = 1 << 0;
    const CAP_WRITE         = 1 << 1;
    const CAP_EXECUTE       = 1 << 2;
    const CAP_NETWORK_BIND  = 1 << 3;
    const CAP_RAW_DISK      = 1 << 4;
    const CAP_MOUNT         = 1 << 5;
    const CAP_DEBUG_PROCESS = 1 << 6;
}
```

---

## Capability Lifecycle Rules

1. **Inheritance & Reduction**: Child processes inherit parent capability masks, but may only reduce capability sets. Capabilities cannot be elevated across `fork()` or `exec()`.
2. **Handle Restriction**: File descriptors and process handles bind specific capabilities at open time.
3. **Privilege Dropping**: Processes dropping root privileges revoke `CAP_RAW_DISK` and `CAP_MOUNT` permanently for the lifetime of that process execution context.
