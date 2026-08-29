use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;
// Linux, BSD & OpenStack Cinder Inspired Block Device & Storage Extensions for SigmaOS

use core::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};

/// Block Record Allocation Strategies
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockRecordStrategy {
    FixedBlocking,
    PermanentBlocking,
    RecordBlocking,
    VariableBlocking,
}

/// OpenStack Cinder Volume Attachment State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VolumeAttachState {
    Detached,
    Attaching,
    Attached,
    Detaching,
    Deleting,
    Error,
}

/// Cinder-style Volume Snapshot
#[derive(Debug, Clone)]
pub struct VolumeSnapshot {
    pub snapshot_id: usize,
    pub volume_id: usize,
    pub size_gb: usize,
    pub created_at: u64,
    pub is_ready: bool,
}

/// OpenStack Cinder Inspired Block Volume
pub struct CinderBlockVolume {
    pub volume_id: usize,
    pub name: String,
    pub size_gb: usize,
    pub block_size: usize,
    pub bootable: bool,
    pub attach_state: VolumeAttachState,
    pub attached_instance_id: Option<usize>,
    pub snapshots: Vec<VolumeSnapshot>,
    pub record_strategy: BlockRecordStrategy,
    pub total_blocks: usize,
    pub data: Vec<u8>,
}

impl CinderBlockVolume {
    pub fn new(
        volume_id: usize,
        name: &str,
        size_gb: usize,
        block_size: usize,
        bootable: bool,
    ) -> Self {
        let total_blocks = (size_gb * 1024 * 1024 * 1024) / block_size.max(512);
        let mut data = Vec::new();
        // Reserve minimal simulated backing buffer
        data.resize(block_size.min(4096) * 16, 0);

        CinderBlockVolume {
            volume_id,
            name: String::from(name),
            size_gb,
            block_size,
            bootable,
            attach_state: VolumeAttachState::Detached,
            attached_instance_id: None,
            snapshots: Vec::new(),
            record_strategy: BlockRecordStrategy::FixedBlocking,
            total_blocks,
            data,
        }
    }

    pub fn attach(&mut self, instance_id: usize) -> Result<(), &'static str> {
        if self.attach_state == VolumeAttachState::Attached {
            return Err("Volume is already attached to an instance");
        }
        self.attach_state = VolumeAttachState::Attached;
        self.attached_instance_id = Some(instance_id);
        Ok(())
    }

    pub fn detach(&mut self) -> Result<(), &'static str> {
        if self.attach_state != VolumeAttachState::Attached {
            return Err("Volume is not attached");
        }
        self.attach_state = VolumeAttachState::Detached;
        self.attached_instance_id = None;
        Ok(())
    }

    pub fn create_snapshot(&mut self, snapshot_id: usize, timestamp: u64) -> VolumeSnapshot {
        let snap = VolumeSnapshot {
            snapshot_id,
            volume_id: self.volume_id,
            size_gb: self.size_gb,
            created_at: timestamp,
            is_ready: true,
        };
        self.snapshots.push(snap.clone());
        snap
    }

    pub fn read_block(&self, block_index: usize, buffer: &mut [u8]) -> Result<usize, &'static str> {
        if block_index >= self.total_blocks {
            return Err("Block index out of bounds");
        }
        let offset = (block_index * self.block_size) % self.data.len();
        let bytes_to_copy = buffer
            .len()
            .min(self.block_size)
            .min(self.data.len() - offset);
        buffer[..bytes_to_copy].copy_from_slice(&self.data[offset..offset + bytes_to_copy]);
        Ok(bytes_to_copy)
    }

    pub fn write_block(
        &mut self,
        block_index: usize,
        buffer: &[u8],
    ) -> Result<usize, &'static str> {
        if block_index >= self.total_blocks {
            return Err("Block index out of bounds");
        }
        let offset = (block_index * self.block_size) % self.data.len();
        let bytes_to_copy = buffer
            .len()
            .min(self.block_size)
            .min(self.data.len() - offset);
        self.data[offset..offset + bytes_to_copy].copy_from_slice(&buffer[..bytes_to_copy]);
        Ok(bytes_to_copy)
    }
}

/// Block Storage Volume Manager
pub struct CinderVolumeManager {
    pub volumes: Vec<CinderBlockVolume>,
    pub next_volume_id: usize,
}

impl CinderVolumeManager {
    pub fn new() -> Self {
        CinderVolumeManager {
            volumes: Vec::new(),
            next_volume_id: 1,
        }
    }

    pub fn create_volume(&mut self, name: &str, size_gb: usize, bootable: bool) -> usize {
        let id = self.next_volume_id;
        self.next_volume_id += 1;
        let vol = CinderBlockVolume::new(id, name, size_gb, 4096, bootable);
        self.volumes.push(vol);
        id
    }

    pub fn get_volume_mut(&mut self, volume_id: usize) -> Option<&mut CinderBlockVolume> {
        self.volumes.iter_mut().find(|v| v.volume_id == volume_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cinder_volume_lifecycle() {
        let mut manager = CinderVolumeManager::new();
        let vol_id = manager.create_volume("boot-vol", 10, true);
        let vol = manager.get_volume_mut(vol_id).unwrap();

        assert_eq!(vol.attach_state, VolumeAttachState::Detached);
        assert!(vol.bootable);

        vol.attach(101).unwrap();
        assert_eq!(vol.attach_state, VolumeAttachState::Attached);
        assert_eq!(vol.attached_instance_id, Some(101));

        let snap = vol.create_snapshot(1, 1000234);
        assert_eq!(snap.volume_id, vol_id);

        let write_buf = [0xA5u8; 512];
        let mut read_buf = [0u8; 512];
        vol.write_block(0, &write_buf).unwrap();
        vol.read_block(0, &mut read_buf).unwrap();
        assert_eq!(read_buf[0], 0xA5);
    }
}
