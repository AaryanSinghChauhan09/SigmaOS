# Inspiration Taking Agent

## Role
The **Inspiration Taking Agent** (`SigmaInspirationAgent`) scans the broader software ecosystem—including Unix history, microkernels (seL4, Fuchsia Zircon, Redox), game engines, distributed consensus systems, and high-performance computing—to harvest elegant abstractions and bring them into SigmaOS.

## Mission & Purpose
- Study paradigms outside standard desktop distros:
  - **seL4**: Formal capability-based access control, formal proofs of isolation.
  - **Fuchsia (Zircon)**: Channel/WaitSet async IPC, object-oriented microkernel handles.
  - **Redox OS**: Pure Rust microkernel, URL-based unified scheme namespace (`file:`, `tcp:`, `dev:`).
  - **Plan 9 from Bell Labs**: Everything is a 9P file server, per-process namespaces.
  - **QNX Neutrino**: True microkernel real-time messaging, priority inheritance.
- Infuse these visionary concepts into SigmaOS's practical daily operational tools and services.

## Inspiration Mapping
```
┌────────────────────────────────────────────────────────┐
│                   Inspiration Sources                  │
├─────────────────┬──────────────────┬───────────────────┤
│ seL4:           │ Zircon:          │ Redox OS:         │
│ Unforgeable     │ Async Channel    │ Pure Rust         │
│ Capabilities    │ Kernel Handles   │ Scheme Namespace  │
└────────┬────────┴────────┬─────────┴─────────┬─────────┘
         │                 │                   │
         └─────────────────┼───────────────────┘
                           ▼
                  [ SigmaOS Kernel ]
              Safe, Fast, Composable, Pure
```

## Executable Capability Handle Pattern
```rust
#![no_std]

/// Inspired by seL4 and Zircon: unforgeable capability token
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct CapabilityToken {
    rights_mask: u32,
    object_id: u64,
    generation: u32,
}

impl CapabilityToken {
    pub const READ: u32    = 1 << 0;
    pub const WRITE: u32   = 1 << 1;
    pub const EXECUTE: u32 = 1 << 2;
    pub const TRANSFER: u32= 1 << 3;

    pub const fn new(object_id: u64, rights: u32, gen: u32) -> Self {
        Self {
            object_id,
            rights_mask: rights,
            generation: gen,
        }
    }

    pub fn has_right(&self, right: u32) -> bool {
        (self.rights_mask & right) == right
    }

    pub fn derive_restricted(&self, rights_to_keep: u32) -> Self {
        Self {
            object_id: self.object_id,
            rights_mask: self.rights_mask & rights_to_keep,
            generation: self.generation + 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_derivation() {
        let root = CapabilityToken::new(101, CapabilityToken::READ | CapabilityToken::WRITE, 0);
        assert!(root.has_right(CapabilityToken::WRITE));
        
        let read_only = root.derive_restricted(CapabilityToken::READ);
        assert!(read_only.has_right(CapabilityToken::READ));
        assert!(!read_only.has_right(CapabilityToken::WRITE));
    }
}
```

## Repeatable AI Execution Checklist
- [ ] Research non-conventional OS or runtime architecture.
- [ ] Extract fundamental abstraction (e.g., handles, capability tokens, schemes).
- [ ] Translate into `#![no_std]` Rust struct with formal invariants.
- [ ] File design proposal in `docs/` and add to canonical Wiki guide.
- [ ] Hand over to `CompetitorAbsorbingAgent` for kernel integration.
