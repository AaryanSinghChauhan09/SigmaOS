#![no_std]
/// core/src/ffi.rs — Sovereign Silicon FFI
/// Raw Windows Kernel32 and Winsock2 declarations.

extern crate alloc;

#[cfg(windows)]
extern "C" {
    // Process & Memory
    pub fn CreateProcessA(lpAppName: *const u8, lpCmd: *mut u8, lpProcAttr: *mut u8, lpThreadAttr: *mut u8, bInherit: i32, dwFlags: u32, lpEnv: *mut u8, lpDir: *const u8, lpSI: *mut u8, lpPI: *mut u8) -> i32;
    pub fn CreateThread(attr: *mut u8, stack: usize, start: usize, param: usize, flags: u32, id: *mut u32) -> usize;
    pub fn WaitForSingleObject(h: usize, ms: u32) -> u32;
    pub fn CloseHandle(h: usize) -> i32;
    
    // File I/O
    pub fn CreateFileA(lpFileName: *const u8, dwAccess: u32, dwShare: u32, lpSA: *mut u8, dwCD: u32, dwFlags: u32, hTemp: usize) -> usize;
    pub fn ReadFile(h: usize, buf: *mut u8, n: u32, nRead: *mut u32, lpO: *mut u8) -> i32;
    pub fn WriteFile(h: usize, buf: *const u8, n: u32, nWritten: *mut u32, lpO: *mut u8) -> i32;
}

#[cfg(windows)]
#[link(name = "ws2_32")]
extern "C" {
    pub fn WSAStartup(v: u16, d: *mut u8) -> i32;
    pub fn socket(af: i32, t: i32, p: i32) -> usize;
    pub fn bind(s: usize, n: *const u8, l: i32) -> i32;
    pub fn listen(s: usize, b: i32) -> i32;
    pub fn accept(s: usize, a: *mut u8, l: *mut i32) -> usize;
    pub fn recv(s: usize, b: *mut u8, l: i32, f: i32) -> i32;
    pub fn send(s: usize, b: *const u8, l: i32, f: i32) -> i32;
    pub fn closesocket(s: usize) -> i32;
}
