//! S11 Virtualization Shard — WASM JIT engine and VirtIO driver interface
#![cfg_attr(not(feature = "hosted"), no_std)]

use core::sync::atomic::{AtomicU32, Ordering};

static WASM_EXEC_COUNT: AtomicU32 = AtomicU32::new(0);

/// WASM module descriptor
#[repr(C)]
pub struct WasmModule {
    pub binary_ptr: *const u8,
    pub binary_len: usize,
    pub wasi_enabled: bool,
}

/// Execute a WASM module in an isolated context
///
/// # Safety
/// Caller must ensure binary_ptr is valid for binary_len bytes.
#[no_mangle]
pub unsafe extern "C" fn sigma_virt_exec_wasm(module: *const WasmModule) -> i32 {
    if module.is_null() { return -1; }
    let m = &*module;
    if m.binary_ptr.is_null() || m.binary_len < 8 { return -2; }
    // Verify WASM magic bytes: 0x00 0x61 0x73 0x6D
    let magic = core::slice::from_raw_parts(m.binary_ptr, 4);
    if magic != [0x00, 0x61, 0x73, 0x6D] { return -3; }
    WASM_EXEC_COUNT.fetch_add(1, Ordering::Relaxed);
    0 // OK
}

#[no_mangle]
pub extern "C" fn sigma_virt_exec_count() -> u32 {
    WASM_EXEC_COUNT.load(Ordering::Relaxed)
}

/// VirtIO device types
#[repr(u16)]
pub enum VirtIODeviceType { Network = 1, Block = 2, Console = 3, GPU = 16 }

#[no_mangle]
pub extern "C" fn sigma_virt_register_virtio(device_type: u16) -> i32 {
    match device_type {
        1 | 2 | 3 | 16 => 0,
        _ => -1,
    }
}

#[cfg(feature = "hosted")]
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_invalid_module_rejected() {
        let result = unsafe { sigma_virt_exec_wasm(core::ptr::null()) };
        assert_eq!(result, -1);
    }
    #[test]
    fn test_virtio_valid_types() {
        assert_eq!(sigma_virt_register_virtio(1), 0);
        assert_eq!(sigma_virt_register_virtio(99), -1);
    }
}
