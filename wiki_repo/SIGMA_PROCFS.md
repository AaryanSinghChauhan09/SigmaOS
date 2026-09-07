# SigmaOS procfs — /proc Virtual Filesystem

## Overview

SigmaOS provides a Linux-compatible `/proc` virtual filesystem that exposes kernel state to userspace. All content is generated dynamically — no physical storage needed.

**Location:** `src/fs/sigma_procfs.rs`

---

## /proc Layout

```
/proc/
├── cpuinfo          CPU topology, model, flags (sse4_2, avx2, aes...)
├── meminfo          Memory usage (MemTotal, MemFree, Cached, Swap...)
├── uptime           System uptime in seconds
├── loadavg          1/5/15-minute load averages + running/total processes
├── stat             CPU times per state, context switches, boot time
├── mounts           Mounted filesystems (sigma_ext, tmpfs, sysfs, proc...)
├── version          Kernel version string
├── net/dev          Network interface RX/TX statistics
├── sys/kernel/hostname
└── <pid>/
    ├── status       Name, State, PID, PPID, UID, VmRSS, Threads
    ├── stat         Raw process statistics (Linux /proc/pid/stat format)
    ├── cmdline      Null-separated command line arguments
    ├── statm        Memory usage in pages
    ├── maps         Virtual memory map (address, perms, offset, name)
    └── fd/          Open file descriptors
```

---

## API Reference

```rust
let mut pfs = ProcfsMount::new("sigmahost", "6.6.0-sigma");
pfs.set_uptime(3600.0);
pfs.set_loadavg(0.42, 0.35, 0.28);

// Register a process
pfs.add_process(ProcProcessInfo {
    pid: 1, ppid: 0, name: "init".into(),
    state: ProcProcessState::Sleeping,
    vm_rss_kb: 4096, threads: 1,
    cmdline: "/sbin/init".into(),
    ..
});

// Read any /proc path
let cpuinfo = pfs.read("cpuinfo").unwrap();
let status = pfs.read("1/status").unwrap();
let maps = pfs.read("1/maps").unwrap();
let entries = pfs.readdir("/proc").unwrap();
```

---

## Process States

| State | Char | Meaning |
|-------|------|---------|
| Running | R | Executing on CPU |
| Sleeping | S | Interruptible sleep |
| DiskSleep | D | Uninterruptible (I/O) |
| Zombie | Z | Exited, not yet reaped |
| Stopped | T | Stopped by signal/debugger |

---

## Comparison

| Feature | Linux /proc | BSD /proc | SigmaOS /proc |
|---------|------------|----------|--------------|
| cpuinfo | Yes | Limited | Yes |
| meminfo | Yes | /proc/meminfo (limited) | Yes |
| per-process maps | Yes | No | Yes |
| net/dev | Yes | No | Yes |
| no_std | No | No | **Yes** |
| Dynamic generation | Yes | Yes | Yes |
