//! Pre/post-update snapshot and rollback management.
//!
//! Every `sigma update` run is wrapped in a transaction.  Before packages
//! are installed a BTRFS snapshot is taken.  If installation fails, or if
//! post-update health checks fail, the transaction is aborted and the
//! snapshot is restored automatically.

/// Why a transaction was rolled back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RollbackReason {
    /// The package manager returned a non-zero exit code.
    UpdateFailed,
    /// Post-update health checks detected a broken system.
    SystemUnstable,
    /// The user explicitly requested a rollback.
    UserRequested,
    /// Automated post-update tests failed.
    PostUpdateTestFailed,
}

impl RollbackReason {
    /// Short label for logging.
    pub fn label(&self) -> &'static str {
        match self {
            Self::UpdateFailed          => "update-failed",
            Self::SystemUnstable        => "system-unstable",
            Self::UserRequested         => "user-requested",
            Self::PostUpdateTestFailed  => "post-update-test-failed",
        }
    }
}

/// Lifecycle state of an update transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateStatus {
    /// Transaction created but not yet started.
    Pending,
    /// Packages are being downloaded and installed.
    InProgress,
    /// All packages installed successfully and health checks passed.
    Completed,
    /// Installation failed; rollback was triggered.
    Failed,
    /// Rolled back to the pre-update snapshot.
    RolledBack,
}

/// A single update transaction record.
#[derive(Debug, Clone)]
pub struct UpdateTransaction {
    /// Unique transaction ID (e.g. `"tx-20261101-001"`).
    pub id: String,
    /// Packages included in this transaction.
    pub package_names: Vec<String>,
    /// ID of the snapshot taken before the update.
    pub pre_snapshot_id: String,
    /// ID of the snapshot taken after a successful update.
    pub post_snapshot_id: Option<String>,
    /// Current status.
    pub status: UpdateStatus,
    /// Unix timestamp when the transaction started.
    pub started_at: u64,
    /// Unix timestamp when the transaction ended (completed or failed).
    pub completed_at: Option<u64>,
}

impl UpdateTransaction {
    /// Create a new pending transaction.
    pub fn new(id: impl Into<String>, packages: &[&str], pre_snapshot: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            package_names: packages.iter().map(|s| s.to_string()).collect(),
            pre_snapshot_id: pre_snapshot.into(),
            post_snapshot_id: None,
            status: UpdateStatus::Pending,
            started_at: 0,
            completed_at: None,
        }
    }

    /// Whether this transaction can still be rolled back.
    pub fn can_rollback(&self) -> bool {
        matches!(
            self.status,
            UpdateStatus::InProgress | UpdateStatus::Completed | UpdateStatus::Failed
        ) && !self.pre_snapshot_id.is_empty()
    }
}

/// Errors from the update rollback manager.
#[derive(Debug, thiserror::Error)]
pub enum RollbackError {
    #[error("transaction not found: {0}")]
    TransactionNotFound(String),
    #[error("snapshot creation failed: {0}")]
    SnapshotCreationFailed(String),
    #[error("rollback failed: snapshot {0} could not be restored")]
    RollbackFailed(String),
    #[error("transaction is in status {0:?}, rollback not possible")]
    InvalidStatus(UpdateStatus),
    #[error("package installation failed: {0}")]
    InstallFailed(String),
    #[error("health check failed: {0}")]
    HealthCheckFailed(String),
}

/// Manages the pre/post-update snapshot lifecycle.
pub struct UpdateRollbackManager {
    transactions: Vec<UpdateTransaction>,
    next_tx_index: u32,
}

impl UpdateRollbackManager {
    /// Create a new manager.
    pub fn new() -> Self {
        Self {
            transactions: Vec::new(),
            next_tx_index: 1,
        }
    }

    /// Begin a new transaction: creates a pre-update snapshot and returns the transaction.
    pub fn begin_transaction(
        &mut self,
        packages: &[&str],
    ) -> Result<UpdateTransaction, RollbackError> {
        let tx_id = format!("tx-{:04}", self.next_tx_index);
        self.next_tx_index += 1;

        // Production: call `btrfs subvolume snapshot / /.snapshots/<id>`
        let snap_id = format!("pre-update-{}", &tx_id);

        let mut tx = UpdateTransaction::new(&tx_id, packages, &snap_id);
        tx.status = UpdateStatus::InProgress;
        tx.started_at = Self::now();

        self.transactions.push(tx.clone());
        Ok(tx)
    }

    /// Mark a transaction as successfully completed.
    ///
    /// Takes a post-update snapshot and records completion.
    pub fn commit_transaction(&mut self, tx_id: &str) -> Result<(), RollbackError> {
        let tx = self.find_mut(tx_id)?;
        if tx.status != UpdateStatus::InProgress {
            return Err(RollbackError::InvalidStatus(tx.status));
        }
        // Production: btrfs subvolume snapshot for post state
        tx.post_snapshot_id = Some(format!("post-update-{tx_id}"));
        tx.status = UpdateStatus::Completed;
        tx.completed_at = Some(Self::now());
        Ok(())
    }

