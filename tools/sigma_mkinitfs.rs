// sigma_mkinitfs.rs
// SigmaOS initramfs builder — replaces sigma_mkinitfs.c
// Zero-dependency Rust implementation
// Produces a minimal CPIO initramfs archive for the bootloader to load

#![no_std]
#![allow(dead_code)]

use core::ffi::c_char;
use core::slice;

// CPIO newc magic header
const CPIO_MAGIC: &[u8; 6] = b"070701";
const CPIO_TRAILER: &[u8; 10] = b"TRAILER!!!";

// ── CPIO Header Structure ────────────────────────────────────────────────────
#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct CpioHeader {
    magic: [u8; 6],
    inode: [u8; 8],
    mode: [u8; 8],
    uid: [u8; 8],
    gid: [u8; 8],
    nlink: [u8; 8],
    mtime: [u8; 8],
    filesize: [u8; 8],
    devmajor: [u8; 8],
    devminor: [u8; 8],
    rdevmajor: [u8; 8],
    rdevminor: [u8; 8],
    namesize: [u8; 8],
    check: [u8; 8],
}

// ── Helper: Convert number to hex string ──────────────────────────────────────
fn u32_to_hex_octal(value: u32, buf: &mut [u8; 8]) {
    let hex_chars = b"0123456789ABCDEF";
    let mut v = value;
    for i in (0..8).rev() {
        buf[i] = hex_chars[(v & 0xF) as usize];
        v >>= 4;
    }
}

// ── Write CPIO header ────────────────────────────────────────────────────────
fn write_cpio_header(name: &[u8], inode: u32, mode: u32, filesize: u32) -> CpioHeader {
    let namesize = name.len() + 1;
    
    let mut header = CpioHeader {
        magic: *CPIO_MAGIC,
        inode: [0; 8],
        mode: [0; 8],
        uid: [0; 8],
        gid: [0; 8],
        nlink: [0; 8],
        mtime: [0; 8],
        filesize: [0; 8],
        devmajor: [0; 8],
        devminor: [0; 8],
        rdevmajor: [0; 8],
        rdevminor: [0; 8],
        namesize: [0; 8],
        check: [0; 8],
    };
    
    u32_to_hex_octal(inode, &mut header.inode);
    u32_to_hex_octal(mode, &mut header.mode);
    u32_to_hex_octal(0, &mut header.uid); // root
    u32_to_hex_octal(0, &mut header.gid); // root
    u32_to_hex_octal(1, &mut header.nlink);
    u32_to_hex_octal(0, &mut header.mtime); // deterministic
    u32_to_hex_octal(filesize, &mut header.filesize);
    u32_to_hex_octal(0, &mut header.devmajor);
    u32_to_hex_octal(1, &mut header.devminor);
    u32_to_hex_octal(0, &mut header.rdevmajor);
    u32_to_hex_octal(0, &mut header.rdevminor);
    u32_to_hex_octal(namesize as u32, &mut header.namesize);
    u32_to_hex_octal(0, &mut header.check);
    
    header
}

// ── Calculate padding for 4-byte alignment ───────────────────────────────────
fn calculate_padding(header_size: usize, name_size: usize) -> usize {
    let total = header_size + name_size;
    (4 - (total % 4)) % 4
}

// ── Build minimal initramfs ────────────────────────────────────────────────────
pub fn build_initramfs() -> CpioHeader {
    // Write the TRAILER to produce a valid (empty) CPIO archive
    write_cpio_header(CPIO_TRAILER, 0, 0, 0)
}

// ── Main entry point for no_std context ───────────────────────────────────────
#[no_mangle]
pub extern "C" fn sigma_mkinitfs_create() -> *const CpioHeader {
    // In a real implementation, this would write to a file
    // For no_std, we return the header structure
    let header = build_initramfs();
    
    // Static allocation for the header
    static mut HEADER: CpioHeader = CpioHeader {
        magic: *CPIO_MAGIC,
        inode: [0; 8],
        mode: [0; 8],
        uid: [0; 8],
        gid: [0; 8],
        nlink: [0; 8],
        mtime: [0; 8],
        filesize: [0; 8],
        devmajor: [0; 8],
        devminor: [0; 8],
        rdevmajor: [0; 8],
        rdevminor: [0; 8],
        namesize: [0; 8],
        check: [0; 8],
    };
    
    unsafe {
        HEADER = header;
        &HEADER as *const CpioHeader
    }
}
