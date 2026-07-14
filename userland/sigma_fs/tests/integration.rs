use sigma_fs::{Vfs, BlockAllocator};

#[test]
fn test_vfs_file_creation_and_lookup() {
    let mut vfs = Vfs::new();
    let id = vfs.create_file("/sigma/boot/kernel.elf", 4096);
    assert_eq!(id, 1);

    let inode = vfs.lookup("/sigma/boot/kernel.elf").unwrap();
    assert_eq!(inode.id, 1);
    assert_eq!(inode.size, 4096);

    assert!(vfs.lookup("/sigma/doesnotexist").is_none());
}

#[test]
fn test_block_allocator_alloc_and_free() {
    let mut alloc = BlockAllocator::new(128, 4096);
    
    let b1 = alloc.alloc().unwrap();
    let b2 = alloc.alloc().unwrap();
    assert_ne!(b1, b2);
    assert_eq!(b1, 0);
    assert_eq!(b2, 1);

    alloc.free(b1);
    let b3 = alloc.alloc().unwrap();
    assert_eq!(b3, 0); // should reuse freed block
}
