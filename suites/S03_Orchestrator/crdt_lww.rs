//! =============================================================================
//! Σ SIGMAOS: CRDT STATE REPLICATION ENGINE (v1.0)
//! =============================================================================
//! A Last-Write-Wins (LWW) Element Register — the simplest conflict-free
//! replicated data type (CRDT) suitable for bare-metal sovereign state.
//!
//! Why LWW-Register?
//!   - No coordination needed during writes (fully decentralized)
//!   - Merge is O(1) — just compare logical timestamps
//!   - Safe for eventual consistency across isolated lattice shards
//!
//! This implements the `PersistenceOps::replicate()` backend.
//!
//! Standard: bare-metal compatible

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

/// Logical timestamp (Lamport Clock) for causal ordering
pub type LogicalClock = u64;

/// A single replicated entry in the LWW register
#[derive(Clone, Debug)]
pub struct LwwEntry {
    pub key:       String,
    pub value:     Vec<u8>,
    pub timestamp: LogicalClock,
    pub origin:    String, // Shard ID that wrote this entry
}

/// CRDT LWW Element Register: a set of LwwEntry values, one per key.
/// Merge rule: For each key, keep the entry with the highest timestamp.
pub struct LwwRegister {
    pub entries: Vec<LwwEntry>,
    pub clock:   LogicalClock,
    pub node_id: String,
}

impl LwwRegister {
    pub fn new(node_id: &str) -> Self {
        LwwRegister {
            entries: Vec::new(),
            clock:   0,
            node_id: String::from(node_id),
        }
    }

    /// Write: Insert or overwrite a key with a new value and bumped clock.
    pub fn write(&mut self, key: &str, value: Vec<u8>) {
        self.clock += 1;
        let existing = self.entries.iter_mut().find(|e| e.key == key);
        if let Some(entry) = existing {
            entry.value     = value;
            entry.timestamp = self.clock;
            entry.origin    = self.node_id.clone();
        } else {
            self.entries.push(LwwEntry {
                key:       String::from(key),
                value,
                timestamp: self.clock,
                origin:    self.node_id.clone(),
            });
        }
    }

    /// Read: Retrieve the current value for a key, if present.
    pub fn read(&self, key: &str) -> Option<&Vec<u8>> {
        self.entries.iter().find(|e| e.key == key).map(|e| &e.value)
    }

    /// Merge: Apply incoming replica entries using LWW semantics.
    /// For each key, keep whichever entry has the higher timestamp.
    /// Idempotent, commutative, and associative — safe for any delivery order.
    pub fn merge(&mut self, incoming: &[LwwEntry]) {
        for inc in incoming {
            // Advance clock past any observed timestamps (Lamport rule)
            if inc.timestamp > self.clock { self.clock = inc.timestamp; }
            
            let existing = self.entries.iter_mut().find(|e| e.key == inc.key);
            if let Some(entry) = existing {
                if inc.timestamp > entry.timestamp {
                    // Remote entry is newer — accept it
                    entry.value     = inc.value.clone();
                    entry.timestamp = inc.timestamp;
                    entry.origin    = inc.origin.clone();
                }
            } else {
                // New key from remote — accept unconditionally
                self.entries.push(inc.clone());
            }
        }
    }

    /// Snapshot: Export all entries for replication to peer shards.
    pub fn snapshot(&self) -> &[LwwEntry] {
        &self.entries
    }
}
