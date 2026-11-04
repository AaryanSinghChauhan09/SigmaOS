//! SigmaOS Release Management
//!
//! Defines release channels, version coordination, rollback policy,
//! and release readiness checks for the SigmaOS multi-repo ecosystem.

// ---------------------------------------------------------------------------
// Version
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SigmaVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub pre: Option<String>, // e.g. "alpha", "beta", "rc.1"
}

impl SigmaVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
            pre: None,
        }
    }

    pub fn pre(mut self, label: impl Into<String>) -> Self {
        self.pre = Some(label.into());
        self
    }

    pub fn to_string(&self) -> String {
        let base = format!("{}.{}.{}", self.major, self.minor, self.patch);
        match &self.pre {
            Some(p) => format!("{}-{}", base, p),
            None => base,
        }
    }

    pub fn is_stable(&self) -> bool {
        self.pre.is_none()
    }
}

/// The current SigmaOS version.
pub const CURRENT_VERSION: &str = "0.9.0-alpha";
pub const CURRENT_MAJOR: u32 = 0;
pub const CURRENT_MINOR: u32 = 9;
pub const CURRENT_PATCH: u32 = 0;

// ---------------------------------------------------------------------------
// Release channels
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseChannel {
    /// Latest stable releases — fully tested, QA gated.
    Stable,
    /// Current development cycle — passes smoke tests but may have rough edges.
    Testing,
    /// Nightly builds — automated, not manually QA'd.
    Nightly,
}

impl ReleaseChannel {
    pub fn name(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Testing => "testing",
            Self::Nightly => "nightly",
        }
    }

    pub fn requires_signed_packages(self) -> bool {
        matches!(self, Self::Stable | Self::Testing)
    }

    pub fn allows_unsigned(self) -> bool {
        matches!(self, Self::Nightly)
    }
}

// ---------------------------------------------------------------------------
// Release readiness
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct ReleaseCheck {
    pub name: &'static str,
    pub passed: bool,
    pub detail: String,
}

pub struct ReleaseReadiness {
    pub version: String,
    pub channel: ReleaseChannel,
    pub checks: Vec<ReleaseCheck>,
}

impl ReleaseReadiness {
    pub fn evaluate(channel: ReleaseChannel) -> Self {
        use crate::sigma_platform::{BackendKind, SigmaPlatform};
        let platform = SigmaPlatform::boot(BackendKind::Stub);

        let checks = vec![
            ReleaseCheck {
                name: "platform_boots",
                passed: platform.boot.is_ready(),
                detail: format!("Boot stage: {:?}", platform.boot.current_stage),
            },
            ReleaseCheck {
                name: "required_services_running",
                passed: platform.system.all_required_running(),
                detail: format!("{} services running", platform.system.running_count()),
            },
            ReleaseCheck {
                name: "base_packages_signed",
                passed: platform.packages.all_signed(),
                detail: format!("{} packages installed", platform.packages.installed_count()),
            },
            ReleaseCheck {
                name: "config_valid",
                passed: platform.config.is_valid(),
                detail: format!(
                    "theme={} lang={}",
                    platform.config.theme, platform.config.language
                ),
            },
            ReleaseCheck {
                name: "security_hardened",
                passed: platform.security.is_hardened(),
                detail: format!("profile={}", platform.security.hardening_profile),
            },
            ReleaseCheck {
                name: "default_apps_available",
                passed: platform.apps.installed_count() >= 4,
                detail: format!("{} apps registered", platform.apps.installed_count()),
            },
            ReleaseCheck {
                name: "signed_packages_enforced",
                passed: if channel.requires_signed_packages() {
                    platform.security.signed_packages_only
                } else {
                    true
                },
                detail: format!("channel={}", channel.name()),
            },
        ];

        Self {
            version: CURRENT_VERSION.to_string(),
            channel,
            checks,
        }
    }

    pub fn is_release_ready(&self) -> bool {
        self.checks.iter().all(|c| c.passed)
    }

    pub fn failed_checks(&self) -> Vec<&ReleaseCheck> {
        self.checks.iter().filter(|c| !c.passed).collect()
    }

    pub fn report(&self) -> Vec<String> {
        let mut lines = vec![format!(
            "Release Readiness — v{}  channel={}",
            self.version,
            self.channel.name()
        )];
        for c in &self.checks {
            lines.push(format!(
                "  [{}] {}  ({})",
                if c.passed { "PASS" } else { "FAIL" },
                c.name,
                c.detail
            ));
        }
        lines.push(format!(
            "Overall: {}",
            if self.is_release_ready() {
                "READY ✅"
            } else {
                "NOT READY ❌"
            }
        ));
        lines
    }
}

// ---------------------------------------------------------------------------
// Rollback policy
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollbackTrigger {
    /// Smoke test failure after update.
    SmokeTestFailure,
    /// Boot failure detected.
    BootFailure,
    /// User-initiated rollback.
    Manual,
}

#[derive(Debug, Clone)]
pub struct RollbackRecord {
    pub from_version: String,
    pub to_version: String,
    pub trigger: RollbackTrigger,
    pub success: bool,
}

pub struct RollbackManager {
    pub history: Vec<RollbackRecord>,
}

impl RollbackManager {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }

    pub fn execute(&mut self, from: &str, to: &str, trigger: RollbackTrigger) -> RollbackRecord {
        let record = RollbackRecord {
            from_version: from.to_string(),
            to_version: to.to_string(),
            trigger,
            success: true, // Real impl: verify boot + smoke test post-rollback
        };
        self.history.push(record.clone());
        record
    }

    pub fn rollback_count(&self) -> usize {
        self.history.len()
    }
}

impl Default for RollbackManager {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_formatting() {
        let v = SigmaVersion::new(0, 9, 0).pre("alpha");
        assert_eq!(v.to_string(), "0.9.0-alpha");
        assert!(!v.is_stable());

        let stable = SigmaVersion::new(1, 0, 0);
        assert_eq!(stable.to_string(), "1.0.0");
        assert!(stable.is_stable());
    }

    #[test]
    fn release_channels() {
        assert!(ReleaseChannel::Stable.requires_signed_packages());
        assert!(ReleaseChannel::Testing.requires_signed_packages());
        assert!(!ReleaseChannel::Nightly.requires_signed_packages());
        assert!(ReleaseChannel::Nightly.allows_unsigned());
    }

    #[test]
    fn release_readiness_testing_channel() {
        let rr = ReleaseReadiness::evaluate(ReleaseChannel::Testing);
        assert!(
            rr.is_release_ready(),
            "Release readiness failed:\n{}",
            rr.report().join("\n")
        );
        assert_eq!(rr.failed_checks().len(), 0);
    }

    #[test]
    fn release_readiness_stable_channel() {
        let rr = ReleaseReadiness::evaluate(ReleaseChannel::Stable);
        // In stub mode all checks should pass
        assert!(rr.is_release_ready());
    }

    #[test]
    fn rollback_manager_records() {
        let mut rm = RollbackManager::new();
        let rec = rm.execute("0.9.1", "0.9.0", RollbackTrigger::SmokeTestFailure);
        assert!(rec.success);
        assert_eq!(rm.rollback_count(), 1);
        assert_eq!(rec.from_version, "0.9.1");
        assert_eq!(rec.to_version, "0.9.0");
    }

    #[test]
    fn report_has_overall_line() {
        let rr = ReleaseReadiness::evaluate(ReleaseChannel::Testing);
        let report = rr.report();
        assert!(
            report.last().unwrap().contains("READY")
                || report.last().unwrap().contains("NOT READY")
        );
    }
}
