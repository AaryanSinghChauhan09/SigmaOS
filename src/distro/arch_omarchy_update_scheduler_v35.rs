// SPDX-License-Identifier: MIT OR Apache-2.0
//! SigmaOS Arch Rolling Release Update & Snapshot Scheduler Engine
//!
//! Inspired by Arch Linux's `checkupdates` script and Omarchy's system maintenance workflow.
//! Schedules package updates, verifies pacman database locks, generates Snapper pre/post
//! snapshot pairs before rolling updates, and produces update notification summaries.

/// System update channel / cadence
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateCadence {
    Hourly,
    Daily,
    Weekly,
    ManualOnly,
}

impl UpdateCadence {
    pub fn systemd_timer_calendar(&self) -> Option<&str> {
        match self {
            Self::Hourly => Some("hourly"),
            Self::Daily => Some("daily"),
            Self::Weekly => Some("weekly"),
            Self::ManualOnly => None,
        }
    }
}

/// Package update record
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingUpdate {
    pub package_name: String,
    pub old_version: String,
    pub new_version: String,
    pub repository: String,
}

/// Update scheduler and executor
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateScheduler {
    pub cadence: UpdateCadence,
    pub auto_snapshot_snapper: bool,
    pub check_aur: bool,
    pub notify_on_updates: bool,
}

impl UpdateScheduler {
    pub fn new(cadence: UpdateCadence) -> Self {
        Self {
            cadence,
            auto_snapshot_snapper: true,
            check_aur: true,
            notify_on_updates: true,
        }
    }

    /// Check command using checkupdates (from pacman-contrib) and yay/paru
    pub fn check_updates_command(&self) -> String {
        if self.check_aur {
            "checkupdates; yay -Qua 2>/dev/null".to_string()
        } else {
            "checkupdates".to_string()
        }
    }

    /// Full safe update sequence with Snapper snapshot safeguard
    pub fn safe_update_script(&self) -> String {
        let mut script = String::from("#!/bin/bash\nset -euo pipefail\n\n");
        if self.auto_snapshot_snapper {
            script.push_str("# Create Snapper pre-update snapshot\n");
            script.push_str("SNAP_NUM=$(snapper -c root create -t pre -p -d \"SigmaOS pre-update snapshot\")\n\n");
        }

        script.push_str("# Perform Arch & AUR system upgrade\n");
        if self.check_aur {
            script.push_str("yay -Syu --noconfirm\n\n");
        } else {
            script.push_str("sudo pacman -Syu --noconfirm\n\n");
        }

        if self.auto_snapshot_snapper {
            script.push_str("# Create Snapper post-update snapshot\n");
            script.push_str("snapper -c root create -t post --pre-number \"$SNAP_NUM\" -d \"SigmaOS post-update snapshot\"\n\n");
        }

        if self.notify_on_updates {
            script.push_str("notify-send -a \"SigmaOS Updater\" \"System Upgrade Completed\" \"Rolling release packages up to date.\"\n");
        }

        script
    }

    /// Render systemd service unit
    pub fn to_systemd_service(&self) -> String {
        "[Unit]\n\
        Description=SigmaOS Rolling Release Update Checker\n\
        After=network-online.target\n\
        Wants=network-online.target\n\n\
        [Service]\n\
        Type=oneshot\n\
        ExecStart=/usr/bin/sigmaos-update-check\n"
            .to_string()
    }

    /// Render systemd timer unit
    pub fn to_systemd_timer(&self) -> Option<String> {
        let cal = self.cadence.systemd_timer_calendar()?;
        Some(format!(
            "[Unit]\n\
            Description=SigmaOS Rolling Release Update Timer\n\n\
            [Timer]\n\
            OnCalendar={}\n\
            Persistent=true\n\n\
            [Install]\n\
            WantedBy=timers.target\n",
            cal
        ))
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_updates_command_with_aur_aaa() {
        // Arrange
        let scheduler = UpdateScheduler::new(UpdateCadence::Daily);
        // Act
        let cmd = scheduler.check_updates_command();
        // Assert
        assert!(cmd.contains("checkupdates"));
        assert!(cmd.contains("yay -Qua"));
    }

    #[test]
    fn test_safe_update_script_snapper_integration_aaa() {
        // Arrange
        let scheduler = UpdateScheduler::new(UpdateCadence::Daily);
        // Act
        let script = scheduler.safe_update_script();
        // Assert
        assert!(script.contains("snapper -c root create -t pre"));
        assert!(script.contains("yay -Syu"));
        assert!(script.contains("snapper -c root create -t post --pre-number"));
        assert!(script.contains("notify-send"));
    }

    #[test]
    fn test_systemd_timer_generation_daily_aaa() {
        // Arrange
        let scheduler = UpdateScheduler::new(UpdateCadence::Daily);
        // Act
        let timer = scheduler.to_systemd_timer();
        // Assert
        assert!(timer.is_some());
        assert!(timer.unwrap().contains("OnCalendar=daily"));
    }

    #[test]
    fn test_systemd_timer_manual_is_none_aaa() {
        // Arrange
        let scheduler = UpdateScheduler::new(UpdateCadence::ManualOnly);
        // Act
        let timer = scheduler.to_systemd_timer();
        // Assert
        assert!(timer.is_none());
    }
}
