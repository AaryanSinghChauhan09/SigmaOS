//! Pipe and FIFO tests for sigmaos-filesystem.

use sigmaos_filesystem::{FdTable, PosixError};

// ─── Anonymous pipe tests ──────────────────────────────────────────────────

/// pipe() returns two distinct file descriptors.
#[test]
fn test_pipe_returns_two_fds() {
    let mut t = FdTable::new();
    let (rfd, wfd) = t.pipe().unwrap();
    assert_ne!(rfd, wfd);
}

/// Data written to the write end can be read from the read end.
#[test]
fn test_pipe_write_then_read() {
    let mut t = FdTable::new();
    let (rfd, wfd) = t.pipe().unwrap();
    let n = t.write(wfd, b"sigma").unwrap();
    assert_eq!(n, 5);
    let mut buf = [0u8; 8];
    let m = t.read(rfd, &mut buf).unwrap();
    assert_eq!(m, 5);
    assert_eq!(&buf[..5], b"sigma");
}

/// Multiple writes accumulate in the pipe buffer.
#[test]
fn test_pipe_multiple_writes() {
    let mut t = FdTable::new();
    let (rfd, wfd) = t.pipe().unwrap();
    t.write(wfd, b"foo").unwrap();
    t.write(wfd, b"bar").unwrap();
    let mut buf = [0u8; 6];
    let n = t.read(rfd, &mut buf).unwrap();
    assert_eq!(&buf[..n], b"foobar");
}

/// Writing to a pipe whose read end is closed returns BrokenPipe.
#[test]
fn test_write_after_read_end_closed() {
    let mut t = FdTable::new();
    let (rfd, wfd) = t.pipe().unwrap();
    t.close(rfd).unwrap();
    let result = t.write(wfd, b"data");
    assert_eq!(result, Err(PosixError::BrokenPipe));
}

/// Reading from an empty pipe with writers still open returns 0 (would-block).
#[test]
fn test_read_empty_pipe_with_writer_open() {
    let mut t = FdTable::new();
    let (rfd, _wfd) = t.pipe().unwrap();
    let mut buf = [0u8; 4];
    let n = t.read(rfd, &mut buf).unwrap();
    assert_eq!(n, 0, "empty pipe with open writer should return 0 (EAGAIN)");
}

/// Reading from an empty pipe after writer closes returns 0 (EOF).
#[test]
fn test_read_empty_pipe_after_writer_closed() {
    let mut t = FdTable::new();
    let (rfd, wfd) = t.pipe().unwrap();
    t.close(wfd).unwrap();
    let mut buf = [0u8; 4];
    let n = t.read(rfd, &mut buf).unwrap();
    assert_eq!(n, 0, "empty pipe with closed writer should return EOF");
}

/// Partial reads consume only available bytes.
#[test]
fn test_pipe_partial_read() {
    let mut t = FdTable::new();
    let (rfd, wfd) = t.pipe().unwrap();
    t.write(wfd, b"hello world").unwrap();
    let mut buf = [0u8; 5];
    let n = t.read(rfd, &mut buf).unwrap();
    assert_eq!(n, 5);
    assert_eq!(&buf[..5], b"hello");
    // Remaining data still in pipe.
    let mut buf2 = [0u8; 6];
    let m = t.read(rfd, &mut buf2).unwrap();
    assert_eq!(m, 6);
    assert_eq!(&buf2[..6], b" world");
}

// ─── Named FIFO tests ─────────────────────────────────────────────────────

/// mkfifo creates a FIFO; it does not yet have open ends.
#[test]
fn test_mkfifo_creates_named_pipe() {
    let mut t = FdTable::new();
    t.mkfifo("/tmp/test.fifo").expect("mkfifo should succeed");
}

/// mkfifo on an already-existing path returns Exists.
#[test]
fn test_mkfifo_duplicate_returns_exists() {
    let mut t = FdTable::new();
    t.mkfifo("/tmp/dup.fifo").unwrap();
    let result = t.mkfifo("/tmp/dup.fifo");
    assert_eq!(result, Err(PosixError::Exists));
}

/// open_fifo on a non-existent FIFO returns NoSuchFile.
#[test]
fn test_open_fifo_missing_returns_no_such_file() {
    let mut t = FdTable::new();
    let result = t.open_fifo("/tmp/ghost.fifo", true);
    assert_eq!(result, Err(PosixError::NoSuchFile));
}

/// Writing to the write end of a FIFO is readable from the read end.
#[test]
fn test_fifo_write_read() {
    let mut t = FdTable::new();
    t.mkfifo("/tmp/rw.fifo").unwrap();
    let rfd = t.open_fifo("/tmp/rw.fifo", true).unwrap();
    let wfd = t.open_fifo("/tmp/rw.fifo", false).unwrap();
    t.write(wfd, b"fifodata").unwrap();
    let mut buf = [0u8; 8];
    let n = t.read(rfd, &mut buf).unwrap();
    assert_eq!(n, 8);
    assert_eq!(&buf[..8], b"fifodata");
}

/// Writing to a FIFO with no readers returns BrokenPipe.
#[test]
fn test_fifo_write_no_readers() {
    let mut t = FdTable::new();
    t.mkfifo("/tmp/noread.fifo").unwrap();
    let wfd = t.open_fifo("/tmp/noread.fifo", false).unwrap();
    // No reader opened — readers_open == 0.
    let result = t.write(wfd, b"x");
    assert_eq!(result, Err(PosixError::BrokenPipe));
}

/// Multiple distinct FIFOs are independent.
#[test]
fn test_multiple_fifos_independent() {
    let mut t = FdTable::new();
    t.mkfifo("/tmp/a.fifo").unwrap();
    t.mkfifo("/tmp/b.fifo").unwrap();
    let ra = t.open_fifo("/tmp/a.fifo", true).unwrap();
    let wa = t.open_fifo("/tmp/a.fifo", false).unwrap();
    let rb = t.open_fifo("/tmp/b.fifo", true).unwrap();
    let wb = t.open_fifo("/tmp/b.fifo", false).unwrap();
    t.write(wa, b"aaa").unwrap();
    t.write(wb, b"bbb").unwrap();
    let mut buf = [0u8; 3];
    let n = t.read(ra, &mut buf).unwrap();
    assert_eq!(&buf[..n], b"aaa");
    let n = t.read(rb, &mut buf).unwrap();
    assert_eq!(&buf[..n], b"bbb");
}
