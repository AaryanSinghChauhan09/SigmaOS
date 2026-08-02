// Btrfs - Linux-style Copy-on-Write filesystem
// Supports snapshots, subvolumes, compression, and checksums

// (no_std only applicable at crate root - removed)

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionType {
    None,
    Zlib,
    Lzo,
    Zstd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChecksumType {
    Crc32c,
    Xxhash,
    Sha256,
}

#[derive(Debug, Clone)]
pub struct BtrfsSubvolume {
    pub id: u64,
    pub parent_id: Option<u64>,
    pub name: String,
    pub uuid: [u8; 16],
    pub compression: CompressionType,
    pub readonly: bool,
}

#[derive(Debug, Clone)]
pub struct BtrfsSnapshot {
    pub id: u64,
    pub source_subvol_id: u64,
    pub name: String,
    pub created_at: u64,
    pub readonly: bool,
}

#[derive(Debug, Clone)]
pub struct BtrfsExtent {
    pub offset: u64,
    pub length: u64,
    pub compression: CompressionType,
    pub checksum: [u8; 32],
}

pub struct BtrfsFilesystem {
    subvolumes: BTreeMap<u64, BtrfsSubvolume>,
    snapshots: BTreeMap<u64, BtrfsSnapshot>,
    extents: BTreeMap<u64, BtrfsExtent>,
    next_subvol_id: u64,
    next_snapshot_id: u64,
    default_compression: CompressionType,
    checksum_type: ChecksumType,
}

impl BtrfsFilesystem {
    pub fn new() -> Self {
        Self {
            subvolumes: BTreeMap::new(),
            snapshots: BTreeMap::new(),
            extents: BTreeMap::new(),
            next_subvol_id: 5, // Start at 5 (0-4 are reserved)
            next_snapshot_id: 1,
            default_compression: CompressionType::Zlib,
            checksum_type: ChecksumType::Crc32c,
        }
    }

    /// Create a new subvolume
    pub fn create_subvolume(
        &mut self,
        name: String,
        parent_id: Option<u64>,
    ) -> Result<u64, &'static str> {
        let id = self.next_subvol_id;
        self.next_subvol_id += 1;

        let uuid = self.generate_uuid();

        let subvol = BtrfsSubvolume {
            id,
            parent_id,
            name: name.clone(),
            uuid,
            compression: self.default_compression,
            readonly: false,
        };

        self.subvolumes.insert(id, subvol);
        Ok(id)
    }

    /// Create a snapshot of a subvolume
    pub fn create_snapshot(
        &mut self,
        source_id: u64,
        name: String,
        readonly: bool,
    ) -> Result<u64, &'static str> {
        if !self.subvolumes.contains_key(&source_id) {
            return Err("Source subvolume not found");
        }

        let id = self.next_snapshot_id;
        self.next_snapshot_id += 1;

        let snapshot = BtrfsSnapshot {
            id,
            source_subvol_id: source_id,
            name: name.clone(),
            created_at: 0, // Would use actual timestamp
            readonly,
        };

        self.snapshots.insert(id, snapshot);
        Ok(id)
    }

    /// Delete a subvolume
    pub fn delete_subvolume(&mut self, id: u64) -> Result<(), &'static str> {
        if id < 5 {
            return Err("Cannot delete reserved subvolume");
        }

        // Check if any snapshots depend on this subvolume
        for snapshot in self.snapshots.values() {
            if snapshot.source_subvol_id == id {
                return Err("Subvolume has snapshots");
            }
        }

        self.subvolumes.remove(&id).ok_or("Subvolume not found")?;

        Ok(())
    }

    /// Set subvolume as readonly
    pub fn set_readonly(&mut self, id: u64, readonly: bool) -> Result<(), &'static str> {
        let subvol = self.subvolumes.get_mut(&id).ok_or("Subvolume not found")?;

        subvol.readonly = readonly;
        Ok(())
    }

    /// Set compression for a subvolume
    pub fn set_compression(
        &mut self,
        id: u64,
        compression: CompressionType,
    ) -> Result<(), &'static str> {
        let subvol = self.subvolumes.get_mut(&id).ok_or("Subvolume not found")?;

        subvol.compression = compression;
        Ok(())
    }

    /// Get subvolume by ID
    pub fn get_subvolume(&self, id: u64) -> Option<&BtrfsSubvolume> {
        self.subvolumes.get(&id)
    }

    /// Get snapshot by ID
    pub fn get_snapshot(&self, id: u64) -> Option<&BtrfsSnapshot> {
        self.snapshots.get(&id)
    }

    /// List all subvolumes
    pub fn list_subvolumes(&self) -> Vec<&BtrfsSubvolume> {
        self.subvolumes.values().collect()
    }

    /// List all snapshots
    pub fn list_snapshots(&self) -> Vec<&BtrfsSnapshot> {
        self.snapshots.values().collect()
    }

    /// Set default compression
    pub fn set_default_compression(&mut self, compression: CompressionType) {
        self.default_compression = compression;
    }

    /// Get default compression
    pub fn default_compression(&self) -> CompressionType {
        self.default_compression
    }

    /// Generate UUID (simplified)
    fn generate_uuid(&self) -> [u8; 16] {
        [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10,
        ]
    }

    /// Get subvolume count
    pub fn subvolume_count(&self) -> usize {
        self.subvolumes.len()
    }

    /// Get snapshot count
    pub fn snapshot_count(&self) -> usize {
        self.snapshots.len()
    }
}

