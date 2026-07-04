// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/linux_compat/proc_shim.rs — /proc and /sys virtual filesystem shim
//
// Many Linux programs read /proc/self/maps, /proc/cpuinfo, /proc/meminfo,
// /sys/class/net/, etc.  This shim intercepts those path reads and returns
// synthesised content derived from SigmaOS kernel state.
//
// Language: Rust #![no_std]
#![no_std]
#![allow(dead_code)]

use core::fmt::Write;

// ── Simple no-alloc string writer ─────────────────────────────────────────
struct BufWriter<'a> {
    buf: &'a mut [u8],
    pos: usize,
}

impl<'a> BufWriter<'a> {
    fn new(buf: &'a mut [u8]) -> Self { Self { buf, pos: 0 } }
    fn len(&self) -> usize { self.pos }
    fn as_bytes(&self) -> &[u8] { &self.buf[..self.pos] }
}

impl<'a> Write for BufWriter<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        let available = self.buf.len() - self.pos;
        let copy = bytes.len().min(available);
        self.buf[self.pos..self.pos + copy].copy_from_slice(&bytes[..copy]);
        self.pos += copy;
        Ok(())
    }
}

// ── Extern kernel state ───────────────────────────────────────────────────
extern "C" {
    fn sigma_mm_free_pages() -> u64;
    fn sigma_mm_used_pages() -> u64;
    fn sigma_getpid() -> u32;
    fn sigma_clock_ns() -> u64;
}

// ── /proc path matching ───────────────────────────────────────────────────
fn path_matches(path: &[u8], prefix: &[u8]) -> bool {
    path.len() >= prefix.len() && &path[..prefix.len()] == prefix
}

/// Main entry: given a /proc or /sys path, fill buf with synthetic content.
/// Returns bytes written, or -1 if path not handled (fall through to VFS).
pub unsafe fn proc_read(path: &[u8], buf: &mut [u8]) -> i64 {
    let mut w = BufWriter::new(buf);

    if path_matches(path, b"/proc/cpuinfo") {
        let _ = write!(w,
            "processor\t: 0\nvendor_id\t: SigmaOS\n\
             cpu family\t: 6\nmodel name\t: SigmaOS Sovereign CPU\n\
             cpu MHz\t\t: 3200.000\ncache size\t: 16384 KB\n\
             physical id\t: 0\ncore id\t: 0\ncpu cores\t: 4\n\
             flags\t\t: fpu vme de pse tsc msr pae mce cx8 apic sep \
             mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 \
             ss ht syscall nx rdtscp lm constant_tsc rep_good nopl \
             pni pclmulqdq ssse3 cx16 sse4_1 sse4_2 popcnt aes avx \
             xsave avx2 bmi1 bmi2\n\n"
        );
        return w.len() as i64;
    }

    if path_matches(path, b"/proc/meminfo") {
        let total  = sigma_mm_used_pages() + sigma_mm_free_pages();
        let free   = sigma_mm_free_pages();
        let total_kb = total * 4;
        let free_kb  = free  * 4;
        let avail_kb = free_kb + free_kb / 4;
        let _ = write!(w,
            "MemTotal:       {total_kb:>10} kB\n\
             MemFree:        {free_kb:>10} kB\n\
             MemAvailable:   {avail_kb:>10} kB\n\
             Buffers:              0 kB\n\
             Cached:               0 kB\n\
             SwapTotal:            0 kB\n\
             SwapFree:             0 kB\n"
        );
        return w.len() as i64;
    }

    if path_matches(path, b"/proc/self/status") {
        let pid = sigma_getpid();
        let _ = write!(w,
            "Name:\tsigma-proc\nPid:\t{pid}\nPPid:\t1\n\
             Uid:\t0 0 0 0\nGid:\t0 0 0 0\n\
             VmRSS:\t4096 kB\nVmSize:\t16384 kB\n\
             Threads:\t1\n"
        );
        return w.len() as i64;
    }

    if path_matches(path, b"/proc/self/maps") {
        let _ = write!(w,
            "00400000-00401000 r-xp 00000000 00:00 0 [sigma-text]\n\
             7ffd0000-7fff0000 rwxp 00000000 00:00 0 [stack]\n"
        );
        return w.len() as i64;
    }

    if path_matches(path, b"/proc/version") {
        let _ = write!(w,
            "Linux version 6.1.0-sigmaos (sigmaos@sigmaos) \
             (gcc version 12.0, SigmaOS build) #1 SMP\n"
        );
        return w.len() as i64;
    }

    if path_matches(path, b"/proc/uptime") {
        let secs = sigma_clock_ns() / 1_000_000_000;
        let _ = write!(w, "{secs}.00 {secs}.00\n");
        return w.len() as i64;
    }

    if path_matches(path, b"/proc/mounts") || path_matches(path, b"/proc/self/mounts") {
        let _ = write!(w,
            "tmpfs / tmpfs rw,nosuid,nodev,noexec,relatime 0 0\n\
             proc /proc proc rw,nosuid,nodev,noexec,relatime 0 0\n\
             sysfs /sys sysfs rw,nosuid,nodev,noexec,relatime 0 0\n\
             devtmpfs /dev devtmpfs rw,nosuid,noexec,relatime 0 0\n"
        );
        return w.len() as i64;
    }

    if path_matches(path, b"/proc/sys/kernel/hostname") {
        let _ = write!(w, "sigmaos\n");
        return w.len() as i64;
    }

    if path_matches(path, b"/proc/sys/kernel/ostype") {
        let _ = write!(w, "Linux\n"); // Linux compat
        return w.len() as i64;
    }

    if path_matches(path, b"/proc/sys/kernel/osrelease") {
        let _ = write!(w, "6.1.0-sigmaos\n");
        return w.len() as i64;
    }

    if path_matches(path, b"/sys/class/net/") {
        // List available network interfaces
        let _ = write!(w, "lo\nsigma0\n");
        return w.len() as i64;
    }

    if path_matches(path, b"/dev/urandom") || path_matches(path, b"/dev/random") {
        // Fill with pseudo-random bytes
        let mut state = sigma_clock_ns() ^ 0xDEAD_BEEF_CAFE_BABE;
        for i in 0..buf.len() {
            state ^= state << 13; state ^= state >> 7; state ^= state << 17;
            buf[i] = (state & 0xFF) as u8;
        }
        return buf.len() as i64;
    }

    if path_matches(path, b"/dev/null") {
        return 0; // reads return 0 bytes
    }

    -1 // not handled — pass to VFS
}

// ── C-ABI export ──────────────────────────────────────────────────────────
#[no_mangle]
pub unsafe extern "C" fn proc_shim_read(
    path: *const u8, path_len: usize,
    buf: *mut u8,    buf_len: usize,
) -> i64 {
    if path.is_null() || buf.is_null() { return -22; }
    let path_slice = core::slice::from_raw_parts(path, path_len);
    let buf_slice  = core::slice::from_raw_parts_mut(buf, buf_len);
    proc_read(path_slice, buf_slice)
}
