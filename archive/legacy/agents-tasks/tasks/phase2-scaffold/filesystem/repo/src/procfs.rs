// SPDX-License-Identifier: MIT
// SigmaOS - procfs (Process Information Pseudo-filesystem) Implementation
// Inspired by Linux procfs (fs/proc/) providing process and kernel information
// in a virtual filesystem mounted at /proc

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;

/// Process state matching Linux task states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Running,
    Sleeping,
    DiskSleep,
    Stopped,
    Zombie,
    Dead,
}

impl ProcessState {
    pub fn to_char(&self) -> char {
        match self {
            ProcessState::Running => 'R',
            ProcessState::Sleeping => 'S',
            ProcessState::DiskSleep => 'D',
            ProcessState::Stopped => 'T',
            ProcessState::Zombie => 'Z',
            ProcessState::Dead => 'X',
        }
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'R' => Some(ProcessState::Running),
            'S' => Some(ProcessState::Sleeping),
            'D' => Some(ProcessState::DiskSleep),
            'T' => Some(ProcessState::Stopped),
            'Z' => Some(ProcessState::Zombie),
            'X' => Some(ProcessState::Dead),
            _ => None,
        }
    }
}

/// Process information structure
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub ppid: u32,
    pub state: ProcessState,
    pub comm: String,
    pub utime: u64,
    pub stime: u64,
    pub vsize: u64,
    pub rss: u64,
}

/// Procfs node representing a file or directory
#[derive(Debug, Clone)]
pub struct ProcNode {
    pub name: String,
    pub is_dir: bool,
    pub children: BTreeMap<String, ProcNode>,
    pub data_generator: fn() -> String,
}

impl ProcNode {
    pub fn new(name: &str, is_dir: bool, data_generator: fn() -> String) -> Self {
        Self {
            name: name.to_string(),
            is_dir,
            children: BTreeMap::new(),
            data_generator,
        }
    }
}

/// Procfs virtual filesystem root
pub struct ProcFs {
    pub root: ProcNode,
    pub processes: BTreeMap<u32, ProcessInfo>,
    pub boot_time: u64,
}

impl ProcFs {
    pub fn new() -> Self {
        let mut root = ProcNode::new("", true, || String::new());

        // Initialize standard procfs entries
        Self::initialize_system_entries(&mut root);

        Self {
            root,
            processes: BTreeMap::new(),
            boot_time: 1700000000, // Unix timestamp
        }
    }

