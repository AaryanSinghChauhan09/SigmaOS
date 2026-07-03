// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// runtime/compat/win32/sigma_ntdll.rs — NT API shim (ntdll.dll replacement)
//
// Provides the minimum NT runtime functions needed to run Win32 binaries
// under sigma-compat.  Each NT function is redirected to the equivalent
// SigmaOS syscall or sigma_handle_table operation.
//
// Windows NT ABI (x86-64 calling convention):
//   NTSTATUS = i32,  HANDLE = usize,  ULONG = u32,  PVOID = *mut u8
//   SUCCESS = 0x0,   ACCESS_DENIED = 0xC0000005
//
// Language: Rust #![no_std]

#![no_std]
#![allow(dead_code, non_snake_case, non_upper_case_globals)]

// ── NT status codes ───────────────────────────────────────────────────────
pub type NtStatus = i32;
pub const STATUS_SUCCESS:         NtStatus = 0x0000_0000;
pub const STATUS_ACCESS_DENIED:   NtStatus = 0xC000_0005_u32 as i32;
pub const STATUS_INVALID_HANDLE:  NtStatus = 0xC000_0008_u32 as i32;
pub const STATUS_NOT_IMPLEMENTED: NtStatus = 0xC000_0002_u32 as i32;
pub const STATUS_NO_MEMORY:       NtStatus = 0xC000_0017_u32 as i32;
pub const STATUS_INVALID_PARAMETER:NtStatus= 0xC000_000D_u32 as i32;
pub const STATUS_BUFFER_TOO_SMALL:NtStatus = 0xC000_0023_u32 as i32;

// ── Core NT types ─────────────────────────────────────────────────────────
pub type Handle    = usize;
pub type Ulong     = u32;
pub type Ulong64   = u64;
pub type Pvoid     = *mut u8;
pub type Pcwstr    = *const u16;

/// UNICODE_STRING — Windows UTF-16 string header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UnicodeString {
    pub length:          u16,  // byte length (not char count)
    pub maximum_length:  u16,
    pub buffer:          *mut u16,
}

impl UnicodeString {
    pub const fn empty() -> Self {
        Self { length: 0, maximum_length: 0, buffer: core::ptr::null_mut() }
    }
}

/// ANSI_STRING
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AnsiString {
    pub length:         u16,
    pub maximum_length: u16,
    pub buffer:         *mut u8,
}

// ── SigmaOS syscall stubs ─────────────────────────────────────────────────
// These extern symbols are resolved by the sigma-compat runtime.
extern "C" {
    fn sigma_alloc(size: usize) -> *mut u8;
    fn sigma_free(ptr: *mut u8);
    fn sigma_mmap(addr: u64, size: usize, prot: u32) -> *mut u8;
    fn sigma_munmap(addr: *mut u8, size: usize) -> i32;
    fn sigma_thread_create(entry: u64, arg: u64, stack_size: usize) -> usize;
    fn sigma_thread_exit(code: i32) -> !;
    fn sigma_sleep_ms(ms: u64);
    fn sigma_file_open(path: *const u8, path_len: usize, flags: u32) -> i64;
    fn sigma_file_close(fd: i64) -> i32;
    fn sigma_file_read(fd: i64, buf: *mut u8, len: usize) -> i64;
    fn sigma_file_write(fd: i64, buf: *const u8, len: usize) -> i64;
    fn sigma_log(msg: *const u8, len: usize);
    fn sigma_getpid() -> u32;
    fn sigma_gettid() -> u32;
    fn sigma_clock_ns() -> u64;
    fn sigma_handle_alloc(kind: u32, data: u64) -> usize;
    fn sigma_handle_free(h: usize) -> i32;
    fn sigma_handle_get_data(h: usize, kind: u32) -> u64;
}

