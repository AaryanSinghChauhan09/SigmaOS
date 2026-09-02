extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
// Linux-inspired Process & ProcFS Emulation for SigmaOS
// Implements advanced process hierarchies, PID namespace isolation, nice priorities, cgroups, signal handling, and dynamic /proc pseudo-filesystem.

use crate::klib::HashMap;
use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinuxProcessState {
    Running,
    Sleeping,
    Stopped,
    Zombie,
    Terminated,
}

impl LinuxProcessState {
    pub fn as_str(&self) -> &'static str {
        match self {
            LinuxProcessState::Running => "R (running)",
            LinuxProcessState::Sleeping => "S (sleeping)",
            LinuxProcessState::Stopped => "T (stopped)",
            LinuxProcessState::Zombie => "Z (zombie)",
            LinuxProcessState::Terminated => "X (dead)",
        }
    }
}

/// Linux-inspired nice values ranging from -20 (highest priority) to 19 (lowest priority).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NiceValue(i32);

impl NiceValue {
    pub fn new(val: i32) -> Self {
        NiceValue(val.clamp(-20, 19))
    }

    pub fn value(&self) -> i32 {
        self.0
    }

    /// Convert Linux Nice value to dynamic Priority levels
    pub fn to_priority(&self) -> i32 {
        // -20 maps to highest priority (e.g., 5), 19 maps to lowest priority (e.g., 0)
        if self.0 <= -15 {
            5 // Critical / Realtime
        } else if self.0 <= -5 {
            4 // High
        } else if self.0 <= 5 {
            3 // Normal
        } else if self.0 <= 15 {
            2 // Low
        } else {
            1 // Idle
        }
    }
}

/// Control Group (cgroup) for tracking resource utilization boundaries.
#[derive(Debug, Clone)]
pub struct CGroup {
    pub name: String,
    pub memory_limit: usize, // in bytes
    pub cpu_weight: usize,   // CPU shares allocation
    pub pids: Vec<usize>,
}

impl CGroup {
    pub fn new(name: &str, memory_limit: usize, cpu_weight: usize) -> Self {
        Self {
            name: name.to_string(),
            memory_limit,
            cpu_weight,
            pids: Vec::new(),
        }
    }
}

/// PID Namespace Isolation
#[derive(Debug, Clone)]
pub struct PidNamespace {
    pub id: usize,
    pub parent_id: Option<usize>,
    /// Maps Virtual (local) PID to Global (real) PID
    pub local_to_global: HashMap<usize, usize>,
    /// Maps Global (real) PID to Virtual (local) PID
    pub global_to_local: HashMap<usize, usize>,
    next_local_pid: usize,
}

impl PidNamespace {
    pub fn new(id: usize, parent_id: Option<usize>) -> Self {
        Self {
            id,
            parent_id,
            local_to_global: HashMap::new(),
            global_to_local: HashMap::new(),
            next_local_pid: 1,
        }
    }

    pub fn register_pid(&mut self, global_pid: usize) -> usize {
        let local_pid = self.next_local_pid;
        self.next_local_pid += 1;
        self.local_to_global.insert(local_pid, global_pid);
        self.global_to_local.insert(global_pid, local_pid);
        local_pid
    }

    pub fn get_global(&self, local_pid: usize) -> Option<usize> {
        self.local_to_global.get(&local_pid).copied()
    }

    pub fn get_local(&self, global_pid: usize) -> Option<usize> {
        self.global_to_local.get(&global_pid).copied()
    }
}

/// Process Entry for our simulated system
#[derive(Debug, Clone)]
pub struct LinuxProcessEntry {
    pub pid: usize,
    pub ppid: usize,
    pub pgid: usize,
    pub sid: usize,
    pub name: String,
    pub state: LinuxProcessState,
    pub nice: NiceValue,
    pub cgroup_name: String,
    pub cmdline: String,
    pub cpu_time: u64,
    pub memory_usage: usize, // in bytes
    pub thread_count: usize,
}

impl LinuxProcessEntry {
    pub fn new(
        pid: usize,
        ppid: usize,
        pgid: usize,
        sid: usize,
        name: &str,
        nice: NiceValue,
        cgroup_name: &str,
        cmdline: &str,
    ) -> Self {
        Self {
            pid,
            ppid,
            pgid,
            sid,
            name: name.to_string(),
            state: LinuxProcessState::Running,
            nice,
            cgroup_name: cgroup_name.to_string(),
            cmdline: cmdline.to_string(),
            cpu_time: 1500,
            memory_usage: 4096000,
            thread_count: 1,
        }
    }
}

