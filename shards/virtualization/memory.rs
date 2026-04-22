#![no_std]
/// shards/virtualization/memory.rs — #![no_std] Rust memory safety layer
/// Pure silicon primitives with zero dependency on HLL standard libraries.

use core::sync::atomic::{AtomicU64, Ordering};
use core::panic::PanicInfo;

static READS:  AtomicU64 = AtomicU64::new(0);
static WRITES: AtomicU64 = AtomicU64::new(0);
static FAULTS: AtomicU64 = AtomicU64::new(0);

const MEM_SIZE: usize = 65536; // 64KB Fixed Silicon Memory

#[derive(Debug)]
pub enum MemError { OutOfBounds, Misaligned, WriteProtected }

pub struct Memory {
    buf:           [u8; MEM_SIZE],
    write_protect: bool,
}

impl Memory {
    pub const fn new() -> Self {
        Memory { buf: [0u8; MEM_SIZE], write_protect: false }
    }

    pub fn read_u8(&self, addr: usize) -> Result<u8, MemError> {
        if addr >= MEM_SIZE {
            FAULTS.fetch_add(1, Ordering::Relaxed);
            return Err(MemError::OutOfBounds);
        }
        READS.fetch_add(1, Ordering::Relaxed);
        Ok(self.buf[addr])
    }

    pub fn write_u8(&mut self, addr: usize, val: u8) -> Result<(), MemError> {
        if self.write_protect { return Err(MemError::WriteProtected); }
        if addr >= MEM_SIZE {
            FAULTS.fetch_add(1, Ordering::Relaxed);
            return Err(MemError::OutOfBounds);
        }
        WRITES.fetch_add(1, Ordering::Relaxed);
        self.buf[addr] = val;
        Ok(())
    }

    pub fn read_u32_le(&self, addr: usize) -> Result<u32, MemError> {
        if addr + 4 > MEM_SIZE {
            FAULTS.fetch_add(1, Ordering::Relaxed);
            return Err(MemError::OutOfBounds);
        }
        READS.fetch_add(1, Ordering::Relaxed);
        Ok(u32::from_le_bytes([
            self.buf[addr], self.buf[addr+1], self.buf[addr+2], self.buf[addr+3]
        ]))
    }

    pub fn write_u32_le(&mut self, addr: usize, val: u32) -> Result<(), MemError> {
        if self.write_protect { return Err(MemError::WriteProtected); }
        if addr + 4 > MEM_SIZE {
            FAULTS.fetch_add(1, Ordering::Relaxed);
            return Err(MemError::OutOfBounds);
        }
        WRITES.fetch_add(1, Ordering::Relaxed);
        let bytes = val.to_le_bytes();
        self.buf[addr..addr+4].copy_from_slice(&bytes);
        Ok(())
    }

    pub fn set_write_protect(&mut self, enabled: bool) { self.write_protect = enabled; }

    pub fn stats(&self) -> (u64, u64, u64) {
        (READS.load(Ordering::Relaxed), WRITES.load(Ordering::Relaxed), FAULTS.load(Ordering::Relaxed))
    }
}

// ── C FFI surface ──────────────────────────────────────────────────────────────
static mut MEM_INSTANCE: Memory = Memory::new();

#[no_mangle]
pub extern "C" fn sigma_mem_read(addr: usize) -> u8 {
    unsafe { MEM_INSTANCE.read_u8(addr).unwrap_or(0) }
}

#[no_mangle]
pub extern "C" fn sigma_mem_write(addr: usize, val: u8) -> i32 {
    unsafe {
        match MEM_INSTANCE.write_u8(addr, val) {
            Ok(()) => 0, _ => -1
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
