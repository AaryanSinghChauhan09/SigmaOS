//! Integration tests: POSIX file operations — open, read, write, close,
//! anonymous pipe, and named FIFO.

use sigmaos_filesystem::{FdTable, OFlags, PosixError};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn fresh() -> FdTable { FdTable::new() }

// ---------------------------------------------------------------------------
// open / close
// ---------------------------------------------------------------------------

#[test]
fn open_returns_valid_fd() {
    let mut t = fresh();
    let fd = t.open("/tmp/test.txt", OFlags::CREAT_WRONLY).expect("open should succeed");
    assert!(fd >= 3, "fd must be >= 3 (0/1/2 are reserved)");
    t.close(fd).unwrap();
}

#[test]
fn close_invalid_fd_returns_bad_fd() {
    let mut t = fresh();
    assert_eq!(t.close(999), Err(PosixError::BadFd));
}

#[test]
fn double_close_returns_bad_fd() {
    let mut t = fresh();
    let fd = t.open("/tmp/x", OFlags::RDWR).unwrap();
    t.close(fd).unwrap();
    assert_eq!(t.close(fd), Err(PosixError::BadFd));
}

// ---------------------------------------------------------------------------
// write / read
// ---------------------------------------------------------------------------

#[test]
fn write_then_read_roundtrip() {
    let mut t = fresh();
    let fd = t.open("/tmp/rw.txt", OFlags::RDWR).unwrap();

    let payload = b"hello, SigmaOS!";
    let n = t.write(fd, payload).unwrap();
    assert_eq!(n, payload.len());

    // Reset offset.
    if let Some(f) = t.files.get_mut(&fd) {
        if let sigmaos_filesystem::posix::FileBackend::Memory { ref mut offset, .. } = f.backend {
            *offset = 0;
        }
    }

    let mut buf = [0u8; 32];
    let m = t.read(fd, &mut buf).unwrap();
    assert_eq!(m, payload.len());
    assert_eq!(&buf[..m], payload);

    t.close(fd).unwrap();
}

#[test]
fn read_past_end_of_file_returns_zero() {
    let mut t = fresh();
    let fd = t.open("/tmp/empty.txt", OFlags::RDONLY).unwrap();
    let mut buf = [0u8; 8];
    let n = t.read(fd, &mut buf).unwrap();
    assert_eq!(n, 0, "reading empty file should return 0");
    t.close(fd).unwrap();
}

#[test]
fn write_to_bad_fd_errors() {
    let mut t = fresh();
    assert_eq!(t.write(77, b"data"), Err(PosixError::BadFd));
}

#[test]
fn read_from_bad_fd_errors() {
    let mut t = fresh();
    let mut buf = [0u8; 4];
    assert_eq!(t.read(77, &mut buf), Err(PosixError::BadFd));
}

// ---------------------------------------------------------------------------
// Anonymous pipe
// ---------------------------------------------------------------------------

#[test]
fn pipe_basic_write_read() {
    let mut t = fresh();
    let (rfd, wfd) = t.pipe().expect("pipe() should succeed");

    t.write(wfd, b"ping").unwrap();

    let mut buf = [0u8; 8];
    let n = t.read(rfd, &mut buf).unwrap();
    assert_eq!(&buf[..n], b"ping");

    t.close(rfd).unwrap();
    t.close(wfd).unwrap();
}

#[test]
fn pipe_multiple_writes_coalesce() {
    let mut t = fresh();
    let (rfd, wfd) = t.pipe().unwrap();

    t.write(wfd, b"foo").unwrap();
    t.write(wfd, b"bar").unwrap();

    let mut buf = [0u8; 16];
    let n = t.read(rfd, &mut buf).unwrap();
    // Both writes are in the buffer; read delivers what's available.
    assert_eq!(&buf[..n], b"foobar");

    t.close(rfd).unwrap();
    t.close(wfd).unwrap();
}

#[test]
fn broken_pipe_when_reader_closed() {
    let mut t = fresh();
    let (rfd, wfd) = t.pipe().unwrap();
    t.close(rfd).unwrap();

    let err = t.write(wfd, b"data");
    assert_eq!(err, Err(PosixError::BrokenPipe));
    t.close(wfd).unwrap();
}

#[test]
fn pipe_read_after_writer_closed_returns_eof() {
    let mut t = fresh();
    let (rfd, wfd) = t.pipe().unwrap();
    t.write(wfd, b"last").unwrap();
    t.close(wfd).unwrap();

    // The data written before close is still in the buffer.
    let mut buf = [0u8; 16];
    let n = t.read(rfd, &mut buf).unwrap();
    assert_eq!(&buf[..n], b"last");

    t.close(rfd).unwrap();
}

// ---------------------------------------------------------------------------
// Named FIFO
// ---------------------------------------------------------------------------

#[test]
fn fifo_create_and_roundtrip() {
    let mut t = fresh();
    t.mkfifo("/tmp/myfifo").expect("mkfifo should succeed");

    let wfd = t.open_fifo("/tmp/myfifo", false).unwrap();
    let rfd = t.open_fifo("/tmp/myfifo", true).unwrap();

    t.write(wfd, b"fifo-msg").unwrap();

    let mut buf = [0u8; 16];
    let n = t.read(rfd, &mut buf).unwrap();
    assert_eq!(&buf[..n], b"fifo-msg");

    t.close(rfd).unwrap();
    t.close(wfd).unwrap();
}

#[test]
fn fifo_double_create_returns_exists() {
    let mut t = fresh();
    t.mkfifo("/tmp/dup").unwrap();
    assert_eq!(t.mkfifo("/tmp/dup"), Err(PosixError::Exists));
}

#[test]
fn open_fifo_on_nonexistent_path_errors() {
    let mut t = fresh();
    assert_eq!(t.open_fifo("/nonexistent", true), Err(PosixError::NoSuchFile));
}
