// Standalone Unit Tests for Process File Descriptor Table
// Part of SigmaOS File Management Subsystem

#[path = "../../src/fs/fd.rs"]
mod fd;
#[path = "../../src/fs/fd_table.rs"]
mod fd_table;

use fd::{FileHandle, FsError, SeekFrom, FD_CLOEXEC, O_RDWR};
use fd_table::ProcessFdTable;

#[test]
fn test_fd_table_allocation_and_closure() {
    let mut table = ProcessFdTable::new(10);
    let handle = table.create_handle(101, O_RDWR, 0);

    let fd1 = table.allocate_fd(handle.clone(), 0).expect("allocate fd1");
    assert_eq!(fd1, 0);

    let fd2 = table.allocate_fd(handle.clone(), FD_CLOEXEC).expect("allocate fd2");
    assert_eq!(fd2, 1);

    assert_eq!(table.active_fd_count(), 2);

    let desc2 = table.get_fd(fd2).expect("get fd2");
    assert!(desc2.is_cloexec());

    let closed = table.close_fd(fd1).expect("close fd1");
    assert_eq!(closed.fd, 0);
    assert_eq!(table.active_fd_count(), 1);

    let err = table.close_fd(fd1);
    assert_eq!(err.err(), Some(FsError::InvalidFileDescriptor));
}

#[test]
fn test_fd_table_exhaustion_and_dup() {
    let mut table = ProcessFdTable::new(3);
    let handle = table.create_handle(202, O_RDWR, 0);

    let _f0 = table.allocate_fd(handle.clone(), 0).unwrap();
    let _f1 = table.allocate_fd(handle.clone(), 0).unwrap();
    let _f2 = table.allocate_fd(handle.clone(), 0).unwrap();

    let err = table.allocate_fd(handle.clone(), 0);
    assert_eq!(err.err(), Some(FsError::NoFileDescriptorsAvailable));

    table.close_fd(1).unwrap();
    let dup_fd = table.dup_fd(0, 0, FD_CLOEXEC).expect("dup_fd into slot 1");
    assert_eq!(dup_fd, 1);
}

#[test]
fn test_dup2_and_exec_cloexec() {
    let mut table = ProcessFdTable::new(10);
    let handle = table.create_handle(303, O_RDWR, 0);

    let f0 = table.allocate_fd(handle.clone(), FD_CLOEXEC).unwrap();
    let _f1 = table.allocate_fd(handle.clone(), 0).unwrap();

    let dup2_res = table.dup2_fd(f0, 5).expect("dup2 to 5");
    assert_eq!(dup2_res, 5);

    assert_eq!(table.get_fd(5).unwrap().is_cloexec(), false);

    table.handle_exec();
    assert_eq!(table.get_fd(f0).err(), Some(FsError::InvalidFileDescriptor));
    assert!(table.get_fd(5).is_ok());
}

#[test]
fn test_file_handle_seek() {
    let handle = FileHandle::new(1, 100, O_RDWR, 0);
    assert_eq!(handle.get_offset(), 0);

    let new_pos = handle.seek(SeekFrom::Start(50), 100).unwrap();
    assert_eq!(new_pos, 50);

    let new_pos = handle.seek(SeekFrom::Current(25), 100).unwrap();
    assert_eq!(new_pos, 75);

    let new_pos = handle.seek(SeekFrom::Current(-10), 100).unwrap();
    assert_eq!(new_pos, 65);

    let new_pos = handle.seek(SeekFrom::End(10), 100).unwrap();
    assert_eq!(new_pos, 110);
}
