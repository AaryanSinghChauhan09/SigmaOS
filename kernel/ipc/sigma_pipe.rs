// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// kernel/ipc/sigma_pipe.rs — Anonymous Pipe (no_std, lock-free ring)
// Language: Rust #![no_std]
// Pattern: OOP via Pipe struct + PipeEnd

#![no_std]
use core::sync::atomic::{AtomicUsize, Ordering};

pub const PIPE_BUF: usize = 4096;

pub struct Pipe {
    buf:    [u8; PIPE_BUF],
    head:   AtomicUsize,
    tail:   AtomicUsize,
    closed_write: core::sync::atomic::AtomicBool,
    closed_read:  core::sync::atomic::AtomicBool,
}

impl Pipe {
    pub const fn new() -> Self {
        Self {
            buf: [0u8; PIPE_BUF],
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
            closed_write: core::sync::atomic::AtomicBool::new(false),
            closed_read:  core::sync::atomic::AtomicBool::new(false),
        }
    }

    pub fn write(&self, data: &[u8]) -> usize {
        if self.closed_read.load(Ordering::Acquire) { return 0; }
        let mut written = 0;
        for &b in data {
            let tail = self.tail.load(Ordering::Relaxed);
            let next = (tail + 1) % PIPE_BUF;
            if next == self.head.load(Ordering::Acquire) { break; } // full
            // Safety: single writer assumed
            unsafe { (self.buf.as_ptr() as *mut u8).add(tail).write(b); }
            self.tail.store(next, Ordering::Release);
            written += 1;
        }
        written
    }

    pub fn read(&self, buf: &mut [u8]) -> usize {
        let mut read = 0;
        for slot in buf.iter_mut() {
            let head = self.head.load(Ordering::Relaxed);
            if head == self.tail.load(Ordering::Acquire) { break; } // empty
            *slot = unsafe { self.buf.as_ptr().add(head).read() };
            self.head.store((head + 1) % PIPE_BUF, Ordering::Release);
            read += 1;
        }
        read
    }

    pub fn close_write(&self) { self.closed_write.store(true, Ordering::Release); }
    pub fn close_read(&self)  { self.closed_read.store(true, Ordering::Release); }
    pub fn is_eof(&self) -> bool {
        self.closed_write.load(Ordering::Acquire)
            && self.head.load(Ordering::Relaxed) == self.tail.load(Ordering::Relaxed)
    }
    pub fn available(&self) -> usize {
        let h = self.head.load(Ordering::Relaxed);
        let t = self.tail.load(Ordering::Relaxed);
        (t + PIPE_BUF - h) % PIPE_BUF
    }
}
