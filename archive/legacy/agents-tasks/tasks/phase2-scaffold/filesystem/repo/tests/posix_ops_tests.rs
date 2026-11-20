//! POSIX operation tests — open, read, write, close, stat for sigmaos-filesystem.

use sigmaos_filesystem::{FdTable, OFlags, PosixError, OpenFile};
use sigmaos_filesystem::posix::FileBackend;

/// open returns a valid (non-negative) file descriptor.
#[test]
fn test_open_returns_fd() {
    let mut t = FdTable::new();
    let fd = t.open("/tmp/hello.txt", OFlags::RDWR).unwrap();
    assert!(fd >= 3, "fd must be >= 3 (0-2 are reserved)");
}

/// write stores bytes; a fresh fd has offset 0 so read retrieves them.
#[test]
fn test_write_then_read() {
    let mut t = FdTable::new();
    let fd = t.open("/tmp/rw.txt", OFlags::RDWR).unwrap();
    let n = t.write(fd, b"hello").unwrap();
    assert_eq!(n, 5);
    // Reset offset for reading.
    if let Some(f) = t.files.get_mut(&fd) {
        if let FileBackend::Memory {
            ref mut offset, ..
        } = f.backend {
            *offset = 0;
        }
    }
    let mut buf = [0u8; 8];
    let n = t.read(fd, &mut buf).unwrap();
    assert_eq!(n, 5);
    assert_eq!(&buf[..5], b"hello");
}

/// close removes the fd; subsequent reads return BadFd.
#[test]
fn test_close_invalidates_fd() {
    let mut t = FdTable::new();
    let fd = t.open("/tmp/close_me.txt", OFlags::RDWR).unwrap();
    t.close(fd).unwrap();
    let result = t.read(fd, &mut [0u8; 4]);
    assert_eq!(result, Err(PosixError::BadFd));
}

/// Reading from a bad fd returns BadFd.
#[test]
fn test_read_bad_fd() {
    let mut t = FdTable::new();
    let result = t.read(999, &mut [0u8; 4]);
    assert_eq!(result, Err(PosixError::BadFd));
}

/// Writing to a bad fd returns BadFd.
#[test]
fn test_write_bad_fd() {
    let mut t = FdTable::new();
    let result = t.write(999, b"data");
    assert_eq!(result, Err(PosixError::BadFd));
}

/// Reading past EOF from an in-memory file returns 0 bytes.
#[test]
fn test_read_at_eof_returns_zero() {
    let mut t = FdTable::new();
    let fd = t.open("/tmp/eof.txt", OFlags::RDWR).unwrap();
    // No data written; offset 0, len 0 → EOF.
    let mut buf = [0u8; 4];
    let n = t.read(fd, &mut buf).unwrap();
    assert_eq!(n, 0);
}

/// Appending multiple writes grows the in-memory buffer.
#[test]
fn test_multiple_writes_accumulate() {
    let mut t = FdTable::new();
    let fd = t.open("/tmp/acc.txt", OFlags::RDWR).unwrap();
    t.write(fd, b"abc").unwrap();
    t.write(fd, b"def").unwrap();
    // Peek at raw data via files map.
    if let Some(f) = t.files.get(&fd) {
        if let FileBackend::Memory { ref data, .. } = f.backend {
            assert_eq!(data, b"abcdef");
        } else {
            panic!("expected Memory backend");
        }
    }
}

/// open can be called multiple times for independent fds.
#[test]
fn test_multiple_opens_independent_fds() {
    let mut t = FdTable::new();
    let fd1 = t.open("/tmp/f1.txt", OFlags::RDWR).unwrap();
    let fd2 = t.open("/tmp/f2.txt", OFlags::RDWR).unwrap();
    assert_ne!(fd1, fd2, "each open should return a distinct fd");
    t.write(fd1, b"one").unwrap();
    t.write(fd2, b"two").unwrap();
    t.close(fd1).unwrap();
    // fd2 still usable after fd1 is closed.
    let mut buf = [0u8; 3];
    if let Some(f) = t.files.get_mut(&fd2) {
        if let FileBackend::Memory {
            ref mut offset, ..
        } = f.backend {
            *offset = 0;
        }
    }
    let n = t.read(fd2, &mut buf).unwrap();
    assert_eq!(&buf[..n], b"two");
}
