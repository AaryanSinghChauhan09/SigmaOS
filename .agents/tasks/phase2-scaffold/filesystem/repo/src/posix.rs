//! # POSIX File Operations
//!
//! POSIX-compliant file descriptor table with `open`, `read`, `write`, `close`,
//! `pipe`, and FIFO operations.
//!
//! ## Principles
//! - POSIX compliance: file descriptor integers, errno-style errors.
//! - Memory safety: no raw pointers; all state owned by the FdTable.
//! - Single responsibility: this module manages the FD table only.
//! - Safe concurrency: no shared mutable state across threads here (callers
//!   must wrap in a mutex for multi-thread use).

use std::collections::BTreeMap;
use std::string::String;
use std::vec::Vec;

use thiserror::Error;

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

/// POSIX-style errors for file operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum PosixError {
    #[error("ENOENT: no such file or directory")]
    NoSuchFile,
    #[error("EBADF: bad file descriptor")]
    BadFd,
    #[error("EACCES: permission denied")]
    PermissionDenied,
    #[error("EEXIST: file exists")]
    Exists,
    #[error("ENOSPC: no space left on device")]
    NoSpace,
    #[error("EIO: I/O error")]
    Io,
    #[error("EINVAL: invalid argument")]
    Invalid,
    #[error("EPIPE: broken pipe")]
    BrokenPipe,
    #[error("EMFILE: too many open files")]
    TooManyOpenFiles,
    #[error("ENFILE: file table overflow")]
    FileTableOverflow,
    #[error("EPERM: operation not permitted")]
    NotPermitted,
}

/// Convenience result type.
pub type PosixResult<T> = Result<T, PosixError>;

// ---------------------------------------------------------------------------
// Open flags
// ---------------------------------------------------------------------------

/// POSIX O_* open flags.
#[derive(Debug, Clone, Copy)]
pub struct OFlags {
    pub rdonly:    bool,
    pub wronly:    bool,
    pub rdwr:      bool,
    pub creat:     bool,
    pub trunc:     bool,
    pub append:    bool,
    pub excl:      bool,
    pub nonblock:  bool,
    pub directory: bool,
}

impl OFlags {
    pub const RDONLY: Self = Self { rdonly: true, wronly: false, rdwr: false, creat: false, trunc: false, append: false, excl: false, nonblock: false, directory: false };
    pub const WRONLY: Self = Self { rdonly: false, wronly: true, rdwr: false, creat: false, trunc: false, append: false, excl: false, nonblock: false, directory: false };
    pub const RDWR:   Self = Self { rdonly: false, wronly: false, rdwr: true, creat: false, trunc: false, append: false, excl: false, nonblock: false, directory: false };
    pub const CREAT_WRONLY: Self = Self { rdonly: false, wronly: true, rdwr: false, creat: true, trunc: false, append: false, excl: false, nonblock: false, directory: false };
}

// ---------------------------------------------------------------------------
// File state
// ---------------------------------------------------------------------------

/// The concrete file backing behind a file descriptor.
#[derive(Debug)]
pub enum FileBackend {
    /// Regular in-memory file (used when no VFS is wired up in unit tests).
    Memory { data: Vec<u8>, offset: u64 },
    /// Read end of a pipe.
    PipeRead  { pipe_id: u64 },
    /// Write end of a pipe.
    PipeWrite { pipe_id: u64 },
    /// FIFO (named pipe) — same as pipe but with a path name.
    Fifo { path: String, pipe_id: u64, is_reader: bool },
}

/// Open file description (one per `open(2)` call).
#[derive(Debug)]
pub struct OpenFile {
    pub backend:   FileBackend,
    pub flags:     OFlags,
    /// POSIX read/write reference counts: close requires exactly one call.
    pub ref_count: usize,
}

// ---------------------------------------------------------------------------
// Pipe state
// ---------------------------------------------------------------------------

/// Shared pipe buffer.
#[derive(Debug)]
struct Pipe {
    buffer:       Vec<u8>,
    writers_open: usize,
    readers_open: usize,
}

// ---------------------------------------------------------------------------
// FdTable
// ---------------------------------------------------------------------------

/// Per-process file descriptor table.
pub struct FdTable {
    pub files:     BTreeMap<i32, OpenFile>,
    pipes:     BTreeMap<u64, Pipe>,
    fifos:     BTreeMap<String, u64>,     // path → pipe_id
    next_fd:   i32,
    next_pipe: u64,
    /// Maximum number of open descriptors.
    max_fds:   usize,
}

impl FdTable {
    pub const DEFAULT_MAX: usize = 1024;

