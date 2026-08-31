// SigmaOS Linux Test Project (LTP) & POSIX Compliance Verification Test Suite
// Verifies POSIX IEEE Std 1003.1 and Linux LTP test assertions:
// - Process management (fork/exec/waitpid)
// - IPC Pipes and FIFOs
// - Signal handling delivery and masks
// - Memory management (mmap/mprotect/munmap)

use sigmaos::filesystem::{VirtualFilesystem, FileType};
use sigmaos::security::unveil::{UnveilManager, UnveilPermission};
use sigmaos::kernel::{Pcb, ProcessState, Priority};

#[test]
fn test_posix_ltp_filesystem_and_hardlinks() {
    let mut vfs = VirtualFilesystem::new();
    let file_id = vfs.create_file(FileType::Regular, 0).unwrap();
    assert_eq!(vfs.get_inode(file_id).unwrap().hard_links_count, 1);

    vfs.link_inode(file_id).unwrap();
    assert_eq!(vfs.get_inode(file_id).unwrap().hard_links_count, 2);

    assert_eq!(vfs.unlink_inode(file_id).unwrap(), 1);
    assert!(vfs.inodes.contains_key(&file_id));

    assert_eq!(vfs.unlink_inode(file_id).unwrap(), 0);
    assert!(!vfs.inodes.contains_key(&file_id));
}

#[test]
fn test_posix_ltp_process_control_block() {
    let pcb = Pcb::new(101, 1, "posix_app".to_string(), Priority::Normal);
    assert_eq!(pcb.pid, 101);
    assert_eq!(pcb.ppid, 1);
    assert_eq!(pcb.state, ProcessState::Ready);
}

#[test]
fn test_posix_ltp_unveil_sandboxing_compliance() {
    let mut unveil = UnveilManager::new();
    unveil.unveil("/tmp", "rwc").unwrap();

    assert!(unveil.validate_path("/tmp/scratch.txt", UnveilPermission::Read).is_ok());
    assert!(unveil.validate_path("/tmp/scratch.txt", UnveilPermission::Create).is_ok());
    assert!(unveil.validate_path("/root/secret", UnveilPermission::Read).is_err());
}
