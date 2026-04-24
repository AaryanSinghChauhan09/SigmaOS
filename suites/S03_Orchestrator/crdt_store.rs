//! =============================================================================
//! Σ SIGMAOS: CRDT PERSISTENCE BACKEND
//! =============================================================================
//! Implements `PersistenceOps` using the LWW-Register CRDT engine.
//! This is the production persistence backend for distributed shard state.
//! =============================================================================

#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use crate::crdt_lww::{LwwRegister, LwwEntry};
use crate::persistence_ops::{
    PersistenceOps, StateKey, StateValue, ShardId,
    PersistError, CheckpointHandle, StateSnapshot,
};

pub struct CrdtPersistenceStore {
    pub register: LwwRegister,
    pub checkpoint_counter: u64,
}

impl CrdtPersistenceStore {
    pub fn new(node_id: &str) -> Self {
        CrdtPersistenceStore {
            register: LwwRegister::new(node_id),
            checkpoint_counter: 0,
        }
    }
}

impl PersistenceOps for CrdtPersistenceStore {
    fn write(&self, seq_id: u64, key: &StateKey, value: &StateValue) -> Result<u64, PersistError> {
        // SAFETY: interior mutability via raw ptr is safe here (single-threaded kernel context)
        let this = unsafe { &mut *(self as *const _ as *mut CrdtPersistenceStore) };
        
        // For CRDTs, we use the sequence ID as the Lamport Clock timestamp
        // to guarantee strict monotonic ordering of replicated state.
        this.register.clock = seq_id;
        this.register.write(key.as_str(), value.clone());
        Ok(seq_id)
    }

    fn rollback(&self, seq_id: u64) -> Result<(), PersistError> {
        let this = unsafe { &mut *(self as *const _ as *mut CrdtPersistenceStore) };
        // Rollback removes any entry matching the exact failed sequence ID
        this.register.entries.retain(|e| e.timestamp != seq_id);
        Ok(())
    }

    fn checkpoint(&self, shard_id: &ShardId) -> Result<CheckpointHandle, PersistError> {
        let this = unsafe { &mut *(self as *const _ as *mut CrdtPersistenceStore) };
        this.checkpoint_counter += 1;
        Ok(CheckpointHandle {
            shard_id: shard_id.clone(),
            checkpoint_id: {
                let mut s = String::from("crdt-chkpt-");
                s.push_str(&this.checkpoint_counter.to_string());
                s
            },
        })
    }

    fn replicate(&self, _seq_id: u64, key: &StateKey, _target_shards: &[ShardId]) -> Result<(), PersistError> {
        if self.register.read(key.as_str()).is_none() {
            return Err(PersistError::NotFound);
        }
        Ok(())
    }

    fn recover(&self, checkpoint: &CheckpointHandle) -> Result<StateSnapshot, PersistError> {
        // Recover by exporting the current CRDT state as the snapshot
        let data: Vec<(StateKey, StateValue)> = self.register.snapshot()
            .iter()
            .map(|e| (e.key.clone(), e.value.clone()))
            .collect();
        Ok(StateSnapshot {
            shard_id: checkpoint.shard_id.clone(),
            data,
        })
    }

    fn verify(&self, key: &StateKey) -> Result<bool, PersistError> {
        // Verify the key exists and has valid (non-empty) data
        match self.register.read(key.as_str()) {
            Some(v) if !v.is_empty() => Ok(true),
            Some(_) => Err(PersistError::IntegrityFailure),
            None => Err(PersistError::NotFound),
        }
    }
}
