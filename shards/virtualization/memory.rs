/// shards/virtualization/memory.rs — Rust memory safety layer
/// Wraps the C VM memory buffer with bounds-checked, type-safe accessors.
/// Exposes C FFI so vm.c can call these from safe contexts.

use core::sync::atomic::{AtomicU64, Ordering};

static READS:  AtomicU64 = AtomicU64::new(0);
static WRITES: AtomicU64 = AtomicU64::new(0);
static FAULTS: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
pub enum MemError { OutOfBounds, Misaligned, WriteProtected }

pub struct Memory {
    buf:          Vec<u8>,
    write_protect: bool,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Memory { buf: vec![0u8; size], write_protect: false }
    }

    pub fn size(&self) -> usize { self.buf.len() }

    pub fn read_u8(&self, addr: usize) -> Result<u8, MemError> {
        if addr >= self.buf.len() {
            FAULTS.fetch_add(1, Ordering::Relaxed);
            return Err(MemError::OutOfBounds);
        }
        READS.fetch_add(1, Ordering::Relaxed);
        Ok(self.buf[addr])
    }

    pub fn write_u8(&mut self, addr: usize, val: u8) -> Result<(), MemError> {
        if self.write_protect { return Err(MemError::WriteProtected); }
        if addr >= self.buf.len() {
            FAULTS.fetch_add(1, Ordering::Relaxed);
            return Err(MemError::OutOfBounds);
        }
        WRITES.fetch_add(1, Ordering::Relaxed);
        self.buf[addr] = val;
        Ok(())
    }

    pub fn read_u32_le(&self, addr: usize) -> Result<u32, MemError> {
        if addr + 4 > self.buf.len() {
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
        if addr + 4 > self.buf.len() {
            FAULTS.fetch_add(1, Ordering::Relaxed);
            return Err(MemError::OutOfBounds);
        }
        WRITES.fetch_add(1, Ordering::Relaxed);
        let bytes = val.to_le_bytes();
        self.buf[addr..addr+4].copy_from_slice(&bytes);
        Ok(())
    }

    pub fn bulk_copy(&mut self, dst: usize, src: &[u8]) -> Result<(), MemError> {
        if dst + src.len() > self.buf.len() { return Err(MemError::OutOfBounds); }
        self.buf[dst..dst+src.len()].copy_from_slice(src);
        Ok(())
    }

    pub fn set_write_protect(&mut self, enabled: bool) { self.write_protect = enabled; }

    pub fn stats(&self) -> (u64, u64, u64) {
        (READS.load(Ordering::Relaxed), WRITES.load(Ordering::Relaxed), FAULTS.load(Ordering::Relaxed))
    }
}

// ── C FFI surface ──────────────────────────────────────────────────────────────
static mut MEM_INSTANCE: Option<Memory> = None;

#[no_mangle]
pub extern "C" fn sigma_mem_init(size: usize) {
    unsafe { MEM_INSTANCE = Some(Memory::new(size)); }
}

#[no_mangle]
pub extern "C" fn sigma_mem_read(addr: usize) -> u8 {
    unsafe { MEM_INSTANCE.as_ref().and_then(|m| m.read_u8(addr).ok()).unwrap_or(0) }
}

#[no_mangle]
pub extern "C" fn sigma_mem_write(addr: usize, val: u8) -> i32 {
    unsafe {
        match MEM_INSTANCE.as_mut().map(|m| m.write_u8(addr, val)) {
            Some(Ok(())) => 0, _ => -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_write() {
        let mut m = Memory::new(256);
        m.write_u8(10, 0xAB).unwrap();
        assert_eq!(m.read_u8(10).unwrap(), 0xAB);
    }
    #[test]
    fn test_out_of_bounds() {
        let m = Memory::new(4);
        assert!(m.read_u8(99).is_err());
    }
    #[test]
    fn test_write_protect() {
        let mut m = Memory::new(64);
        m.set_write_protect(true);
        assert!(m.write_u8(0, 1).is_err());
    }
}
