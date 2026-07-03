// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/fs/procfs.rs — /proc filesystem (kernel info interface)
// Language: Rust #![no_std]
#![no_std]
#![allow(dead_code)]

use core::fmt::Write;

struct BufW<'a> { buf: &'a mut [u8], pos: usize }
impl<'a> BufW<'a> {
    fn new(b: &'a mut [u8]) -> Self { Self { buf: b, pos: 0 } }
    fn len(&self) -> usize { self.pos }
}
impl<'a> Write for BufW<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let src = s.as_bytes();
        let avail = self.buf.len() - self.pos;
        let n = src.len().min(avail);
        self.buf[self.pos..self.pos+n].copy_from_slice(&src[..n]);
        self.pos += n;
        Ok(())
    }
}

extern "C" {
    fn sigma_mm_free_pages() -> u64;
    fn sigma_mm_used_pages() -> u64;
    fn sigma_clock_ns() -> u64;
    fn sigma_getpid() -> u32;
    fn sigma_task_count() -> usize;
    fn sigma_acpi_cpu_count() -> usize;
}

pub unsafe fn procfs_read(path: &[u8], buf: &mut [u8]) -> i64 {
    let mut w = BufW::new(buf);

    // Helper: path starts with
    macro_rules! starts { ($p:expr) => { path.len() >= $p.len() && &path[..$p.len()] == $p } }

    if starts!(b"/proc/cpuinfo") {
        let ncpu = sigma_acpi_cpu_count().max(1);
        for i in 0..ncpu {
            let _ = write!(w, "processor\t: {i}\nvendor_id\t: SigmaOS\n\
                cpu family\t: 25\nmodel name\t: SigmaOS Zenith CPU @ 3.20GHz\n\
                cpu MHz\t\t: 3200.000\ncache size\t: 16384 KB\n\
                physical id\t: 0\ncore id\t\t: {i}\ncpu cores\t: {ncpu}\n\
                flags\t\t: fpu vme de pse tsc msr pae cx8 apic sep \
                mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 \
                ss ht syscall nx rdtscp lm rep_good nopl pni cx16 \
                sse4_1 sse4_2 popcnt aes avx xsave avx2 bmi1 bmi2\n\n");
        }
        return w.len() as i64;
    }

    if starts!(b"/proc/meminfo") {
        let total_kb = (sigma_mm_used_pages() + sigma_mm_free_pages()) * 4;
        let free_kb  = sigma_mm_free_pages() * 4;
        let avail_kb = free_kb + free_kb / 4;
        let _ = write!(w,
            "MemTotal:       {total_kb:>10} kB\nMemFree:        {free_kb:>10} kB\n\
             MemAvailable:   {avail_kb:>10} kB\nBuffers:              0 kB\n\
             Cached:               0 kB\nSwapCached:           0 kB\n\
             Active:               0 kB\nInactive:             0 kB\n\
             SwapTotal:            0 kB\nSwapFree:             0 kB\n\
             Dirty:                0 kB\nWriteback:            0 kB\n\
             AnonPages:            0 kB\nMapped:               0 kB\n\
             Shmem:                0 kB\nKReclaimable:         0 kB\n\
             Slab:                 0 kB\nSReclaimable:         0 kB\n\
             SUnreclaim:           0 kB\nKernelStack:         64 kB\n\
             PageTables:          16 kB\nNFS_Unstable:         0 kB\n\
             Bounce:               0 kB\nWritebackTmp:         0 kB\n\
             CommitLimit:    {total_kb:>10} kB\nCommitted_AS:         0 kB\n\
             VmallocTotal:  34359738367 kB\nVmallocUsed:          0 kB\n\
             VmallocChunk:         0 kB\nPercpu:               0 kB\n");
        return w.len() as i64;
    }

    if starts!(b"/proc/version") {
        let _ = write!(w, "Linux version 6.1.0-sigmaos (sigmaos@sigmaos) \
            (rustc 1.80.0-nightly, Zig 0.13.0) #1 SMP SigmaOS Zenith\n");
        return w.len() as i64;
    }

    if starts!(b"/proc/uptime") {
        let secs = sigma_clock_ns() / 1_000_000_000;
        let _ = write!(w, "{secs}.00 {secs}.00\n");
        return w.len() as i64;
    }

    if starts!(b"/proc/loadavg") {
        let ntasks = sigma_task_count();
        let _ = write!(w, "0.00 0.00 0.00 {ntasks}/{ntasks} {}\n", sigma_getpid());
        return w.len() as i64;
    }

    if starts!(b"/proc/self/status") || starts!(b"/proc/status") {
        let pid = sigma_getpid();
        let free_kb = sigma_mm_free_pages() * 4;
        let _ = write!(w,
            "Name:\tsigmaos\nState:\tS (sleeping)\nTgid:\t{pid}\nPid:\t{pid}\n\
             PPid:\t1\nUid:\t0\t0\t0\t0\nGid:\t0\t0\t0\t0\n\
             VmPeak:\t65536 kB\nVmSize:\t65536 kB\nVmLck:\t0 kB\n\
             VmPin:\t0 kB\nVmHWM:\t4096 kB\nVmRSS:\t4096 kB\n\
             VmData:\t1024 kB\nVmStk:\t64 kB\nVmExe:\t256 kB\n\
             VmLib:\t0 kB\nVmPTE:\t0 kB\nVmSwap:\t0 kB\n\
             Threads:\t1\nSigPnd:\t0000000000000000\n\
             SigBlk:\t0000000000000000\nSigIgn:\t0000000000000000\n\
             SigCgt:\t0000000000000000\n\
             CapInh:\t0000000000000000\nCapPrm:\t000001ffffffffff\n\
             CapEff:\t000001ffffffffff\nCapBnd:\t000001ffffffffff\n\
             CapAmb:\t0000000000000000\n\
             Seccomp:\t0\nCpus_allowed:\tf\nMems_allowed:\t1\n");
        return w.len() as i64;
    }

    if starts!(b"/proc/self/maps") {
        let _ = write!(w,
            "00400000-00500000 r-xp 00000000 00:00 0 [sigma-kernel-text]\n\
             00500000-00600000 r--p 00100000 00:00 0 [sigma-kernel-rodata]\n\
             00600000-00700000 rw-p 00200000 00:00 0 [sigma-kernel-data]\n\
             7ffd0000-7fff0000 rw-p 00000000 00:00 0 [stack]\n\
             ffffffffff600000-ffffffffff601000 r-xp 00000000 00:00 0 [vdso]\n");
        return w.len() as i64;
    }

    if starts!(b"/proc/mounts") || starts!(b"/proc/self/mounts") {
        let _ = write!(w,
            "rootfs / rootfs rw 0 0\n\
             sysfs /sys sysfs rw,nosuid,nodev,noexec,relatime 0 0\n\
             proc /proc proc rw,nosuid,nodev,noexec,relatime 0 0\n\
             devtmpfs /dev devtmpfs rw,nosuid,size=65536k,nr_inodes=16384 0 0\n\
             tmpfs /tmp tmpfs rw,nosuid,nodev 0 0\n\
             tmpfs /run tmpfs rw,nosuid,nodev,mode=755 0 0\n");
        return w.len() as i64;
    }

    if starts!(b"/proc/stat") {
        let jiffies = sigma_clock_ns() / 1_000_000;
        let ncpu = sigma_acpi_cpu_count().max(1);
        let _ = write!(w, "cpu  {jiffies} 0 0 0 0 0 0 0 0 0\n");
        for i in 0..ncpu {
            let _ = write!(w, "cpu{i} {jiffies} 0 0 0 0 0 0 0 0 0\n");
        }
        let _ = write!(w,
            "intr 0\nbtime {}\nprocesses {}\nprocs_running 1\nprocs_blocked 0\n",
            sigma_clock_ns() / 1_000_000_000, sigma_task_count());
        return w.len() as i64;
    }

    if starts!(b"/proc/sys/kernel/hostname") {
        let _ = write!(w, "sigmaos\n");
        return w.len() as i64;
    }

    if starts!(b"/proc/sys/kernel/osrelease") {
        let _ = write!(w, "6.1.0-sigmaos\n");
        return w.len() as i64;
    }

    if starts!(b"/proc/sys/kernel/ostype") {
        let _ = write!(w, "Linux\n");
        return w.len() as i64;
    }

    if starts!(b"/proc/net/if_inet6") || starts!(b"/proc/net/dev") {
        let _ = write!(w, "Inter-|   Receive                                                |  Transmit\n\
             face |bytes    packets errs drop fifo frame compressed multicast|\
             bytes    packets errs drop fifo colls carrier compressed\n\
             lo:       0       0    0    0    0     0          0         0        0       0    0    0    0     0       0          0\n\
             eth0:     0       0    0    0    0     0          0         0        0       0    0    0    0     0       0          0\n");
        return w.len() as i64;
    }

    -1 // not handled
}

#[no_mangle]
pub unsafe extern "C" fn procfs_read_c(
    path: *const u8, path_len: usize,
    buf: *mut u8, buf_len: usize,
) -> i64 {
    if path.is_null() || buf.is_null() { return -22; }
    let p = core::slice::from_raw_parts(path, path_len);
    let b = core::slice::from_raw_parts_mut(buf, buf_len);
    procfs_read(p, b)
}
