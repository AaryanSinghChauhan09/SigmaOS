//! WASM Shard — cross-language bridge and WASI capability enforcement
#![no_std]

const WASM_MAGIC: [u8; 4] = [0x00, 0x61, 0x73, 0x6D];
const WASM_VERSION: [u8; 4] = [0x01, 0x00, 0x00, 0x00];
const SHARED_MEM_SIZE: usize = 65536;

static mut SHARED_MEM: [u8; SHARED_MEM_SIZE] = [0u8; SHARED_MEM_SIZE];

/// Validate a WASM binary header
#[no_mangle]
pub extern "C" fn sigma_wasm_validate(ptr: *const u8, len: usize) -> i32 {
    if ptr.is_null() || len < 8 { return -1; }
    let header = unsafe { core::slice::from_raw_parts(ptr, 8) };
    if header[..4] != WASM_MAGIC   { return -2; }
    if header[4..8] != WASM_VERSION { return -3; }
    0
}

/// Get a pointer to the shared linear memory for WASM ↔ native data exchange
#[no_mangle]
pub extern "C" fn sigma_wasm_shared_mem() -> *mut u8 {
    unsafe { SHARED_MEM.as_mut_ptr() }
}

#[no_mangle]
pub extern "C" fn sigma_wasm_shared_mem_size() -> usize { SHARED_MEM_SIZE }

/// Write data into the shared memory from the Rust side
#[no_mangle]
pub unsafe extern "C" fn sigma_wasm_mem_write(
    offset: usize, src: *const u8, len: usize
) -> i32 {
    if offset + len > SHARED_MEM_SIZE || src.is_null() { return -1; }
    core::ptr::copy_nonoverlapping(src, SHARED_MEM.as_mut_ptr().add(offset), len);
    0
}
