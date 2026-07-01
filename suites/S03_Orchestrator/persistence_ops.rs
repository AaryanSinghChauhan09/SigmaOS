//! =============================================================================
//! Σ SIGMAOS: DECENTRALIZED PERSISTENCE LAYER (v1.0)
//! =============================================================================
//! Abstract contracts for Decentralized Persistence.
//! Defines the interface between SigmaOS shards and storage backends (CRDT,
//! in-memory, disk) to ensure durability and fault-tolerance across the lattice.
//!
//! Standard: bare-metal compatible
//! =============================================================================

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub type StateKey = String;
pub type StateValue = Vec<u8>;
pub type ShardId = String;

#[derive(Debug)]
pub enum PersistError {
    NotFound,
    IntegrityFailure,
    NetworkFailure,
    StorageFull,
    Unknown,
}

pub struct CheckpointHandle {
    pub shard_id: ShardId,
    pub checkpoint_id: String,
}

pub struct StateSnapshot {
    pub shard_id: ShardId,
    pub data: Vec<(StateKey, StateValue)>,
}

/// Abstract contract for decentralized persistence.
/// Backends (In-Memory, Log-Structured, CRDT) implement this trait.
pub trait PersistenceOps {
    /// Commit a piece of state into the persistence layer with a monotonic sequence ID.
    /// Returns the sequence ID on success.
    fn write(&self, seq_id: u64, key: &StateKey, value: &StateValue) -> Result<u64, PersistError>;

    /// Rollback a pending write if the corresponding IPC delivery or atomic operation fails.
    fn rollback(&self, seq_id: u64) -> Result<(), PersistError>;

    /// Capture a consistent snapshot of a shard’s state
    fn checkpoint(&self, shard_id: &ShardId) -> Result<CheckpointHandle, PersistError>;

    /// Replicate state across multiple shards/nodes for fault tolerance, enforcing sequence ordering
    fn replicate(&self, seq_id: u64, key: &StateKey, target_shards: &[ShardId]) -> Result<(), PersistError>;

    /// Restore state from a checkpoint or replica
    fn recover(&self, checkpoint: &CheckpointHandle) -> Result<StateSnapshot, PersistError>;

    /// Validate cryptographic integrity of persisted state
    fn verify(&self, key: &StateKey) -> Result<bool, PersistError>;
}