/// Linux Process Signals
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinuxSignal {
    SigInt = 2,
    SigKill = 9,
    SigTerm = 15,
}

/// The Core Proc pseudo-filesystem manager
pub struct ProcFileSystem {
    pub processes: HashMap<usize, LinuxProcessEntry>,
    pub cgroups: HashMap<String, CGroup>,
    pub namespaces: HashMap<usize, PidNamespace>,
    pub active_namespace_id: usize,
    pub system_uptime: u64, // Simulated seconds
    pub total_memory: usize,
    pub used_memory: usize,
    pub cpu_model: String,
    pub cpu_cores: usize,
}

impl ProcFileSystem {
    pub fn new() -> Self {
        let mut pfs = Self {
            processes: HashMap::new(),
            cgroups: HashMap::new(),
            namespaces: HashMap::new(),
            active_namespace_id: 0,
            system_uptime: 43200,          // 12 hours
            total_memory: 16777216 * 1024, // 16GB
            used_memory: 4194304 * 1024,   // 4GB
            cpu_model: "Sigma Core AI-Native 9".to_string(),
            cpu_cores: 16,
        };

        // Create default root cgroups
        pfs.cgroups.insert(
            "system.slice".to_string(),
            CGroup::new("system.slice", 8 * 1024 * 1024 * 1024, 1024),
        );
        pfs.cgroups.insert(
            "user.slice".to_string(),
            CGroup::new("user.slice", 8 * 1024 * 1024 * 1024, 1024),
        );

        // Create root PID namespace
        let mut root_ns = PidNamespace::new(0, None);
        root_ns.register_pid(1); // systemd/init
        root_ns.register_pid(2); // kthreadd
        pfs.namespaces.insert(0, root_ns);

        // Add standard startup processes
        pfs.processes.insert(
            1,
            LinuxProcessEntry::new(
                1,
                0,
                1,
                1,
                "systemd",
                NiceValue::new(0),
                "system.slice",
                "/sbin/init",
            ),
        );
        pfs.processes.insert(
            2,
            LinuxProcessEntry::new(
                2,
                0,
                0,
                0,
                "kthreadd",
                NiceValue::new(-20),
                "system.slice",
                "[kthreadd]",
            ),
        );

        pfs
    }

    /// Create or spawn a new process in the simulated OS
    pub fn spawn_process(
        &mut self,
        name: &str,
        ppid: usize,
        nice: NiceValue,
        cgroup: &str,
        cmdline: &str,
    ) -> usize {
        let next_pid = self.processes.keys().copied().max().unwrap_or(0) + 1;
        let pgid = ppid; // Default to parent's pgid
        let sid = ppid; // Default to parent's session id

        let mut entry =
            LinuxProcessEntry::new(next_pid, ppid, pgid, sid, name, nice, cgroup, cmdline);

        // Add to cgroup list of PIDs
        if let Some(cg) = self.cgroups.get_mut(cgroup) {
            cg.pids.push(next_pid);
        }

        // Add to active namespace
        if let Some(ns) = self.namespaces.get_mut(&self.active_namespace_id) {
            ns.register_pid(next_pid);
        }

        self.processes.insert(next_pid, entry);
        next_pid
    }

    /// Fork an existing process
    pub fn fork_process(&mut self, parent_pid: usize) -> Result<usize, String> {
        let parent = self
            .processes
            .get(&parent_pid)
            .ok_or("Parent process not found")?
            .clone();
        let next_pid = self.processes.keys().copied().max().unwrap_or(0) + 1;

        let mut child = parent.clone();
        child.pid = next_pid;
        child.ppid = parent_pid;
        child.state = LinuxProcessState::Running;

        if let Some(cg) = self.cgroups.get_mut(&parent.cgroup_name) {
            cg.pids.push(next_pid);
        }

        if let Some(ns) = self.namespaces.get_mut(&self.active_namespace_id) {
            ns.register_pid(next_pid);
        }

        self.processes.insert(next_pid, child);
        Ok(next_pid)
    }