    fn initialize_system_entries(root: &mut ProcNode) {
        // /proc/meminfo - Memory information
        root.children.insert(
            "meminfo".to_string(),
            ProcNode::new("meminfo", false, || {
                format!(
                    "MemTotal:        4194304 kB\n\
                     MemFree:         2048576 kB\n\
                     MemAvailable:    3072000 kB\n\
                     Buffers:          262144 kB\n\
                     Cached:          1048576 kB\n\
                     SwapCached:            0 kB\n\
                     Active:          1310720 kB\n\
                     Inactive:         786432 kB\n\
                     Active(anon):     786432 kB\n\
                     Inactive(anon):   262144 kB\n\
                     Active(file):     524288 kB\n\
                     Inactive(file):   524288 kB\n\
                     Unevictable:           0 kB\n\
                     Mlocked:               0 kB\n\
                     SwapTotal:       2097152 kB\n\
                     SwapFree:        2097152 kB\n\
                     Dirty:                0 kB\n\
                     Writeback:            0 kB\n\
                     AnonPages:        786432 kB\n\
                     Mapped:           262144 kB\n\
                     Shmem:                0 kB\n\
                     Slab:             131072 kB\n\
                     SReclaimable:      65536 kB\n\
                     SUnreclaim:        65536 kB\n\
                     KernelStack:       16384 kB\n\
                     PageTables:        32768 kB\n\
                     NFS_Unstable:          0 kB\n\
                     Bounce:                0 kB\n\
                     WritebackTmp:          0 kB\n\
                     CommitLimit:     4194304 kB\n\
                     Committed_AS:    1048576 kB\n\
                     VmallocTotal:   34359738367 kB\n\
                     VmallocUsed:           0 kB\n\
                     VmallocChunk:          0 kB\n\
                     HardwareCorrupted:     0 kB\n\
                     AnonHugePages:         0 kB\n\
                     ShmemHugePages:        0 kB\n\
                     ShmemPmdMapped:        0 kB\n\
                     CmaTotal:              0 kB\n\
                     CmaFree:               0 kB\n\
                     HugePages_Total:       0\n\
                     HugePages_Free:        0\n\
                     HugePages_Rsvd:        0\n\
                     HugePages_Surp:        0\n\
                     Hugepagesize:       2048 kB\n\
                     DirectMap4k:        65536 kB\n\
                     DirectMap2M:      2097152 kB\n\
                     DirectMap1G:           0 kB\n"
                )
            }),
        );

        // /proc/cpuinfo - CPU information
        root.children.insert(
            "cpuinfo".to_string(),
            ProcNode::new("cpuinfo", false, || {
                format!(
                    "processor\t: 0\n\
                     vendor_id\t: GenuineIntel\n\
                     cpu family\t: 6\n\
                     model\t\t: 158\n\
                     model name\t: Intel(R) Core(TM) i7-10700K CPU @ 3.80GHz\n\
                     stepping\t: 10\n\
                     microcode\t: 0xea\n\
                     cpu MHz\t\t: 3800.000\n\
                     cache size\t: 16384 KB\n\
                     physical id\t: 0\n\
                     siblings\t: 8\n\
                     core id\t\t: 0\n\
                     cpu cores\t: 8\n\
                     apicid\t\t: 0\n\
                     initial apicid\t: 0\n\
                     fpu\t\t: yes\n\
                     fpu_exception\t: yes\n\
                     cpuid level\t: 22\n\
                     wp\t\t: yes\n\
                     flags\t\t: fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ss ht syscall nx pdpe1gb rdtscp lm constant_tsc rep_good nopl xtopology nonstop_tsc cpuid tsc_known_freq pni pclmulqdq ssse3 fma cx16 pcid sse4_1 sse4_2 x2apic movbe popcnt aes xsave avx f16c rdrand hypervisor lahf_lm abm 3dnowprefetch invpcid_single ssbd ibrs ibpb stibp ibrs_enhanced fsgsbase tsc_adjust bmi1 avx2 smep bmi2 erms invpcid rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves clzero xsaveerptr wbnoinvd arat npt nrip_save tsc_scale vmx arch_capabilities\n\
                     vmx flags\t: vnmi preempt invvpid ept_x_only flexpriority tsc_offset vtpr mtf vapic ept vpid unrestricted_guest ple shadow_vmcs pml ept_mode_based_exec\n\
                     bugs\t\t: spectre_v1 spectre_v2 spec_store_bypass swapgs mds\n\
                     bogomips\t: 7600.00\n\
                     clflush size\t: 64\n\
                     cache_alignment\t: 64\n\
                     address sizes\t: 39 bits physical, 48 bits virtual\n\
                     power management:\n"
                )
            }),
        );

        // /proc/uptime - System uptime
        root.children.insert(
            "uptime".to_string(),
            ProcNode::new("uptime", false, || {
                "1234.56 789.01\n".to_string()
            }),
        );

        // /proc/version - Kernel version
        root.children.insert(
            "version".to_string(),
            ProcNode::new("version", false, || {
                "SigmaOS version 0.1.0 (rustc 1.70.0) #1 SMP PREEMPT_RT Sat Oct 11 20:00:00 UTC 2026\n".to_string()
            }),
        );

        // /proc/cmdline - Kernel command line
        root.children.insert(
            "cmdline".to_string(),
            ProcNode::new("cmdline", false, || {
                "BOOT_IMAGE=/vmlinuz-0.1.0 root=/dev/sda1 ro quiet splash\n".to_string()
            }),
        );

        // /proc/mounts - Mounted filesystems
        root.children.insert(
            "mounts".to_string(),
            ProcNode::new("mounts", false, || {
                "/dev/sda1 / ext4 rw,relatime 0 0\n\
                 /dev/sda2 /home ext4 rw,relatime 0 0\n\
                 devtmpfs /dev devtmpfs rw,nosuid,relatime,size=4096k,nr_inodes=1024 0 0\n\
                 proc /proc proc rw,nosuid,nodev,noexec,relatime 0 0\n\
                 sysfs /sys sysfs rw,nosuid,nodev,noexec,relatime 0 0\n\
                 tmpfs /run tmpfs rw,nosuid,nodev,noexec,relatime,size=8192k 0 0\n".to_string()
            }),
        );

        // /proc/filesystems - Supported filesystems
        root.children.insert(
            "filesystems".to_string(),
            ProcNode::new("filesystems", false, || {
                "nodev\tsysfs\n\
                 nodev\ttmpfs\n\
                 nodev\tdevtmpfs\n\
                 nodev\tproc\n\
                 nodev\tconfigfs\n\
                 nodev\tdebugfs\n\
                 nodev\ttracefs\n\
                 \text4\n\
                 \tbtrfs\n\
                 \tzfs\n\
                 \tntfs\n\
                 \tfat32\n\
                 \texfat\n\
                 \tsquashfs\n".to_string()
            }),
        );

        // /proc/interrupts - Interrupt statistics
        root.children.insert(
            "interrupts".to_string(),
            ProcNode::new("interrupts", false, || {
                "           CPU0       CPU1       CPU2       CPU3       CPU4       CPU5       CPU6       CPU7\n\
                 0:         23          0          0          0          0          0          0          0   IO-APIC   2-edge      timer\n\
                 1:          5          0          0          0          0          0          0          0   IO-APIC   1-edge      i8042\n\
                 8:          1          0          0          0          0          0          0          0   IO-APIC   8-edge      rtc0\n\
                 9:          0          0          0          0          0          0          0          0   IO-APIC   9-fasteoi   acpi\n\
                 10:      1234         56         78         90        102        114        126        138   IO-APIC  10-fasteoi   eth0\n\
                 12:          0          0          0          0          0          0          0          0   IO-APIC  12-fasteoi   i8042\n\
                 NMI:          0          0          0          0          0          0          0          0   Non-maskable interrupts\n\
                 LOC:     56789      12345      23456      34567      45678      56789      67890      78901   Local timer interrupts\n\
                 SPU:          0          0          0          0          0          0          0          0   Spurious interrupts\n\
                 PMI:          0          0          0          0          0          0          0          0   Performance monitoring interrupts\n\
                 IWI:          0          0          0          0          0          0          0          0   IRQ work interrupts\n\
                 RTR:          0          0          0          0          0          0          0          0   APIC ICR write retries\n\
                 RES:     12345      23456      34567      45678      56789      67890      78901      89012   Rescheduling interrupts\n\
                 CAL:       2345      3456      4567      5678      6789      7890      8901      9012   Function call interrupts\n\
                 TLB:       1234      2345      3456      4567      5678      6789      7890      8901   TLB shootdowns\n\
                 TRM:          0          0          0          0          0          0          0          0   Thermal event interrupts\n\
                 THR:          0          0          0          0          0          0          0          0   Threshold APIC interrupts\n\
                 DFR:          0          0          0          0          0          0          0          0   Deferred Error APIC interrupts\n\
                 MCE:          0          0          0          0          0          0          0          0   Machine check exceptions\n\
                 MCP:          0          0          0          0          0          0          0          0   Machine check polls\n\
                 ERR:          0\n\
                 MIS:          0\n".to_string()
            }),
        );

        // /proc/loadavg - System load average
        root.children.insert(
            "loadavg".to_string(),
            ProcNode::new("loadavg", false, || {
                "0.45 0.67 0.89 2/1234 5678\n".to_string()
            }),
        );

        // /proc/stat - Kernel statistics
        root.children.insert(
            "stat".to_string(),
            ProcNode::new("stat", false, || {
                "cpu  123456 234567 345678 456789 567890 678901 789012 890123 0 0\n\
                 cpu0 15432 29321 43210 57100 70989 84878 98767 112656 0 0\n\
                 cpu1 15432 29321 43210 57100 70989 84878 98767 112656 0 0\n\
                 cpu2 15432 29321 43210 57100 70989 84878 98767 112656 0 0\n\
                 cpu3 15432 29321 43210 57100 70989 84878 98767 112656 0 0\n\
                 cpu4 15432 29321 43210 57100 70989 84878 98767 112656 0 0\n\
                 cpu5 15432 29321 43210 57100 70989 84878 98767 112656 0 0\n\
                 cpu6 15432 29321 43210 57100 70989 84878 98767 112656 0 0\n\
                 cpu7 15432 29321 43210 57100 70989 84878 98767 112656 0 0\n\
                 intr 12345678 234567 345678 456789 567890 678901 789012 890123 901234 1012345\n\
                 ctxt 987654321\n\
                 btime 1700000000\n\
                 processes 12345\n\
                 procs_running 5\n\
                 procs_blocked 2\n\
                 softirq 12345678 2345678 345678 456789 567890 678901 789012 890123 901234 1012345\n".to_string()
            }),
        );
    }

