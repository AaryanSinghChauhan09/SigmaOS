#![no_std]

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
                // Blocking logic would go here
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
                // Blocking logic would go here
            }
            *byte = self.buffer[read_pos];
            read_pos = (read_pos + 1) % PIPE_BUF_SIZE;
            read += 1;
        }

        self.read_pos.store(read_pos, Ordering::Release);
        read
    }
}
