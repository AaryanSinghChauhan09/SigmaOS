// SigmaOS Sovereign Privilege Escalation Engine (Sigsudo / Sigdoas)
// Zero-dependency Rust #![no_std] / std implementation of privilege delegation & authentication.

#[cfg(not(test))]
#[cfg(not(test))]
use alloc::string::{String, ToString};
#[cfg(not(test))]
use alloc::vec::Vec;

#[cfg(test)]
#[cfg(test)]

/// Privilege Escalation Action Result
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SudoAuthResult {
    Authorized,
    AuthenticationRequired,
    PermissionDenied,
}

fn is_allowed_environment_key(key: &str) -> bool {
    matches!(
        key,
        "TERM"
            | "LANG"
            | "LC_ALL"
            | "LC_ADDRESS"
            | "LC_COLLATE"
            | "LC_CTYPE"
            | "LC_IDENTIFICATION"
            | "LC_MEASUREMENT"
            | "LC_MESSAGES"
            | "LC_MONETARY"
            | "LC_NAME"
            | "LC_NUMERIC"
            | "LC_PAPER"
            | "LC_TELEPHONE"
            | "LC_TIME"
    )
}

/// Sudoers & Doas Rule Entry
#[derive(Debug, Clone)]
pub struct SudoRule {
    pub entity: String,      // User ("jules") or Group ("%wheel" / ":wheel")
    pub target_user: String, // Target UID / user name ("root" / "ALL")
    pub nopasswd: bool,
    pub command_pattern: String, // Allowed command pattern ("*" or "/usr/bin/reboot")
    pub keepenv: bool,
}

/// Privilege Elevation Engine
#[derive(Debug, Clone)]
pub struct SovereignSudoEngine {
    pub rules: Vec<SudoRule>,
    pub ticket_cache_valid_secs: u64,
    pub enforce_wheel_group: bool,
}

impl SovereignSudoEngine {
    pub fn new() -> Self {
        let mut rules = Vec::new();
        // Default rule: Wheel group members can execute any command
        rules.push(SudoRule {
            entity: String::from("%wheel"),
            target_user: String::from("ALL"),
            nopasswd: false,
            command_pattern: String::from("*"),
            keepenv: true,
        });

        Self {
            rules,
            ticket_cache_valid_secs: 300,
            enforce_wheel_group: true,
        }
    }

    pub fn add_rule(&mut self, rule: SudoRule) {
        self.rules.push(rule);
    }

    /// Evaluates if a user is authorized to run a command as a target user
    pub fn authorize(
        &self,
        user: &str,
        groups: &[&str],
        target_user: &str,
        command: &str,
    ) -> SudoAuthResult {
        // Security Hardening: Reject inputs containing embedded NUL bytes (C-ABI truncation mitigation)
        // or ASCII control characters (CWE-150 / CWE-117 command & log injection mitigation).
        if user.as_bytes().contains(&0)
            || target_user.as_bytes().contains(&0)
            || command.as_bytes().contains(&0)
            || user.bytes().any(|b| b < 32 || b == 127)
            || target_user.bytes().any(|b| b < 32 || b == 127)
            || command.bytes().any(|b| b < 32 || b == 127)
        {
            return SudoAuthResult::PermissionDenied;
        }

        for rule in &self.rules {
            let entity_match = if rule.entity.starts_with('%') || rule.entity.starts_with(':') {
                let group_name = &rule.entity[1..];
                groups.iter().any(|&g| g == group_name)
            } else {
                rule.entity == "ALL" || rule.entity == user
            };

            let target_match = rule.target_user == "ALL" || rule.target_user == target_user;
            let cmd_match = rule.command_pattern == "*" || rule.command_pattern == command;

            if entity_match && target_match && cmd_match {
                if rule.nopasswd {
                    return SudoAuthResult::Authorized;
                } else {
                    return SudoAuthResult::AuthenticationRequired;
                }
            }
        }

        SudoAuthResult::PermissionDenied
    }

