//! Btrfs CoW Snapshot Management Engine
//! Extends Btrfs with advanced snapshot, extent tree, and subvolume management
//! Inspired by Linux Btrfs extent tree and reference counting
//! Reference: Linux fs/btrfs/extent-tree.c, fs/btrfs/ctree.c

#![no_std]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

/// Extent flags indicating extent type and sharing status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtentFlags {
    Data,           // Regular data extent
    Tree,           // Metadata tree extent
    SharedData,     // Shared data extent (multiple references)
    SharedTree,     // Shared tree extent
}

/// Btrfs extent item in the extent tree
#[derive(Debug, Clone, Copy)]
pub struct ExtentItem {
    pub bytenr: u64,        // Logical byte number (address)
    pub length: u64,        // Extent length in bytes
    pub ref_count: u32,     // Reference count (CoW sharing)
    pub flags: ExtentFlags,
}

/// Btrfs extent B-tree
#[derive(Debug, Clone)]
pub struct BtrfsExtentTree {
    pub items: Vec<ExtentItem>,
    pub root_bytenr: u64,
}

impl BtrfsExtentTree {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            root_bytenr: 0,
        }
    }

    /// Add or update extent reference
    pub fn add_ref(&mut self, bytenr: u64, length: u64, flags: ExtentFlags) {
        // Find existing extent
        for item in self.items.iter_mut() {
            if item.bytenr == bytenr {
                item.ref_count += 1;
                return;
            }
        }
        
        // Create new extent
        self.items.push(ExtentItem {
            bytenr,
            length,
            ref_count: 1,
            flags,
        });
    }

    /// Remove extent reference (decrement ref count)
    pub fn drop_ref(&mut self, bytenr: u64) -> bool {
        for i in 0..self.items.len() {
            if self.items[i].bytenr == bytenr {
                if self.items[i].ref_count > 1 {
                    self.items[i].ref_count -= 1;
                    return false;  // Still referenced
                } else {
                    self.items.remove(i);
                    return true;   // Orphaned, can be freed
                }
            }
        }
        false
    }

    /// Get extent by address
    pub fn get_extent(&self, bytenr: u64) -> Option<&ExtentItem> {
        self.items.iter().find(|item| item.bytenr == bytenr)
    }
}

impl Default for BtrfsExtentTree {
    fn default() -> Self {
        Self::new()
    }
}

/// Btrfs subvolume (similar to snapshot but writable by default)
#[derive(Debug, Clone)]
pub struct BtrfsSubvolume {
    pub id: u64,
    pub name: String,
    pub root_bytenr: u64,
    pub parent_id: u64,
    pub read_only: bool,
    pub snapshot_of: Option<u64>,  // If snapshot, points to source subvolume
}

/// Btrfs snapshot error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BtrfsSnapshotError {
    SubvolumeNotFound,
    SnapshotExists,
    ExtentError,
    Message(String),
}

/// Extent difference between two snapshots
#[derive(Debug, Clone, Copy)]
pub struct ExtentDiff {
    pub bytenr: u64,
    pub length: u64,
    pub kind: DiffKind,
}

/// Difference type for incremental snapshots
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffKind {
    Added,      // New extent in target snapshot
    Removed,    // Extent removed in target snapshot
    Modified,   // Extent exists in both but modified
}

/// Btrfs snapshot management engine
pub struct BtrfsSnapshotEngine {
    pub subvolumes: Vec<BtrfsSubvolume>,
    pub extent_tree: BtrfsExtentTree,
    pub next_id: u64,
}

impl BtrfsSnapshotEngine {
    /// Create new snapshot engine
    pub fn new() -> Self {
        Self {
            subvolumes: Vec::new(),
            extent_tree: BtrfsExtentTree::new(),
            next_id: 1,
        }
    }

    /// Create a snapshot of a subvolume
    /// 
    /// # Arguments
    /// * `source_id` - Source subvolume ID to snapshot
    /// * `name` - Name for the new snapshot
    /// * `read_only` - Whether the snapshot is read-only
    pub fn create_snapshot(
        &mut self,
        source_id: u64,
        name: String,
        read_only: bool,
    ) -> Result<u64, BtrfsSnapshotError> {
        // Find source subvolume
        let source = self.subvolumes
            .iter()
            .find(|sv| sv.id == source_id)
            .ok_or(BtrfsSnapshotError::SubvolumeNotFound)?;
        
        // Check if snapshot with name already exists
        if self.subvolumes.iter().any(|sv| sv.name == name) {
            return Err(BtrfsSnapshotError::SnapshotExists);
        }

        let snapshot_id = self.next_id;
        self.next_id += 1;

        // Create snapshot with CoW semantics (share extents initially)
        let snapshot = BtrfsSubvolume {
            id: snapshot_id,
            name,
            root_bytenr: source.root_bytenr,  // Initially shares root
            parent_id: source_id,
            read_only,
            snapshot_of: Some(source_id),
        };

        // Increment reference counts on all extents (CoW sharing)
        // In real implementation, would walk the tree and increment refs
        self.extent_tree.add_ref(source.root_bytenr, 4096, ExtentFlags::Tree);

        self.subvolumes.push(snapshot);
        Ok(snapshot_id)
    }

    /// Delete a snapshot
    pub fn delete_snapshot(&mut self, id: u64) -> Result<(), BtrfsSnapshotError> {
        // Find snapshot index
        let index = self.subvolumes
            .iter()
            .position(|sv| sv.id == id)
            .ok_or(BtrfsSnapshotError::SubvolumeNotFound)?;

        let subvol = &self.subvolumes[index];
        let root_bytenr = subvol.root_bytenr;

        // Remove snapshot
        self.subvolumes.remove(index);

        // Decrement extent references
        // If ref count reaches 0, extent becomes orphaned and can be freed
        let _orphaned = self.extent_tree.drop_ref(root_bytenr);

        Ok(())
    }

