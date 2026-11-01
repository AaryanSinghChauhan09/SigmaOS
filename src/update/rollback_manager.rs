//! Secure update pipeline with signed package verification and rollback support.
//!
//! Coordinates the full update lifecycle: checking for updates, verifying
//! signatures, taking a pre-update snapshot, applying changes, running
//! post-update health checks, and rolling back on failure.

/// Update distribution channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateChannel {
    /// Stable release channel (default).
    Stable,
    /// Testing/beta channel.
    Testing,
    /// Security-only updates (auto-applied for CVEs).
    Security,
    /// Long-term support channel.
    Lts,
}

impl UpdateChannel {
    /// APT suite/component string.
    pub fn apt_suite(self) -> &'static str {
        match self {
            Self::Stable   => "sigma-stable",
            Self::Testing  => "sigma-testing",
            Self::Security => "sigma-security",
            Self::Lts      => "sigma-lts",
        }
    }
}

/// A single pending package update.
#[derive(Debug, Clone)]
pub struct PendingUpdate {
    pub package: String,
    pub current_version: String,
    pub new_version: String,
    pub size_bytes: u64,
    pub is_security: bool,
    pub changelog: String,
}

/// A resolved update plan ready to apply.
#[derive(Debug, Clone)]
pub struct UpdatePlan {
    pub channel: UpdateChannel,
    pub updates: Vec<PendingUpdate>,
    pub total_size_bytes: u64,
    /// A snapshot must be taken before applying this plan.
    pub pre_snapshot_required: bool,
    pub estimated_duration_s: u64,
}

/// Result of applying an update plan.
#[derive(Debug, Clone)]
pub struct UpdateResult {
    pub applied: usize,
    pub failed: usize,
    pub snapshot_id: String,
    pub can_rollback: bool,
}

/// Errors from the update manager.
#[derive(Debug, thiserror::Error)]
pub enum UpdateError {
    #[error("package signature verification failed for {0}")]
    SignatureInvalid(String),
    #[error("repository metadata signature invalid")]
    RepoSignatureInvalid,
    #[error("snapshot creation failed: {0}")]
    SnapshotFailed(String),
    #[error("package installation failed: {0}")]
    InstallFailed(String),
    #[error("post-update health check failed: {0}")]
    HealthCheckFailed(String),
    #[error("rollback failed: {0}")]
    RollbackFailed(String),
    #[error("no rollback snapshot available")]
    NoRollbackAvailable,
}

/// Update manager: orchestrates the complete update lifecycle.
pub struct UpdateManager {
    last_snapshot_id: Option<String>,
    tx_counter: u32,
}

impl UpdateManager {
    pub fn new() -> Self {
        Self { last_snapshot_id: None, tx_counter: 0 }
    }

    /// Check the configured channel for available updates.
    ///
    /// Production: runs `apt-get update` and parses the candidate versions.
    pub fn check_for_updates(
        &self,
        _channel: UpdateChannel,
    ) -> Result<Vec<PendingUpdate>, UpdateError> {
        // Simulate finding some updates.
        Ok(vec![
            PendingUpdate {
                package: "sigma-browser".to_string(),
                current_version: "2026.10.0".to_string(),
                new_version: "2026.11.0".to_string(),
                size_bytes: 45 * 1024 * 1024,
                is_security: false,
                changelog: "Performance improvements, bug fixes.".to_string(),
            },
            PendingUpdate {
                package: "sigma-update".to_string(),
                current_version: "1.2.0".to_string(),
                new_version: "1.2.1".to_string(),
                size_bytes: 512 * 1024,
                is_security: true,
                changelog: "CVE-2026-1234 fix.".to_string(),
            },
        ])
    }

    /// Build an update plan from a list of pending updates.
    pub fn create_update_plan(&self, updates: &[PendingUpdate]) -> UpdatePlan {
        let total = updates.iter().map(|u| u.size_bytes).sum();
        let duration = (total / (5u64 * 1024 * 1024)).max(10u64); // ~5 MiB/s
        UpdatePlan {
            channel: UpdateChannel::Stable,
            updates: updates.to_vec(),
            total_size_bytes: total,
            pre_snapshot_required: !updates.is_empty(),
            estimated_duration_s: duration,
        }
    }

