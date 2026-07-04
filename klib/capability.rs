// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: capability manager - fine-grained capability tokens
//! Hand-rolled zero-dependency implementation, no_std, no pre-defined libraries/functions
//! =========================================================================

#![no_std]

/// Capability type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapabilityType {
    Memory,
    File,
    Device,
    Ipc,
    Scheduler,
}

/// Capability token
#[derive(Debug, Clone, Copy)]
pub struct Capability {
    ctype: CapabilityType,
    object_id: u64,
    permissions: u32,
    valid: bool,
}

impl Capability {
    pub const fn new(ctype: CapabilityType, object_id: u64, permissions: u32) -> Self {
        Self {
            ctype,
            object_id,
            permissions,
            valid: true,
        }
    }

    pub fn invalidate(&mut self) {
        self.valid = false;
    }

    pub fn is_valid(&self) -> bool {
        self.valid
    }

    pub fn has_permission(&self, perm: u32) -> bool {
        self.valid && (self.permissions & perm) == perm
    }
}

/// Capability manager, keeps track of capabilities per process
pub struct CapabilityManager {
    capabilities: [Option<Capability>; 128],
    next_slot: usize,
}

impl CapabilityManager {
    pub const fn new() -> Self {
        Self {
            capabilities: [None; 128],
            next_slot: 0,
        }
    }

    /// Add a new capability, returns slot index or None if full
    pub fn add_capability(&mut self, cap: Capability) -> Option<usize> {
        let mut slot = self.next_slot;
        for _ in 0..128 {
            if self.capabilities[slot].is_none() {
                self.capabilities[slot] = Some(cap);
                self.next_slot = (slot + 1) % 128;
                return Some(slot);
            }
            slot = (slot + 1) % 128;
        }
        None
    }

    /// Get a capability by slot index
    pub fn get_capability(&self, slot: usize) -> Option<&Capability> {
        if slot >= 128 {
            return None;
        }
        self.capabilities[slot].as_ref()
    }

    /// Invalidate a capability
    pub fn invalidate_capability(&mut self, slot: usize) {
        if slot < 128 {
            if let Some(ref mut cap) = self.capabilities[slot] {
                cap.invalidate();
            }
        }
    }
}
