// SPDX-License-Identifier: MIT OR Apache-2.0
//! SigmaOS User Account & PAM Authentication Provisioning Engine
//!
//! Inspired by Arch Linux's user management conventions and Omarchy's
//! out-of-the-box user environment. Automates creation of system/regular users,
//! standard security group assignments (wheel, audio, video, input, storage),
//! shell configuration (zsh/bash/fish), sudoers wheel nopasswd/rules, and PAM policies.

use std::collections::BTreeSet;

/// User account shell choice
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserShell {
    Zsh,
    Bash,
    Fish,
    Nologin,
}

impl UserShell {
    pub fn binary_path(&self) -> &str {
        match self {
            Self::Zsh => "/bin/zsh",
            Self::Bash => "/bin/bash",
            Self::Fish => "/usr/bin/fish",
            Self::Nologin => "/usr/bin/nologin",
        }
    }
}

/// Sudoers privileges configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SudoRule {
    WheelWithPassword,
    WheelNoPassword,
    Disabled,
}

/// User account provisioning descriptor
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserAccount {
    pub username: String,
    pub uid: Option<u32>,
    pub shell: UserShell,
    pub groups: BTreeSet<String>,
    pub sudo_rule: SudoRule,
    pub create_home: bool,
    pub home_dir: Option<String>,
}

impl UserAccount {
    /// Create a standard regular desktop user for SigmaOS
    pub fn new_desktop_user(username: &str) -> Self {
        let mut groups = BTreeSet::new();
        // Arch standard interactive desktop groups
        for g in &["wheel", "audio", "video", "input", "storage", "optical", "network"] {
            groups.insert(g.to_string());
        }
        Self {
            username: username.to_string(),
            uid: None,
            shell: UserShell::Zsh,
            groups,
            sudo_rule: SudoRule::WheelWithPassword,
            create_home: true,
            home_dir: None,
        }
    }

    /// Add an extra secondary group
    pub fn add_group(&mut self, group: &str) {
        self.groups.insert(group.to_string());
    }

    /// Render useradd shell command
    pub fn to_useradd_cmd(&self) -> String {
        let mut parts = vec!["useradd".to_string()];

        if self.create_home {
            parts.push("-m".to_string());
        }

        if let Some(ref h) = self.home_dir {
            parts.push(format!("-d {}", h));
        }

        parts.push(format!("-s {}", self.shell.binary_path()));

        if let Some(uid) = self.uid {
            parts.push(format!("-u {}", uid));
        }

        if !self.groups.is_empty() {
            let grp_list: Vec<&str> = self.groups.iter().map(|s| s.as_str()).collect();
            parts.push(format!("-G {}", grp_list.join(",")));
        }

        parts.push(self.username.clone());
        parts.join(" ")
    }

    /// Render sudoers drop-in line for this user or group
    pub fn sudoers_dropin(&self) -> Option<String> {
        match self.sudo_rule {
            SudoRule::WheelWithPassword => Some("%wheel ALL=(ALL:ALL) ALL".to_string()),
            SudoRule::WheelNoPassword => Some("%wheel ALL=(ALL:ALL) NOPASSWD: ALL".to_string()),
            SudoRule::Disabled => None,
        }
    }

    /// Render PAM configuration line for session/limits
    pub fn pam_system_auth_snippet() -> String {
        "# SigmaOS PAM system-auth snippet\n\
        auth      sufficient pam_unix.so try_first_pass nullok\n\
        auth      required   pam_deny.so\n\
        account   required   pam_unix.so\n\
        password  sufficient pam_unix.so sha512 shadow try_first_pass use_authtok\n\
        password  required   pam_deny.so\n\
        session   required   pam_limits.so\n\
        session   required   pam_unix.so\n"
            .to_string()
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_useradd_cmd_generation_aaa() {
        // Arrange
        let user = UserAccount::new_desktop_user("sigma");
        // Act
        let cmd = user.to_useradd_cmd();
        // Assert
        assert!(cmd.starts_with("useradd -m -s /bin/zsh -G"));
        assert!(cmd.contains("wheel"));
        assert!(cmd.contains("video"));
        assert!(cmd.ends_with("sigma"));
    }

    #[test]
    fn test_sudoers_dropin_wheel_password_aaa() {
        // Arrange
        let user = UserAccount::new_desktop_user("sigma");
        // Act
        let sudo_rule = user.sudoers_dropin();
        // Assert
        assert_eq!(
            sudo_rule,
            Some("%wheel ALL=(ALL:ALL) ALL".to_string())
        );
    }

    #[test]
    fn test_sudoers_dropin_nopasswd_aaa() {
        // Arrange
        let mut user = UserAccount::new_desktop_user("sigma");
        user.sudo_rule = SudoRule::WheelNoPassword;
        // Act
        let sudo_rule = user.sudoers_dropin();
        // Assert
        assert_eq!(
            sudo_rule,
            Some("%wheel ALL=(ALL:ALL) NOPASSWD: ALL".to_string())
        );
    }

    #[test]
    fn test_pam_snippet_contents_aaa() {
        // Arrange & Act
        let pam = UserAccount::pam_system_auth_snippet();
        // Assert
        assert!(pam.contains("pam_unix.so"));
        assert!(pam.contains("sha512"));
        assert!(pam.contains("pam_limits.so"));
    }
}
