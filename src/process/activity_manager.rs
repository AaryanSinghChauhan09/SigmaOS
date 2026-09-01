extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
// System Activity Manager for SigmaOS
// Inspired by Linux systemd cgroup activity tracking, Android ActivityManager,
// Garuda Zen interactivity governor, FreeBSD process activity accounting, and macOS Activity Monitor.

use crate::klib::HashMap;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Process activity state classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivityState {
    Active,
    Interactive,
    Background,
    Idle,
    Suspended,
    Throttled,
    Terminated,
}

impl ActivityState {
    pub fn as_str(&self) -> &'static str {
        match self {
            ActivityState::Active => "Active",
            ActivityState::Interactive => "Interactive (Foreground)",
            ActivityState::Background => "Background",
            ActivityState::Idle => "Idle",
            ActivityState::Suspended => "Suspended",
            ActivityState::Throttled => "Throttled (Power Saver)",
            ActivityState::Terminated => "Terminated",
        }
    }
}

/// Register state snapshot for process threads
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RegisterSnapshot {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub rip: u64,
    pub rflags: u64,
    pub cs: u16,
    pub ds: u16,
    pub ss: u16,
    pub es: u16,
    pub fs: u16,
    pub gs: u16,
}

/// Address space binding record for loaded executable binaries and libraries
#[derive(Debug, Clone)]
pub struct AddressSpaceBinding {
    pub binary_path: String,
    pub virtual_base: u64,
    pub size_bytes: u64,
    pub aslr_offset: u64,
    pub is_executable: bool,
    pub is_wx_compliant: bool,
    pub bound_libraries: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ApplicationPerformanceProfile {
    pub latency_score: f32,
    pub throughput_score: f32,
}

#[derive(Debug, Clone, Default)]
pub struct ProcessPledgePromises {
    pub promises: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ProcessResourceLimits {
    pub max_memory: usize,
    pub max_fds: usize,
}

#[derive(Debug, Clone, Default)]
pub struct PsiMetrics {
    pub some_avg10: f32,
    pub full_avg10: f32,
}

/// Process activity metric record
#[derive(Debug, Clone)]
pub struct ProcessActivityRecord {
    pub pid: usize,
    pub ppid: usize,
    pub name: String,
    pub state: ActivityState,
    pub cpu_usage_pct: f32,
    pub memory_footprint_bytes: usize,
    pub io_read_bytes: u64,
    pub io_write_bytes: u64,
    pub thread_count: usize,
    pub priority: i32, // Nice value (-20 to 19)
    pub is_foreground: bool,
    pub power_throttling_enabled: bool,
    pub register_snapshot: Option<RegisterSnapshot>,
    pub address_binding: Option<AddressSpaceBinding>,
    pub last_active_timestamp: u64,
}

impl ProcessActivityRecord {
    pub fn new(pid: usize, ppid: usize, name: &str, priority: i32) -> Self {
        Self {
            pid,
            ppid,
            name: name.to_string(),
            state: ActivityState::Active,
            cpu_usage_pct: 0.0,
            memory_footprint_bytes: 4096 * 1024, // Default 4MB baseline
            io_read_bytes: 0,
            io_write_bytes: 0,
            thread_count: 1,
            priority: priority.clamp(-20, 19),
            is_foreground: false,
            power_throttling_enabled: false,
            register_snapshot: None,
            address_binding: None,
            last_active_timestamp: 1000,
        }
    }
}

/// System Activity Manager orchestrator
pub struct ActivityManager {
    pub activities: HashMap<usize, ProcessActivityRecord>,
    pub active_foreground_pid: Option<usize>,
    pub total_cpu_cycles_tracked: AtomicUsize,
    pub power_saving_mode: bool,
}

impl ActivityManager {
    pub fn new() -> Self {
        Self {
            activities: HashMap::new(),
            active_foreground_pid: None,
            total_cpu_cycles_tracked: AtomicUsize::new(0),
            power_saving_mode: false,
        }
    }

    /// Register a newly spawned process for activity management
    pub fn register_process(&mut self, pid: usize, ppid: usize, name: &str, priority: i32) {
        let record = ProcessActivityRecord::new(pid, ppid, name, priority);
        self.activities.insert(pid, record);
    }

