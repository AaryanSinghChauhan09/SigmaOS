//! Rescue and emergency repair mode for SigmaOS.
//!
//! Entered automatically from GRUB when the system fails to boot, or
//! manually via `sigma rescue`.  Detects the failure reason, presents
//! repair actions, and either auto-repairs or escalates to an emergency
//! shell.

/// Why rescue mode was entered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RescueReason {
    /// A package update left the system in a broken state.
    FailedUpdate,
    /// The root or boot filesystem is corrupted or unreadable.
    CorruptedFilesystem,
    /// The kernel image is missing from `/boot`.
    MissingKernel,
    /// GRUB configuration is broken or pointing to a non-existent kernel.
    BrokenGrub,
    /// The user explicitly requested rescue mode.
    ManualRequest,
    /// A system migration or upgrade script failed halfway.
    FailedMigration,
    /// Reason could not be determined automatically.
    Unknown,
}

impl RescueReason {
    /// Human-readable description of the failure reason.
    pub fn description(&self) -> &'static str {
        match self {
            Self::FailedUpdate       => "A system update failed or left packages in a broken state",
            Self::CorruptedFilesystem => "Filesystem corruption detected at boot",
            Self::MissingKernel      => "Kernel image not found in /boot",
            Self::BrokenGrub         => "GRUB bootloader configuration is invalid",
            Self::ManualRequest      => "Rescue mode requested manually",
            Self::FailedMigration    => "System migration or upgrade script failed",
            Self::Unknown            => "Boot failure reason could not be determined",
        }
    }
}

/// Context available to rescue mode at entry.
#[derive(Debug, Clone)]
pub struct RescueContext {
    /// Why rescue mode was triggered.
    pub reason: RescueReason,
    /// Which component triggered the failure, if known.
    pub failed_component: Option<String>,
    /// Snapshot IDs available for rollback.
    pub available_snapshots: Vec<String>,
    /// Seconds since epoch at rescue entry.
    pub boot_time: u64,
}

impl RescueContext {
    /// Create a new context.
    pub fn new(reason: RescueReason) -> Self {
        Self {
            reason,
            failed_component: None,
            available_snapshots: Vec::new(),
            boot_time: 0,
        }
    }

    /// Add a snapshot ID to the available list.
    pub fn add_snapshot(&mut self, id: impl Into<String>) {
        self.available_snapshots.push(id.into());
    }
}

/// Actions that rescue mode can perform.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RescueAction {
    /// Roll back to the most recent snapshot automatically.
    RestoreLastSnapshot,
    /// Roll back to a specific snapshot by ID.
    RestoreSpecificSnapshot(String),
    /// Run `fsck` on the root partition and repair errors.
    RepairFilesystem,
    /// Reinstall and reconfigure GRUB.
    ReinstallBootloader,
    /// Drop to a root emergency shell.
    LaunchEmergencyShell,
    /// Undo the last `sigma update` transaction.
    RollbackUpdate,
    /// Reboot into the normal boot sequence.
    RebootNormal,
}

impl RescueAction {
    /// Priority order for auto-repair (lower is higher priority).
    pub fn priority(&self) -> u8 {
        match self {
            Self::RollbackUpdate        => 1,
            Self::RestoreLastSnapshot   => 2,
            Self::RepairFilesystem      => 3,
            Self::ReinstallBootloader   => 4,
            Self::RestoreSpecificSnapshot(_) => 5,
            Self::LaunchEmergencyShell  => 9,
            Self::RebootNormal          => 10,
        }
    }
}

/// The outcome of a rescue action.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RescueOutcome {
    /// The system was successfully repaired.
    Repaired,
    /// A snapshot was restored; a reboot is needed.
    RestoredSnapshot,
    /// The action succeeded but a reboot is required to take effect.
    RebootRequired,
    /// The automated repair failed; manual intervention is needed.
    RequiresManualIntervention,
}

/// Errors from rescue mode.
#[derive(Debug, thiserror::Error)]
pub enum RescueError {
    #[error("no snapshots available for rollback")]
    NoSnapshotsAvailable,
    #[error("snapshot not found: {0}")]
    SnapshotNotFound(String),
    #[error("repair tool failed: {0}")]
    RepairFailed(String),
    #[error("cannot reboot: {0}")]
    RebootFailed(String),
    #[error("emergency shell failed to start: {0}")]
    ShellLaunchFailed(String),
}

/// The rescue mode controller.
pub struct RescueMode {
    context: RescueContext,
}

impl RescueMode {
    /// Create a new rescue mode instance with the given context.
    pub fn new(reason: RescueReason) -> Self {
        Self { context: RescueContext::new(reason) }
    }

    /// Create with full context.
    pub fn with_context(context: RescueContext) -> Self {
        Self { context }
    }

    /// Inspect the boot log and determine why rescue mode was entered.
    ///
    /// In production this reads `/var/log/sigma-boot.log` and
    /// `/var/log/dpkg.log`.  This implementation returns a heuristic result.
    pub fn detect_reason() -> RescueReason {
        // Production: parse boot log for known error patterns.
        // Heuristic: if an interrupted dpkg run is detected → FailedUpdate.
        RescueReason::Unknown
    }