    /// Add a process to procfs
    pub fn add_process(&mut self, process: ProcessInfo) {
        let pid = process.pid;
        let mut proc_dir = ProcNode::new(&pid.to_string(), true, || String::new());

        // /proc/[pid]/status - Process status
        let pid_clone = pid;
        let comm_clone = process.comm.clone();
        let state_clone = process.state;
        let ppid_clone = process.ppid;
        let vsize_clone = process.vsize;
        let rss_clone = process.rss;
        let utime_clone = process.utime;
        let stime_clone = process.stime;

        proc_dir.children.insert(
            "status".to_string(),
            ProcNode::new("status", false, move || {
                format!(
                    "Name:\t{}\n\
                     Umask:\t0002\n\
                     State:\t{} (running)\n\
                     Tgid:\t{}\n\
                     Ngid:\t0\n\
                     Pid:\t{}\n\
                     PPid:\t{}\n\
                     TracerPid:\t0\n\
                     Uid:\t1000\t1000\t1000\t1000\n\
                     Gid:\t1000\t1000\t1000\t1000\n\
                     FDSize:\t256\n\
                     Groups:\t1000\n\
                     NStgid:\t{}\n\
                     NSpid:\t{}\n\
                     NSpgid:\t1234\n\
                     NSsid:\t5678\n\
                     VmPeak:\t{} kB\n\
                     VmSize:\t{} kB\n\
                     VmLck:\t0 kB\n\
                     VmPin:\t0 kB\n\
                     VmHWM:\t{} kB\n\
                     VmRSS:\t{} kB\n\
                     RssAnon:\t{} kB\n\
                     RssFile:\t{} kB\n\
                     RssShmem:\t0 kB\n\
                     VmData:\t131072 kB\n\
                     VmStk:\t8192 kB\n\
                     VmExe:\t4096 kB\n\
                     VmLib:\t65536 kB\n\
                     VmPTE:\t1024 kB\n\
                     VmSwap:\t0 kB\n\
                     HugetlbPages:\t0 kB\n\
                     Threads:\t1\n\
                     SigQ:\t0/12345\n\
                     SigPnd:\t0000000000000000\n\
                     ShdPnd:\t0000000000000000\n\
                     SigBlk:\t0000000000000000\n\
                     SigIgn:\t0000000000000000\n\
                     SigCgt:\t0000000000000000\n\
                     CapInh:\t0000000000000000\n\
                     CapPrm:\t0000000000000000\n\
                     CapEff:\t0000000000000000\n\
                     CapBnd:\t0000000000000000\n\
                     CapAmb:\t0000000000000000\n\
                     NoNewPrivs:\t0\n\
                     Seccomp:\t0\n\
                     Speculation_Store_Bypass:\tthread vulnerable\n\
                     SpeculationRet:\tthread vulnerable\n\
                     Cpus_allowed:\tff\n\
                     Cpus_allowed_list:\t0-7\n\
                     Mems_allowed:\t00000000,00000001\n\
                     Mems_allowed_list:\t0\n\
                     voluntary_context_switches:\t1234\n\
                     nonvoluntary_context_switches:\t56\n",
                    comm_clone,
                    state_clone.to_char(),
                    pid_clone,
                    pid_clone,
                    ppid_clone,
                    pid_clone,
                    pid_clone,
                    vsize_clone,
                    vsize_clone,
                    rss_clone,
                    rss_clone,
                    rss_clone / 2,
                    rss_clone / 2
                )
            }),
        );

        // /proc/[pid]/cmdline - Process command line
        let comm_clone2 = process.comm.clone();
        proc_dir.children.insert(
            "cmdline".to_string(),
            ProcNode::new("cmdline", false, move || {
                format!("{}\0", comm_clone2)
            }),
        );

        // /proc/[pid]/stat - Process statistics
        let pid_clone2 = pid;
        let comm_clone3 = process.comm.clone();
        let state_clone2 = process.state;
        let ppid_clone2 = process.ppid;
        let utime_clone2 = process.utime;
        let stime_clone2 = process.stime;
        let vsize_clone2 = process.vsize;
        let rss_clone2 = process.rss;

        proc_dir.children.insert(
            "stat".to_string(),
            ProcNode::new("stat", false, move || {
                format!(
                    "{} ({}) {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
                    pid_clone2,
                    comm_clone3,
                    state_clone2.to_char(),
                    ppid_clone2,
                    1234, // pgrp
                    5678, // session
                    1,    // tty_nr
                    0,    // tpgid
                    0,    // flags
                    utime_clone2,
                    stime_clone2,
                    0,    // cutime
                    0,    // cstime
                    0,    // priority
                    0,    // nice
                    0,    // num_threads
                    0,    // itrealvalue
                    0,    // starttime
                    vsize_clone2,
                    rss_clone2,
                    0,    // rsslim
                    0,    // startcode
                    0,    // endcode
                    0,    // startstack
                    0,    // kstkesp
                    0,    // kstkeip
                    0,    // signal
                    0,    // blocked
                    0,    // sigignore
                    0,    // sigcatch
                    0,    // wchan
                    0,    // nswap
                    0,    // cnswap
                    0,    // exit_signal
                    0,    // processor
                    0,    // rt_priority
                    0,    // policy
                    0,    // delayacct_blkio_ticks
                    0,    // guest_time
                    0,    // cguest_time
                    0,    // start_data
                    0,    // end_data
                    0,    // start_brk
                    0,    // arg_start
                    0,    // arg_end
                    0,    // env_start
                    0,    // env_end
                    0     // exit_code
                )
            }),
        );

        // /proc/[pid]/maps - Memory maps
        proc_dir.children.insert(
            "maps".to_string(),
            ProcNode::new("maps", false, || {
                "00400000-00452000 r-xp 00000000 08:01 1234 /usr/bin/bash\n\
                 00652000-00653000 r--p 00012000 08:01 1234 /usr/bin/bash\n\
                 00653000-00654000 rw-p 00013000 08:01 1234 /usr/bin/bash\n\
                 00654000-00675000 rw-p 00000000 00:00 0 [heap]\n\
                 7f8e00000000-7f8e00201000 r-xp 00000000 08:01 5678 /usr/lib/x86_64-linux-gnu/libc-2.31.so\n\
                 7f8e00201000-7f8e00401000 ---p 00000000 00:00 0\n\
                 7f8e00401000-7f8e00405000 r--p 00201000 08:01 5678 /usr/lib/x86_64-linux-gnu/libc-2.31.so\n\
                 7ffc0d3ff000-7ffc0d420000 rw-p 00000000 00:00 0 [stack]\n\
                 7ffc0d434000-7ffc0d438000 r--p 00000000 00:00 0 [vvar]\n\
                 7ffc0d438000-7ffc0d43a000 r-xp 00000000 00:00 0 [vdso]\n\
                 ffffffffff600000-ffffffffff601000 r-xp 00000000 00:00 0 [vsyscall]\n".to_string()
            }),
        );

        // /proc/[pid]/fd - File descriptors directory
        let mut fd_dir = ProcNode::new("fd", true, || String::new());
        fd_dir.children.insert(
            "0".to_string(),
            ProcNode::new("0", false, || "/dev/pts/0".to_string()),
        );
        fd_dir.children.insert(
            "1".to_string(),
            ProcNode::new("1", false, || "/dev/pts/0".to_string()),
        );
        fd_dir.children.insert(
            "2".to_string(),
            ProcNode::new("2", false, || "/dev/pts/0".to_string()),
        );
        proc_dir.children.insert("fd".to_string(), fd_dir);

        self.root.children.insert(pid.to_string(), proc_dir);
        self.processes.insert(pid, process);
    }

