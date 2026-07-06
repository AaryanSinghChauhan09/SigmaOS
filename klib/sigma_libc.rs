// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// klib/sigma_libc.rs — Rust libc shim
// Implements: A minimal, statically linked standard C library shim
// written in Rust to support ported applications without relying on glibc/musl.

#![no_std]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

pub type size_t = usize;
pub type ssize_t = isize;
pub type c_int = i32;
pub type c_char = i8;
pub type c_void = core::ffi::c_void;

#[no_mangle]
pub unsafe extern "C" fn strlen(s: *const c_char) -> size_t {
    let mut len = 0;
    while *s.add(len) != 0 {
        len += 1;
    }
    len
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void {
    let d = dest as *mut u8;
    let s = src as *const u8;
    for i in 0..n {
        *d.add(i) = *s.add(i);
    }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void {
    let p = s as *mut u8;
    let val = c as u8;
    for i in 0..n {
        *p.add(i) = val;
    }
    s
}

// STUB: Malloc/Free would interface with our memory allocator
#[no_mangle]
pub unsafe extern "C" fn malloc(_size: size_t) -> *mut c_void {
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn free(_ptr: *mut c_void) {}
