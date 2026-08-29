#![cfg_attr(not(test), no_std)]
use alloc::vec;
// SigmaOS Process Supervisor
// Linux/BSD distro-inspired process management
// Handles process supervision, monitoring, and lifecycle management



extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

/// Process state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Running,
    Stopped,
    Failed,
    Restarting,
    Sleeping,
}

/// Process supervisor configuration
#[derive(Debug, Clone)]
pub struct ProcessConfig {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub working_dir: String,
    pub auto_restart: bool,
    pub restart_delay: u32,
    pub max_restarts: u32,
    pub stdout_log: String,
    pub stderr_log: String,
    pub environment: BTreeMap<String, String>,
}

/// Process status
#[derive(Debug, Clone)]
pub struct ProcessStatus {
    pub pid: Option<u32>,
    pub state: ProcessState,
    pub uptime: u64,
    pub restart_count: u32,
    pub last_exit_code: Option<i32>,
    pub memory_usage: u64,
    pub cpu_usage: f32,
}

/// Process supervisor
pub struct ProcessSupervisor {
    pub processes: BTreeMap<String, ProcessConfig>,
    pub process_status: BTreeMap<String, ProcessStatus>,
    pub config_dir: String,
    pub run_dir: String,
}

impl ProcessSupervisor {
    pub fn new(config_dir: &str, run_dir: &str) -> Self {
        Self {
            processes: BTreeMap::new(),
            process_status: BTreeMap::new(),
            config_dir: String::from(config_dir),
            run_dir: String::from(run_dir),
        }
    }

    /// Initialize process supervisor
    pub fn initialize(&self) -> Result<(), SupervisorError> {
        Ok(())
    }

    /// Add a process to supervise
    pub fn add_process(&mut self, config: ProcessConfig) -> Result<(), SupervisorError> {
        let name = config.name.clone();
        self.processes.insert(name.clone(), config);
        
        let status = ProcessStatus {
            pid: None,
            state: ProcessState::Stopped,
            uptime: 0,
            restart_count: 0,
            last_exit_code: None,
            memory_usage: 0,
            cpu_usage: 0.0,
        };
        self.process_status.insert(name, status);
        
        Ok(())
    }

    /// Remove a process from supervision
    pub fn remove_process(&mut self, name: &str) -> Result<(), SupervisorError> {
        self.processes.remove(name);
        self.process_status.remove(name);
        Ok(())
    }

    /// Start a process
    pub fn start_process(&mut self, name: &str) -> Result<(), SupervisorError> {
        if let Some(config) = self.processes.get(name) {
            let pid_offset = self.process_status.len() as u32;
            if let Some(status) = self.process_status.get_mut(name) {
                status.state = ProcessState::Running;
                status.pid = Some(1000 + pid_offset); // Simulated PID
                status.uptime = 0;
                status.restart_count = 0;
            }
            Ok(())
        } else {
            Err(SupervisorError::ProcessNotFound(String::from(name)))
        }
    }

    /// Stop a process
    pub fn stop_process(&mut self, name: &str) -> Result<(), SupervisorError> {
        if let Some(status) = self.process_status.get_mut(name) {
            status.state = ProcessState::Stopped;
            status.pid = None;
            Ok(())
        } else {
            Err(SupervisorError::ProcessNotFound(String::from(name)))
        }
    }

    /// Restart a process
    pub fn restart_process(&mut self, name: &str) -> Result<(), SupervisorError> {
        self.stop_process(name)?;
        self.start_process(name)?;
        
        if let Some(status) = self.process_status.get_mut(name) {
            status.restart_count += 1;
        }
        
        Ok(())
    }

    /// Get process status
    pub fn get_process_status(&self, name: &str) -> Option<&ProcessStatus> {
        self.process_status.get(name)
    }

    /// Get all processes
    pub fn get_all_processes(&self) -> Vec<String> {
        self.processes.keys().cloned().collect()
    }

    /// Get running processes
    pub fn get_running_processes(&self) -> Vec<String> {
        self.process_status.iter()
            .filter(|(_, status)| status.state == ProcessState::Running)
            .map(|(name, _)| name.clone())
            .collect()
    }

