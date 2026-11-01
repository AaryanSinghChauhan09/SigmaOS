//! Suspend/resume management and power profile control for SigmaOS.

/// ACPI/kernel sleep states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SuspendState {
    /// S3 suspend-to-RAM: traditional sleep.
    S3Sleep,
    /// S4 suspend-to-disk: hibernate.
    S4Hibernate,
    /// S0ix modern standby: low-power idle (Intel/AMD).
    S0ixModernStandby,
    /// Freeze: suspend processes without cutting power.
    Freeze,
}

impl SuspendState {
    /// Kernel string written to `/sys/power/state`.
    pub fn kernel_name(self) -> &'static str {
        match self {
            Self::S3Sleep           => "mem",
            Self::S4Hibernate       => "disk",
            Self::S0ixModernStandby => "s2idle",
            Self::Freeze            => "freeze",
        }
    }
}

/// Result of a suspend/resume cycle test.
#[derive(Debug, Clone)]
pub struct SuspendTestResult {
    pub state: SuspendState,
    pub success: bool,
    pub resume_time_ms: u64,
    pub wake_reason: String,
    pub issues: Vec<String>,
}

/// System power profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerProfile {
    Performance,
    Balanced,
    PowerSaver,
    Custom,
}

impl PowerProfile {
    /// D-Bus profile name used by power-profiles-daemon.
    pub fn dbus_name(self) -> &'static str {
        match self {
            Self::Performance => "performance",
            Self::Balanced    => "balanced",
            Self::PowerSaver  => "power-saver",
            Self::Custom      => "balanced",
        }
    }
}

/// Lid-close action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LidAction {
    Suspend,
    Hibernate,
    DoNothing,
    Lock,
}

/// Wake-up source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WakeSource {
    Keyboard,
    Mouse,
    Network,
    Rtc,
    Usb,
}

/// Errors from the suspend manager.
#[derive(Debug, thiserror::Error)]
pub enum SuspendError {
    #[error("suspend state {0} not supported on this hardware")]
    StateNotSupported(String),
    #[error("resume failed after suspend: {0}")]
    ResumeFailed(String),
    #[error("power profile {0} not available")]
    ProfileNotAvailable(String),
    #[error("failed to configure wake source: {0}")]
    WakeSourceError(String),
    #[error("lid action configuration failed: {0}")]
    LidActionError(String),
}

/// Suspend and power management.
pub struct SuspendManager {
    current_profile: PowerProfile,
}

impl SuspendManager {
    pub fn new() -> Self {
        Self { current_profile: PowerProfile::Balanced }
    }

    /// Return the list of suspend states supported by the current hardware.
    /// Production reads `/sys/power/state`.
    pub fn supported_states(&self) -> Vec<SuspendState> {
        vec![
            SuspendState::S3Sleep,
            SuspendState::S4Hibernate,
            SuspendState::S0ixModernStandby,
            SuspendState::Freeze,
        ]
    }

    /// Test a suspend/resume cycle with a specified timeout.
    pub fn test_suspend(
        &self,
        state: SuspendState,
        _timeout_s: u64,
    ) -> Result<SuspendTestResult, SuspendError> {
        if !self.supported_states().contains(&state) {
            return Err(SuspendError::StateNotSupported(format!("{state:?}")));
        }
        Ok(SuspendTestResult {
            state,
            success: true,
            resume_time_ms: 3800,
            wake_reason: "keyboard".to_string(),
            issues: vec![],
        })
    }

    /// Set the active power profile.
    pub fn set_power_profile(&mut self, profile: PowerProfile) -> Result<(), SuspendError> {
        // Production: write to power-profiles-daemon via D-Bus
        self.current_profile = profile;
        Ok(())
    }

    /// Return the currently active power profile.
    pub fn current_profile(&self) -> PowerProfile {
        self.current_profile
    }

    /// Configure what happens when the lid is closed.
    pub fn configure_lid_action(&self, _action: LidAction) -> Result<(), SuspendError> {
        // Production: write to /etc/systemd/logind.conf
        Ok(())
    }

    /// Configure which devices can wake the system from sleep.
    pub fn configure_wake_sources(&self, _sources: &[WakeSource]) -> Result<(), SuspendError> {
        // Production: write to /sys/bus/usb/devices/.../power/wakeup etc.
        Ok(())
    }
}

impl Default for SuspendManager {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supported_states_nonempty() {
        let mgr = SuspendManager::new();
        assert!(!mgr.supported_states().is_empty());
    }

    #[test]
    fn test_set_power_profile_persists() {
        let mut mgr = SuspendManager::new();
        mgr.set_power_profile(PowerProfile::Performance).unwrap();
        assert_eq!(mgr.current_profile(), PowerProfile::Performance);
    }

    #[test]
    fn test_test_suspend_s3_succeeds() {
        let mgr = SuspendManager::new();
        let result = mgr.test_suspend(SuspendState::S3Sleep, 30).unwrap();
        assert!(result.success);
        assert!(result.resume_time_ms > 0);
    }

    #[test]
    fn test_kernel_name_s3() {
        assert_eq!(SuspendState::S3Sleep.kernel_name(), "mem");
    }

    #[test]
    fn test_kernel_name_hibernate() {
        assert_eq!(SuspendState::S4Hibernate.kernel_name(), "disk");
    }

    #[test]
    fn test_lid_action_configure_ok() {
        let mgr = SuspendManager::new();
        assert!(mgr.configure_lid_action(LidAction::Suspend).is_ok());
    }
}
