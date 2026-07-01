//! capability_table.rs — SigmaOS Sovereign Capability Derivation Forest
//! A sparse, revocation-safe capability derivation tree.
//! Each capability node tracks its parent, children, and access rights.
//! Revoking a node recursively invalidates all derived descendants.
//!
//! Sovereign: #![no_std], zero allocations (static slab).

#![no_std]
#![allow(dead_code)]

use crate::microkernel::{Capability, Rights, ObjType};

// ─── Derivation Tree Node ──────────────────────────────────────────────────────
pub const CAP_TREE_SIZE: usize = 4096;

pub struct CapNode {
    pub cap:      Capability,
    pub rights:   Rights,
    pub parent:   u16,         // 0xFFFF = root
    pub first_child: u16,
    pub next_sibling: u16,
    pub generation: u32,
    pub valid:    bool,
}

impl CapNode {
    const EMPTY: Self = Self {
        cap:          Capability::NULL,
        rights:       Rights::NONE,
        parent:       0xFFFF,
        first_child:  0xFFFF,
        next_sibling: 0xFFFF,
        generation:   0,
        valid:        false,
    };
}

// ─── Capability Table ─────────────────────────────────────────────────────────
pub struct CapabilityTable {
    nodes:      [CapNode; CAP_TREE_SIZE],
    free_head:  u16,     // head of free list
    free_list:  [u16; CAP_TREE_SIZE],
    used:       u16,
}

impl CapabilityTable {
    pub const fn new() -> Self {
        let mut tbl = Self {
            nodes:     [CapNode::EMPTY; CAP_TREE_SIZE],
            free_list: [0u16; CAP_TREE_SIZE],
            free_head: 0,
            used:      0,
        };
        // Initialise free list (done at runtime since const loops are limited)
        tbl
    }

    pub fn init(&mut self) {
        for i in 0..CAP_TREE_SIZE {
            self.free_list[i] = i as u16;
            self.nodes[i]     = CapNode::EMPTY;
        }
        self.free_head = 0;
        self.used      = 0;
    }

    fn alloc_slot(&mut self) -> Option<u16> {
        if self.used as usize >= CAP_TREE_SIZE { return None; }
        let slot = self.free_list[self.free_head as usize];
        self.free_head = (self.free_head + 1) % CAP_TREE_SIZE as u16;
        self.used += 1;
        Some(slot)
    }

    fn free_slot(&mut self, idx: u16) {
        let pos = (self.free_head.wrapping_sub(1)) % CAP_TREE_SIZE as u16;
        self.free_list[pos as usize] = idx;
        self.free_head = pos;
        if self.used > 0 { self.used -= 1; }
    }

    /// Insert a root capability (no parent). Returns slot index or None.
    pub fn insert_root(&mut self, cap: Capability, rights: Rights) -> Option<u16> {
        let slot = self.alloc_slot()?;
        self.nodes[slot as usize] = CapNode {
            cap,
            rights,
            parent:       0xFFFF,
            first_child:  0xFFFF,
            next_sibling: 0xFFFF,
            generation:   0,
            valid:        true,
        };
        Some(slot)
    }

    /// Derive a child capability from `parent_slot` with reduced rights.
    /// Returns new slot or None (invalid parent, insufficient rights).
    pub fn derive(&mut self, parent_slot: u16, new_rights: Rights) -> Option<u16> {
        let p = parent_slot as usize;
        if p >= CAP_TREE_SIZE || !self.nodes[p].valid { return None; }
        // New rights must be subset of parent rights
        if (new_rights.0 & !self.nodes[p].rights.0) != 0 { return None; }
        if !self.nodes[p].rights.has(Rights::GRANT) { return None; }

        let child_slot = self.alloc_slot()?;
        let parent_cap = self.nodes[p].cap;
        let parent_gen = self.nodes[p].generation;

        self.nodes[child_slot as usize] = CapNode {
            cap:          parent_cap,
            rights:       new_rights,
            parent:       parent_slot,
            first_child:  0xFFFF,
            next_sibling: self.nodes[p].first_child,
            generation:   parent_gen,
            valid:        true,
        };
        self.nodes[p].first_child = child_slot;
        Some(child_slot)
    }

    /// Revoke a capability and all its descendants (recursive via iteration).
    pub fn revoke(&mut self, slot: u16) {
        // Iterative post-order traversal using our own stack
        let mut stack  = [0u16; 256];
        let mut sp: usize = 0;
        stack[sp] = slot;
        sp += 1;

        while sp > 0 {
            sp -= 1;
            let cur = stack[sp] as usize;
            if cur >= CAP_TREE_SIZE || !self.nodes[cur].valid { continue; }

            // Push children
            let mut child = self.nodes[cur].first_child;
            while child != 0xFFFF {
                if sp < 255 {
                    stack[sp] = child;
                    sp += 1;
                }
                child = self.nodes[child as usize].next_sibling;
            }

            // Invalidate this node
            self.nodes[cur].valid        = false;
            self.nodes[cur].cap          = Capability::NULL;
            self.nodes[cur].generation  += 1;
            self.free_slot(cur as u16);
        }

        // Remove from parent's child list
        let parent = self.nodes[slot as usize].parent;
        if parent != 0xFFFF && (parent as usize) < CAP_TREE_SIZE {
            let mut sib = self.nodes[parent as usize].first_child;
            if sib == slot {
                self.nodes[parent as usize].first_child = self.nodes[slot as usize].next_sibling;
            } else {
                while sib != 0xFFFF && sib != slot {
                    let next = self.nodes[sib as usize].next_sibling;
                    if next == slot {
                        self.nodes[sib as usize].next_sibling =
                            self.nodes[slot as usize].next_sibling;
                        break;
                    }
                    sib = next;
                }
            }
        }
    }

    /// Lookup: returns the rights associated with a capability, or None if invalid.
    pub fn lookup(&self, slot: u16) -> Option<(Capability, Rights)> {
        let n = &self.nodes[slot as usize];
        if !n.valid { None }
        else        { Some((n.cap, n.rights)) }
    }

    /// O(1) type check on a slot.
    pub fn check_type(&self, slot: u16, expected: ObjType) -> bool {
        let n = &self.nodes[slot as usize];
        n.valid && n.cap.obj_type() == expected
    }
}
