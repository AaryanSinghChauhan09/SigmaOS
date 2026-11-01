//! Capability-based privilege model for SigmaOS.
//!
//! Defines per-application trust levels, allowed Linux capabilities, and
//! syscall allowlists.  Applied via AppArmor profiles and systemd service
//! hardening directives.

/// Linux capabilities (subset relevant to SigmaOS).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Capability {
    /// Change file ownership.
    CapChown,
    /// Override DAC access controls.
    CapDacOverride,
    /// Read DAC access controls.
    CapDacReadSearch,
    /// Override file ownership restrictions.
    CapFowner,
    /// Set UID/GID.
    CapSetuid,
    CapSetgid,
    /// Manage network interfaces, routing, firewall.
    CapNetAdmin,
    /// Bind to ports below 1024.
    CapNetBindService,
    /// Trace other processes (ptrace).
    CapSysPtrace,
    /// System administration (mount, swapon, etc.).
    CapSysAdmin,
    /// Load/unload kernel modules.
    CapSysModule,
    /// Set process priorities.
    CapSysNice,
    /// Raw I/O access.
    CapSysRawio,
    /// Reboot/shutdown.
    CapSysBoot,
    /// Override resource limits.
    CapSysResource,
    /// Use chroot.
    CapSysChroot,
    /// Kill any process.
    CapKill,
    /// Audit write (write to audit log).
    CapAuditWrite,
    /// Set capabilities on files.
    CapSetpcap,
    /// Make arbitrary ioctl calls on ttys.
    CapSysTtyConfig,
}

impl Capability {
    /// Return `true` if this capability is considered dangerous/privileged.
    pub fn is_dangerous(self) -> bool {
        matches!(
            self,
            Self::CapSysAdmin
            | Self::CapNetAdmin
            | Self::CapSysPtrace
            | Self::CapSysModule
            | Self::CapSysRawio
            | Self::CapSetuid
            | Self::CapSetgid
        )
    }
}

/// Application trust level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AppTrustLevel {
    /// Core OS services (init, udev, etc.). Minimal restrictions.
    System,
    /// Official SigmaOS apps shipped in the default bundle.
    Trusted,
    /// Apps installed from the official repo but not in the default bundle.
    Sandboxed,
    /// Third-party or unverified apps.
    Untrusted,
}

/// Per-application permission record.
#[derive(Debug, Clone)]
pub struct AppPermission {
    /// Application identifier (reverse DNS, e.g. `"org.sigmaos.browser"`).
    pub app_id: String,
    /// Trust level.
    pub trust: AppTrustLevel,
    /// Linux capabilities this app is allowed.
    pub allowed_caps: Vec<Capability>,
    /// Allowed syscalls (seccomp allowlist names).
    pub allowed_syscalls: Vec<String>,
    /// Can the app open TCP/UDP network connections?
    pub can_network: bool,
    /// Can the app access the Wayland display?
    pub can_display: bool,
    /// Can the app use PipeWire audio?
    pub can_audio: bool,
}

/// Process privilege descriptor.
#[derive(Debug, Clone)]
pub struct ProcessPrivilege {
    /// Effective UID.
    pub uid: u32,
    /// Effective GID.
    pub gid: u32,
    /// Active capabilities.
    pub capabilities: Vec<Capability>,
    /// Whether `PR_SET_NO_NEW_PRIVS` has been set.
    pub no_new_privs: bool,
    /// Whether a seccomp filter is active.
    pub seccomp_enabled: bool,
}

/// Audit entry for a running application.
#[derive(Debug, Clone)]
pub struct PrivilegeAuditEntry {
    pub app_id: String,
    pub trust: AppTrustLevel,
    pub cap_count: usize,
    pub has_dangerous_caps: bool,
}

/// Errors from the privilege manager.
#[derive(Debug, thiserror::Error)]
pub enum PrivilegeError {
    #[error("app {0} not registered")]
    AppNotRegistered(String),
    #[error("capability {0:?} not allowed for trust level {1:?}")]
    CapabilityDenied(Capability, AppTrustLevel),
    #[error("privilege drop failed: {0}")]
    DropFailed(String),
    #[error("seccomp filter installation failed: {0}")]
    SeccompFailed(String),
}

/// Privilege manager: register, check, and audit app permissions.
pub struct PrivilegeManager {
    permissions: Vec<AppPermission>,
}

impl PrivilegeManager {
    pub fn new() -> Self {
        Self { permissions: Vec::new() }
    }

    /// Build the default permission set for a given trust level.
    pub fn default_permission(trust: AppTrustLevel) -> AppPermission {
        let (caps, syscalls, network, display, audio) = match trust {
            AppTrustLevel::System => (
                vec![Capability::CapNetAdmin, Capability::CapSysAdmin,
                     Capability::CapSetuid, Capability::CapSetgid],
                vec!["all".to_string()],
                true, true, true,
            ),
            AppTrustLevel::Trusted => (
                vec![Capability::CapNetBindService],
                vec!["read".to_string(), "write".to_string(), "open".to_string(),
                     "close".to_string(), "mmap".to_string(), "brk".to_string(),
                     "socket".to_string(), "connect".to_string(), "sendmsg".to_string(),
                     "recvmsg".to_string(), "ioctl".to_string(), "futex".to_string()],
                true, true, true,
            ),
            AppTrustLevel::Sandboxed => (
                vec![],
                vec!["read".to_string(), "write".to_string(), "open".to_string(),
                     "close".to_string(), "mmap".to_string(), "brk".to_string(),
                     "socket".to_string(), "connect".to_string(), "futex".to_string()],
                true, true, true,
            ),
            AppTrustLevel::Untrusted => (
                vec![],
                vec!["read".to_string(), "write".to_string(), "open".to_string(),
                     "close".to_string(), "brk".to_string(), "futex".to_string()],
                false, false, false,
            ),
        };
        AppPermission {
            app_id: String::new(),
            trust,
            allowed_caps: caps,
            allowed_syscalls: syscalls,
            can_network: network,
            can_display: display,
            can_audio: audio,
        }
    }

