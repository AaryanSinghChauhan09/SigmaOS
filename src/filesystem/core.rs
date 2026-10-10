use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsCoreError {
    AlreadyExists,
    NotFound,
    CapacityExceeded,
    TransactionFailed,
    LostUpdateDetected,
    FsCorrupted,
    InvalidPath,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsHealthStatus {
    Healthy,
    Degraded,
    ReadOnly,
    Corrupted,
}

#[derive(Debug, Clone)]
pub struct VolumeCapacityBounds {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub free_bytes: u64,
    pub total_inodes: u64,
    pub used_inodes: u64,
}

#[derive(Debug, Clone)]
pub struct FileTransactionRecord {
    pub tx_id: u64,
    pub file_path: String,
    pub sequence_number: u64,
    pub committed: bool,
}

pub struct FilesystemCoreManager {
    pub volumes: BTreeMap<String, VolumeCapacityBounds>,
    pub health_map: BTreeMap<String, FsHealthStatus>,
    pub active_transactions: BTreeMap<u64, FileTransactionRecord>,
    pub file_sequences: BTreeMap<String, u64>,
    pub next_tx_id: u64,
}

impl FilesystemCoreManager {
    pub fn new() -> Self {
        Self {
            volumes: BTreeMap::new(),
            health_map: BTreeMap::new(),
            active_transactions: BTreeMap::new(),
            file_sequences: BTreeMap::new(),
            next_tx_id: 1,
        }
    }

    pub fn register_volume(
        &mut self,
        vol_name: &str,
        total_bytes: u64,
        total_inodes: u64,
    ) -> Result<(), FsCoreError> {
        let name = vol_name.to_string();
        if self.volumes.contains_key(&name) {
            return Err(FsCoreError::AlreadyExists);
        }

        let bounds = VolumeCapacityBounds {
            total_bytes,
            used_bytes: 0,
            free_bytes: total_bytes,
            total_inodes,
            used_inodes: 0,
        };

        self.volumes.insert(name.clone(), bounds);
        self.health_map.insert(name, FsHealthStatus::Healthy);
        Ok(())
    }

    pub fn allocate_space(
        &mut self,
        vol_name: &str,
        bytes: u64,
        inodes: u64,
    ) -> Result<(), FsCoreError> {
        let bounds = self.volumes.get_mut(vol_name).ok_or(FsCoreError::NotFound)?;
        if bounds.used_bytes + bytes > bounds.total_bytes
            || bounds.used_inodes + inodes > bounds.total_inodes
        {
            return Err(FsCoreError::CapacityExceeded);
        }

        bounds.used_bytes += bytes;
        bounds.free_bytes -= bytes;
        bounds.used_inodes += inodes;
        Ok(())
    }

    pub fn free_space(&mut self, vol_name: &str, bytes: u64, inodes: u64) -> Result<(), FsCoreError> {
        let bounds = self.volumes.get_mut(vol_name).ok_or(FsCoreError::NotFound)?;
        bounds.used_bytes = bounds.used_bytes.saturating_sub(bytes);
        bounds.free_bytes = (bounds.total_bytes - bounds.used_bytes).min(bounds.total_bytes);
        bounds.used_inodes = bounds.used_inodes.saturating_sub(inodes);
        Ok(())
    }

    pub fn begin_transaction(&mut self, file_path: &str) -> Result<u64, FsCoreError> {
        let seq = self.file_sequences.get(file_path).copied().unwrap_or(0) + 1;
        let tx_id = self.next_tx_id;
        self.next_tx_id += 1;

        let rec = FileTransactionRecord {
            tx_id,
            file_path: file_path.to_string(),
            sequence_number: seq,
            committed: false,
        };

        self.active_transactions.insert(tx_id, rec);
        Ok(tx_id)
    }

    pub fn commit_transaction(
        &mut self,
        tx_id: u64,
        expected_seq: u64,
    ) -> Result<(), FsCoreError> {
        let rec = match self.active_transactions.get_mut(&tx_id) {
            Some(r) => r,
            None => return Err(FsCoreError::TransactionFailed),
        };

        if rec.sequence_number != expected_seq {
            return Err(FsCoreError::LostUpdateDetected);
        }

        rec.committed = true;
        self.file_sequences
            .insert(rec.file_path.clone(), rec.sequence_number);
        self.active_transactions.remove(&tx_id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filesystem_core_capacity_and_lost_update_prevention() {
        let mut core = FilesystemCoreManager::new();
        assert!(core.register_volume("rootvol", 1024 * 1024, 100).is_ok());

        assert!(core.allocate_space("rootvol", 512 * 1024, 50).is_ok());
        let bounds = core.volumes.get("rootvol").unwrap();
        assert_eq!(bounds.free_bytes, 512 * 1024);

        // Capacity overflow test
        assert_eq!(
            core.allocate_space("rootvol", 600 * 1024, 10),
            Err(FsCoreError::CapacityExceeded)
        );

        // Transaction lost-update test
        let tx_id = core.begin_transaction("/etc/config").unwrap();
        assert_eq!(
            core.commit_transaction(tx_id, 999),
            Err(FsCoreError::LostUpdateDetected)
        );
        assert!(core.commit_transaction(tx_id, 1).is_ok());
    }
}
