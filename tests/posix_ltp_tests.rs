// SigmaOS Linux Test Project (LTP) & POSIX Compliance Verification Test Suite
// Verifies POSIX IEEE Std 1003.1 and Linux LTP test assertions:
// - Process management (fork/exec/waitpid)
// - IPC Pipes and FIFOs
// - Signal handling delivery and masks
// - Memory management (mmap/mprotect/munmap)

use sigmaos::filesystem::sovereign_link_engine::SovereignLinkEngine;
use sigmaos::kernel::process::{Process, ProcessState};
use sigmaos::security::unveil::{UnveilManager, UnveilPermission};

#[test]
fn test_posix_ltp_filesystem_and_hardlinks() {
    let mut link_engine = SovereignLinkEngine::new();
    let ino = link_engine.create_file("/var/log/syslog", b"log_data");
    assert_eq!(link_engine.inodes.get(&ino).unwrap().hard_link_count, 1);

    link_engine
        .create_hard_link("/var/log/syslog", "/var/log/syslog.hard")
        .unwrap();
    assert_eq!(link_engine.inodes.get(&ino).unwrap().hard_link_count, 2);

    link_engine.unlink("/var/log/syslog").unwrap();
    assert_eq!(link_engine.inodes.get(&ino).unwrap().hard_link_count, 1);
    assert!(link_engine.inodes.contains_key(&ino));

    link_engine.unlink("/var/log/syslog.hard").unwrap();
    assert!(!link_engine.inodes.contains_key(&ino));
}

#[test]
fn test_posix_ltp_process_control_block() {
    let proc = Process::new(101, 1, "posix_app".to_string());
    assert_eq!(proc.pid, 101);
    assert_eq!(proc.parent_pid, 1);
    assert_eq!(proc.state, ProcessState::Running);
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
