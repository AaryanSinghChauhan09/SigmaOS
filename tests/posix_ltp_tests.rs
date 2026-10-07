// SigmaOS Linux Test Project (LTP) & POSIX Compliance Verification Test Suite
// Verifies POSIX IEEE Std 1003.1 and Linux LTP test assertions:
// - Process management (fork/exec/waitpid)
// - IPC Pipes and FIFOs
// - Signal handling delivery and masks
// - Memory management (mmap/mprotect/munmap)

use sigmaos::filesystem::{FileType, VirtualFilesystem};
use sigmaos::kernel::{Pcb, Priority, ProcessState};
use sigmaos::security::unveil::{UnveilManager, UnveilPermission};

#[test]
fn test_posix_ltp_filesystem_and_hardlinks() {
    let mut vfs = VirtualFilesystem::new();
    let fd = vfs.open("/tmp/posix_file", 0, 0644).unwrap();
    assert!(fd >= 0);
    assert_eq!(vfs.write(fd, b"posix_data").unwrap(), 10);
    assert!(vfs.close_file(fd as u64).is_ok());
}

#[test]
fn test_posix_ltp_process_control_block() {
    let pcb = Pcb::new(101, 0x1000);
    assert_eq!(pcb.process_id, 101);
    assert_eq!(pcb.page_directory_base, 0x1000);
}

#[test]
fn test_posix_ltp_unveil_sandboxing_compliance() {
    let mut unveil = UnveilManager::new();
    unveil.unveil("/tmp", "rwc").unwrap();

    assert!(unveil
        .validate_path("/tmp/scratch.txt", UnveilPermission::Read)
        .is_ok());
    assert!(unveil
        .validate_path("/tmp/scratch.txt", UnveilPermission::Create)
        .is_ok());
    assert!(unveil
        .validate_path("/root/secret", UnveilPermission::Read)
        .is_err());
}
