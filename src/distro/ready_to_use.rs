// SPDX-License-Identifier: MIT
// Ready-to-Use OS Usability Primitives
// Linux & BSD-inspired Service Supervision, Mount Management, User Session Environment, and Hotplug PnP Hardware Driver Binding.

extern crate alloc;

#[cfg(not(test))]
use crate::klib::{HashMap, Vec};

#[cfg(test)]
use std::collections::HashMap;

#[cfg(test)]
use std::vec::Vec;

use alloc::string::{String, ToString};

/// Service execution state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    Stopped,
    Starting,
    Running,
    Failed,
    Disabled,
}

/// Service specification unit (systemd/rc.d inspired)
#[derive(Debug, Clone)]
pub struct ServiceUnit {
    pub name: String,
    pub description: String,
    pub state: ServiceState,
    pub dependencies: Vec<String>,
    pub auto_restart: bool,
    pub pid: Option<u64>,
}

impl ServiceUnit {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            state: ServiceState::Stopped,
            dependencies: Vec::new(),
            auto_restart: true,
            pid: None,
        }
    }

    pub fn with_dependency(mut self, dep: &str) -> Self {
        self.dependencies.push(dep.to_string());
        self
    }
}

/// Linux systemd / BSD rc.d inspired Service Manager
pub struct DistroServiceManager {
    pub services: HashMap<String, ServiceUnit>,
    pub target_level: String,
    pub next_pid: u64,
}

impl DistroServiceManager {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
            target_level: String::from("multi-user.target"),
            next_pid: 100,
        }
    }

    pub fn register_service(&mut self, unit: ServiceUnit) {
        self.services.insert(unit.name.clone(), unit);
    }

    pub fn start_service(&mut self, name: &str) -> Result<u64, &'static str> {
        if let Some(unit) = self.services.get_mut(name) {
            if unit.state == ServiceState::Disabled {
                return Err("Service is disabled");
            }
            if unit.state == ServiceState::Running {
                return Ok(unit.pid.unwrap_or(0));
            }
            unit.state = ServiceState::Starting;
            let pid = self.next_pid;
            self.next_pid += 1;
            unit.pid = Some(pid);
            unit.state = ServiceState::Running;
            Ok(pid)
        } else {
            Err("Service not found")
        }
    }

    pub fn stop_service(&mut self, name: &str) -> Result<(), &'static str> {
        if let Some(unit) = self.services.get_mut(name) {
            unit.state = ServiceState::Stopped;
            unit.pid = None;
            Ok(())
        } else {
            Err("Service not found")
        }
    }

    pub fn restart_service(&mut self, name: &str) -> Result<u64, &'static str> {
        self.stop_service(name)?;
        self.start_service(name)
    }

    pub fn get_active_services(&self) -> Vec<String> {
        let mut active: Vec<String> = Vec::new();
        for (name, unit) in self.services.iter() {
            if unit.state == ServiceState::Running {
                let s_name: String = name.clone();
                active.push(s_name);
            }
        }
        active
    }

    pub fn switch_target(&mut self, target: &str) {
        self.target_level = target.to_string();
    }
}

impl Default for DistroServiceManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Supported filesystem types for mounting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MountType {
    Ext4,
    Tmpfs,
    Devtmpfs,
    Procfs,
    Sysfs,
    Overlayfs,
}

/// Fstab / Mount table entry
#[derive(Debug, Clone)]
pub struct MountEntry {
    pub spec: String,
    pub file: String,
    pub vfstype: MountType,
    pub options: Vec<String>,
    pub mounted: bool,
}

/// Universal Mount & Fstab Management Engine
pub struct UniversalMountEngine {
    pub mount_table: Vec<MountEntry>,
}

impl UniversalMountEngine {
    pub fn new() -> Self {
        Self {
            mount_table: Vec::new(),
        }
    }

