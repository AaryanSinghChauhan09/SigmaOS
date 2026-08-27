//! FreeBSD-Inspired Jail Containerization System
//!
//! Lightweight OS-level virtualization with process isolation

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;
#[derive(Debug, Clone)]
pub struct JailConfig {
    pub name: String,
    pub hostname: String,
    pub ip_address: Option<String>,
    pub root_path: PathBuf,
    pub allow_raw_sockets: bool,
    pub allow_chflags: bool,
    pub allow_mount: bool,
    pub exec_start: Option<String>,
    pub exec_stop: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum JailState {
    Stopped,
    Starting,
    Running,
    Stopping,
}

#[derive(Debug)]
pub struct Jail {
    config: JailConfig,
    state: JailState,
    jid: Option<u32>,
    processes: Vec<u32>,
}

#[derive(Debug)]
pub struct SigmaJailManager {
    jails: HashMap<String, Jail>,
    next_jid: u32,
}

impl SigmaJailManager {
    pub fn new() -> Self {
        Self {
            jails: HashMap::new(),
            next_jid: 1,
        }
    }

    /// Create a new jail
    pub fn create_jail(&mut self, config: JailConfig) -> Result<(), Box<dyn std::error::Error>> {
        if self.jails.contains_key(&config.name) {
            return Err(format!("Jail '{}' already exists", config.name).into());
        }

        // Validate jail configuration
        self.validate_config(&config)?;

        // Create jail root directory structure
        self.setup_jail_environment(&config)?;

        let jail = Jail {
            config: config.clone(),
            state: JailState::Stopped,
            jid: None,
            processes: Vec::new(),
        };

        self.jails.insert(config.name.clone(), jail);
        Ok(())
    }

    /// Start a jail
    pub fn start_jail(&mut self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(jail) = self.jails.get(name) {
            if jail.state != JailState::Stopped {
                return Err(format!("Jail '{}' is not stopped", name).into());
            }
        } else {
            return Err(format!("Jail '{}' not found", name).into());
        }

        let jail = self.jails.get_mut(name).unwrap();
        jail.state = JailState::Starting;

        // Assign JID
        let jid = self.next_jid;
        self.next_jid += 1;
        jail.jid = Some(jid);

        // Clone config before using it
        let ip_address = jail.config.ip_address.clone();
        let config_clone = jail.config.clone();
        let exec_start = jail.config.exec_start.clone();

        // Release mutable borrow
        drop(jail);

        // Create network namespace if IP specified
        if let Some(ip) = &ip_address {
            self.setup_jail_network(jid, ip)?;
        }

        // Mount jail filesystem
        self.mount_jail_fs(&config_clone)?;

        // Apply security restrictions
        self.apply_jail_restrictions(jid, &config_clone)?;

        // Execute startup script
        if let Some(start_cmd) = &exec_start {
            self.execute_in_jail(jid, start_cmd)?;
        }

        // Set final state
        if let Some(jail) = self.jails.get_mut(name) {
            jail.state = JailState::Running;
        }
        println!("Jail '{}' started with JID {}", name, jid);

        Ok(())
    }

    /// Stop a jail
    pub fn stop_jail(&mut self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let (exec_stop, jid, config, processes) = if let Some(jail) = self.jails.get(name) {
            if jail.state != JailState::Running {
                return Err(format!("Jail '{}' is not running", name).into());
            }
            (
                jail.config.exec_stop.clone(),
                jail.jid,
                jail.config.clone(),
                jail.processes.clone(),
            )
        } else {
            return Err(format!("Jail '{}' not found", name).into());
        };

        if let Some(jail) = self.jails.get_mut(name) {
            jail.state = JailState::Stopping;
        }

        // Execute stop script
        if let Some(exec_stop) = &exec_stop {
            if let Some(jid) = jid {
                let _ = self.execute_in_jail(jid, exec_stop);
            }
        }

        // Kill all processes in jail
        self.kill_jail_processes_by_pids(&processes)?;

        // Unmount jail filesystem
        self.unmount_jail_fs(&config)?;

        // Cleanup network
        if let Some(jid) = jid {
            self.cleanup_jail_network(jid)?;
        }

        if let Some(jail) = self.jails.get_mut(name) {
            jail.state = JailState::Stopped;
            jail.jid = None;
            jail.processes.clear();
        }

        println!("Jail '{}' stopped", name);
        Ok(())
    }

    /// Execute command in jail
    pub fn exec_in_jail(
        &self,
        name: &str,
        command: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(jail) = self.jails.get(name) {
            if jail.state != JailState::Running {
                return Err(format!("Jail '{}' is not running", name).into());
            }

            if let Some(jid) = jail.jid {
                let output = self.execute_in_jail(jid, command)?;
                Ok(output)
            } else {
                Err("Jail has no JID".into())
            }
        } else {
            Err(format!("Jail '{}' not found", name).into())
        }
    }