    /// List all snapshots
    pub fn list_snapshots(&self) -> &[BtrfsSubvolume] {
        &self.subvolumes
    }

    /// Compute incremental difference between two snapshots
    pub fn get_diff(&self, from_id: u64, to_id: u64) -> Vec<ExtentDiff> {
        let mut diffs = Vec::new();

        let from_subvol = self.subvolumes.iter().find(|sv| sv.id == from_id);
        let to_subvol = self.subvolumes.iter().find(|sv| sv.id == to_id);

        if from_subvol.is_none() || to_subvol.is_none() {
            return diffs;
        }

        // In a real implementation, would compare extent trees
        // For now, provide a simple example
        for extent in &self.extent_tree.items {
            // Simplified: mark extents as modified
            diffs.push(ExtentDiff {
                bytenr: extent.bytenr,
                length: extent.length,
                kind: DiffKind::Modified,
            });
        }

        diffs
    }

    /// Get subvolume by ID
    pub fn get_subvolume(&self, id: u64) -> Option<&BtrfsSubvolume> {
        self.subvolumes.iter().find(|sv| sv.id == id)
    }
}

impl Default for BtrfsSnapshotEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_read_only_snapshot() {
        let mut engine = BtrfsSnapshotEngine::new();
        
        // Create root subvolume
        let root = BtrfsSubvolume {
            id: 1,
            name: String::from("root"),
            root_bytenr: 0x1000,
            parent_id: 0,
            read_only: false,
            snapshot_of: None,
        };
        engine.subvolumes.push(root);
        engine.next_id = 2;

        // Create read-only snapshot
        let snap_id = engine.create_snapshot(1, String::from("snap1"), true).unwrap();
        assert_eq!(snap_id, 2);
        
        let snapshot = engine.get_subvolume(snap_id).unwrap();
        assert!(snapshot.read_only);
        assert_eq!(snapshot.snapshot_of, Some(1));
    }

    #[test]
    fn test_create_writable_snapshot() {
        let mut engine = BtrfsSnapshotEngine::new();
        
        // Create source
        let root = BtrfsSubvolume {
            id: 1,
            name: String::from("source"),
            root_bytenr: 0x2000,
            parent_id: 0,
            read_only: false,
            snapshot_of: None,
        };
        engine.subvolumes.push(root);
        engine.next_id = 2;

        // Create writable snapshot
        let snap_id = engine.create_snapshot(1, String::from("writable_snap"), false).unwrap();
        
        let snapshot = engine.get_subvolume(snap_id).unwrap();
        assert!(!snapshot.read_only);
    }

    #[test]
    fn test_snapshot_reference_counting() {
        let mut engine = BtrfsSnapshotEngine::new();
        
        // Add extent
        engine.extent_tree.add_ref(0x1000, 4096, ExtentFlags::Data);
        assert_eq!(engine.extent_tree.items[0].ref_count, 1);
        
        // Add another reference (snapshot shares extent)
        engine.extent_tree.add_ref(0x1000, 4096, ExtentFlags::Data);
        assert_eq!(engine.extent_tree.items[0].ref_count, 2);
        
        // Drop reference
        let orphaned = engine.extent_tree.drop_ref(0x1000);
        assert!(!orphaned);  // Still one reference
        assert_eq!(engine.extent_tree.items[0].ref_count, 1);
        
        // Drop last reference
        let orphaned = engine.extent_tree.drop_ref(0x1000);
        assert!(orphaned);   // Now orphaned
        assert_eq!(engine.extent_tree.items.len(), 0);
    }

    #[test]
    fn test_snapshot_deletion_orphan_cleanup() {
        let mut engine = BtrfsSnapshotEngine::new();
        
        // Create subvolume
        let root = BtrfsSubvolume {
            id: 1,
            name: String::from("root"),
            root_bytenr: 0x1000,
            parent_id: 0,
            read_only: false,
            snapshot_of: None,
        };
        engine.subvolumes.push(root);
        engine.next_id = 2;
        engine.extent_tree.add_ref(0x1000, 4096, ExtentFlags::Tree);

        // Create snapshot (increments ref count)
        let snap_id = engine.create_snapshot(1, String::from("snap"), true).unwrap();
        assert_eq!(engine.extent_tree.items[0].ref_count, 2);

        // Delete snapshot (decrements ref count)
        engine.delete_snapshot(snap_id).unwrap();
        assert_eq!(engine.extent_tree.items[0].ref_count, 1);
    }

    #[test]
    fn test_incremental_diff() {
        let mut engine = BtrfsSnapshotEngine::new();
        
        // Create two subvolumes
        let sv1 = BtrfsSubvolume {
            id: 1,
            name: String::from("sv1"),
            root_bytenr: 0x1000,
            parent_id: 0,
            read_only: false,
            snapshot_of: None,
        };
        let sv2 = BtrfsSubvolume {
            id: 2,
            name: String::from("sv2"),
            root_bytenr: 0x2000,
            parent_id: 0,
            read_only: false,
            snapshot_of: None,
        };
        engine.subvolumes.push(sv1);
        engine.subvolumes.push(sv2);
        
        // Add some extents
        engine.extent_tree.add_ref(0x1000, 4096, ExtentFlags::Data);
        engine.extent_tree.add_ref(0x2000, 8192, ExtentFlags::Data);

        // Get diff
        let diffs = engine.get_diff(1, 2);
        assert!(!diffs.is_empty());
    }
}
