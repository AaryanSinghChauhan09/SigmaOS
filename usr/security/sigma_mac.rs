// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/security/sigma_mac.rs — Sigma Mandatory Access Control (AppArmor/SELinux)
//
// Implements AppArmor/SELinux-style mandatory access control with profiles,
// policies, contexts, and enforcement modes.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── MAC Types ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnforcementMode {
    Enforce,
    Complain,
    Kill,
    Unconfined,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Permission {
    Read,
    Write,
    Execute,
    Append,
    Create,
    Delete,
    Link,
    Rename,
    Setattr,
    getattr,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Capability {
    CAP_CHOWN,
    CAP_DAC_OVERRIDE,
    CAP_DAC_READ_SEARCH,
    CAP_FOWNER,
    CAP_FSETID,
    CAP_KILL,
    CAP_SETGID,
    CAP_SETUID,
    CAP_SETPCAP,
    CAP_LINUX_IMMUTABLE,
    CAP_NET_BIND_SERVICE,
    CAP_NET_BROADCAST,
    CAP_NET_ADMIN,
    CAP_NET_RAW,
    CAP_IPC_LOCK,
    CAP_IPC_OWNER,
    CAP_SYS_MODULE,
    CAP_SYS_RAWIO,
    CAP_SYS_CHROOT,
    CAP_SYS_PTRACE,
    CAP_SYS_PACCT,
    CAP_SYS_ADMIN,
    CAP_SYS_BOOT,
    CAP_SYS_NICE,
    CAP_SYS_RESOURCE,
    CAP_SYS_TIME,
    CAP_SYS_TTY_CONFIG,
    CAP_MKNOD,
    CAP_LEASE,
    CAP_AUDIT_WRITE,
    CAP_AUDIT_CONTROL,
    CAP_SETFCAP,
}

#[derive(Debug, Clone)]
pub struct FileRule {
    pub path: String,
    pub permissions: Vec<Permission>,
    pub owner: Option<String>,
    pub deny: bool,
}

#[derive(Debug, Clone)]
pub struct NetworkRule {
    pub family: String,  // inet, inet6, unix
    pub sock_type: String,  // stream, dgram, raw
    pub protocol: Option<String>,
    pub action: String,  // allow, deny
    pub port: Option<u16>,
}

#[derive(Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub mode: EnforcementMode,
    pub file_rules: Vec<FileRule>,
    pub network_rules: Vec<NetworkRule>,
    pub capabilities: Vec<Capability>,
    pub exec_rules: Vec<String>,
    pub ptrace_rules: Vec<String>,
    pub signal_rules: Vec<String>,
    pub mount_rules: Vec<String>,
    pub pivot_root_rules: Vec<String>,
    pub unix_rules: Vec<String>,
    pub dbus_rules: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SecurityContext {
    pub user: String,
    pub role: String,
    pub type_: String,
    pub level: String,
}

#[derive(Debug, Clone)]
pub struct Process {
    pub pid: u32,
    pub comm: String,
    pub profile: String,
    pub context: SecurityContext,
    pub state: String,
}

// ─── MAC Manager ───────────────────────────────────────────────────────────

pub struct MACManager {
    pub profiles: HashMap<String, Profile>,
    pub processes: HashMap<u32, Process>,
    pub default_policy: String,
    pub audit_log: Vec<String>,
    pub loaded_modules: Vec<String>,
}

impl MACManager {
    pub fn new() -> Self {
        let mut manager = MACManager {
            profiles: HashMap::new(),
            processes: HashMap::new(),
            default_policy: "allow".to_string(),
            audit_log: Vec::new(),
            loaded_modules: vec![
                "apparmor".to_string(),
                "selinux".to_string(),
                "smack".to_string(),
                "tomoyo".to_string(),
            ],
        };
        
        manager.init_default_profiles();
        manager
    }

    /// Initialize default security profiles
    fn init_default_profiles(&mut self) {
        // Unconfined profile
        self.profiles.insert("unconfined".to_string(), Profile {
            name: "unconfined".to_string(),
            mode: EnforcementMode::Unconfined,
            file_rules: vec![],
            network_rules: vec![],
            capabilities: vec![
                Capability::CAP_CHOWN,
                Capability::CAP_DAC_OVERRIDE,
                Capability::CAP_FOWNER,
                Capability::CAP_SETUID,
                Capability::CAP_SETGID,
                Capability::CAP_NET_BIND_SERVICE,
                Capability::CAP_NET_ADMIN,
                Capability::CAP_SYS_ADMIN,
            ],
            exec_rules: vec!["/**".to_string()],
            ptrace_rules: vec!["peer=unconfined".to_string()],
            signal_rules: vec!["peer=unconfined".to_string()],
            mount_rules: vec!["/**".to_string()],
            pivot_root_rules: vec!["/**".to_string()],
            unix_rules: vec!["/**".to_string()],
            dbus_rules: vec!["/**".to_string()],
        });

        // System services profile
        self.profiles.insert("system-services".to_string(), Profile {
            name: "system-services".to_string(),
            mode: EnforcementMode::Enforce,
            file_rules: vec![
                FileRule {
                    path: "/etc/**".to_string(),
                    permissions: vec![Permission::Read, Permission::getattr],
                    owner: None,
                    deny: false,
                },
                FileRule {
                    path: "/var/lib/**".to_string(),
                    permissions: vec![Permission::Read, Permission::Write, Permission::Create],
                    owner: None,
                    deny: false,
                },
                FileRule {
                    path: "/var/log/**".to_string(),
                    permissions: vec![Permission::Read, Permission::Write, Permission::Append],
                    owner: None,
                    deny: false,
                },
                FileRule {
                    path: "/usr/bin/**".to_string(),
                    permissions: vec![Permission::Read, Permission::Execute],
                    owner: None,
                    deny: false,
                },
            ],
            network_rules: vec![
                NetworkRule {
                    family: "inet".to_string(),
                    sock_type: "stream".to_string(),
                    protocol: Some("tcp".to_string()),
                    action: "allow".to_string(),
                    port: None,
                },
                NetworkRule {
                    family: "inet".to_string(),
                    sock_type: "dgram".to_string(),
                    protocol: Some("udp".to_string()),
                    action: "allow".to_string(),
                    port: None,
                },
            ],
            capabilities: vec![
                Capability::CAP_NET_BIND_SERVICE,
                Capability::CAP_NET_ADMIN,
                Capability::CAP_CHOWN,
                Capability::CAP_DAC_OVERRIDE,
            ],
            exec_rules: vec![
                "/usr/bin/**".to_string(),
                "/usr/sbin/**".to_string(),
            ],
            ptrace_rules: vec![],
            signal_rules: vec![],
            mount_rules: vec![],
            pivot_root_rules: vec![],
            unix_rules: vec!["/**".to_string()],
            dbus_rules: vec!["system".to_string()],
        });

        // Web server profile
        self.profiles.insert("web-server".to_string(), Profile {
            name: "web-server".to_string(),
            mode: EnforcementMode::Enforce,
            file_rules: vec![
                FileRule {
                    path: "/var/www/**".to_string(),
                    permissions: vec![Permission::Read, Permission::getattr],
                    owner: Some("www-data".to_string()),
                    deny: false,
                },
                FileRule {
                    path: "/var/log/httpd/**".to_string(),
                    permissions: vec![Permission::Read, Permission::Write, Permission::Append],
                    owner: None,
                    deny: false,
                },
                FileRule {
                    path: "/etc/httpd/**".to_string(),
                    permissions: vec![Permission::Read, Permission::getattr],
                    owner: None,
                    deny: false,
                },
                FileRule {
                    path: "/tmp/**".to_string(),
                    permissions: vec![Permission::Read, Permission::Write, Permission::Create],
                    owner: None,
                    deny: false,
                },
            ],
            network_rules: vec![
                NetworkRule {
                    family: "inet".to_string(),
                    sock_type: "stream".to_string(),
                    protocol: Some("tcp".to_string()),
                    action: "allow".to_string(),
                    port: Some(80),
                },
                NetworkRule {
                    family: "inet".to_string(),
                    sock_type: "stream".to_string(),
                    protocol: Some("tcp".to_string()),
                    action: "allow".to_string(),
                    port: Some(443),
                },
            ],
            capabilities: vec![
                Capability::CAP_NET_BIND_SERVICE,
                Capability::CAP_SETUID,
                Capability::CAP_SETGID,
                Capability::CAP_CHOWN,
            ],
            exec_rules: vec![
                "/usr/bin/httpd".to_string(),
                "/usr/bin/php".to_string(),
            ],
            ptrace_rules: vec![],
            signal_rules: vec![],
            mount_rules: vec![],
            pivot_root_rules: vec![],
            unix_rules: vec!["/**".to_string()],
            dbus_rules: vec![],
        });

        // User applications profile
        self.profiles.insert("user-apps".to_string(), Profile {
            name: "user-apps".to_string(),
            mode: EnforcementMode::Complain,
            file_rules: vec![
                FileRule {
                    path: "/home/**".to_string(),
                    permissions: vec![Permission::Read, Permission::Write, Permission::Create, Permission::Delete],
                    owner: None,
                    deny: false,
                },
                FileRule {
                    path: "/tmp/**".to_string(),
                    permissions: vec![Permission::Read, Permission::Write, Permission::Create],
                    owner: None,
                    deny: false,
                },
                FileRule {
                    path: "/usr/share/**".to_string(),
                    permissions: vec![Permission::Read, Permission::getattr],
                    owner: None,
                    deny: false,
                },
            ],
            network_rules: vec![
                NetworkRule {
                    family: "inet".to_string(),
                    sock_type: "stream".to_string(),
                    protocol: Some("tcp".to_string()),
                    action: "allow".to_string(),
                    port: None,
                },
            ],
            capabilities: vec![],
            exec_rules: vec![
                "/usr/bin/**".to_string(),
                "/opt/**".to_string(),
            ],
            ptrace_rules: vec![],
            signal_rules: vec![],
            mount_rules: vec![],
            pivot_root_rules: vec![],
            unix_rules: vec!["/**".to_string()],
            dbus_rules: vec!["session".to_string()],
        });
    }

    /// Create a new profile
    pub fn create_profile(&mut self, name: String, mode: EnforcementMode) -> Result<Profile, String> {
        if self.profiles.contains_key(&name) {
            return Err("Profile already exists".to_string());
        }

        let profile = Profile {
            name: name.clone(),
            mode,
            file_rules: vec![],
            network_rules: vec![],
            capabilities: vec![],
            exec_rules: vec![],
            ptrace_rules: vec![],
            signal_rules: vec![],
            mount_rules: vec![],
            pivot_root_rules: vec![],
            unix_rules: vec![],
            dbus_rules: vec![],
        };

        self.profiles.insert(name.clone(), profile.clone());
        Ok(profile)
    }

    /// Add file rule to profile
    pub fn add_file_rule(&mut self, profile: &str, path: String, permissions: Vec<Permission>, deny: bool) -> Result<(), String> {
        if let Some(p) = self.profiles.get_mut(profile) {
            p.file_rules.push(FileRule {
                path,
                permissions,
                owner: None,
                deny,
            });
            Ok(())
        } else {
            Err("Profile not found".to_string())
        }
    }

    /// Add network rule to profile
    pub fn add_network_rule(&mut self, profile: &str, family: String, sock_type: String, action: String, port: Option<u16>) -> Result<(), String> {
        if let Some(p) = self.profiles.get_mut(profile) {
            p.network_rules.push(NetworkRule {
                family,
                sock_type,
                protocol: None,
                action,
                port,
            });
            Ok(())
        } else {
            Err("Profile not found".to_string())
        }
    }

    /// Add capability to profile
    pub fn add_capability(&mut self, profile: &str, cap: Capability) -> Result<(), String> {
        if let Some(p) = self.profiles.get_mut(profile) {
            if !p.capabilities.contains(&cap) {
                p.capabilities.push(cap);
            }
            Ok(())
        } else {
            Err("Profile not found".to_string())
        }
    }

    /// Set profile enforcement mode
    pub fn set_enforcement_mode(&mut self, profile: &str, mode: EnforcementMode) -> Result<(), String> {
        if let Some(p) = self.profiles.get_mut(profile) {
            p.mode = mode;
            self.audit_log.push(format!("Profile {} mode changed to {:?}", profile, mode));
            Ok(())
        } else {
            Err("Profile not found".to_string())
        }
    }

    /// Assign profile to process
    pub fn assign_profile(&mut self, pid: u32, profile: String) -> Result<(), String> {
        if !self.profiles.contains_key(&profile) {
            return Err("Profile not found".to_string());
        }

        let process = Process {
            pid,
            comm: format!("process_{}", pid),
            profile: profile.clone(),
            context: SecurityContext {
                user: "system_u".to_string(),
                role: "system_r".to_string(),
                type_: profile.clone(),
                level: "s0".to_string(),
            },
            state: "running".to_string(),
        };

        self.processes.insert(pid, process);
        self.audit_log.push(format!("Profile {} assigned to PID {}", profile, pid));
        Ok(())
    }

    /// Check if access is allowed
    pub fn check_access(&self, profile: &str, resource: &str, permission: Permission) -> bool {
        if let Some(p) = self.profiles.get(profile) {
            if p.mode == EnforcementMode::Unconfined {
                return true;
            }

            for rule in &p.file_rules {
                if resource.starts_with(&rule.path) || rule.path == resource {
                    if rule.deny {
                        return false;
                    }
                    if rule.permissions.contains(&permission) {
                        return true;
                    }
                }
            }

            // Default deny
            false
        } else {
            false
        }
    }

    /// Get profile status
    pub fn get_profile_status(&self, name: &str) -> Option<&Profile> {
        self.profiles.get(name)
    }

    /// List all profiles
    pub fn list_profiles(&self) -> Vec<&Profile> {
        self.profiles.values().collect()
    }

    /// Get audit log
    pub fn get_audit_log(&self) -> &Vec<String> {
        &self.audit_log
    }

    /// Clear audit log
    pub fn clear_audit_log(&mut self) {
        self.audit_log.clear();
    }

    /// Generate profile report
    pub fn generate_report(&self, profile: &str) -> Result<String, String> {
        if let Some(p) = self.profiles.get(profile) {
            let mut report = format!("Profile: {}\n", p.name);
            report.push_str(&format!("Mode: {:?}\n", p.mode));
            report.push_str(&format!("File Rules: {}\n", p.file_rules.len()));
            report.push_str(&format!("Network Rules: {}\n", p.network_rules.len()));
            report.push_str(&format!("Capabilities: {}\n", p.capabilities.len()));
            report.push_str(&format!("Exec Rules: {}\n", p.exec_rules.len()));
            
            // Count processes using this profile
            let process_count = self.processes.values()
                .filter(|proc| proc.profile == profile)
                .count();
            report.push_str(&format!("Active Processes: {}\n", process_count));
            
            Ok(report)
        } else {
            Err("Profile not found".to_string())
        }
    }

    /// Get security context for process
    pub fn get_process_context(&self, pid: u32) -> Option<&SecurityContext> {
        self.processes.get(&pid).map(|p| &p.context)
    }

    /// Set security context for process
    pub fn set_process_context(&mut self, pid: u32, user: String, role: String, type_: String, level: String) -> Result<(), String> {
        if let Some(process) = self.processes.get_mut(&pid) {
            process.context = SecurityContext {
                user,
                role,
                type_,
                level,
            };
            Ok(())
        } else {
            Err("Process not found".to_string())
        }
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────--

fn main() {
    let mut mac = MACManager::new();
    
    println!("Sigma MAC Manager v0.1 - AppArmor/SELinux-style Security");
    
    loop {
        println!("\n--- MAC Commands ---");
        println!("profiles           - List all profiles");
        println!("profile <name>     - Get profile status");
        println!("create <name> <mode> - Create profile (enforce/complain/kill/unconfined)");
        println!("set_mode <name> <mode> - Set enforcement mode");
        println!("add_file <name> <path> <perms> - Add file rule");
        println!("add_net <name> <family> <type> <action> [port] - Add network rule");
        println!("add_cap <name> <cap> - Add capability");
        println!("assign <pid> <profile> - Assign profile to process");
        println!("check <profile> <resource> <perm> - Check access");
        println!("report <name>     - Generate profile report");
        println!("audit              - Show audit log");
        println!("clear_audit        - Clear audit log");
        println!("quit               - Exit");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "profiles" => {
                println!("--- Security Profiles ---");
                for profile in mac.list_profiles() {
                    println!("{} - {:?}", profile.name, profile.mode);
                }
            }
            "profile" => {
                if let Some(name) = parts.get(1) {
                    if let Some(profile) = mac.get_profile_status(name) {
                        println!("--- Profile ---");
                        println!("Name: {}", profile.name);
                        println!("Mode: {:?}", profile.mode);
                        println!("File Rules: {}", profile.file_rules.len());
                        println!("Network Rules: {}", profile.network_rules.len());
                        println!("Capabilities: {}", profile.capabilities.len());
                    }
                }
            }
            "create" => {
                if parts.len() >= 3 {
                    let name = parts[1].to_string();
                    let mode = match parts[2] {
                        "enforce" => EnforcementMode::Enforce,
                        "complain" => EnforcementMode::Complain,
                        "kill" => EnforcementMode::Kill,
                        "unconfined" => EnforcementMode::Unconfined,
                        _ => {
                            println!("Invalid mode");
                            continue;
                        }
                    };
                    match mac.create_profile(name, mode) {
                        Ok(_) => println!("Profile created"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "set_mode" => {
                if parts.len() >= 3 {
                    let name = parts[1];
                    let mode = match parts[2] {
                        "enforce" => EnforcementMode::Enforce,
                        "complain" => EnforcementMode::Complain,
                        "kill" => EnforcementMode::Kill,
                        "unconfined" => EnforcementMode::Unconfined,
                        _ => {
                            println!("Invalid mode");
                            continue;
                        }
                    };
                    match mac.set_enforcement_mode(name, mode) {
                        Ok(_) => println!("Mode set"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "add_file" => {
                if parts.len() >= 4 {
                    let name = parts[1];
                    let path = parts[2].to_string();
                    let perms_str = parts[3];
                    let permissions: Vec<Permission> = perms_str.split(',')
                        .map(|p| match p.trim() {
                            "read" => Permission::Read,
                            "write" => Permission::Write,
                            "execute" => Permission::Execute,
                            "create" => Permission::Create,
                            "delete" => Permission::Delete,
                            _ => Permission::Read,
                        })
                        .collect();
                    match mac.add_file_rule(name, path, permissions, false) {
                        Ok(_) => println!("File rule added"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "add_net" => {
                if parts.len() >= 5 {
                    let name = parts[1];
                    let family = parts[2].to_string();
                    let sock_type = parts[3].to_string();
                    let action = parts[4].to_string();
                    let port = parts.get(5).and_then(|p| p.parse::<u16>().ok());
                    match mac.add_network_rule(name, family, sock_type, action, port) {
                        Ok(_) => println!("Network rule added"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "add_cap" => {
                if parts.len() >= 3 {
                    let name = parts[1];
                    let cap = match parts[2] {
                        "chown" => Capability::CAP_CHOWN,
                        "dac_override" => Capability::CAP_DAC_OVERRIDE,
                        "fowner" => Capability::CAP_FOWNER,
                        "setuid" => Capability::CAP_SETUID,
                        "setgid" => Capability::CAP_SETGID,
                        "net_bind_service" => Capability::CAP_NET_BIND_SERVICE,
                        "net_admin" => Capability::CAP_NET_ADMIN,
                        "sys_admin" => Capability::CAP_SYS_ADMIN,
                        _ => {
                            println!("Unknown capability");
                            continue;
                        }
                    };
                    match mac.add_capability(name, cap) {
                        Ok(_) => println!("Capability added"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "assign" => {
                if parts.len() >= 3 {
                    let pid = parts[1].parse::<u32>().unwrap_or(0);
                    let profile = parts[2].to_string();
                    match mac.assign_profile(pid, profile) {
                        Ok(_) => println!("Profile assigned"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "check" => {
                if parts.len() >= 4 {
                    let profile = parts[1];
                    let resource = parts[2];
                    let perm = match parts[3] {
                        "read" => Permission::Read,
                        "write" => Permission::Write,
                        "execute" => Permission::Execute,
                        _ => Permission::Read,
                    };
                    let allowed = mac.check_access(profile, resource, perm);
                    println!("Access: {}", if allowed { "ALLOWED" } else { "DENIED" });
                }
            }
            "report" => {
                if let Some(name) = parts.get(1) {
                    match mac.generate_report(name) {
                        Ok(report) => println!("{}", report),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "audit" => {
                println!("--- Audit Log ---");
                for entry in mac.get_audit_log() {
                    println!("{}", entry);
                }
            }
            "clear_audit" => {
                mac.clear_audit_log();
                println!("Audit log cleared");
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