    /// List all jails
    pub fn list_jails(&self) -> Vec<(&String, &JailState, Option<u32>)> {
        self.jails
            .iter()
            .map(|(name, jail)| (name, &jail.state, jail.jid))
            .collect()
    }

    /// Get jail information
    pub fn jail_info(&self, name: &str) -> Option<JailInfo> {
        if let Some(jail) = self.jails.get(name) {
            Some(JailInfo {
                name: name.to_string(),
                state: jail.state.clone(),
                jid: jail.jid,
                hostname: jail.config.hostname.clone(),
                ip_address: jail.config.ip_address.clone(),
                root_path: jail.config.root_path.clone(),
                process_count: jail.processes.len(),
            })
        } else {
            None
        }
    }

    fn validate_config(&self, config: &JailConfig) -> Result<(), Box<dyn std::error::Error>> {
        // Validate jail name
        if config.name.is_empty() || config.name.contains('/') {
            return Err("Invalid jail name".into());
        }

        // Validate root path
        if !config.root_path.exists() {
            std::fs::create_dir_all(&config.root_path)?;
        }

        // Validate IP address format
        if let Some(ip) = &config.ip_address {
            std::net::Ipv4Addr::from_str(ip).map_err(|_| "Invalid IP address format")?;
        }

        Ok(())
    }

    fn setup_jail_environment(
        &self,
        config: &JailConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let root_path = &config.root_path;

        // Create basic directory structure
        let dirs = [
            "bin", "boot", "dev", "etc", "lib", "proc", "root", "sbin", "sys", "tmp", "usr", "var",
            "home",
        ];

        for dir in dirs {
            std::fs::create_dir_all(root_path.join(dir))?;
        }

        // Copy essential binaries and libraries
        self.copy_essential_files(root_path)?;

        // Set up basic configuration files
        self.create_jail_config_files(config)?;

        Ok(())
    }

    fn copy_essential_files(&self, root_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        // Copy essential binaries
        let essential_bins = ["/bin/sh", "/bin/ls", "/bin/cat", "/bin/echo"];

        for bin in essential_bins {
            if Path::new(bin).exists() {
                let dest = root_path.join(&bin[1..]);
                if let Some(parent) = dest.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                let _ = std::fs::copy(bin, dest);
            }
        }

        // Copy essential libraries
        let lib_dirs = ["/lib", "/lib64", "/usr/lib"];

        for lib_dir in lib_dirs {
            let src = Path::new(lib_dir);
            if src.exists() {
                let dest = root_path.join(&lib_dir[1..]);
                // Copy select libraries (simplified)
                if src.join("libc.so.6").exists() {
                    std::fs::create_dir_all(&dest)?;
                    let _ = std::fs::copy(src.join("libc.so.6"), dest.join("libc.so.6"));
                }
            }
        }

        Ok(())
    }

    fn create_jail_config_files(
        &self,
        config: &JailConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let etc_path = config.root_path.join("etc");
        std::fs::create_dir_all(&etc_path)?;

        // Create hostname file
        let _ = std::fs::write(etc_path.join("hostname"), &config.hostname);

        // Create hosts file
        let mut hosts_content = String::new();
        hosts_content.push_str("127.0.0.1\tlocalhost\n");
        if let Some(ip) = &config.ip_address {
            hosts_content.push_str(&format!("{}\t{}\n", ip, config.hostname));
        }
        let _ = std::fs::write(etc_path.join("hosts"), hosts_content);

        // Create resolv.conf
        let _ = std::fs::write(etc_path.join("resolv.conf"), "nameserver 8.8.8.8\n");

        Ok(())
    }

    fn setup_jail_network(&self, jid: u32, ip: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Create virtual network interface for jail
        let if_name = format!("jail{}", jid);

        Command::new("ip")
            .args([
                "link",
                "add",
                &if_name,
                "type",
                "veth",
                "peer",
                "name",
                &format!("{}-host", if_name),
            ])
            .output()?;

        // Assign IP to jail interface
        Command::new("ip")
            .args(["addr", "add", &format!("{}/24", ip), "dev", &if_name])
            .output()?;

        // Bring interface up
        Command::new("ip")
            .args(["link", "set", &if_name, "up"])
            .output()?;

        Ok(())
    }

    fn mount_jail_fs(&self, config: &JailConfig) -> Result<(), Box<dyn std::error::Error>> {
        let root_path = &config.root_path;

        // Mount proc filesystem
        Command::new("mount")
            .args([
                "-t",
                "proc",
                "proc",
                &root_path.join("proc").to_string_lossy(),
            ])
            .output()?;

        // Mount sysfs
        Command::new("mount")
            .args([
                "-t",
                "sysfs",
                "sysfs",
                &root_path.join("sys").to_string_lossy(),
            ])
            .output()?;

        // Mount devtmpfs
        Command::new("mount")
            .args([
                "-t",
                "devtmpfs",
                "devtmpfs",
                &root_path.join("dev").to_string_lossy(),
            ])
            .output()?;

        Ok(())
    }

