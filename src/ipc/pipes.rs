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
                if self.is_nonblock {
                    break;
                }
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
}