    pub fn parse_fstab_line(&mut self, line: &str) -> Result<(), &'static str> {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            return Ok(());
        }
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() < 3 {
            return Err("Invalid fstab line format");
        }
        let spec = parts[0].to_string();
        let file = parts[1].to_string();
        let vfstype = match parts[2] {
            "ext4" => MountType::Ext4,
            "tmpfs" => MountType::Tmpfs,
            "devtmpfs" => MountType::Devtmpfs,
            "proc" => MountType::Procfs,
            "sysfs" => MountType::Sysfs,
            "overlay" => MountType::Overlayfs,
            _ => MountType::Ext4,
        };
        let mut options: Vec<String> = Vec::new();
        if parts.len() >= 4 {
            for opt in parts[3].split(',') {
                let opt_str: String = opt.trim().to_string();
                options.push(opt_str);
            }
        }
        self.mount_table.push(MountEntry {
            spec,
            file,
            vfstype,
            options,
            mounted: false,
        });
        Ok(())
    }

    pub fn mount_all_fstab(&mut self) -> usize {
        let mut count = 0;
        for entry in &mut self.mount_table {
            if !entry.mounted {
                entry.mounted = true;
                count += 1;
            }
        }
        count
    }

    pub fn mount_essential_pseudo_filesystems(&mut self) {
        let pseudo = [
            ("devtmpfs", "/dev", MountType::Devtmpfs),
            ("proc", "/proc", MountType::Procfs),
            ("sysfs", "/sys", MountType::Sysfs),
            ("tmpfs", "/tmp", MountType::Tmpfs),
            ("tmpfs", "/run", MountType::Tmpfs),
        ];
        for (spec, file, vfstype) in pseudo {
            self.mount_table.push(MountEntry {
                spec: spec.to_string(),
                file: file.to_string(),
                vfstype,
                options: Vec::new(),
                mounted: true,
            });
        }
    }

    pub fn get_mounted_filesystems(&self) -> Vec<String> {
        let mut list: Vec<String> = Vec::new();
        for entry in self.mount_table.iter() {
            if entry.mounted {
                let f_path: String = entry.file.clone();
                list.push(f_path);
            }
        }
        list
    }
}

impl Default for UniversalMountEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// User account profile
#[derive(Debug, Clone)]
pub struct UserAccount {
    pub username: String,
    pub uid: u32,
    pub gid: u32,
    pub home_dir: String,
    pub shell: String,
    pub password_hash: String,
}

/// User interactive session environment
#[derive(Debug, Clone)]
pub struct SessionEnvironment {
    pub username: String,
    pub env_vars: HashMap<String, String>,
    pub active: bool,
}

/// User Login & Session Environment Manager
pub struct InteractiveUserEnvironment {
    pub accounts: HashMap<String, UserAccount>,
    pub active_session: Option<SessionEnvironment>,
}

impl InteractiveUserEnvironment {
    pub fn new() -> Self {
        let mut accounts = HashMap::new();
        accounts.insert(
            String::from("root"),
            UserAccount {
                username: String::from("root"),
                uid: 0,
                gid: 0,
                home_dir: String::from("/root"),
                shell: String::from("/bin/sigmash"),
                password_hash: String::from("root_hash"),
            },
        );
        Self {
            accounts,
            active_session: None,
        }
    }

    pub fn add_account(&mut self, account: UserAccount) {
        self.accounts.insert(account.username.clone(), account);
    }

    pub fn authenticate_and_login(
        &mut self,
        username: &str,
        password_attempt: &str,
    ) -> Result<String, &'static str> {
        if let Some(account) = self.accounts.get(username) {
            if account.password_hash == password_attempt {
                let mut env_vars = HashMap::new();
                env_vars.insert(String::from("USER"), account.username.clone());
                env_vars.insert(String::from("HOME"), account.home_dir.clone());
                env_vars.insert(String::from("SHELL"), account.shell.clone());
                env_vars.insert(
                    String::from("PATH"),
                    String::from("/bin:/sbin:/usr/bin:/usr/sbin"),
                );

                self.active_session = Some(SessionEnvironment {
                    username: account.username.clone(),
                    env_vars,
                    active: true,
                });
                Ok(account.home_dir.clone())
            } else {
                Err("Authentication failed: invalid password")
            }
        } else {
            Err("Authentication failed: user not found")
        }
    }

    pub fn generate_motd_banner(&self) -> String {
        let user = self
            .active_session
            .as_ref()
            .map(|s| s.username.as_str())
            .unwrap_or("guest");
        let mut banner = String::from("====================================================\n");
        banner.push_str(" Welcome to SigmaOS Ready-to-Use Sovereign Kernel\n");
        banner.push_str(" Logged in as: ");
        banner.push_str(user);
        banner.push_str("\n====================================================\n");
        banner
    }

    pub fn logout(&mut self) {
        self.active_session = None;
    }
}