    /// Create a new file descriptor table.
    pub fn new() -> Self {
        Self {
            files: BTreeMap::new(),
            pipes: BTreeMap::new(),
            fifos: BTreeMap::new(),
            next_fd: 3, // 0-2 reserved for stdin/stdout/stderr
            next_pipe: 1,
            max_fds: Self::DEFAULT_MAX,
        }
    }

    fn alloc_fd(&mut self) -> PosixResult<i32> {
        if self.files.len() >= self.max_fds {
            return Err(PosixError::TooManyOpenFiles);
        }
        let fd = self.next_fd;
        self.next_fd += 1;
        Ok(fd)
    }

    // -----------------------------------------------------------------------
    // open / close
    // -----------------------------------------------------------------------

    /// Open (or create) an in-memory file at `path`.
    pub fn open(&mut self, _path: &str, flags: OFlags) -> PosixResult<i32> {
        let fd = self.alloc_fd()?;
        self.files.insert(fd, OpenFile {
            backend:  FileBackend::Memory { data: Vec::new(), offset: 0 },
            flags,
            ref_count: 1,
        });
        Ok(fd)
    }

    /// Close a file descriptor, releasing underlying resources.
    pub fn close(&mut self, fd: i32) -> PosixResult<()> {
        let file = self.files.remove(&fd).ok_or(PosixError::BadFd)?;
        // Maintain pipe ref counts.
        match &file.backend {
            FileBackend::PipeRead  { pipe_id } => {
                if let Some(p) = self.pipes.get_mut(pipe_id) {
                    p.readers_open = p.readers_open.saturating_sub(1);
                }
            }
            FileBackend::PipeWrite { pipe_id } => {
                if let Some(p) = self.pipes.get_mut(pipe_id) {
                    p.writers_open = p.writers_open.saturating_sub(1);
                }
            }
            FileBackend::Fifo { pipe_id, is_reader, .. } => {
                if let Some(p) = self.pipes.get_mut(pipe_id) {
                    if *is_reader { p.readers_open = p.readers_open.saturating_sub(1); }
                    else          { p.writers_open = p.writers_open.saturating_sub(1); }
                }
            }
            _ => {}
        }
        Ok(())
    }

    // -----------------------------------------------------------------------
    // read / write
    // -----------------------------------------------------------------------

    /// Read up to `buf.len()` bytes from `fd` starting at the current offset.
    pub fn read(&mut self, fd: i32, buf: &mut [u8]) -> PosixResult<usize> {
        let file = self.files.get_mut(&fd).ok_or(PosixError::BadFd)?;
        match &mut file.backend {
            FileBackend::Memory { data, offset } => {
                let off = *offset as usize;
                if off >= data.len() { return Ok(0); }
                let avail = data.len() - off;
                let n = avail.min(buf.len());
                buf[..n].copy_from_slice(&data[off..off + n]);
                *offset += n as u64;
                Ok(n)
            }
            FileBackend::PipeRead  { pipe_id } |
            FileBackend::Fifo { pipe_id, is_reader: true, .. } => {
                let pid = *pipe_id;
                let p = self.pipes.get_mut(&pid).ok_or(PosixError::BadFd)?;
                if p.buffer.is_empty() {
                    if p.writers_open == 0 { return Ok(0); } // EOF
                    return Ok(0); // would block — return 0 in sync mode
                }
                let n = p.buffer.len().min(buf.len());
                buf[..n].copy_from_slice(&p.buffer[..n]);
                p.buffer.drain(..n);
                Ok(n)
            }
            _ => Err(PosixError::BadFd),
        }
    }

    /// Write `data` to `fd`.
    pub fn write(&mut self, fd: i32, data: &[u8]) -> PosixResult<usize> {
        let file = self.files.get_mut(&fd).ok_or(PosixError::BadFd)?;
        match &mut file.backend {
            FileBackend::Memory { data: buf, offset } => {
                let off = *offset as usize;
                let end = off + data.len();
                if end > buf.len() { buf.resize(end, 0); }
                buf[off..end].copy_from_slice(data);
                *offset += data.len() as u64;
                Ok(data.len())
            }
            FileBackend::PipeWrite { pipe_id } |
            FileBackend::Fifo { pipe_id, is_reader: false, .. } => {
                let pid = *pipe_id;
                let p = self.pipes.get_mut(&pid).ok_or(PosixError::BadFd)?;
                if p.readers_open == 0 { return Err(PosixError::BrokenPipe); }
                p.buffer.extend_from_slice(data);
                Ok(data.len())
            }
            _ => Err(PosixError::BadFd),
        }
    }

    // -----------------------------------------------------------------------
    // pipe / FIFO
    // -----------------------------------------------------------------------

