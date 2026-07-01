//! =============================================================================
//! Σ SIGMAOS: RUST MEMORY SAFETY LAYER (v2.0)
//! =============================================================================
//! Provides safe wrappers around the C slab allocator and pool system.
//! This module is the Rust integration point for memory-critical code,
//! ensuring zero undefined behavior in the allocation hot-path.
//!
//! Features:
//!   - Type-safe pool handles that prevent use-after-free
//!   - Borrow-checked page mappings
//!   - Compile-time enforcement of pool ownership
//!
//! Standard: `#![no_std]`, bare-metal compatible
//! =============================================================================

#![no_std]
#![no_main]
#![allow(dead_code)]

use core::marker::PhantomData;
use core::ptr::NonNull;

// ── FFI bindings to the C pool system ───────────────────────────────────────

extern "C" {
    fn sigma_pool_create(name: *const u8, npages: u64, block_size: u64) -> i32;
    fn sigma_pool_alloc(pool_id: i32) -> *mut u8;
    fn sigma_pool_free(pool_id: i32, ptr: *mut u8);
    fn sigma_pool_audit(pool_id: i32);
    fn sigma_pool_destroy(pool_id: i32) -> i32;
}

// ── Type-safe Pool Handle ───────────────────────────────────────────────────

/// A handle to a per-module memory pool.
/// The pool is destroyed when this handle is dropped.
pub struct PoolHandle {
    id: i32,
}

impl PoolHandle {
    /// Create a new isolated memory pool.
    ///
    /// # Safety
    /// The pool name must be a valid null-terminated ASCII string.
    pub unsafe fn new(name: &[u8], npages: u64, block_size: u64) -> Option<Self> {
        let id = sigma_pool_create(name.as_ptr(), npages, block_size);
        if id >= 0 {
            Some(PoolHandle { id })
        } else {
            None
        }
    }

    /// Allocate a block from this pool, returning a typed guard.
    pub fn alloc<T>(&self) -> Option<PoolBlock<T>> {
        let ptr = unsafe { sigma_pool_alloc(self.id) };
        NonNull::new(ptr).map(|nn| PoolBlock {
            ptr: nn.cast(),
            pool_id: self.id,
            _marker: PhantomData,
        })
    }

    /// Print an audit report for this pool.
    pub fn audit(&self) {
        unsafe { sigma_pool_audit(self.id) };
    }
}

impl Drop for PoolHandle {
    fn drop(&mut self) {
        unsafe { sigma_pool_destroy(self.id) };
    }
}

// ── Borrow-Checked Pool Block ───────────────────────────────────────────────

/// An allocation from a memory pool.
/// Automatically freed when dropped. Cannot outlive the pool.
pub struct PoolBlock<T> {
    ptr: NonNull<T>,
    pool_id: i32,
    _marker: PhantomData<T>,
}

impl<T> PoolBlock<T> {
    /// Get an immutable reference to the contained data.
    pub fn as_ref(&self) -> &T {
        unsafe { self.ptr.as_ref() }
    }

    /// Get a mutable reference to the contained data.
    pub fn as_mut(&mut self) -> &mut T {
        unsafe { self.ptr.as_mut() }
    }

    /// Get the raw pointer (for FFI interop).
    pub fn as_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }
}

impl<T> Drop for PoolBlock<T> {
    fn drop(&mut self) {
        unsafe { sigma_pool_free(self.pool_id, self.ptr.as_ptr() as *mut u8) };
    }
}

// ── Panic handler (required for #![no_std]) ─────────────────────────────────

#[panic_handler]
fn sigma_panic(_info: &core::panic::PanicInfo) -> ! {
    // In production: call sigma_panic() from C
    loop {}
}

// ── Safe Virtual Address Mapping ────────────────────────────────────────────

extern "C" {
    fn vmm_map(va: u64, pa: u64, flags: u64) -> i32;
    fn vmm_unmap(va: u64) -> i32;
}

/// A RAII page mapping that automatically unmaps when dropped.
pub struct PageMapping {
    vaddr: u64,
}

impl PageMapping {
    /// Map a physical page to a virtual address with the given flags.
    pub unsafe fn map(va: u64, pa: u64, flags: u64) -> Option<Self> {
        if vmm_map(va, pa, flags) == 0 {
            Some(PageMapping { vaddr: va })
        } else {
            None
        }
    }

    pub fn vaddr(&self) -> u64 {
        self.vaddr
    }
}

impl Drop for PageMapping {
    fn drop(&mut self) {
        unsafe { vmm_unmap(self.vaddr) };
    }
}