    /// Remove a process from procfs
    pub fn remove_process(&mut self, pid: u32) {
        self.root.children.remove(&pid.to_string());
        self.processes.remove(&pid);
    }

    /// Read a path in procfs
    pub fn read_path(&self, path: &str) -> Option<String> {
        let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        let mut current = &self.root;

        for part in parts {
            if let Some(child) = current.children.get(part) {
                current = child;
            } else {
                return None;
            }
        }

        if current.is_dir {
            None
        } else {
            Some((current.data_generator)())
        }
    }

    /// List children of a procfs directory
    pub fn list_children(&self, path: &str) -> Vec<String> {
        let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        let mut current = &self.root;

        for part in parts {
            if let Some(child) = current.children.get(part) {
                current = child;
            } else {
                return Vec::new();
            }
        }

        let mut result = Vec::new();
        for name in current.children.keys() {
            result.push(name.clone());
        }
        result
    }
}

impl Default for ProcFs {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_procfs_initialization() {
        let procfs = ProcFs::new();
        assert!(procfs.root.children.contains_key("meminfo"));
        assert!(procfs.root.children.contains_key("cpuinfo"));
        assert!(procfs.root.children.contains_key("uptime"));
        assert!(procfs.root.children.contains_key("version"));
    }

    #[test]
    fn test_procfs_read_meminfo() {
        let procfs = ProcFs::new();
        let meminfo = procfs.read_path("meminfo");
        assert!(meminfo.is_some());
        assert!(meminfo.unwrap().contains("MemTotal"));
    }

