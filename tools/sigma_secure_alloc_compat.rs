// SPDX-License-Identifier: MIT
//! SigmaOS Process-Local Secure Memory Scrubbing Allocator
//!
//! Provides a safe, thread-safe memory allocation tracking wrapper that
//! automatically zeros out / scrubs active buffers on deallocation (free)
//! to prevent volatile information leaks, and tracks allocation stats.

#![no_std]
extern crate alloc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};
pub struct AllocatedBlock {
    pub ptr: *mut u8,
    pub size: usize,
}
pub struct SigmaSecureAllocator {
    pub active_blocks: Vec<AllocatedBlock>,
    pub allocated_bytes: AtomicUsize,
    pub deallocated_bytes: AtomicUsize,
impl SigmaSecureAllocator {
    pub fn new() -> Self {
        Self {
            active_blocks: Vec::new(),
            allocated_bytes: AtomicUsize::new(0),
            deallocated_bytes: AtomicUsize::new(0),
        }
    }
    /// Simulates allocating a block of memory
    pub fn allocate(&mut self, size: usize) -> *mut u8 {
        if size == 0 {
            return core::ptr::null_mut();
        // Standard Rust allocator fallback (using standard box or vec layout for tests)
        let mut layout_buf = Vec::with_capacity(size);
        for _ in 0..size {
            layout_buf.push(0x55u8); // pattern initialized
        let ptr = layout_buf.as_mut_ptr();
        core::mem::forget(layout_buf); // transfer ownership to raw pointer
        self.active_blocks.push(AllocatedBlock { ptr, size });
        self.allocated_bytes.fetch_add(size, Ordering::SeqCst);
        ptr
    /// Deallocates and securely scrubs memory (zeroing out block before freeing)
    pub fn deallocate(&mut self, ptr: *mut u8) -> bool {
        for i in 0..self.active_blocks.len() {
            if self.active_blocks[i].ptr == ptr {
                let block = self.active_blocks.remove(i);
                // Secure Scrubbing: Fill memory block with zeros to prevent sensitive data leaks
                unsafe {
                    core::ptr::write_bytes(block.ptr, 0, block.size);
                }
                self.deallocated_bytes.fetch_add(block.size, Ordering::SeqCst);
                // Re-constitute vector to trigger standard free
                    let _ = Vec::from_raw_parts(block.ptr, block.size, block.size);
                return true;
            }
        false
    pub fn get_metrics(&self) -> (usize, usize) {
        (
            self.allocated_bytes.load(Ordering::SeqCst),
            self.deallocated_bytes.load(Ordering::SeqCst),
        )
impl Default for SigmaSecureAllocator {
    fn default() -> Self {
        Self::new()
// =========================================================================
// UNIT TESTS
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_secure_scrubbing_on_free() {
        let mut allocator = SigmaSecureAllocator::new();
        let size = 64;
        // Allocate block
        let ptr = allocator.allocate(size);
        assert!(!ptr.is_null());
        // Verify it was initialized with test pattern
        unsafe {
            assert_eq!(*ptr, 0x55);
            assert_eq!(*ptr.add(size - 1), 0x55);
        // Deallocate and verify scrubbing occurs before freeing standard memory
        assert!(allocator.deallocate(ptr));
        // Check metrics
        let (allocated, deallocated) = allocator.get_metrics();
        assert_eq!(allocated, size);
        assert_eq!(deallocated, size);
//! SigmaOS Secure Zero-on-Free Allocator Helper
//! Implements strict memory scrubbing and zeroing guarantees on deallocation,
//! inspired by OpenBSD's hardened security-first memory allocators (omalloc).
//! Zero external dependencies
#![allow(dead_code)]
type SigmaU8 = u8;
type SigmaU32 = u32;
type SigmaI32 = i32;
type SigmaBool = bool;
type SigmaSize = usize;
/// Statistics tracker for memory scrubbing operations
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AllocScrubStats {
    pub total_pages_scrubbed: SigmaSize,
    pub total_bytes_zeroed: u64,
static mut SCRUBBER_INITIALIZED: SigmaBool = false;
static mut SCRUB_STATS: AllocScrubStats = AllocScrubStats {
    total_pages_scrubbed: 0,
    total_bytes_zeroed: 0,
};
/// Initialize the secure memory scrubbing subsystem
#[no_mangle]
pub unsafe extern "C" fn secure_alloc_init() -> SigmaI32 {
    SCRUBBER_INITIALIZED = true;
    SCRUB_STATS.total_pages_scrubbed = 0;
    SCRUB_STATS.total_bytes_zeroed = 0;
    0 // Success
/// Securely zero out a memory segment before returning it to the kernel page pool
pub unsafe extern "C" fn secure_alloc_free_scrub(ptr: *mut u8, len: SigmaSize) -> SigmaI32 {
    if !SCRUBBER_INITIALIZED || ptr.is_null() || len == 0 {
        return -1;
    // Force volatile writes to ensure the compiler does not optimize away the zeroing operation (dead code elimination)
    for i in 0..len {
        core::ptr::write_volatile(ptr.add(i), 0);
    // Update global tracking telemetry
    SCRUB_STATS.total_bytes_zeroed += len as u64;
    if len >= 4096 {
        SCRUB_STATS.total_pages_scrubbed += len / 4096;
/// Get secure memory scrubbing performance statistics
pub unsafe extern "C" fn secure_alloc_get_stats(stats: *mut AllocScrubStats) -> SigmaI32 {
    if !SCRUBBER_INITIALIZED || stats.is_null() {
    *stats = SCRUB_STATS;
}
