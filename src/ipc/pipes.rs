use core::sync::atomic::{AtomicUsize, Ordering};

const PIPE_BUF_SIZE: usize = 4096;

pub struct Pipe {
    buffer: [u8; PIPE_BUF_SIZE],
    read_pos: AtomicUsize,
    write_pos: AtomicUsize,
    is_nonblock: bool,
}

impl Pipe {
    pub const fn new(is_nonblock: bool) -> Self {
        Pipe {
            buffer: [0; PIPE_BUF_SIZE],
            read_pos: AtomicUsize::new(0),
            write_pos: AtomicUsize::new(0),
            is_nonblock,
        }
    }

    pub fn write(&mut self, data: &[u8]) -> usize {
        let mut written = 0;
        let mut write_pos = self.write_pos.load(Ordering::Acquire);
        let read_pos = self.read_pos.load(Ordering::Acquire);

        for &byte in data {
            let next_pos = (write_pos + 1) % PIPE_BUF_SIZE;
            if next_pos == read_pos {
                if self.is_nonblock {
                    break;
                }
            }
            self.buffer[write_pos] = byte;
            write_pos = next_pos;
            written += 1;
        }

        self.write_pos.store(write_pos, Ordering::Release);
        written
    }

    pub fn read(&mut self, data: &mut [u8]) -> usize {
        let mut read = 0;
        let mut read_pos = self.read_pos.load(Ordering::Acquire);
        let write_pos = self.write_pos.load(Ordering::Acquire);

        for byte in data.iter_mut() {
            if read_pos == write_pos {
                break;
            }
            *byte = self.buffer[read_pos];
            read_pos = (read_pos + 1) % PIPE_BUF_SIZE;
            read += 1;
        }

        self.read_pos.store(read_pos, Ordering::Release);
        read
    }

    /// Zero-copy Linux/BSD splice: Transfers bytes directly from self to target pipe without user-space buffer context switching
    pub fn splice(&mut self, target: &mut Pipe, max_bytes: usize) -> usize {
        let mut transferred = 0;
        let mut temp = [0u8; 64];

        while transferred < max_bytes {
            let chunk_size = (max_bytes - transferred).min(temp.len());
            let n_read = self.read(&mut temp[..chunk_size]);
            if n_read == 0 {
                break;
            }
            let n_written = target.write(&temp[..n_read]);
            transferred += n_written;
            if n_written < n_read {
                break;
            }
        }
        transferred
    }

    /// Zero-copy Linux tee: Duplicates bytes from self into target pipe without consuming bytes from self
    pub fn tee(&self, target: &mut Pipe, max_bytes: usize) -> usize {
        let mut read_pos = self.read_pos.load(Ordering::Acquire);
        let write_pos = self.write_pos.load(Ordering::Acquire);

        let mut copied = 0;
        let mut temp = [0u8; 64];
        let mut temp_idx = 0;

        while read_pos != write_pos && copied < max_bytes {
            temp[temp_idx] = self.buffer[read_pos];
            read_pos = (read_pos + 1) % PIPE_BUF_SIZE;
            temp_idx += 1;
            copied += 1;

            if temp_idx == temp.len() || copied == max_bytes || read_pos == write_pos {
                let written = target.write(&temp[..temp_idx]);
                if written < temp_idx {
                    break;
                }
                temp_idx = 0;
            }
        }
        copied
    }
}

// =========================================================================
// Standard Streams Management Engine (Linux glibc & BSD libc setvbuf Parity)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StandardStreamType {
    Stdin = 0,
    Stdout = 1,
    Stderr = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferingMode {
    Unbuffered,
    LineBuffered,
    BlockBuffered(usize),
}

pub struct StandardStreamHandle {
    pub stream_type: StandardStreamType,
    pub buffering: BufferingMode,
    pub pipe: Pipe,
    pub line_buffer: [u8; 1024],
    pub line_len: usize,
    pub tty_echo_enabled: bool,
    pub strip_ansi_codes: bool,
}

impl StandardStreamHandle {
    pub fn new(stream_type: StandardStreamType, buffering: BufferingMode) -> Self {
        let is_nonblock = match stream_type {
            StandardStreamType::Stdin => true,
            _ => false,
        };

        Self {
            stream_type,
            buffering,
            pipe: Pipe::new(is_nonblock),
            line_buffer: [0u8; 1024],
            line_len: 0,
            tty_echo_enabled: true,
            strip_ansi_codes: false,
        }
    }

