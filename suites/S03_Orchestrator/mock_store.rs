//! =============================================================================
//! Σ SIGMAOS: MOCK PERSISTENCE STORE
//! =============================================================================
//! An in-memory stub implementation of the PersistenceOps contract.
//! Used for testing fault-tolerance logic before binding to a real disk or 
//! network CRDT log.
//! =============================================================================

#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use crate::persistence_ops::{
    PersistenceOps, StateKey, StateValue, ShardId, 
    PersistError, CheckpointHandle, StateSnapshot
};

pub struct MockPersistenceStore {
    pub is_online: bool,
}

impl MockPersistenceStore {
    pub fn new() -> Self {
        MockPersistenceStore { is_online: true }
    }
}

impl PersistenceOps for MockPersistenceStore {
    fn write(&self, seq_id: u64, _key: &StateKey, _value: &StateValue) -> Result<u64, PersistError> {
        if !self.is_online { return Err(PersistError::NetworkFailure); }
        Ok(seq_id)
    }

    fn rollback(&self, _seq_id: u64) -> Result<(), PersistError> {
        if !self.is_online { return Err(PersistError::NetworkFailure); }
        Ok(())
    }

    fn checkpoint(&self, shard_id: &ShardId) -> Result<CheckpointHandle, PersistError> {
        if !self.is_online { return Err(PersistError::NetworkFailure); }
        Ok(CheckpointHandle {
            shard_id: shard_id.clone(),
            checkpoint_id: String::from("chkpt-mock-1234"),
        })
    }

    fn replicate(&self, _seq_id: u64, _key: &StateKey, _target_shards: &[ShardId]) -> Result<(), PersistError> {
        if !self.is_online { return Err(PersistError::NetworkFailure); }
        Ok(())
    }

    fn recover(&self, checkpoint: &CheckpointHandle) -> Result<StateSnapshot, PersistError> {
        if !self.is_online { return Err(PersistError::NetworkFailure); }
        Ok(StateSnapshot {
            shard_id: checkpoint.shard_id.clone(),
            data: Vec::new(),
        })
    }

    fn verify(&self, _key: &StateKey) -> Result<bool, PersistError> {
        if !self.is_online { return Err(PersistError::NetworkFailure); }
        // Assume signature verification succeeds for mock
        Ok(true)
    }
}