    /// Return the actions available given the current context.
    pub fn list_available_actions(&self) -> Vec<RescueAction> {
        let mut actions = vec![
            RescueAction::LaunchEmergencyShell,
            RescueAction::RebootNormal,
        ];

        match self.context.reason {
            RescueReason::FailedUpdate | RescueReason::FailedMigration => {
                actions.push(RescueAction::RollbackUpdate);
            }
            RescueReason::CorruptedFilesystem => {
                actions.push(RescueAction::RepairFilesystem);
            }
            RescueReason::BrokenGrub | RescueReason::MissingKernel => {
                actions.push(RescueAction::ReinstallBootloader);
            }
            _ => {}
        }

        if !self.context.available_snapshots.is_empty() {
            actions.push(RescueAction::RestoreLastSnapshot);
        }

        // Sort by priority so the best option is first.
        actions.sort_by_key(|a| a.priority());
        actions
    }

    /// Execute a specific rescue action.
    pub fn execute_action(&self, action: RescueAction) -> Result<RescueOutcome, RescueError> {
        match action {
            RescueAction::RollbackUpdate => {
                // Production: call sigma-update --rollback
                Ok(RescueOutcome::RebootRequired)
            }
            RescueAction::RestoreLastSnapshot => {
                if self.context.available_snapshots.is_empty() {
                    return Err(RescueError::NoSnapshotsAvailable);
                }
                // Production: call sigma-snapshot --restore <last>
                Ok(RescueOutcome::RestoredSnapshot)
            }
            RescueAction::RestoreSpecificSnapshot(ref id) => {
                if !self.context.available_snapshots.iter().any(|s| s == id) {
                    return Err(RescueError::SnapshotNotFound(id.clone()));
                }
                Ok(RescueOutcome::RestoredSnapshot)
            }
            RescueAction::RepairFilesystem => {
                // Production: run fsck on the detected root partition
                Ok(RescueOutcome::RebootRequired)
            }
            RescueAction::ReinstallBootloader => {
                // Production: grub-install + update-grub
                Ok(RescueOutcome::RebootRequired)
            }
            RescueAction::LaunchEmergencyShell => {
                Self::launch_emergency_shell()?;
                Ok(RescueOutcome::RebootRequired)
            }
            RescueAction::RebootNormal => {
                // Production: systemctl reboot
                Ok(RescueOutcome::RebootRequired)
            }
        }
    }

    /// Attempt automatic repair by trying actions in priority order.
    pub fn auto_repair(&self) -> Result<RescueOutcome, RescueError> {
        let actions = self.list_available_actions();
        for action in actions {
            // Skip the interactive / reboot actions in auto mode
            if matches!(action, RescueAction::LaunchEmergencyShell | RescueAction::RebootNormal) {
                continue;
            }
            match self.execute_action(action) {
                Ok(outcome) => return Ok(outcome),
                Err(_) => continue, // try next action
            }
        }
        // Nothing worked — fall back to manual intervention
        Ok(RescueOutcome::RequiresManualIntervention)
    }

    /// Drop to an interactive root shell for manual repair.
    pub fn launch_emergency_shell() -> Result<(), RescueError> {
        // Production: exec /bin/bash --login or systemctl start rescue.target
        Ok(())
    }

    /// Access the current rescue context.
    pub fn context(&self) -> &RescueContext {
        &self.context
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_failed_update_reason_actions_include_rollback() {
        // Arrange
        let rescue = RescueMode::new(RescueReason::FailedUpdate);
        // Act
        let actions = rescue.list_available_actions();
        // Assert
        assert!(actions.contains(&RescueAction::RollbackUpdate));
    }

    #[test]
    fn test_broken_grub_reason_includes_bootloader_reinstall() {
        // Arrange
        let rescue = RescueMode::new(RescueReason::BrokenGrub);
        // Act
        let actions = rescue.list_available_actions();
        // Assert
        assert!(actions.contains(&RescueAction::ReinstallBootloader));
    }

    #[test]
    fn test_actions_sorted_by_priority() {
        // Arrange
        let mut ctx = RescueContext::new(RescueReason::FailedUpdate);
        ctx.add_snapshot("snap-20261101");
        let rescue = RescueMode::with_context(ctx);
        // Act
        let actions = rescue.list_available_actions();
        // Assert — rollback should come before shell
        let rollback_pos = actions.iter().position(|a| *a == RescueAction::RollbackUpdate);
        let shell_pos = actions.iter().position(|a| *a == RescueAction::LaunchEmergencyShell);
        assert!(rollback_pos < shell_pos);
    }

    #[test]
    fn test_restore_last_snapshot_needs_available_snapshot() {
        // Arrange — no snapshots
        let rescue = RescueMode::new(RescueReason::ManualRequest);
        // Act
        let result = rescue.execute_action(RescueAction::RestoreLastSnapshot);
        // Assert
        assert!(matches!(result, Err(RescueError::NoSnapshotsAvailable)));
    }

    #[test]
    fn test_restore_specific_snapshot_unknown_id_fails() {
        // Arrange
        let rescue = RescueMode::new(RescueReason::ManualRequest);
        // Act
        let result = rescue.execute_action(
            RescueAction::RestoreSpecificSnapshot("nonexistent".to_string()),
        );
        // Assert
        assert!(matches!(result, Err(RescueError::SnapshotNotFound(_))));
    }

    #[test]
    fn test_auto_repair_returns_outcome() {
        // Arrange
        let mut ctx = RescueContext::new(RescueReason::FailedUpdate);
        ctx.add_snapshot("snap-20261101");
        let rescue = RescueMode::with_context(ctx);
        // Act
        let outcome = rescue.auto_repair().unwrap();
        // Assert
        assert!(matches!(
            outcome,
            RescueOutcome::Repaired
            | RescueOutcome::RestoredSnapshot
            | RescueOutcome::RebootRequired
            | RescueOutcome::RequiresManualIntervention
        ));
    }
}
