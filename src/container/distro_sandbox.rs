extern crate alloc;
use alloc::vec;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;
// Distro Sandbox & Container Isolation Engine for SigmaOS
// Inspired by Linux namespaces (unshare), Landlock LSM, Seccomp BPF, and cgroups v2.

use crate::klib::HashMap;

/// Linux-inspired Namespace Flags for process isolation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NamespaceFlags {
    pub mount_ns: bool, // CLONE_NEWNS
    pub pid_ns: bool,   // CLONE_NEWPID
    pub net_ns: bool,   // CLONE_NEWNET
    pub user_ns: bool,  // CLONE_NEWUSER
    pub uts_ns: bool,   // CLONE_NEWUTS
    pub ipc_ns: bool,   // CLONE_NEWIPC
}

impl NamespaceFlags {
    pub fn full_isolation() -> Self {
        Self {
            mount_ns: true,
            pid_ns: true,
            net_ns: true,
            user_ns: true,
            uts_ns: true,
            ipc_ns: true,
        }
    }
}

/// Landlock LSM Path Access Rules (Linux 5.13+ Landlock LSM parity)
#[derive(Debug, Clone)]
pub struct LandlockPathRules {
    pub read_only_paths: Vec<String>,
    pub read_write_paths: Vec<String>,
    pub exec_paths: Vec<String>,
    pub forbidden_paths: Vec<String>,
}

impl LandlockPathRules {
    pub fn new() -> Self {
        Self {
            read_only_paths: vec!["/usr".to_string(), "/lib".to_string(), "/etc".to_string()],
            read_write_paths: vec!["/tmp".to_string()],
            exec_paths: vec!["/usr/bin".to_string()],
            forbidden_paths: vec!["/proc/kcore".to_string(), "/sys/firmware".to_string()],
        }
    }
}

impl Default for LandlockPathRules {
    fn default() -> Self {
        Self::new()
    }
}

/// Seccomp BPF Action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeccompAction {
    Allow,
    Errno(u16),
    KillProcess,
}

/// Seccomp BPF Syscall Filter Policy
#[derive(Debug, Clone)]
pub struct SeccompPolicy {
    pub default_action: SeccompAction,
    pub blocked_syscalls: Vec<u32>,
}

impl SeccompPolicy {
    pub fn hardened() -> Self {
        Self {
            default_action: SeccompAction::Allow,
            blocked_syscalls: vec![
                101, // ptrace
                165, // reboot
                169, // reboot/kexec
                313, // finit_module
            ],
        }
    }
}

/// cgroups v2 Resource Boundary Limits
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CgroupV2Limits {
    pub memory_max_bytes: usize,
    pub cpu_weight: u32, // 1-1000 range
    pub pids_max: u32,
    pub io_weight: u32,
}

impl CgroupV2Limits {
    pub fn default_container_limits() -> Self {
        Self {
            memory_max_bytes: 512 * 1024 * 1024, // 512MB
            cpu_weight: 100,
            pids_max: 256,
            io_weight: 100,
        }
    }
}

/// Distro Sandbox Container Instance Record
#[derive(Debug, Clone)]
pub struct DistroSandboxInstance {
    pub container_id: String,
    pub namespaces: NamespaceFlags,
    pub landlock_rules: LandlockPathRules,
    pub seccomp_policy: SeccompPolicy,
    pub cgroup_limits: CgroupV2Limits,
    pub rootfs_path: String,
    pub is_active: bool,
}

/// Main Distro Sandbox Engine
pub struct DistroSandboxEngine {
    pub active_sandboxes: HashMap<String, DistroSandboxInstance>,
}

impl DistroSandboxEngine {
    pub fn new() -> Self {
        Self {
            active_sandboxes: HashMap::new(),
        }
    }

    /// Create and initialize a new sandboxed container environment
    pub fn create_sandbox(
        &mut self,
        container_id: &str,
        rootfs: &str,
        namespaces: NamespaceFlags,
        cgroup_limits: CgroupV2Limits,
    ) -> Result<(), &'static str> {
        if self.active_sandboxes.contains_key(container_id) {
            return Err("Sandbox container ID already exists");
        }

        let instance = DistroSandboxInstance {
            container_id: container_id.to_string(),
            namespaces,
            landlock_rules: LandlockPathRules::new(),
            seccomp_policy: SeccompPolicy::hardened(),
            cgroup_limits,
            rootfs_path: rootfs.to_string(),
            is_active: true,
        };

        self.active_sandboxes
            .insert(container_id.to_string(), instance);
        Ok(())
    }

    /// Validate syscall invocation against Seccomp BPF policy
    pub fn validate_syscall(&self, container_id: &str, syscall_num: u32) -> SeccompAction {
        if let Some(sandbox) = self.active_sandboxes.get(container_id) {
            if sandbox
                .seccomp_policy
                .blocked_syscalls
                .contains(&syscall_num)
            {
                return SeccompAction::KillProcess;
            }
            sandbox.seccomp_policy.default_action
        } else {
            SeccompAction::Allow
        }
    }

    /// Validate path access against Landlock LSM rules
    pub fn validate_path_access(
        &self,
        container_id: &str,
        target_path: &str,
        is_write: bool,
    ) -> bool {
        if let Some(sandbox) = self.active_sandboxes.get(container_id) {
            // 1. Check forbidden paths
            if sandbox
                .landlock_rules
                .forbidden_paths
                .iter()
                .any(|p| target_path.starts_with(p))
            {
                return false;
            }

            // 2. Check write intent rules
            if is_write {
                return sandbox
                    .landlock_rules
                    .read_write_paths
                    .iter()
                    .any(|p| target_path.starts_with(p));
            }

            // 3. Read access check
            sandbox
                .landlock_rules
                .read_only_paths
                .iter()
                .any(|p| target_path.starts_with(p))
                || sandbox
                    .landlock_rules
                    .read_write_paths
                    .iter()
                    .any(|p| target_path.starts_with(p))
        } else {
            true
        }
    }
}

impl Default for DistroSandboxEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distro_sandbox_creation_seccomp_and_landlock() {
        let mut engine = DistroSandboxEngine::new();

        let ns = NamespaceFlags::full_isolation();
        let limits = CgroupV2Limits::default_container_limits();

        engine
            .create_sandbox("app_sandbox_1", "/var/lib/sigma/rootfs_1", ns, limits)
            .unwrap();

        // 1. Test Seccomp BPF syscall checks
        assert_eq!(
            engine.validate_syscall("app_sandbox_1", 1),
            SeccompAction::Allow
        ); // sys_write
        assert_eq!(
            engine.validate_syscall("app_sandbox_1", 101),
            SeccompAction::KillProcess
        ); // ptrace blocked!

        // 2. Test Landlock LSM path access rules
        assert!(engine.validate_path_access("app_sandbox_1", "/usr/share/doc", false)); // Read allowed
        assert!(!engine.validate_path_access("app_sandbox_1", "/usr/share/doc", true)); // Write forbidden on read-only path
        assert!(engine.validate_path_access("app_sandbox_1", "/tmp/scratch.tmp", true)); // Write allowed on /tmp
        assert!(!engine.validate_path_access("app_sandbox_1", "/proc/kcore", false));
        // Forbidden path!
    }
}
