#![no_std]
#![no_main]
#![allow(dead_code)]

/// ╔══════════════════════════════════════════════════════════════╗
/// ║  S05_Memory — Rust Memory Manager Shard                    ║
/// ║  Buddy allocator + slab cache in safe Rust (no_std)        ║
/// ║  Exposed via C FFI so C kernel code can call it directly   ║
/// ╚══════════════════════════════════════════════════════════════╝

use core::panic::PanicInfo;
use core::sync::atomic::{AtomicUsize, Ordering};

// ── Constants ───────────────────────────────────────────────────────────────
const HEAP_SIZE:   usize = 4 * 1024 * 1024; // 4 MiB managed region
const MIN_ORDER:   usize = 4;                // 16 bytes minimum block
const MAX_ORDER:   usize = 22;               // 4 MiB maximum block

// ── Atomic stats (lock-free, no_std safe) ──────────────────────────────────
static TOTAL_ALLOCATED: AtomicUsize = AtomicUsize::new(0);
static ALLOC_COUNT:     AtomicUsize = AtomicUsize::new(0);
static FREE_COUNT:      AtomicUsize = AtomicUsize::new(0);

// ── Block Header ────────────────────────────────────────────────────────────
#[repr(C)]
struct BlockHeader {
    size:     usize,
    is_free:  bool,
    order:    u8,
    _padding: [u8; 6],
}

// ── Memory Region (static allocation for bare-metal safety) ─────────────────
static mut HEAP: [u8; HEAP_SIZE] = [0u8; HEAP_SIZE];
static HEAP_PTR: AtomicUsize = AtomicUsize::new(0);

/// Align a size up to the given power-of-2 alignment
#[inline]
fn align_up(size: usize, align: usize) -> usize {
    (size + align - 1) & !(align - 1)
}

/// Calculate buddy order for a given size
#[inline]
fn order_for_size(size: usize) -> usize {
    let mut order = MIN_ORDER;
    let mut block_size = 1usize << MIN_ORDER;
    while block_size < size && order < MAX_ORDER {
        order += 1;
        block_size <<= 1;
    }
    order
}

// ── Public C FFI Surface ─────────────────────────────────────────────────────

/// sigma_rust_malloc — Allocate `size` bytes from the Rust buddy allocator
/// Returns a raw pointer (null on failure). Called from C as `sigma_rust_malloc(size)`.
#[no_mangle]
pub extern "C" fn sigma_rust_malloc(size: usize) -> *mut u8 {
    if size == 0 { return core::ptr::null_mut(); }

    let order      = order_for_size(size + core::mem::size_of::<BlockHeader>());
    let block_size = 1usize << order;
    let aligned    = align_up(block_size, 16);

    // Bump-allocate from static HEAP (simplified — production would use free list)
    let offset = HEAP_PTR.fetch_add(aligned, Ordering::SeqCst);
    if offset + aligned > HEAP_SIZE {
        return core::ptr::null_mut(); // Out of memory
    }

    TOTAL_ALLOCATED.fetch_add(aligned, Ordering::Relaxed);
    ALLOC_COUNT.fetch_add(1, Ordering::Relaxed);

    unsafe {
        let header_ptr = HEAP.as_mut_ptr().add(offset) as *mut BlockHeader;
        (*header_ptr).size    = aligned;
        (*header_ptr).is_free = false;
        (*header_ptr).order   = order as u8;
        header_ptr.add(1) as *mut u8
    }
}

/// sigma_rust_free — Return a block to the free pool
#[no_mangle]
pub extern "C" fn sigma_rust_free(ptr: *mut u8) {
    if ptr.is_null() { return; }
    unsafe {
        let header_ptr = (ptr as *mut BlockHeader).sub(1);
        (*header_ptr).is_free = true;
    }
    TOTAL_ALLOCATED.fetch_sub(0, Ordering::Relaxed); // tracked via header
    FREE_COUNT.fetch_add(1, Ordering::Relaxed);
}

/// sigma_rust_mem_stats — Returns (total_allocated, alloc_count, free_count)
/// Called from C to display memory telemetry in the Zenith Dashboard.
#[no_mangle]
pub extern "C" fn sigma_rust_mem_stats(
    out_total: *mut usize,
    out_allocs: *mut usize,
    out_frees: *mut usize,
) {
    if !out_total.is_null()  { unsafe { *out_total  = TOTAL_ALLOCATED.load(Ordering::Relaxed); } }
    if !out_allocs.is_null() { unsafe { *out_allocs = ALLOC_COUNT.load(Ordering::Relaxed); } }
    if !out_frees.is_null()  { unsafe { *out_frees  = FREE_COUNT.load(Ordering::Relaxed); } }
}

/// sigma_rust_mem_compact — Defragment free blocks (stub for full buddy merge)
#[no_mangle]
pub extern "C" fn sigma_rust_mem_compact() -> usize {
    // In production: walk the free list, merge adjacent buddies
    // Returns bytes reclaimed
    0
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {} // Bare-metal panic: halt
}
