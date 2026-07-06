// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS Service Configuration Parser
//! Parses INI-like service files without allocation.
//! no_std, no alloc.

#![no_std]
#![allow(dead_code)]

type SigmaI32 = i32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ParsedService {
    pub name: [u8; 32],
    pub exec_path: [u8; 64],
    pub auto_restart: bool,
    pub runlevel: u8,
}

/// Helper: copy string from buffer into fixed array
unsafe fn copy_str(dst: &mut [u8], src: *const u8, src_len: usize) {
    let len = src_len.min(dst.len() - 1);
    for i in 0..len {
        dst[i] = *src.add(i);
    }
    dst[len] = 0;
}

/// Parses a simple service descriptor from a raw byte buffer.
/// Expects formats like:
/// Name=udevd\nExec=/sbin/udevd\nRestart=true\nRunlevel=3\n
#[no_mangle]
pub unsafe extern "C" fn sigma_service_parse(buf: *const u8, len: usize, out: *mut ParsedService) -> SigmaI32 {
    if buf.is_null() || out.is_null() || len == 0 { return -1; }

    let srv = &mut *out;
    srv.name[0] = 0;
    srv.exec_path[0] = 0;
    srv.auto_restart = false;
    srv.runlevel = 3; // Default

    let mut i = 0;
    while i < len {
        let line_start = i;
        while i < len && *buf.add(i) != b'\n' { i += 1; }
        let line_len = i - line_start;

        if line_len > 5 {
            let line = core::slice::from_raw_parts(buf.add(line_start), line_len);
            if line.starts_with(b"Name=") {
                copy_str(&mut srv.name, buf.add(line_start + 5), line_len - 5);
            } else if line.starts_with(b"Exec=") {
                copy_str(&mut srv.exec_path, buf.add(line_start + 5), line_len - 5);
            } else if line.starts_with(b"Restart=true") {
                srv.auto_restart = true;
            } else if line.starts_with(b"Runlevel=") {
                let rl_char = *buf.add(line_start + 9);
                if rl_char >= b'0' && rl_char <= b'9' {
                    srv.runlevel = rl_char - b'0';
                }
            }
        }
        i += 1; // skip \n
    }
    
    // Require at least a name and exec path
    if srv.name[0] == 0 || srv.exec_path[0] == 0 {
        return -1;
    }
    0
}