    /// Register a thread for a process
    pub fn register_thread(
        &mut self,
        pid: usize,
        _tid: usize,
        _name: &str,
    ) -> Result<(), &'static str> {
        let proc = self.activities.get_mut(&pid).ok_or("Process not found")?;
        proc.thread_count += 1;
        Ok(())
    }

    /// Update resource utilization metrics for a process
    pub fn update_activity_metrics(
        &mut self,
        pid: usize,
        cpu_pct: f32,
        memory_bytes: usize,
        io_read: u64,
        io_write: u64,
        current_timestamp: u64,
    ) -> Result<(), &'static str> {
        let proc = self.activities.get_mut(&pid).ok_or("Process not found")?;
        proc.cpu_usage_pct = cpu_pct;
        proc.memory_footprint_bytes = memory_bytes;
        proc.io_read_bytes += io_read;
        proc.io_write_bytes += io_write;
        proc.last_active_timestamp = current_timestamp;

        self.total_cpu_cycles_tracked
            .fetch_add((cpu_pct * 100.0) as usize, Ordering::SeqCst);

        // Auto-classify activity state
        if proc.state != ActivityState::Terminated && proc.state != ActivityState::Suspended {
            if proc.is_foreground {
                proc.state = ActivityState::Interactive;
            } else if proc.power_throttling_enabled {
                proc.state = ActivityState::Throttled;
            } else if cpu_pct > 0.5 {
                proc.state = ActivityState::Active;
            } else {
                proc.state = ActivityState::Idle;
            }
        }

        Ok(())
    }

    /// Set foreground process (receives interactivity priority boost)
    pub fn set_foreground_process(&mut self, pid: usize) -> Result<(), &'static str> {
        if !self.activities.contains_key(&pid) {
            return Err("Process not found");
        }

        // Demote previous foreground process
        if let Some(prev_pid) = self.active_foreground_pid {
            if let Some(prev) = self.activities.get_mut(&prev_pid) {
                prev.is_foreground = false;
                if prev.state == ActivityState::Interactive {
                    prev.state = ActivityState::Background;
                }
            }
        }

        // Promote new foreground process (Garuda Zen / Android ActivityManager parity)
        if let Some(proc) = self.activities.get_mut(&pid) {
            proc.is_foreground = true;
            proc.state = ActivityState::Interactive;
            proc.priority = (proc.priority - 5).clamp(-20, 19); // Priority boost for GUI responsiveness
        }

        self.active_foreground_pid = Some(pid);
        Ok(())
    }

    /// Apply energy/power throttling to background processes
    pub fn set_power_throttling(&mut self, pid: usize, enabled: bool) -> Result<(), &'static str> {
        let proc = self.activities.get_mut(&pid).ok_or("Process not found")?;
        proc.power_throttling_enabled = enabled;
        if enabled && !proc.is_foreground {
            proc.state = ActivityState::Throttled;
            proc.priority = (proc.priority + 5).clamp(-20, 19); // Priority demotion to save CPU cycles
        }
        Ok(())
    }

    /// Capture thread register state snapshot
    pub fn capture_register_snapshot(
        &mut self,
        pid: usize,
        registers: RegisterSnapshot,
    ) -> Result<(), &'static str> {
        let proc = self.activities.get_mut(&pid).ok_or("Process not found")?;
        proc.register_snapshot = Some(registers);
        Ok(())
    }

    /// Record executable address space binding
    pub fn bind_address_space(
        &mut self,
        pid: usize,
        binary_path: &str,
        virt_base: u64,
        size: u64,
        aslr_offset: u64,
        is_wx_compliant: bool,
    ) -> Result<(), &'static str> {
        let proc = self.activities.get_mut(&pid).ok_or("Process not found")?;
        proc.address_binding = Some(AddressSpaceBinding {
            binary_path: binary_path.to_string(),
            virtual_base: virt_base,
            size_bytes: size,
            aslr_offset,
            is_executable: true,
            is_wx_compliant,
            bound_libraries: Vec::new(),
        });
        Ok(())
    }

    /// Add bound dynamic library to process address space
    pub fn add_bound_library(&mut self, pid: usize, lib_name: &str) -> Result<(), &'static str> {
        let proc = self.activities.get_mut(&pid).ok_or("Process not found")?;
        if let Some(ref mut binding) = proc.address_binding {
            binding.bound_libraries.push(lib_name.to_string());
            Ok(())
        } else {
            Err("No address space binding registered for process")
        }
    }

    /// Reclaim idle/background processes under OOM pressure (Linux oom_killer & Garuda nohang parity)
    pub fn reclaim_background_activity(
        &mut self,
        current_timestamp: u64,
        max_idle_sec: u64,
    ) -> Vec<usize> {
        let mut terminated_pids = Vec::new();

        for (pid, proc) in self.activities.iter_mut() {
            if !proc.is_foreground
                && proc.state != ActivityState::Terminated
                && current_timestamp.saturating_sub(proc.last_active_timestamp) > max_idle_sec
            {
                proc.state = ActivityState::Terminated;
                terminated_pids.push(*pid);
            }
        }

        terminated_pids
    }

    /// Retrieve process activity details
    pub fn get_process_activity(&self, pid: usize) -> Option<&ProcessActivityRecord> {
        self.activities.get(&pid)
    }

    /// Summary report of all managed process activities
    pub fn summary(&self) -> String {
        let mut active_count = 0;
        let mut interactive_count = 0;
        let mut throttled_count = 0;

        for proc in self.activities.values() {
            match proc.state {
                ActivityState::Active => active_count += 1,
                ActivityState::Interactive => interactive_count += 1,
                ActivityState::Throttled => throttled_count += 1,
                _ => {}
            }
        }

        format!(
            "System Activity Manager: {} total processes tracked ({} Interactive, {} Active, {} Throttled)",
            self.activities.len(),
            interactive_count,
            active_count,
            throttled_count
        )
    }
}

