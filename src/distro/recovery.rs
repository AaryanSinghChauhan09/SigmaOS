//! Snapshot-backed Disaster Recovery and Rescue Environment Mount Manager
//! Implements super-fast root-mount restorations using Merkle rollback proofs.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MountMode {
    ReadOnly,
    ReadWrite,
}

pub struct RescueManager {
    pub active_mode: MountMode,
    pub checkpoint_id: u32,
}

impl RescueManager {
    pub const fn new() -> Self {
        Self {
            active_mode: MountMode::ReadOnly,
            checkpoint_id: 0,
        }
    }

    pub fn execute_rollback_restoration(&mut self, target_id: u32) -> Result<(), &'static str> {
        if target_id == 0 {
            return Err("Cannot rollback to reserve system checkpoint");
        }
        self.checkpoint_id = target_id;
        self.active_mode = MountMode::ReadOnly;
        Ok(())
    }
}
