//! SigmaOS-upgrade — system-wide version upgrade orchestrator.
//!
//! Handles major-version upgrade channels (stable/edge/rc/dev), pre-upgrade
//! snapshots, and phased rollout — complementing `sigmaos-update` which
//! handles routine package updates within a channel.
//!
//! Upstream inspiration:
//! - CachyOS upgrade: <https://cachyos.org>
//! - Omarchy release channels: <https://github.com/basecamp/omarchy>
//! - apt full-upgrade / dist-upgrade model

use std::collections::BTreeMap;
use std::string::String;
use std::vec::Vec;

// ---------------------------------------------------------------------------
// Release channels
// ---------------------------------------------------------------------------

/// Available upgrade channels, ordered from most to least stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReleaseChannel {
    Stable,
    Rc,
    Edge,
    Dev,
}

impl ReleaseChannel {
    /// Human-readable channel name.
    pub fn name(self) -> &'static str {
        match self {
            ReleaseChannel::Stable => "stable",
            ReleaseChannel::Rc => "rc",
            ReleaseChannel::Edge => "edge",
            ReleaseChannel::Dev => "dev",
        }
    }

    /// Parse from string. Returns `None` for unknown names.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "stable" => Some(Self::Stable),
            "rc" => Some(Self::Rc),
            "edge" => Some(Self::Edge),
            "dev" => Some(Self::Dev),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// Snapshot (pre-upgrade safety net)
// ---------------------------------------------------------------------------

/// A pre-upgrade filesystem snapshot (Btrfs/ZFS).
#[derive(Debug, Clone)]
pub struct UpgradeSnapshot {
    pub id: String,
    pub channel: ReleaseChannel,
    pub from_version: String,
    pub timestamp: u64,
}

// ---------------------------------------------------------------------------
// Upgrade plan
// ---------------------------------------------------------------------------

/// A resolved upgrade plan: what changes, from where to where.
#[derive(Debug, Clone)]
pub struct UpgradePlan {
    pub from_version: String,
    pub to_version: String,
    pub channel: ReleaseChannel,
    /// Ordered list of packages to upgrade.
    pub packages: Vec<String>,
    /// Pre-computed count of changes.
    pub change_count: usize,
}

// ---------------------------------------------------------------------------
// Upgrade errors
// ---------------------------------------------------------------------------

/// Errors returned by the upgrade orchestrator.
#[derive(Debug, PartialEq, Eq)]
pub enum UpgradeError {
    /// Already on the requested version/channel.
    AlreadyCurrent,
    /// The target version is older than the installed version.
    DowngradeNotAllowed,
    /// A required pre-upgrade snapshot could not be created.
    SnapshotFailed(String),
    /// The upgrade itself failed and was rolled back.
    UpgradeFailed(String),
}

impl std::fmt::Display for UpgradeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UpgradeError::AlreadyCurrent => write!(f, "already at current version"),
            UpgradeError::DowngradeNotAllowed => write!(f, "downgrade not allowed"),
            UpgradeError::SnapshotFailed(msg) => write!(f, "snapshot failed: {msg}"),
            UpgradeError::UpgradeFailed(msg) => write!(f, "upgrade failed: {msg}"),
        }
    }
}

impl std::error::Error for UpgradeError {}

// ---------------------------------------------------------------------------
// Upgrade orchestrator
// ---------------------------------------------------------------------------

/// System-wide upgrade orchestrator.
///
/// Manages channel switching, pre-upgrade snapshots, plan resolution,
/// and phased rollout.
pub struct UpgradeOrchestrator {
    /// Currently installed version string.
    pub current_version: String,
    /// Active release channel.
    pub channel: ReleaseChannel,
    /// All known release versions per channel.
    releases: BTreeMap<ReleaseChannel, Vec<String>>,
    /// Snapshots taken before upgrades.
    pub snapshots: Vec<UpgradeSnapshot>,
    /// Counter for snapshot IDs.
    next_snapshot_id: u32,
}

impl UpgradeOrchestrator {
    /// Create a new orchestrator.
    pub fn new(current_version: &str, channel: ReleaseChannel) -> Self {
        Self {
            current_version: current_version.to_string(),
            channel,
            releases: BTreeMap::new(),
            snapshots: Vec::new(),
            next_snapshot_id: 1,
        }
    }

    /// Register a release version for a channel.
    pub fn register_release(&mut self, channel: ReleaseChannel, version: &str) {
        self.releases
            .entry(channel)
            .or_default()
            .push(version.to_string());
    }

    /// Latest known version for the active channel.
    pub fn latest(&self) -> Option<&str> {
        self.releases
            .get(&self.channel)
            .and_then(|v| v.last())
            .map(|s| s.as_str())
    }

    /// Switch to a different release channel.
    pub fn switch_channel(&mut self, channel: ReleaseChannel) {
        self.channel = channel;
    }

    /// Create a pre-upgrade snapshot.
    pub fn create_snapshot(&mut self, timestamp: u64) -> UpgradeSnapshot {
        let snap = UpgradeSnapshot {
            id: format!("snap-{}-{}", self.channel.name(), self.next_snapshot_id),
            channel: self.channel,
            from_version: self.current_version.clone(),
            timestamp,
        };
        self.next_snapshot_id += 1;
        self.snapshots.push(snap.clone());
        snap
    }