    /// Monitor and auto-restart failed processes
    pub fn monitor_processes(&mut self) -> Result<(), SupervisorError> {
        for (name, config) in self.processes.clone().iter() {
            if let Some(status) = self.process_status.get_mut(name) {
                if status.state == ProcessState::Failed && config.auto_restart {
                    if status.restart_count < config.max_restarts {
                        status.state = ProcessState::Restarting;
                        self.restart_process(name)?;
                    }
                }
            }
        }
        Ok(())
    }

    /// Update process statistics
    pub fn update_process_stats(&mut self, name: &str, memory: u64, cpu: f32) -> Result<(), SupervisorError> {
        if let Some(status) = self.process_status.get_mut(name) {
            status.memory_usage = memory;
            status.cpu_usage = cpu;
            Ok(())
        } else {
            Err(SupervisorError::ProcessNotFound(String::from(name)))
        }
    }

    /// Create default system processes
    pub fn create_default_processes(&mut self) -> Result<(), SupervisorError> {
        let sshd = ProcessConfig {
            name: String::from("sshd"),
            command: String::from("/usr/sbin/sshd"),
            args: vec![String::from("-D")],
            working_dir: String::from("/"),
            auto_restart: true,
            restart_delay: 5,
            max_restarts: 10,
            stdout_log: String::from("/var/log/sshd.log"),
            stderr_log: String::from("/var/log/sshd.err"),
            environment: BTreeMap::new(),
        };
        self.add_process(sshd)?;

        let cron = ProcessConfig {
            name: String::from("cron"),
            command: String::from("/usr/sbin/cron"),
            args: vec![String::from("-f")],
            working_dir: String::from("/"),
            auto_restart: true,
            restart_delay: 5,
            max_restarts: 10,
            stdout_log: String::from("/var/log/cron.log"),
            stderr_log: String::from("/var/log/cron.err"),
            environment: BTreeMap::new(),
        };
        self.add_process(cron)?;

        let network = ProcessConfig {
            name: String::from("network-manager"),
            command: String::from("/usr/sbin/network-manager"),
            args: vec![],
            working_dir: String::from("/"),
            auto_restart: true,
            restart_delay: 5,
            max_restarts: 10,
            stdout_log: String::from("/var/log/network.log"),
            stderr_log: String::from("/var/log/network.err"),
            environment: BTreeMap::new(),
        };
        self.add_process(network)?;

        Ok(())
    }

    /// Save process configurations
    pub fn save_configs(&self) -> Result<(), SupervisorError> {
        Ok(())
    }

    /// Load process configurations
    pub fn load_configs(&mut self) -> Result<(), SupervisorError> {
        Ok(())
    }
}

/// Supervisor errors
#[derive(Debug)]
pub enum SupervisorError {
    ProcessNotFound(String),
    StartError(String),
    StopError(String),
    ConfigError(String),
    MonitorError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_supervisor() {
        let mut supervisor = ProcessSupervisor::new("/etc/supervisor", "/var/run/supervisor");
        supervisor.initialize().unwrap();
        
        assert!(supervisor.create_default_processes().is_ok());
        assert_eq!(supervisor.processes.len(), 3);
    }

    #[test]
    fn test_process_lifecycle() {
        let mut supervisor = ProcessSupervisor::new("/etc/supervisor", "/var/run/supervisor");
        supervisor.initialize().unwrap();
        supervisor.create_default_processes().unwrap();
        
        assert!(supervisor.start_process("sshd").is_ok());
        assert!(supervisor.stop_process("sshd").is_ok());
        assert!(supervisor.restart_process("sshd").is_ok());
    }

    #[test]
    fn test_process_status() {
        let mut supervisor = ProcessSupervisor::new("/etc/supervisor", "/var/run/supervisor");
        supervisor.initialize().unwrap();
        supervisor.create_default_processes().unwrap();
        
        supervisor.start_process("sshd").unwrap();
        let status = supervisor.get_process_status("sshd").unwrap();
        assert_eq!(status.state, ProcessState::Running);
    }

    #[test]
    fn test_running_processes() {
        let mut supervisor = ProcessSupervisor::new("/etc/supervisor", "/var/run/supervisor");
        supervisor.initialize().unwrap();
        supervisor.create_default_processes().unwrap();
        
        supervisor.start_process("sshd").unwrap();
        supervisor.start_process("cron").unwrap();
        
        let running = supervisor.get_running_processes();
        assert_eq!(running.len(), 2);
    }
}