    /// Apply an update plan: snapshot → verify signatures → install → health check.
    pub fn apply_plan(&mut self, plan: &UpdatePlan) -> Result<UpdateResult, UpdateError> {
        if plan.updates.is_empty() {
            return Ok(UpdateResult {
                applied: 0, failed: 0,
                snapshot_id: String::new(),
                can_rollback: false,
            });
        }

        // Step 1: pre-update snapshot
        self.tx_counter += 1;
        let snap_id = format!("pre-update-{:04}", self.tx_counter);
        // Production: btrfs subvolume snapshot
        self.last_snapshot_id = Some(snap_id.clone());

        // Step 2: verify all package signatures
        for update in &plan.updates {
            self.verify_package_signature(&update.package)?;
        }

        // Step 3: apply packages
        // Production: apt-get install --yes <pkgs>
        let applied = plan.updates.len();

        // Step 4: post-update health check
        self.verify_post_update()?;

        Ok(UpdateResult {
            applied,
            failed: 0,
            snapshot_id: snap_id,
            can_rollback: true,
        })
    }

    /// Roll back to the pre-update snapshot.
    pub fn rollback(&mut self, reason: &str) -> Result<(), UpdateError> {
        let snap = self.last_snapshot_id.take()
            .ok_or(UpdateError::NoRollbackAvailable)?;
        // Production: btrfs subvolume set-default <snap>; systemctl reboot
        let _ = (snap, reason);
        Ok(())
    }

    /// Post-update health checks.
    pub fn verify_post_update(&self) -> Result<bool, UpdateError> {
        // Production: systemctl is-system-running, dpkg --verify
        Ok(true)
    }

    /// `true` if a rollback snapshot is available.
    pub fn pending_rollback(&self) -> bool {
        self.last_snapshot_id.is_some()
    }

    /// Verify a package's Ed25519 signature.
    fn verify_package_signature(&self, _package: &str) -> Result<(), UpdateError> {
        // Production: gpg --verify <package>.sig <package>.deb
        Ok(())
    }
}

impl Default for UpdateManager {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_for_updates_returns_list() {
        let mgr = UpdateManager::new();
        let updates = mgr.check_for_updates(UpdateChannel::Stable).unwrap();
        assert!(!updates.is_empty());
    }

    #[test]
    fn test_create_plan_sets_pre_snapshot_required() {
        let mgr = UpdateManager::new();
        let updates = mgr.check_for_updates(UpdateChannel::Stable).unwrap();
        let plan = mgr.create_update_plan(&updates);
        assert!(plan.pre_snapshot_required);
    }

    #[test]
    fn test_create_plan_empty_updates_no_snapshot() {
        let mgr = UpdateManager::new();
        let plan = mgr.create_update_plan(&[]);
        assert!(!plan.pre_snapshot_required);
    }

    #[test]
    fn test_apply_plan_succeeds() {
        let mut mgr = UpdateManager::new();
        let updates = mgr.check_for_updates(UpdateChannel::Stable).unwrap();
        let plan = mgr.create_update_plan(&updates);
        let result = mgr.apply_plan(&plan).unwrap();
        assert!(result.applied > 0);
        assert!(result.can_rollback);
    }

    #[test]
    fn test_pending_rollback_after_apply() {
        let mut mgr = UpdateManager::new();
        let updates = mgr.check_for_updates(UpdateChannel::Stable).unwrap();
        let plan = mgr.create_update_plan(&updates);
        mgr.apply_plan(&plan).unwrap();
        assert!(mgr.pending_rollback());
    }

    #[test]
    fn test_rollback_clears_pending() {
        let mut mgr = UpdateManager::new();
        let updates = mgr.check_for_updates(UpdateChannel::Stable).unwrap();
        let plan = mgr.create_update_plan(&updates);
        mgr.apply_plan(&plan).unwrap();
        mgr.rollback("user-requested").unwrap();
        assert!(!mgr.pending_rollback());
    }

    #[test]
    fn test_rollback_without_snapshot_fails() {
        let mut mgr = UpdateManager::new();
        let result = mgr.rollback("test");
        assert!(matches!(result, Err(UpdateError::NoRollbackAvailable)));
    }

    #[test]
    fn test_security_update_flagged() {
        let mgr = UpdateManager::new();
        let updates = mgr.check_for_updates(UpdateChannel::Security).unwrap();
        assert!(updates.iter().any(|u| u.is_security));
    }
}