    /// Re-parent orphans when a parent exits, and clean up / turn to zombie.
    /// If parent has already waited, we fully reap. Otherwise, turns to zombie.
    pub fn exit_process(&mut self, pid: usize, exit_code: i32) -> Result<(), String> {
        if pid == 1 {
            return Err("Cannot exit system init process (PID 1)".to_string());
        }

        // 1. Re-parent children of this process to init (PID 1)
        for proc in self.processes.values_mut() {
            if proc.ppid == pid {
                proc.ppid = 1;
            }
        }

        // 2. Set state to Zombie
        if let Some(proc) = self.processes.get_mut(&pid) {
            proc.state = LinuxProcessState::Zombie;
        } else {
            return Err("Process not found".to_string());
        }

        Ok(())
    }

    /// Reap a zombie process via wait / waitpid simulation
    pub fn reap_process(&mut self, pid: usize) -> Result<(), String> {
        if let Some(proc) = self.processes.get(&pid) {
            if proc.state == LinuxProcessState::Zombie {
                // Remove from cgroup
                if let Some(cg) = self.cgroups.get_mut(&proc.cgroup_name) {
                    cg.pids.retain(|&p| p != pid);
                }
                self.processes.remove(&pid);
                Ok(())
            } else {
                Err("Process is not in a zombie state".to_string())
            }
        } else {
            Err("Process not found".to_string())
        }
    }

    /// Forward a signal to a process or cascade to process group (if negative pid is passed)
    pub fn send_signal(&mut self, target: i32, signal: LinuxSignal) -> Result<(), String> {
        if target < 0 {
            // Signal process group
            let pgid = (-target) as usize;
            let mut pids_to_kill = Vec::new();
            for proc in self.processes.values() {
                if proc.pgid == pgid {
                    pids_to_kill.push(proc.pid);
                }
            }
            for pid in pids_to_kill {
                self.apply_signal_to_process(pid, signal)?;
            }
        } else {
            // Single process
            self.apply_signal_to_process(target as usize, signal)?;
        }
        Ok(())
    }

    fn apply_signal_to_process(&mut self, pid: usize, signal: LinuxSignal) -> Result<(), String> {
        match signal {
            LinuxSignal::SigKill => {
                if pid == 1 {
                    return Err("Cannot SIGKILL PID 1".to_string());
                }
                self.exit_process(pid, 137)?;
                self.reap_process(pid)?; // SIGKILL immediately reaps if simulated
            }
            LinuxSignal::SigTerm => {
                if pid == 1 {
                    return Err("Cannot SIGTERM PID 1".to_string());
                }
                self.exit_process(pid, 143)?;
            }
            LinuxSignal::SigInt => {
                if let Some(proc) = self.processes.get_mut(&pid) {
                    if pid != 1 {
                        proc.state = LinuxProcessState::Stopped;
                    }
                }
            }
        }
        Ok(())
    }

    /// Resolve virtual file read operations for path-based `/proc`
    pub fn read_file(&self, path: &str) -> Result<String, String> {
        let clean_path = path.trim_start_matches('/');
        if clean_path == "proc/meminfo" || clean_path == "meminfo" {
            Ok(self.generate_meminfo())
        } else if clean_path == "proc/cpuinfo" || clean_path == "cpuinfo" {
            Ok(self.generate_cpuinfo())
        } else if clean_path == "proc/uptime" || clean_path == "uptime" {
            Ok(self.generate_uptime())
        } else if clean_path == "proc/cgroups" || clean_path == "cgroups" {
            Ok(self.generate_cgroups())
        } else {
            // Check for /proc/<pid>/status, /proc/<pid>/cmdline, /proc/<pid>/stat
            let parts: Vec<&str> = clean_path.split('/').collect();
            if parts.len() >= 2 && parts[0] == "proc" {
                if let Ok(pid) = parts[1].parse::<usize>() {
                    if parts.len() == 3 {
                        match parts[2] {
                            "status" => return self.generate_status(pid),
                            "cmdline" => return self.generate_cmdline(pid),
                            "stat" => return self.generate_stat(pid),
                            _ => {}
                        }
                    }
                }
            }
            Err(format!("File /{} not found in ProcFS", clean_path))
        }
    }

    pub fn generate_meminfo(&self) -> String {
        let free_mem = self.total_memory - self.used_memory;
        format!(
            "MemTotal:       {:12} kB\n\
             MemFree:        {:12} kB\n\
             MemAvailable:   {:12} kB\n\
             Buffers:              131072 kB\n\
             Cached:              2097152 kB\n\
             SwapCached:                0 kB\n\
             Active:             3145728 kB\n\
             Inactive:           1048576 kB\n",
            self.total_memory,
            free_mem,
            free_mem + 1048576
        )
    }

