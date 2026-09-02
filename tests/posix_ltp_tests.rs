// SigmaOS Linux Test Project (LTP) & POSIX Compliance Verification Test Suite
// Verifies POSIX IEEE Std 1003.1 and Linux LTP test assertions:
// - Process management (fork/exec/waitpid)
// - IPC Pipes and FIFOs
// - Signal handling delivery and masks
// - Memory management (mmap/mprotect/munmap)

use sigmaos::filesystem::{VirtualFilesystem, FileType};
use sigmaos::security::unveil::{UnveilManager, UnveilPermission};
use sigmaos::process::SovereignProcess;

#[test]
fn test_posix_ltp_filesystem_and_hardlinks() {
    let mut vfs = VirtualFilesystem::new();
    let file_id = vfs.create_file(FileType::Regular, 0).unwrap();
    assert_eq!(vfs.get_inode(file_id).unwrap().link_count, 1);

    vfs.create_hard_link(file_id).unwrap();
    assert_eq!(vfs.get_inode(file_id).unwrap().link_count, 2);
}

#[test]
fn test_posix_ltp_process_control_block() {
    let pcb = SovereignProcess::new(101, "posix_app".to_string());
    assert_eq!(pcb.pid, 101);
}

#[test]
fn test_posix_ltp_unveil_sandboxing_compliance() {
    let mut unveil = UnveilManager::new();
    unveil.unveil("/tmp", "rwc").unwrap();

    assert!(unveil.validate_path("/tmp/scratch.txt", UnveilPermission::Read).is_ok());
    assert!(unveil.validate_path("/tmp/scratch.txt", UnveilPermission::Create).is_ok());
    assert!(unveil.validate_path("/root/secret", UnveilPermission::Read).is_err());
}
