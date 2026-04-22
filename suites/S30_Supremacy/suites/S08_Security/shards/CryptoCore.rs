#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn rust_crypto_verify_signature(data: *const u8, data_len: usize, sig: *const u8) -> i32 {
    // Stub for Ed25519 or similar memory-safe cryptography implementation in Rust.
    // This allows the C code to call `rust_crypto_verify_signature`.
    
    if data.is_null() || sig.is_null() || data_len == 0 {
        return -1; // Invalid inputs
    }

    // Safety: In a full implementation, we'd wrap this with safe slices
    // let data_slice = unsafe { core::slice::from_raw_parts(data, data_len) };
    
    0 // Return 0 for success
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