    pub fn generate_cpuinfo(&self) -> String {
        let mut out = String::new();
        for i in 0..self.cpu_cores {
            out.push_str(&format!(
                "processor       : {}\n\
                 vendor_id       : GenuineSigma\n\
                 cpu family      : 9\n\
                 model           : 1\n\
                 model name      : {}\n\
                 cpu MHz         : 3600.000\n\
                 cache size      : 32768 KB\n\
                 core id         : {}\n\
                 cpu cores       : {}\n\
                 flags           : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm pbe syscall nx pdpe1gb rdtscp lm constant_tsc art arch_perfmon pebs bts rep_good nopl xtopology nonstop_tsc cpuid tsc_known_freq pni pclmulqdq dtes64 monitor ds_cpl vmx smx est tm2 ssse3 sdbg fma cx16 xtpr pdcm pcid sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes xsave avx f16c rdrand lahf_lm abm 3dnowprefetch cpuid_fault epb cat_l3 cdp_l3 invpcid_single intel_pt ssbd ibrs ibpb stibp ibrs_enhanced tpr_shadow vnmi flexpriority ept vpid ept_ad fsgsbase tsc_adjust bmi1 avx2 smep bmi2 erms invpcid rtm mpx rdt_a avx512f avx512dq rdseed adx smap clflushopt clwb avx512cd avx512bw avx512vl xsaveopt xsavec xgetbv1 xsaves dtherm ida arat pln pts hwp hwp_notify hwp_act_window hwp_epp\n\n",
                i, self.cpu_model, i, self.cpu_cores
            ));
        }
        out
    }

    pub fn generate_uptime(&self) -> String {
        format!(
            "{}.25 {}.89\n",
            self.system_uptime,
            self.system_uptime * self.cpu_cores as u64
        )
    }

    pub fn generate_cgroups(&self) -> String {
        let mut out = "#subsys_name\thierarchy\tnum_cgroups\tenabled\n".to_string();
        for (name, cg) in &self.cgroups {
            out.push_str(&format!("{}\t1\t{}\t1\n", name, cg.pids.len()));
        }
        out
    }

    pub fn generate_status(&self, pid: usize) -> Result<String, String> {
        let proc = self
            .processes
            .get(&pid)
            .ok_or_else(|| format!("PID {} not found", pid))?;
        Ok(format!(
            "Name:           {}\n\
             State:          {}\n\
             Tgid:           {}\n\
             Pid:            {}\n\
             PPid:           {}\n\
             TracerPid:      0\n\
             Uid:            0       0       0       0\n\
             Gid:            0       0       0       0\n\
             FDSize:         64\n\
             Groups:         0 \n\
             NiceValue:      {}\n\
             CGroup:         {}\n\
             Threads:        {}\n",
            proc.name,
            proc.state.as_str(),
            proc.pid,
            proc.pid,
            proc.ppid,
            proc.nice.value(),
            proc.cgroup_name,
            proc.thread_count
        ))
    }

    pub fn generate_cmdline(&self, pid: usize) -> Result<String, String> {
        let proc = self
            .processes
            .get(&pid)
            .ok_or_else(|| format!("PID {} not found", pid))?;
        Ok(format!("{}\0", proc.cmdline))
    }