impl Default for ActivityManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_activity_manager_lifecycle_and_interactivity_boost() {
        let mut am = ActivityManager::new();
        am.register_process(101, 1, "browser", 0);
        am.register_process(102, 1, "compiler", 0);

        // Set browser as foreground process -> should receive interactivity priority boost
        am.set_foreground_process(101).unwrap();

        let browser = am.get_process_activity(101).unwrap();
        assert!(browser.is_foreground);
        assert_eq!(browser.state, ActivityState::Interactive);
        assert_eq!(browser.priority, -5); // Priority boosted from 0 to -5

        // Update compiler metrics -> background active process
        am.update_activity_metrics(102, 85.0, 1024 * 1024 * 50, 2048, 1024, 1005)
            .unwrap();
        let compiler = am.get_process_activity(102).unwrap();
        assert_eq!(compiler.state, ActivityState::Active);
        assert_eq!(compiler.cpu_usage_pct, 85.0);
    }

    #[test]
    fn test_power_throttling_and_reclamation() {
        let mut am = ActivityManager::new();
        am.register_process(201, 1, "background_sync", 0);

        // Apply power throttling
        am.set_power_throttling(201, true).unwrap();
        let proc = am.get_process_activity(201).unwrap();
        assert_eq!(proc.state, ActivityState::Throttled);
        assert_eq!(proc.priority, 5); // Demoted priority

        // Test idle activity reclamation
        am.update_activity_metrics(201, 0.0, 1024 * 1024, 0, 0, 1000)
            .unwrap();
        let reclaimed = am.reclaim_background_activity(2000, 500); // 1000 sec idle
        assert_eq!(reclaimed, vec![201]);

        let reclaimed_proc = am.get_process_activity(201).unwrap();
        assert_eq!(reclaimed_proc.state, ActivityState::Terminated);
    }

    #[test]
    fn test_register_snapshot_and_address_binding() {
        let mut am = ActivityManager::new();
        am.register_process(301, 1, "shell", 0);

        let regs = RegisterSnapshot {
            rax: 0x1,
            rbx: 0x2,
            rip: 0x0000_7FFF_0000_1000,
            rsp: 0x0000_7FFF_FFFF_F000,
            cs: 0x23,
            ..Default::default()
        };
        am.capture_register_snapshot(301, regs).unwrap();

        am.bind_address_space(
            301,
            "/bin/shell",
            0x0000_7FFF_0000_0000,
            0x10000,
            0x20000,
            true,
        )
        .unwrap();
        am.add_bound_library(301, "libsigma.so").unwrap();

        let proc = am.get_process_activity(301).unwrap();
        assert_eq!(proc.register_snapshot.unwrap().rip, 0x0000_7FFF_0000_1000);

        let binding = proc.address_binding.as_ref().unwrap();
        assert_eq!(binding.binary_path, "/bin/shell");
        assert!(binding.is_wx_compliant);
        assert_eq!(binding.bound_libraries, vec!["libsigma.so"]);
    }
}