// ── RtlInitUnicodeString ──────────────────────────────────────────────────
/// Initialise a UNICODE_STRING from a null-terminated UTF-16 string.
#[no_mangle]
pub unsafe extern "C" fn RtlInitUnicodeString(
    dest: *mut UnicodeString,
    src:  Pcwstr,
) {
    if dest.is_null() { return; }
    if src.is_null() {
        (*dest) = UnicodeString::empty();
        return;
    }
    let mut len: u16 = 0;
    let mut p = src;
    while *p != 0 { p = p.add(1); len = len.wrapping_add(2); }
    (*dest).length         = len;
    (*dest).maximum_length = len + 2; // include null terminator
    (*dest).buffer         = src as *mut u16;
}

/// Free a UNICODE_STRING that was heap-allocated (sets to empty).
#[no_mangle]
pub unsafe extern "C" fn RtlFreeUnicodeString(s: *mut UnicodeString) {
    if s.is_null() { return; }
    if !(*s).buffer.is_null() {
        sigma_free((*s).buffer as *mut u8);
    }
    *s = UnicodeString::empty();
}

/// Copy a UNICODE_STRING (allocates new buffer).
#[no_mangle]
pub unsafe extern "C" fn RtlCopyUnicodeString(
    dest: *mut UnicodeString,
    src:  *const UnicodeString,
) -> NtStatus {
    if dest.is_null() || src.is_null() { return STATUS_INVALID_PARAMETER; }
    let src_len  = (*src).length as usize;
    let buf = sigma_alloc(src_len + 2);
    if buf.is_null() { return STATUS_NO_MEMORY; }
    core::ptr::copy_nonoverlapping((*src).buffer as *const u8, buf, src_len);
    (buf as *mut u16).add(src_len / 2).write(0); // null-terminate
    (*dest).length         = src_len as u16;
    (*dest).maximum_length = src_len as u16 + 2;
    (*dest).buffer         = buf as *mut u16;
    STATUS_SUCCESS
}

// ── NtAllocateVirtualMemory / NtFreeVirtualMemory ────────────────────────
pub const MEM_COMMIT:   u32 = 0x1000;
pub const MEM_RESERVE:  u32 = 0x2000;
pub const MEM_RELEASE:  u32 = 0x8000;
pub const PAGE_READONLY:       u32 = 0x02;
pub const PAGE_READWRITE:      u32 = 0x04;
pub const PAGE_EXECUTE_READ:   u32 = 0x20;
pub const PAGE_NOACCESS:       u32 = 0x01;

#[no_mangle]
pub unsafe extern "C" fn NtAllocateVirtualMemory(
    process_handle: Handle,
    base_address:   *mut Pvoid,
    zero_bits:      usize,
    region_size:    *mut usize,
    alloc_type:     Ulong,
    protect:        Ulong,
) -> NtStatus {
    let _ = (process_handle, zero_bits);
    if base_address.is_null() || region_size.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    if alloc_type & (MEM_COMMIT | MEM_RESERVE) == 0 {
        return STATUS_INVALID_PARAMETER;
    }

    // Map protect flags to SigmaOS mmap prot bits
    let prot: u32 = match protect {
        PAGE_READONLY       => 0x1,          // PROT_READ
        PAGE_READWRITE      => 0x3,          // PROT_READ | PROT_WRITE
        PAGE_EXECUTE_READ   => 0x5,          // PROT_READ | PROT_EXEC  (W^X enforced)
        PAGE_NOACCESS       => 0x0,
        _                   => 0x3,
    };

    let size  = *region_size;
    let hint  = *base_address as u64;
    let ptr   = sigma_mmap(hint, size, prot);
    if ptr.is_null() { return STATUS_NO_MEMORY; }

    *base_address = ptr;
    STATUS_SUCCESS
}

#[no_mangle]
pub unsafe extern "C" fn NtFreeVirtualMemory(
    process_handle: Handle,
    base_address:   *mut Pvoid,
    region_size:    *mut usize,
    free_type:      Ulong,
) -> NtStatus {
    let _ = (process_handle, free_type);
    if base_address.is_null() || region_size.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    let ret = sigma_munmap(*base_address, *region_size);
    if ret == 0 { STATUS_SUCCESS } else { STATUS_INVALID_PARAMETER }
}

