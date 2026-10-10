// Standalone Unit Tests for Inode and Directory Subsystem
// Part of SigmaOS File Management Subsystem

#[path = "../../src/fs/fd.rs"]
mod fd;
#[path = "../../src/fs/inode.rs"]
mod inode;
#[path = "../../src/fs/directory.rs"]
mod directory;

use fd::FsError;
use inode::{InodeManager, InodeType};
use directory::Directory;

#[test]
fn test_inode_lifecycle_and_unlinking() {
    let mut mgr = InodeManager::new();
    let inode = mgr
        .create_inode(InodeType::RegularFile, 0o644, 1000, 1000, 1600000000)
        .expect("create inode");
    assert_eq!(inode.metadata.inode_id, 1);
    assert_eq!(mgr.active_inode_count(), 1);

    let fetched = mgr.get_inode(1).expect("get inode");
    assert_eq!(fetched.metadata.uid, 1000);

    mgr.unlink_inode(1).expect("unlink inode");
    assert_eq!(mgr.active_inode_count(), 0);

    let err = mgr.unlink_inode(1);
    assert_eq!(err.err(), Some(FsError::NotFound));
}

#[test]
fn test_directory_operations_and_capacity() {
    let mut dir = Directory::new(10, 1);
    assert_eq!(dir.list_entries().len(), 2);

    dir.add_entry("test.txt", 20, InodeType::RegularFile)
        .expect("add test.txt");
    assert_eq!(dir.list_entries().len(), 3);

    let looked_up = dir.lookup("test.txt").expect("lookup test.txt");
    assert_eq!(looked_up.inode_id, 20);

    let err = dir.add_entry("test.txt", 21, InodeType::RegularFile);
    assert_eq!(err.err(), Some(FsError::FileAlreadyExists));

    let removed = dir.remove_entry("test.txt").expect("remove test.txt");
    assert_eq!(removed.inode_id, 20);

    let err_dot = dir.remove_entry(".");
    assert_eq!(err_dot.err(), Some(FsError::InvalidOperation));
}