impl Default for BtrfsFilesystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_subvolume() {
        let mut fs = BtrfsFilesystem::new();

        let id = fs.create_subvolume("test".to_string(), None).unwrap();
        assert_eq!(fs.subvolume_count(), 1);

        let subvol = fs.get_subvolume(id).unwrap();
        assert_eq!(subvol.name, "test");
    }

    #[test]
    fn test_create_snapshot() {
        let mut fs = BtrfsFilesystem::new();

        let subvol_id = fs.create_subvolume("test".to_string(), None).unwrap();
        let snapshot_id = fs
            .create_snapshot(subvol_id, "test_snapshot".to_string(), true)
            .unwrap();

        assert_eq!(fs.snapshot_count(), 1);

        let snapshot = fs.get_snapshot(snapshot_id).unwrap();
        assert_eq!(snapshot.source_subvol_id, subvol_id);
    }

    #[test]
    fn test_delete_subvolume() {
        let mut fs = BtrfsFilesystem::new();

        let id = fs.create_subvolume("test".to_string(), None).unwrap();
        fs.delete_subvolume(id).unwrap();

        assert_eq!(fs.subvolume_count(), 0);
    }

    #[test]
    fn test_delete_with_snapshots() {
        let mut fs = BtrfsFilesystem::new();

        let subvol_id = fs.create_subvolume("test".to_string(), None).unwrap();
        fs.create_snapshot(subvol_id, "snapshot".to_string(), true)
            .unwrap();

        let result = fs.delete_subvolume(subvol_id);
        assert!(result.is_err());
    }

    #[test]
    fn test_set_readonly() {
        let mut fs = BtrfsFilesystem::new();

        let id = fs.create_subvolume("test".to_string(), None).unwrap();
        fs.set_readonly(id, true).unwrap();

        let subvol = fs.get_subvolume(id).unwrap();
        assert!(subvol.readonly);
    }

    #[test]
    fn test_set_compression() {
        let mut fs = BtrfsFilesystem::new();

        let id = fs.create_subvolume("test".to_string(), None).unwrap();
        fs.set_compression(id, CompressionType::Lzo).unwrap();

        let subvol = fs.get_subvolume(id).unwrap();
        assert_eq!(subvol.compression, CompressionType::Lzo);
    }

    #[test]
    fn test_list_subvolumes() {
        let mut fs = BtrfsFilesystem::new();

        fs.create_subvolume("test1".to_string(), None).unwrap();
        fs.create_subvolume("test2".to_string(), None).unwrap();

        let subvols = fs.list_subvolumes();
        assert_eq!(subvols.len(), 2);
    }
}