    /// Keeps only explicitly permitted environment variable names for elevated execution.
    /// This does not validate associated values; the executor must set trusted PATH and HOME values.
    pub fn sanitize_environment(&self, env_keys: &[&str]) -> Vec<String> {
        env_keys
            .iter()
            .filter(|&&key| is_allowed_environment_key(key))
            .map(|&k| String::from(k))
            .collect()
    }
}

impl Default for SovereignSudoEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sudo_engine_authorization() {
        let mut engine = SovereignSudoEngine::new();

        // Test wheel group member (requires auth)
        let res = engine.authorize("jules", &["wheel", "users"], "root", "/usr/bin/systemctl");
        assert_eq!(res, SudoAuthResult::AuthenticationRequired);

        // Add nopasswd rule for specific command
        engine.add_rule(SudoRule {
            entity: String::from("jules"),
            target_user: String::from("root"),
            nopasswd: true,
            command_pattern: String::from("/usr/bin/reboot"),
            keepenv: false,
        });

        let res_reboot = engine.authorize("jules", &["users"], "root", "/usr/bin/reboot");
        assert_eq!(res_reboot, SudoAuthResult::Authorized);

        // Test unauthorized command
        let res_unauth = engine.authorize("guest", &["guests"], "root", "/usr/bin/reboot");
        assert_eq!(res_unauth, SudoAuthResult::PermissionDenied);

        // Test environment sanitization
        let clean_env = engine.sanitize_environment(&[
            "TERM",
            "LANG",
            "LC_CTYPE",
            "LD_PRELOAD",
            "HOME",
            "GCONV_PATH",
            "PERL5LIB",
            "IFS",
            "NODE_OPTIONS",
            "BASH_ENV",
            "LC_CUSTOM",
            "LC_BAD-NAME",
        ]);
        assert!(clean_env.contains(&String::from("TERM")));
        assert!(clean_env.contains(&String::from("LANG")));
        assert!(clean_env.contains(&String::from("LC_CTYPE")));
        assert!(!clean_env.contains(&String::from("PATH")));
        assert!(!clean_env.contains(&String::from("HOME")));
        assert!(!clean_env.contains(&String::from("LD_PRELOAD")));
        assert!(!clean_env.contains(&String::from("GCONV_PATH")));
        assert!(!clean_env.contains(&String::from("PERL5LIB")));
        assert!(!clean_env.contains(&String::from("IFS")));
        assert!(!clean_env.contains(&String::from("NODE_OPTIONS")));
        assert!(!clean_env.contains(&String::from("BASH_ENV")));
        assert!(!clean_env.contains(&String::from("LC_CUSTOM")));
        assert!(!clean_env.contains(&String::from("LC_BAD-NAME")));
    }

    #[test]
    fn test_sudo_engine_null_byte_and_control_char_rejection() {
        let engine = SovereignSudoEngine::new();

        // NUL byte injection in user, target_user, or command must be denied
        assert_eq!(
            engine.authorize("jules\0evil", &["wheel"], "root", "/usr/bin/reboot"),
            SudoAuthResult::PermissionDenied
        );
        assert_eq!(
            engine.authorize("jules", &["wheel"], "root\0admin", "/usr/bin/reboot"),
            SudoAuthResult::PermissionDenied
        );
        assert_eq!(
            engine.authorize("jules", &["wheel"], "root", "/usr/bin/reboot\0--force"),
            SudoAuthResult::PermissionDenied
        );

        // Control character injection in user or command must be denied
        assert_eq!(
            engine.authorize("jules\nadmin", &["wheel"], "root", "/usr/bin/reboot"),
            SudoAuthResult::PermissionDenied
        );
        assert_eq!(
            engine.authorize("jules", &["wheel"], "root", "/usr/bin/reboot\r\n"),
            SudoAuthResult::PermissionDenied
        );
    }
}