    #[test]
    fn test_procfs_add_process() {
        let mut procfs = ProcFs::new();
        let process = ProcessInfo {
            pid: 1234,
            ppid: 1,
            state: ProcessState::Running,
            comm: "test_process".to_string(),
            utime: 100,
            stime: 50,
            vsize: 8192,
            rss: 4096,
        };
        procfs.add_process(process);

        assert!(procfs.root.children.contains_key("1234"));
        assert!(procfs.processes.contains_key(&1234));
    }

    #[test]
    fn test_procfs_read_process_status() {
        let mut procfs = ProcFs::new();
        let process = ProcessInfo {
            pid: 5678,
            ppid: 1,
            state: ProcessState::Sleeping,
            comm: "sleeping_process".to_string(),
            utime: 200,
            stime: 100,
            vsize: 16384,
            rss: 8192,
        };
        procfs.add_process(process);

        let status = procfs.read_path("5678/status");
        assert!(status.is_some());
        assert!(status.unwrap().contains("sleeping_process"));
    }

    #[test]
    fn test_process_state_conversion() {
        assert_eq!(ProcessState::Running.to_char(), 'R');
        assert_eq!(ProcessState::Zombie.to_char(), 'Z');
        assert_eq!(ProcessState::from_char('S'), Some(ProcessState::Sleeping));
        assert_eq!(ProcessState::from_char('X'), Some(ProcessState::Dead));
        assert_eq!(ProcessState::from_char('?'), None);
    }
}