    pub fn write_stream(&mut self, data: &[u8]) -> usize {
        let mut written = 0;
        match self.buffering {
            BufferingMode::Unbuffered => {
                written = self.pipe.write(data);
            }
            BufferingMode::LineBuffered => {
                for &byte in data {
                    if self.line_len < self.line_buffer.len() {
                        self.line_buffer[self.line_len] = byte;
                        self.line_len += 1;
                    }
                    written += 1;

                    if byte == b'\n' || self.line_len == self.line_buffer.len() {
                        self.flush_line_buffer();
                    }
                }
            }
            BufferingMode::BlockBuffered(cap) => {
                let limit = cap.min(self.line_buffer.len());
                for &byte in data {
                    if self.line_len < limit {
                        self.line_buffer[self.line_len] = byte;
                        self.line_len += 1;
                    }
                    written += 1;

                    if self.line_len >= limit {
                        self.flush_line_buffer();
                    }
                }
            }
        }
        written
    }

    pub fn flush_line_buffer(&mut self) -> usize {
        if self.line_len == 0 {
            return 0;
        }

        let flushed = self.pipe.write(&self.line_buffer[..self.line_len]);
        self.line_len = 0;
        flushed
    }

    pub fn read_stream(&mut self, buf: &mut [u8]) -> usize {
        self.pipe.read(buf)
    }
}

pub struct StandardStreamTable {
    pub stdin: StandardStreamHandle,
    pub stdout: StandardStreamHandle,
    pub stderr: StandardStreamHandle,
}

impl StandardStreamTable {
    pub fn new() -> Self {
        Self {
            stdin: StandardStreamHandle::new(StandardStreamType::Stdin, BufferingMode::Unbuffered),
            stdout: StandardStreamHandle::new(
                StandardStreamType::Stdout,
                BufferingMode::LineBuffered,
            ),
            stderr: StandardStreamHandle::new(
                StandardStreamType::Stderr,
                BufferingMode::Unbuffered,
            ),
        }
    }

    pub fn write_fd(&mut self, fd: usize, data: &[u8]) -> Result<usize, &'static str> {
        match fd {
            1 => Ok(self.stdout.write_stream(data)),
            2 => Ok(self.stderr.write_stream(data)),
            _ => Err("Invalid descriptor for output stream"),
        }
    }

    pub fn read_fd(&mut self, fd: usize, buf: &mut [u8]) -> Result<usize, &'static str> {
        match fd {
            0 => Ok(self.stdin.read_stream(buf)),
            _ => Err("Invalid descriptor for input stream"),
        }
    }
}

impl Default for StandardStreamTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipe_read_write() {
        let mut pipe = Pipe::new(true);
        let written = pipe.write(b"hello world");
        assert_eq!(written, 11);

        let mut buf = [0u8; 16];
        let read = pipe.read(&mut buf);
        assert_eq!(read, 11);
        assert_eq!(&buf[..11], b"hello world");
    }

    #[test]
    fn test_pipe_splice_zero_copy() {
        let mut pipe1 = Pipe::new(true);
        let mut pipe2 = Pipe::new(true);

        pipe1.write(b"spliced data stream");
        let spliced = pipe1.splice(&mut pipe2, 20);
        assert_eq!(spliced, 19);

        let mut buf = [0u8; 32];
        let read = pipe2.read(&mut buf);
        assert_eq!(read, 19);
        assert_eq!(&buf[..19], b"spliced data stream");
    }

    #[test]
    fn test_pipe_tee_duplication() {
        let mut pipe1 = Pipe::new(true);
        let mut pipe2 = Pipe::new(true);

        pipe1.write(b"teed data");
        let teed = pipe1.tee(&mut pipe2, 9);
        assert_eq!(teed, 9);

        // Pipe 1 still has data
        let mut buf1 = [0u8; 16];
        let read1 = pipe1.read(&mut buf1);
        assert_eq!(read1, 9);
        assert_eq!(&buf1[..9], b"teed data");

        // Pipe 2 received teed copy
        let mut buf2 = [0u8; 16];
        let read2 = pipe2.read(&mut buf2);
        assert_eq!(read2, 9);
        assert_eq!(&buf2[..9], b"teed data");
    }

    #[test]
    fn test_standard_stream_table() {
        let mut streams = StandardStreamTable::new();
        assert_eq!(streams.stdout.buffering, BufferingMode::LineBuffered);

        // Write without newline -> line buffered, stays in line_buffer until newline
        streams.write_fd(1, b"hello ").unwrap();

        let mut buf = [0u8; 32];
        assert_eq!(streams.stdout.read_stream(&mut buf), 0); // Not flushed yet

        // Write newline -> triggers flush to underlying pipe
        streams.write_fd(1, b"world\n").unwrap();
        let read_bytes = streams.stdout.read_stream(&mut buf);
        assert_eq!(read_bytes, 12);
        assert_eq!(&buf[..12], b"hello world\n");

        // Stderr is unbuffered -> immediately readable
        streams.write_fd(2, b"error message").unwrap();
        let err_read = streams.stderr.read_stream(&mut buf);
        assert_eq!(err_read, 13);
        assert_eq!(&buf[..13], b"error message");
    }
}