// ── NtCreateThread / NtTerminateThread ────────────────────────────────────
#[no_mangle]
pub unsafe extern "C" fn NtCreateThread(
    thread_handle: *mut Handle,
    _access:       Ulong,
    _obj_attrs:    *mut u8,
    _process:      Handle,
    start_routine: u64,
    start_context: u64,
    stack_size:    usize,
) -> NtStatus {
    if thread_handle.is_null() { return STATUS_INVALID_PARAMETER; }
    let tid = sigma_thread_create(start_routine, start_context, stack_size);
    *thread_handle = sigma_handle_alloc(0x100, tid as u64); // 0x100 = HANDLE_THREAD
    STATUS_SUCCESS
}

#[no_mangle]
pub unsafe extern "C" fn NtTerminateThread(
    _thread_handle: Handle,
    exit_status:    i32,
) -> NtStatus {
    sigma_thread_exit(exit_status);
}

// ── NtDelayExecution (Sleep) ──────────────────────────────────────────────
#[no_mangle]
pub unsafe extern "C" fn NtDelayExecution(
    _alertable:      bool,
    delay_interval:  *const i64, // negative = relative 100-ns units
) -> NtStatus {
    if delay_interval.is_null() { return STATUS_INVALID_PARAMETER; }
    let interval = *delay_interval;
    // Convert 100-ns units to ms (negative = relative)
    let ms = if interval < 0 { ((-interval) / 10_000) as u64 } else { 0 };
    if ms > 0 { sigma_sleep_ms(ms); }
    STATUS_SUCCESS
}

// ── NtQuerySystemTime ─────────────────────────────────────────────────────
// Windows FILETIME: 100-ns intervals since 1601-01-01
// Unix epoch offset from 1601: 11644473600 seconds
const EPOCH_DIFF_100NS: u64 = 116_444_736_000_000_000;

#[no_mangle]
pub unsafe extern "C" fn NtQuerySystemTime(system_time: *mut u64) -> NtStatus {
    if system_time.is_null() { return STATUS_INVALID_PARAMETER; }
    let ns = sigma_clock_ns();
    *system_time = EPOCH_DIFF_100NS + ns / 100;
    STATUS_SUCCESS
}

// ── RtlGetNativeSystemInformation ────────────────────────────────────────
#[no_mangle]
pub unsafe extern "C" fn RtlGetNativeSystemInformation(
    info_class:    u32,
    buffer:        *mut u8,
    buffer_size:   u32,
    return_length: *mut u32,
) -> NtStatus {
    let _ = (info_class, buffer, buffer_size, return_length);
    STATUS_NOT_IMPLEMENTED
}

// ── RtlAnsiStringToUnicodeString ─────────────────────────────────────────
#[no_mangle]
pub unsafe extern "C" fn RtlAnsiStringToUnicodeString(
    dest:       *mut UnicodeString,
    src:        *const AnsiString,
    alloc_dest: bool,
) -> NtStatus {
    if dest.is_null() || src.is_null() { return STATUS_INVALID_PARAMETER; }
    let src_len = (*src).length as usize;
    let needed  = src_len * 2 + 2;

    if alloc_dest {
        let buf = sigma_alloc(needed);
        if buf.is_null() { return STATUS_NO_MEMORY; }
        (*dest).buffer         = buf as *mut u16;
        (*dest).maximum_length = needed as u16;
    } else if (*dest).maximum_length as usize < needed {
        return STATUS_BUFFER_TOO_SMALL;
    }

    // ASCII→UTF-16 (codepoints 0x00–0x7F only)
    let src_buf = (*src).buffer;
    let dst_buf = (*dest).buffer;
    for i in 0..src_len {
        *dst_buf.add(i) = *src_buf.add(i) as u16;
    }
    *dst_buf.add(src_len) = 0u16;
    (*dest).length = (src_len * 2) as u16;
    STATUS_SUCCESS
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop { unsafe { core::arch::asm!("cli; hlt", options(nomem, nostack)); } }
}