impl Default for InteractiveUserEnvironment {
    fn default() -> Self {
        Self::new()
    }
}

/// Hardware category for udev/devd style handling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceCategory {
    Storage,
    Network,
    Graphics,
    Input,
    Audio,
    Misc,
}

/// Hardware hotplug event
#[derive(Debug, Clone)]
pub enum HardwareEvent {
    DeviceAdded(String, DeviceCategory),
    DeviceRemoved(String),
}

/// Device node registration
#[derive(Debug, Clone)]
pub struct DeviceNode {
    pub dev_path: String,
    pub category: DeviceCategory,
    pub driver_bound: Option<String>,
}

/// Hotplug Hardware & Driver Binding Manager
pub struct PlugAndPlayHardwareManager {
    pub devices: HashMap<String, DeviceNode>,
    pub registered_drivers: HashMap<String, Vec<DeviceCategory>>,
}

impl PlugAndPlayHardwareManager {
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
            registered_drivers: HashMap::new(),
        }
    }

    pub fn register_driver(&mut self, driver_name: &str, categories: Vec<DeviceCategory>) {
        self.registered_drivers
            .insert(driver_name.to_string(), categories);
    }

    pub fn handle_event(&mut self, event: HardwareEvent) -> Option<String> {
        match event {
            HardwareEvent::DeviceAdded(dev_path, category) => {
                let mut bound_driver: Option<String> = None;
                for (driver_name, supported_cats) in self.registered_drivers.iter() {
                    let cats: &Vec<DeviceCategory> = supported_cats;
                    if cats.contains(&category) {
                        let d_name: String = driver_name.clone();
                        bound_driver = Some(d_name);
                        break;
                    }
                }
                let node = DeviceNode {
                    dev_path: dev_path.clone(),
                    category,
                    driver_bound: bound_driver.clone(),
                };
                self.devices.insert(dev_path, node);
                bound_driver
            }
            HardwareEvent::DeviceRemoved(dev_path) => {
                self.devices.remove(&dev_path);
                None
            }
        }
    }

    pub fn get_bound_devices(&self) -> Vec<String> {
        let mut list: Vec<String> = Vec::new();
        for (path, node) in self.devices.iter() {
            if node.driver_bound.is_some() {
                let d_path: String = path.clone();
                list.push(d_path);
            }
        }
        list
    }
}

impl Default for PlugAndPlayHardwareManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_manager() {
        let mut sm = DistroServiceManager::new();
        let service = ServiceUnit::new("networkd", "Network Supervisor");
        sm.register_service(service);

        let pid = sm.start_service("networkd").unwrap();
        assert!(pid >= 100);
        assert_eq!(sm.get_active_services().len(), 1);

        sm.stop_service("networkd").unwrap();
        assert_eq!(sm.get_active_services().len(), 0);
    }

    #[test]
    fn test_mount_engine() {
        let mut me = UniversalMountEngine::new();
        me.parse_fstab_line("/dev/nvme0n1p1 / ext4 defaults 0 1")
            .unwrap();
        assert_eq!(me.mount_table.len(), 1);

        me.mount_all_fstab();
        assert_eq!(me.get_mounted_filesystems().len(), 1);

        me.mount_essential_pseudo_filesystems();
        assert_eq!(me.get_mounted_filesystems().len(), 6);
    }

    #[test]
    fn test_user_environment() {
        let mut env = InteractiveUserEnvironment::new();
        let res = env.authenticate_and_login("root", "root_hash");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "/root");

        let motd = env.generate_motd_banner();
        assert!(motd.contains("root"));

        env.logout();
        assert!(env.active_session.is_none());
    }

    #[test]
    fn test_pnp_hardware_manager() {
        let mut pnp = PlugAndPlayHardwareManager::new();
        let mut cats = Vec::new();
        cats.push(DeviceCategory::Network);
        pnp.register_driver("e1000", cats);

        let driver = pnp.handle_event(HardwareEvent::DeviceAdded(
            String::from("/dev/net0"),
            DeviceCategory::Network,
        ));
        assert_eq!(driver, Some(String::from("e1000")));
        assert_eq!(pnp.get_bound_devices().len(), 1);
    }
}
