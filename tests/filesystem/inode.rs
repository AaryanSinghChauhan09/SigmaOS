// Inode & Directory Subsystem Standalone Integration Tests
// Part of SigmaOS File Management Subsystem

#[path = "../../src/fs/inode.rs"]
mod inode;

#[path = "../../src/fs/directory.rs"]
mod directory;

#[path = "../../src/fs/fd.rs"]
mod fd;

use directory::Directory;
use inode::{InodeManager, InodeState, InodeType};

#[test]
fn test_inode_manager_allocation_lookup_and_unlink() {
    let mut manager = InodeManager::new();

    let inode1 = manager
        .create_inode(InodeType::RegularFile, 0o644, 1000, 1000, 0)
        .expect("Inode allocation should succeed");
    let inode_id = inode1.metadata.inode_id;
    assert_eq!(inode_id, 1);

    {
        let inode_ref = manager
            .get_inode(inode_id)
            .expect("Inode should exist in manager");
        assert_eq!(inode_ref.get_ref_count(), 1);
        assert_eq!(inode_ref.get_state(), InodeState::Active);
        assert_eq!(inode_ref.metadata.nlink, 1);
    }

    // Lookup non-existent inode
    assert!(manager.get_inode(999).is_err());

    // Unlink inode
    assert!(manager.unlink_inode(inode_id).is_ok());

    // Second unlink on already unlinked/removed inode should return error (double-unlink prevention)
    assert!(manager.unlink_inode(inode_id).is_err());
}

#[test]
fn test_directory_capacity_bounds_and_lost_update_prevention() {
    let mut dir = Directory::new(100, 100); // Inode 100
    dir.max_capacity = 4; // "." and ".." take 2 entries, max 4 entries total

    assert!(dir.add_entry("file1.txt", 101, InodeType::RegularFile).is_ok());
    assert!(dir.add_entry("file2.txt", 102, InodeType::RegularFile).is_ok());

    // Exceed capacity bound
    let err = dir.add_entry("file3.txt", 103, InodeType::RegularFile);
    assert!(err.is_err());

    // Remove entry and re-add
    assert!(dir.remove_entry("file1.txt").is_ok());
    assert!(dir.lookup("file1.txt").is_err());

    assert!(dir.add_entry("file3.txt", 103, InodeType::RegularFile).is_ok());
    assert_eq!(dir.lookup("file3.txt").unwrap().inode_id, 103);
}