    /// Abort a transaction: restore the pre-update snapshot.
    pub fn abort_transaction(
        &mut self,
        tx_id: &str,
        reason: RollbackReason,
    ) -> Result<(), RollbackError> {
        let snap_id = {
            let tx = self.find_mut(tx_id)?;
            if !tx.can_rollback() {
                return Err(RollbackError::InvalidStatus(tx.status));
            }
            let s = tx.pre_snapshot_id.clone();
            tx.status = UpdateStatus::RolledBack;
            tx.completed_at = Some(Self::now());
            s
        };
        // Production: btrfs subvolume set-default <snap_id>; reboot or remount
        let _ = (snap_id, reason); // used in production
        Ok(())
    }

    /// Explicitly roll back to the state before a completed transaction.
    pub fn rollback_to_before(&mut self, tx_id: &str) -> Result<(), RollbackError> {
        self.abort_transaction(tx_id, RollbackReason::UserRequested)
    }

    /// Return all recorded transactions (most recent last).
    pub fn list_transactions(&self) -> &[UpdateTransaction] {
        &self.transactions
    }

    /// Run post-update health checks: return `true` if the system is healthy.
    ///
    /// Production checks: `systemctl is-system-running`, key service status,
    /// package database integrity.
    pub fn verify_transaction(&self, tx_id: &str) -> Result<bool, RollbackError> {
        let tx = self.find(tx_id)?;
        if tx.status != UpdateStatus::Completed {
            return Err(RollbackError::InvalidStatus(tx.status));
        }
        // Simplified: always healthy in simulation.
        Ok(true)
    }

    /// `true` if the last transaction can still be rolled back.
    pub fn pending_rollback(&self) -> bool {
        self.transactions.last().map_or(false, |tx| tx.can_rollback())
    }

    // --- private helpers ---

    fn find(&self, id: &str) -> Result<&UpdateTransaction, RollbackError> {
        self.transactions.iter().find(|t| t.id == id)
            .ok_or_else(|| RollbackError::TransactionNotFound(id.to_string()))
    }

    fn find_mut(&mut self, id: &str) -> Result<&mut UpdateTransaction, RollbackError> {
        self.transactions.iter_mut().find(|t| t.id == id)
            .ok_or_else(|| RollbackError::TransactionNotFound(id.to_string()))
    }

    fn now() -> u64 {
        // Production: std::time::SystemTime::UNIX_EPOCH.elapsed().unwrap().as_secs()
        0
    }
}

impl Default for UpdateRollbackManager {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_begin_transaction_creates_snapshot() {
        // Arrange
        let mut mgr = UpdateRollbackManager::new();
        // Act
        let tx = mgr.begin_transaction(&["sigma-browser", "sigma-update"]).unwrap();
        // Assert
        assert!(!tx.pre_snapshot_id.is_empty());
        assert_eq!(tx.status, UpdateStatus::InProgress);
        assert_eq!(tx.package_names.len(), 2);
    }

    #[test]
    fn test_commit_marks_completed() {
        // Arrange
        let mut mgr = UpdateRollbackManager::new();
        let tx = mgr.begin_transaction(&["curl"]).unwrap();
        // Act
        mgr.commit_transaction(&tx.id).unwrap();
        // Assert
        let committed = mgr.list_transactions().last().unwrap();
        assert_eq!(committed.status, UpdateStatus::Completed);
        assert!(committed.post_snapshot_id.is_some());
    }

    #[test]
    fn test_abort_marks_rolled_back() {
        // Arrange
        let mut mgr = UpdateRollbackManager::new();
        let tx = mgr.begin_transaction(&["broken-pkg"]).unwrap();
        // Act
        mgr.abort_transaction(&tx.id, RollbackReason::UpdateFailed).unwrap();
        // Assert
        let aborted = mgr.list_transactions().last().unwrap();
        assert_eq!(aborted.status, UpdateStatus::RolledBack);
    }

    #[test]
    fn test_rollback_non_existent_tx_fails() {
        // Arrange
        let mut mgr = UpdateRollbackManager::new();
        // Act
        let result = mgr.rollback_to_before("tx-9999");
        // Assert
        assert!(matches!(result, Err(RollbackError::TransactionNotFound(_))));
    }

    #[test]
    fn test_pending_rollback_true_when_in_progress() {
        // Arrange
        let mut mgr = UpdateRollbackManager::new();
        let _ = mgr.begin_transaction(&["pkg-a"]).unwrap();
        // Act & Assert
        assert!(mgr.pending_rollback());
    }

    #[test]
    fn test_verify_transaction_healthy() {
        // Arrange
        let mut mgr = UpdateRollbackManager::new();
        let tx = mgr.begin_transaction(&["pkg-b"]).unwrap();
        mgr.commit_transaction(&tx.id).unwrap();
        // Act
        let healthy = mgr.verify_transaction(&tx.id).unwrap();
        // Assert
        assert!(healthy);
    }
}