    /// Register an app's permission record.
    pub fn grant_permission(
        &mut self,
        app_id: &str,
        mut permission: AppPermission,
    ) -> Result<(), PrivilegeError> {
        permission.app_id = app_id.to_string();
        // Reject if Sandboxed/Untrusted app tries to claim dangerous caps
        if permission.trust >= AppTrustLevel::Sandboxed {
            for cap in &permission.allowed_caps {
                if cap.is_dangerous() {
                    return Err(PrivilegeError::CapabilityDenied(*cap, permission.trust));
                }
            }
        }
        self.permissions.retain(|p| p.app_id != app_id);
        self.permissions.push(permission);
        Ok(())
    }

    /// Simulate dropping privileges to the target descriptor.
    pub fn drop_privileges(target: &ProcessPrivilege) -> Result<(), PrivilegeError> {
        if target.uid == 0 && !target.no_new_privs {
            return Err(PrivilegeError::DropFailed(
                "cannot remain root without no_new_privs".to_string(),
            ));
        }
        Ok(())
    }

    /// Check whether an app has a specific capability.
    pub fn check_permission(&self, app_id: &str, cap: &Capability) -> bool {
        self.permissions.iter()
            .find(|p| p.app_id == app_id)
            .map_or(false, |p| p.allowed_caps.contains(cap))
    }

    /// Return audit entries for all registered apps.
    pub fn audit_privileges(&self) -> Vec<PrivilegeAuditEntry> {
        self.permissions.iter().map(|p| PrivilegeAuditEntry {
            app_id: p.app_id.clone(),
            trust: p.trust,
            cap_count: p.allowed_caps.len(),
            has_dangerous_caps: p.allowed_caps.iter().any(|c| c.is_dangerous()),
        }).collect()
    }

    /// Apply `PR_SET_NO_NEW_PRIVS` to the current process.
    pub fn apply_no_new_privs() -> Result<(), PrivilegeError> {
        // Production: libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0)
        Ok(())
    }
}

impl Default for PrivilegeManager {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sandboxed_default_has_no_dangerous_caps() {
        // Arrange
        let perm = PrivilegeManager::default_permission(AppTrustLevel::Sandboxed);
        // Act & Assert
        assert!(!perm.allowed_caps.iter().any(|c| c.is_dangerous()));
    }

    #[test]
    fn test_untrusted_default_no_network() {
        let perm = PrivilegeManager::default_permission(AppTrustLevel::Untrusted);
        assert!(!perm.can_network);
        assert!(!perm.can_display);
    }

    #[test]
    fn test_grant_permission_sandboxed_rejects_dangerous_cap() {
        // Arrange
        let mut mgr = PrivilegeManager::new();
        let mut perm = PrivilegeManager::default_permission(AppTrustLevel::Sandboxed);
        perm.allowed_caps.push(Capability::CapSysAdmin); // dangerous!
        // Act
        let result = mgr.grant_permission("org.test.app", perm);
        // Assert
        assert!(matches!(result, Err(PrivilegeError::CapabilityDenied(_, _))));
    }

    #[test]
    fn test_check_permission_returns_true_when_granted() {
        // Arrange
        let mut mgr = PrivilegeManager::new();
        let mut perm = PrivilegeManager::default_permission(AppTrustLevel::Trusted);
        perm.allowed_caps.push(Capability::CapNetBindService);
        mgr.grant_permission("org.sigmaos.server", perm).unwrap();
        // Act & Assert
        assert!(mgr.check_permission("org.sigmaos.server", &Capability::CapNetBindService));
    }

    #[test]
    fn test_audit_flags_dangerous_caps() {
        // Arrange
        let mut mgr = PrivilegeManager::new();
        let sys_perm = PrivilegeManager::default_permission(AppTrustLevel::System);
        mgr.grant_permission("org.sigmaos.system", sys_perm).unwrap();
        // Act
        let audit = mgr.audit_privileges();
        // Assert
        let entry = audit.iter().find(|e| e.app_id == "org.sigmaos.system").unwrap();
        assert!(entry.has_dangerous_caps);
    }

    #[test]
    fn test_drop_privileges_fails_for_root_without_no_new_privs() {
        // Arrange
        let priv_ = ProcessPrivilege {
            uid: 0, gid: 0,
            capabilities: vec![],
            no_new_privs: false,
            seccomp_enabled: false,
        };
        // Act
        let result = PrivilegeManager::drop_privileges(&priv_);
        // Assert
        assert!(matches!(result, Err(PrivilegeError::DropFailed(_))));
    }
}