    fn apply_jail_restrictions(
        &self,
        jid: u32,
        _config: &JailConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Apply cgroup restrictions
        let cgroup_path = format!("/sys/fs/cgroup/sigma-jail-{}", jid);
        std::fs::create_dir_all(&cgroup_path)?;

        // Limit memory (example: 512MB)
        std::fs::write(format!("{}/memory.max", cgroup_path), "536870912")?;

        // Limit CPU (example: 50%)
        std::fs::write(format!("{}/cpu.max", cgroup_path), "50000 100000")?;

        Ok(())
    }

    fn execute_in_jail(
        &self,
        jid: u32,
        command: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let root_path = match self.jails.values().find(|j| j.jid == Some(jid)) {
            Some(j) => j.config.root_path.clone(),
            None => return Err("Jail not found".into()),
        };

        // Use chroot and namespaces to execute in jail context
        let output = Command::new("unshare")
            .args(["-p", "-f", "chroot"])
            .arg(&root_path)
            .arg("sh")
            .arg("-c")
            .arg(command)
            .output()?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(format!(
                "Command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )
            .into())
        }
    }

    fn kill_jail_processes_by_pids(&self, processes: &[u32]) -> Result<(), Box<dyn std::error::Error>> {
        // Kill all processes in jail's process group
        for pid in processes {
            let _ = Command::new("kill")
                .arg("-TERM")
                .arg(pid.to_string())
                .output();
        }

        // Wait and force kill if necessary
        std::thread::sleep(std::time::Duration::from_secs(5));

        for pid in processes {
            let _ = Command::new("kill")
                .arg("-KILL")
                .arg(pid.to_string())
                .output();
        }

        Ok(())
    }

    fn kill_jail_processes(&self, jail: &Jail) -> Result<(), Box<dyn std::error::Error>> {
        self.kill_jail_processes_by_pids(&jail.processes)
    }

    fn unmount_jail_fs(&self, config: &JailConfig) -> Result<(), Box<dyn std::error::Error>> {
        let root_path = &config.root_path;

        // Unmount filesystems
        let _ = Command::new("umount").arg(root_path.join("proc")).output();
        let _ = Command::new("umount").arg(root_path.join("sys")).output();
        let _ = Command::new("umount").arg(root_path.join("dev")).output();

        Ok(())
    }

    fn cleanup_jail_network(&self, jid: u32) -> Result<(), Box<dyn std::error::Error>> {
        let if_name = format!("jail{}", jid);

        // Remove network interface
        let _ = Command::new("ip")
            .args(["link", "delete", &if_name])
            .output();

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct JailInfo {
    pub name: String,
    pub state: JailState,
    pub jid: Option<u32>,
    pub hostname: String,
    pub ip_address: Option<String>,
    pub root_path: PathBuf,
    pub process_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    // Simple temporary directory implementation for testing
    struct TestTempDir {
        path: PathBuf,
    }

    impl TestTempDir {
        fn new() -> std::io::Result<Self> {
            let path = std::env::temp_dir().join(format!("sigma_test_{}", std::process::id()));
            std::fs::create_dir_all(&path)?;
            Ok(TestTempDir { path })
        }

        fn path(&self) -> &PathBuf {
            &self.path
        }
    }

    impl Drop for TestTempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.path);
        }
    }

    #[test]
    fn test_jail_creation() {
        let mut manager = SigmaJailManager::new();
        let temp_dir = TestTempDir::new().unwrap();

        let config = JailConfig {
            name: "test-jail".to_string(),
            hostname: "testjail".to_string(),
            ip_address: Some("192.168.1.100".to_string()),
            root_path: temp_dir.path().to_path_buf(),
            allow_raw_sockets: false,
            allow_chflags: false,
            allow_mount: false,
            exec_start: Some("/bin/sh".to_string()),
            exec_stop: None,
        };

        assert!(manager.create_jail(config).is_ok());
        assert!(manager.jails.contains_key("test-jail"));
    }

    #[test]
    fn test_jail_info() {
        let mut manager = SigmaJailManager::new();
        let temp_dir = TestTempDir::new().unwrap();

        let config = JailConfig {
            name: "info-test".to_string(),
            hostname: "infotest".to_string(),
            ip_address: None,
            root_path: temp_dir.path().to_path_buf(),
            allow_raw_sockets: false,
            allow_chflags: false,
            allow_mount: false,
            exec_start: None,
            exec_stop: None,
        };

        manager.create_jail(config).unwrap();
        let info = manager.jail_info("info-test").unwrap();
        assert_eq!(info.name, "info-test");
        assert_eq!(info.state, JailState::Stopped);
    }
}
