use crate::policy::Policy;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone)]
pub struct Profile {
    pub id: u64,
    pub name: String,
    pub active_policy: Policy,
}

impl Profile {
    pub fn new(policy: Policy) -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
            name: policy.name.clone(),
            active_policy: policy,
        }
    }
}

// ── AppArmor Absorption: Path-based restriction profiles ────────────────────

/// Replaces AppArmor's text-based path profiles with a native type-safe
/// restriction system. Each `SigmaProfile` defines which filesystem paths a
/// process is allowed to read, write, or execute.
#[derive(Debug, Clone)]
pub struct SigmaProfile {
    pub name: String,
    pub allowed_read_paths: Vec<String>,
    pub allowed_write_paths: Vec<String>,
    pub allowed_exec_paths: Vec<String>,
    pub deny_all_network: bool,
}

impl SigmaProfile {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            allowed_read_paths: Vec::new(),
            allowed_write_paths: Vec::new(),
            allowed_exec_paths: Vec::new(),
            deny_all_network: true,
        }
    }

    pub fn allow_read(&mut self, path: &str) {
        self.allowed_read_paths.push(path.to_string());
    }

    pub fn allow_write(&mut self, path: &str) {
        self.allowed_write_paths.push(path.to_string());
    }

    pub fn allow_exec(&mut self, path: &str) {
        self.allowed_exec_paths.push(path.to_string());
    }

    /// Check if a given path is readable under this profile.
    pub fn can_read(&self, path: &str) -> bool {
        self.allowed_read_paths.iter().any(|p| path.starts_with(p))
    }

    /// Check if a given path is writable under this profile.
    pub fn can_write(&self, path: &str) -> bool {
        self.allowed_write_paths.iter().any(|p| path.starts_with(p))
    }

    /// Check if a given path is executable under this profile.
    pub fn can_exec(&self, path: &str) -> bool {
        self.allowed_exec_paths.iter().any(|p| path.starts_with(p))
    }
}

// ── SELinux Absorption: Label-based Mandatory Access Control contexts ────────

/// Replaces SELinux's `user:role:type:level` context labels with native Rust
/// structures. Each process and resource can be assigned a `SigmaContext`
/// that is enforced at the kernel's enforcement engine level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SigmaContext {
    pub user: String,
    pub role: String,
    pub context_type: String,
    pub level: String,
}

impl SigmaContext {
    pub fn new(user: &str, role: &str, context_type: &str, level: &str) -> Self {
        Self {
            user: user.to_string(),
            role: role.to_string(),
            context_type: context_type.to_string(),
            level: level.to_string(),
        }
    }

    /// Check if a subject context can transition to a target context.
    pub fn can_transition(&self, target: &SigmaContext) -> bool {
        // A subject with sysadm_r can transition to any target
        if self.role == "sysadm_r" {
            return true;
        }
        // Same-role transitions are allowed within the same level
        self.role == target.role && self.level == target.level
    }
}

/// Manages capability profiles, replacing AppArmor's text-based profile configurations.
pub struct ProfileSystem {
    profiles: Vec<Profile>,
    sigma_profiles: Vec<SigmaProfile>,
    sigma_contexts: Vec<SigmaContext>,
}

impl Default for ProfileSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl ProfileSystem {
    pub fn new() -> Self {
        Self {
            profiles: Vec::new(),
            sigma_profiles: Vec::new(),
            sigma_contexts: Vec::new(),
        }
    }

    pub fn create(&mut self, policy: Policy) -> Profile {
        let profile = Profile::new(policy);
        self.profiles.push(profile.clone());
        profile
    }

    pub fn get_profile(&self, name: &str) -> Option<&Profile> {
        self.profiles.iter().find(|p| p.name == name)
    }

    /// Register an AppArmor-style path-based profile.
    pub fn register_sigma_profile(&mut self, profile: SigmaProfile) {
        self.sigma_profiles.push(profile);
    }

    /// Get an AppArmor-style profile by name.
    pub fn get_sigma_profile(&self, name: &str) -> Option<&SigmaProfile> {
        self.sigma_profiles.iter().find(|p| p.name == name)
    }

    /// Register an SELinux-style context.
    pub fn register_context(&mut self, ctx: SigmaContext) {
        self.sigma_contexts.push(ctx);
    }

    /// Check if a context transition is allowed.
    pub fn check_context_transition(&self, from: &SigmaContext, to: &SigmaContext) -> bool {
        from.can_transition(to)
    }
}