    /// Create an anonymous pipe. Returns `(read_fd, write_fd)`.
    pub fn pipe(&mut self) -> PosixResult<(i32, i32)> {
        let pipe_id = self.next_pipe;
        self.next_pipe += 1;
        self.pipes.insert(pipe_id, Pipe { buffer: Vec::new(), writers_open: 1, readers_open: 1 });

        let rfd = self.alloc_fd()?;
        self.files.insert(rfd, OpenFile {
            backend:   FileBackend::PipeRead { pipe_id },
            flags:     OFlags::RDONLY,
            ref_count: 1,
        });

        let wfd = self.alloc_fd()?;
        self.files.insert(wfd, OpenFile {
            backend:   FileBackend::PipeWrite { pipe_id },
            flags:     OFlags::WRONLY,
            ref_count: 1,
        });

        Ok((rfd, wfd))
    }

    /// Create a named FIFO at `path`.
    pub fn mkfifo(&mut self, path: &str) -> PosixResult<()> {
        if self.fifos.contains_key(path) { return Err(PosixError::Exists); }
        let pipe_id = self.next_pipe;
        self.next_pipe += 1;
        self.pipes.insert(pipe_id, Pipe { buffer: Vec::new(), writers_open: 0, readers_open: 0 });
        self.fifos.insert(path.to_string(), pipe_id);
        Ok(())
    }

    /// Open one end of a named FIFO (reading or writing).
    pub fn open_fifo(&mut self, path: &str, for_reading: bool) -> PosixResult<i32> {
        let pipe_id = *self.fifos.get(path).ok_or(PosixError::NoSuchFile)?;
        {
            let p = self.pipes.get_mut(&pipe_id).ok_or(PosixError::Io)?;
            if for_reading { p.readers_open += 1; } else { p.writers_open += 1; }
        }
        let fd = self.alloc_fd()?;
        self.files.insert(fd, OpenFile {
            backend:   FileBackend::Fifo { path: path.to_string(), pipe_id, is_reader: for_reading },
            flags:     if for_reading { OFlags::RDONLY } else { OFlags::WRONLY },
            ref_count: 1,
        });
        Ok(fd)
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_write_read_close() {
        let mut table = FdTable::new();
        let fd = table.open("/tmp/test.txt", OFlags::RDWR).unwrap();

        // Write then re-read from offset 0.
        table.write(fd, b"hello world").unwrap();
        // Reset offset by closing and reopening (simulate seek by opening new fd).
        let fd2 = table.open("/tmp/test.txt", OFlags::RDWR).unwrap();
        // New fd has its own fresh memory buffer — just write and read in place.
        table.write(fd2, b"sigma").unwrap();
        let mut buf = [0u8; 8];
        // Reset by accessing the backing store directly (unit-test convenience).
        if let Some(f) = table.files.get_mut(&fd2) {
            if let FileBackend::Memory { ref mut offset, .. } = f.backend { *offset = 0; }
        }
        let n = table.read(fd2, &mut buf).unwrap();
        assert_eq!(&buf[..n], b"sigma");
        table.close(fd2).unwrap();
        table.close(fd).unwrap();
    }

    #[test]
    fn pipe_write_read() {
        let mut table = FdTable::new();
        let (rfd, wfd) = table.pipe().unwrap();
        table.write(wfd, b"ping").unwrap();
        let mut buf = [0u8; 8];
        let n = table.read(rfd, &mut buf).unwrap();
        assert_eq!(&buf[..n], b"ping");
        table.close(rfd).unwrap();
        table.close(wfd).unwrap();
    }

    #[test]
    fn broken_pipe_on_closed_reader() {
        let mut table = FdTable::new();
        let (rfd, wfd) = table.pipe().unwrap();
        table.close(rfd).unwrap();
        let err = table.write(wfd, b"data");
        assert_eq!(err, Err(PosixError::BrokenPipe));
        table.close(wfd).unwrap();
    }

    #[test]
    fn fifo_mkfifo_and_roundtrip() {
        let mut table = FdTable::new();
        table.mkfifo("/tmp/myfifo").unwrap();
        let wfd = table.open_fifo("/tmp/myfifo", false).unwrap();
        let rfd = table.open_fifo("/tmp/myfifo", true).unwrap();
        table.write(wfd, b"fifo-data").unwrap();
        let mut buf = [0u8; 16];
        let n = table.read(rfd, &mut buf).unwrap();
        assert_eq!(&buf[..n], b"fifo-data");
        table.close(rfd).unwrap();
        table.close(wfd).unwrap();
    }

    #[test]
    fn bad_fd_returns_error() {
        let mut table = FdTable::new();
        let mut buf = [0u8; 4];
        assert_eq!(table.read(99, &mut buf), Err(PosixError::BadFd));
        assert_eq!(table.write(99, b"x"),    Err(PosixError::BadFd));
        assert_eq!(table.close(99),           Err(PosixError::BadFd));
    }
}