    pub fn generate_stat(&self, pid: usize) -> Result<String, String> {
        let proc = self
            .processes
            .get(&pid)
            .ok_or_else(|| format!("PID {} not found", pid))?;
        // Return standard space-separated values mapped to proc stat structure
        Ok(format!(
            "{} ({}) {} {} {} {} 0 0 0 0 0 0 0 {} {} 0 0 {} 0 {} 0 {} 0\n",
            proc.pid,
            proc.name,
            proc.state.as_str().chars().next().unwrap_or('R'),
            proc.ppid,
            proc.pgid,
            proc.sid,
            proc.cpu_time,
            proc.cpu_time,
            proc.nice.value(),
            proc.thread_count,
            proc.memory_usage
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nice_value_and_priority() {
        let high_nice = NiceValue::new(19);
        let low_nice = NiceValue::new(-20);
        let normal_nice = NiceValue::new(0);

        assert_eq!(high_nice.value(), 19);
        assert_eq!(low_nice.value(), -20);
        assert_eq!(normal_nice.value(), 0);

        // Verify nice-to-priority mappings
        assert_eq!(high_nice.to_priority(), 1); // lowest priority
        assert_eq!(low_nice.to_priority(), 5); // highest priority
        assert_eq!(normal_nice.to_priority(), 3); // normal priority
    }

    #[test]
    fn test_cgroup_tracking() {
        let mut cg = CGroup::new("test.slice", 1024 * 1024, 100);
        cg.pids.push(42);
        cg.pids.push(43);

        assert_eq!(cg.name, "test.slice");
        assert_eq!(cg.memory_limit, 1024 * 1024);
        assert_eq!(cg.cpu_weight, 100);
        assert_eq!(cg.pids, vec![42, 43]);
    }

    #[test]
    fn test_pid_namespace_mapping() {
        let mut ns = PidNamespace::new(1, Some(0));
        let local_1 = ns.register_pid(101);
        let local_2 = ns.register_pid(102);

        assert_eq!(local_1, 1);
        assert_eq!(local_2, 2);

        assert_eq!(ns.get_global(1), Some(101));
        assert_eq!(ns.get_global(2), Some(102));
        assert_eq!(ns.get_local(101), Some(1));
        assert_eq!(ns.get_local(102), Some(2));
    }

    #[test]
    fn test_procfs_generation() {
        let mut pfs = ProcFileSystem::new();

        // Spawn a dummy process
        let pid = pfs.spawn_process(
            "dummy-daemon",
            1,
            NiceValue::new(-10),
            "user.slice",
            "/usr/bin/dummy-daemon --arg",
        );
        assert!(pid > 2);

        // Read /proc/meminfo
        let meminfo = pfs.read_file("/proc/meminfo").unwrap();
        assert!(meminfo.contains("MemTotal:"));

        // Read /proc/cpuinfo
        let cpuinfo = pfs.read_file("/proc/cpuinfo").unwrap();
        assert!(cpuinfo.contains("GenuineSigma"));

        // Read /proc/uptime
        let uptime = pfs.read_file("/proc/uptime").unwrap();
        assert!(uptime.contains("43200"));

        // Read /proc/cgroups
        let cgroups = pfs.read_file("/proc/cgroups").unwrap();
        assert!(cgroups.contains("user.slice"));

        // Read /proc/<pid>/status
        let status = pfs.read_file(&format!("/proc/{}/status", pid)).unwrap();
        assert!(status.contains("Name:           dummy-daemon"));
        assert!(status.contains("NiceValue:      -10"));

        // Read /proc/<pid>/cmdline
        let cmdline = pfs.read_file(&format!("/proc/{}/cmdline", pid)).unwrap();
        assert!(cmdline.contains("/usr/bin/dummy-daemon --arg"));

        // Read /proc/<pid>/stat
        let stat = pfs.read_file(&format!("/proc/{}/stat", pid)).unwrap();
        assert!(stat.contains("dummy-daemon"));
    }

    #[test]
    fn test_orphan_reparenting_and_zombie_reaping() {
        let mut pfs = ProcFileSystem::new();

        // Spawn a parent and child
        let parent_pid = pfs.spawn_process("parent", 1, NiceValue::new(0), "user.slice", "parent");
        let child_pid = pfs.spawn_process(
            "child",
            parent_pid,
            NiceValue::new(0),
            "user.slice",
            "child",
        );

        // Parent exits -> child should be re-parented to init (PID 1)
        pfs.exit_process(parent_pid, 0).unwrap();

        let child_proc = pfs.processes.get(&child_pid).unwrap();
        assert_eq!(child_proc.ppid, 1);

        // Parent should turn into zombie
        let parent_proc = pfs.processes.get(&parent_pid).unwrap();
        assert_eq!(parent_proc.state, LinuxProcessState::Zombie);

        // Reap the zombie
        pfs.reap_process(parent_pid).unwrap();
        assert!(pfs.processes.get(&parent_pid).is_none());
    }

    #[test]
    fn test_process_signaling() {
        let mut pfs = ProcFileSystem::new();
        let pid = pfs.spawn_process("victim", 1, NiceValue::new(0), "user.slice", "victim");

        // Send SigInt -> victim stops
        pfs.send_signal(pid as i32, LinuxSignal::SigInt).unwrap();
        assert_eq!(
            pfs.processes.get(&pid).unwrap().state,
            LinuxProcessState::Stopped
        );

        // Send SigKill -> victim is terminated and reaped
        pfs.send_signal(pid as i32, LinuxSignal::SigKill).unwrap();
        assert!(pfs.processes.get(&pid).is_none());
    }
}