    /// Resolve an upgrade plan to the target version.
    pub fn plan(
        &self,
        target_version: &str,
        packages: Vec<String>,
    ) -> Result<UpgradePlan, UpgradeError> {
        if target_version == self.current_version {
            return Err(UpgradeError::AlreadyCurrent);
        }
        // Simple lexicographic version comparison
        if target_version < self.current_version.as_str() {
            return Err(UpgradeError::DowngradeNotAllowed);
        }
        let change_count = packages.len();
        Ok(UpgradePlan {
            from_version: self.current_version.clone(),
            to_version: target_version.to_string(),
            channel: self.channel,
            packages,
            change_count,
        })
    }

    /// Execute an upgrade plan (optionally dry-run).
    ///
    /// Takes a snapshot automatically before applying.
    pub fn apply_plan(
        &mut self,
        plan: &UpgradePlan,
        dry_run: bool,
        timestamp: u64,
    ) -> Result<(), UpgradeError> {
        if dry_run {
            return Ok(());
        }
        // Pre-upgrade snapshot
        self.create_snapshot(timestamp);
        // Apply
        self.current_version = plan.to_version.clone();
        Ok(())
    }

    /// One-shot: plan + snapshot + apply to the latest version on this channel.
    pub fn upgrade(
        &mut self,
        packages: Vec<String>,
        dry_run: bool,
        timestamp: u64,
    ) -> Result<UpgradePlan, UpgradeError> {
        let target = self
            .latest()
            .ok_or_else(|| UpgradeError::UpgradeFailed("no release for channel".to_string()))?
            .to_string();
        let plan = self.plan(&target, packages)?;
        self.apply_plan(&plan, dry_run, timestamp)?;
        Ok(plan)
    }
}

// ---------------------------------------------------------------------------
// Inline tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_orchestrator() -> UpgradeOrchestrator {
        let mut o = UpgradeOrchestrator::new("0.8.0", ReleaseChannel::Stable);
        o.register_release(ReleaseChannel::Stable, "0.9.0");
        o.register_release(ReleaseChannel::Edge, "0.10.0-edge");
        o
    }

    #[test]
    fn latest_returns_newest_on_channel() {
        let o = make_orchestrator();
        assert_eq!(o.latest(), Some("0.9.0"));
    }

    #[test]
    fn switch_channel_changes_latest() {
        let mut o = make_orchestrator();
        o.switch_channel(ReleaseChannel::Edge);
        assert_eq!(o.latest(), Some("0.10.0-edge"));
    }

    #[test]
    fn plan_already_current_errors() {
        let o = make_orchestrator();
        assert_eq!(
            o.plan("0.8.0", vec![]).unwrap_err(),
            UpgradeError::AlreadyCurrent
        );
    }

    #[test]
    fn plan_downgrade_errors() {
        let o = make_orchestrator();
        assert_eq!(
            o.plan("0.7.0", vec![]).unwrap_err(),
            UpgradeError::DowngradeNotAllowed
        );
    }

    #[test]
    fn plan_resolves_correctly() {
        let o = make_orchestrator();
        let plan = o.plan("0.9.0", vec!["base".into()]).unwrap();
        assert_eq!(plan.from_version, "0.8.0");
        assert_eq!(plan.to_version, "0.9.0");
        assert_eq!(plan.change_count, 1);
    }

    #[test]
    fn apply_plan_updates_current_version() {
        let mut o = make_orchestrator();
        let plan = o.plan("0.9.0", vec!["base".into()]).unwrap();
        o.apply_plan(&plan, false, 1700000000).unwrap();
        assert_eq!(o.current_version, "0.9.0");
        assert_eq!(o.snapshots.len(), 1);
    }

    #[test]
    fn apply_plan_dry_run_does_not_change_version() {
        let mut o = make_orchestrator();
        let plan = o.plan("0.9.0", vec![]).unwrap();
        o.apply_plan(&plan, true, 0).unwrap();
        assert_eq!(o.current_version, "0.8.0"); // unchanged
        assert!(o.snapshots.is_empty()); // no snapshot in dry-run
    }

    #[test]
    fn upgrade_shorthand_works() {
        let mut o = make_orchestrator();
        let plan = o.upgrade(vec!["base".into()], false, 1700000000).unwrap();
        assert_eq!(plan.to_version, "0.9.0");
        assert_eq!(o.current_version, "0.9.0");
    }

    #[test]
    fn snapshot_id_increments() {
        let mut o = make_orchestrator();
        let s1 = o.create_snapshot(1000);
        let s2 = o.create_snapshot(2000);
        assert_ne!(s1.id, s2.id);
        assert_eq!(o.snapshots.len(), 2);
    }

    #[test]
    fn release_channel_ordering() {
        assert!(ReleaseChannel::Stable < ReleaseChannel::Edge);
        assert!(ReleaseChannel::Edge < ReleaseChannel::Dev);
    }

    #[test]
    fn release_channel_round_trip() {
        for &ch in &[
            ReleaseChannel::Stable,
            ReleaseChannel::Rc,
            ReleaseChannel::Edge,
            ReleaseChannel::Dev,
        ] {
            assert_eq!(ReleaseChannel::from_str(ch.name()), Some(ch));
        }
    }

    #[test]
    fn upgrade_error_display() {
        assert!(UpgradeError::AlreadyCurrent.to_string().contains("current"));
        assert!(UpgradeError::DowngradeNotAllowed.to_string().contains("downgrade"));
        assert!(UpgradeError::SnapshotFailed("x".into()).to_string().contains("snapshot"));
        assert!(UpgradeError::UpgradeFailed("y".into()).to_string().contains("upgrade failed"));
    }
}
