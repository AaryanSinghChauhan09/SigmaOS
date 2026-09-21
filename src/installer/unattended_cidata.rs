//! Omarchy Unattended Installation Engine (cidata / cloud-init NoCloud drive support)
//! Parses volume labeled `cidata` carrying installer configuration files to perform
//! hands-free automated installation into disposable dev environments and VM instances.

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;
use std::format;

/// Cloud-init / cidata unattended configuration specs
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CiDataUnattendedConfig {
    pub volume_label: String,
    pub hostname: String,
    pub target_disk: String,
    pub timezone: String,
    pub keyboard_layout: String,
    pub username: Option<String>,
    pub password_hash: Option<String>,
    pub git_full_name: Option<String>,
    pub git_email: Option<String>,
    pub disk_encryption_requested: bool,
    pub ssh_authorized_keys: Vec<String>,
    pub tailscale_authkey: Option<String>,
    pub defer_provisioning: bool,
    pub sshd_enabled: bool,
    pub firewall_ssh_open: bool,
    pub tailscale_auto_join: bool,
}

/// Unattended cidata installation engine
pub struct CiDataUnattendedEngine {
    pub detected_drive_label: Option<String>,
    pub active_config: Option<CiDataUnattendedConfig>,
    pub status_logs: Vec<String>,
}

impl CiDataUnattendedEngine {
    pub fn new() -> Self {
        Self {
            detected_drive_label: None,
            active_config: None,
            status_logs: Vec::new(),
        }
    }

    /// Detect if a cidata / NoCloud volume is present in detected drives/media
    pub fn detect_cidata_drive(&mut self, drive_labels: &[&str]) -> bool {
        for label in drive_labels {
            if label.eq_ignore_ascii_case("cidata") || label.eq_ignore_ascii_case("nocloud") {
                self.detected_drive_label = Some((*label).to_string());
                self.status_logs.push(format!("Detected unattended cidata drive volume: {}", label));
                return true;
            }
        }
        false
    }

    /// Parse cidata configuration files from a file tree map
    pub fn parse_cidata_files(
        &mut self,
        files: &BTreeMap<String, String>,
    ) -> Result<CiDataUnattendedConfig, String> {
        let mut config = CiDataUnattendedConfig::default();
        config.volume_label = self.detected_drive_label.clone().unwrap_or_else(|| "cidata".to_string());

        // Check for defer-provisioning flag
        if files.contains_key("defer-provisioning") || files.contains_key("defer_provisioning") {
            config.defer_provisioning = true;
            self.status_logs.push("Detected 'defer-provisioning' flag - deferring user account creation to first boot wizard.".to_string());
        }

        // Parse user_configuration.json if present
        if let Some(user_config) = files.get("user_configuration.json") {
            self.parse_json_user_config(user_config, &mut config)?;
        } else if !config.defer_provisioning {
            return Err("Missing required file 'user_configuration.json' on cidata volume".to_string());
        }

        // Parse user_credentials.json if present
        if !config.defer_provisioning {
            if let Some(creds) = files.get("user_credentials.json") {
                self.parse_json_user_credentials(creds, &mut config)?;
            } else {
                return Err("Missing required file 'user_credentials.json' on cidata volume (or missing defer-provisioning file)".to_string());
            }
        }

        // Parse optional user_full_name.txt
        if let Some(full_name) = files.get("user_full_name.txt") {
            config.git_full_name = Some(full_name.trim().to_string());
        }

        // Parse optional user_email_address.txt
        if let Some(email) = files.get("user_email_address.txt") {
            config.git_email = Some(email.trim().to_string());
        }

        // Parse optional user_encrypt_installation.txt
        if let Some(encrypt) = files.get("user_encrypt_installation.txt") {
            if encrypt.trim().eq_ignore_ascii_case("true") || encrypt.trim() == "1" {
                config.disk_encryption_requested = true;
            }
        }

        // Parse optional authorized_keys
        if let Some(keys) = files.get("authorized_keys") {
            let key_lines: Vec<String> = keys
                .lines()
                .map(|l| l.trim().to_string())
                .filter(|l| !l.is_empty() && !l.starts_with('#'))
                .collect();
            if !key_lines.is_empty() {
                config.ssh_authorized_keys = key_lines;
                config.sshd_enabled = true;
                config.firewall_ssh_open = true;
                self.status_logs.push("SSH authorized_keys configured: enabling sshd service and opening port 22 in firewall.".to_string());
            }
        }

        // Parse optional tailscale_authkey
        if let Some(ts_key) = files.get("tailscale_authkey") {
            let key = ts_key.trim().to_string();
            if !key.is_empty() {
                config.tailscale_authkey = Some(key);
                config.tailscale_auto_join = true;
                self.status_logs.push("Tailscale auth key detected: queueing background tailnet auto-join service on first boot.".to_string());
            }
        }

        self.active_config = Some(config.clone());
        Ok(config)
    }

    /// Process installation execution plan based on cidata config
    pub fn execute_unattended_provisioning(
        &mut self,
        config: &CiDataUnattendedConfig,
    ) -> Result<String, String> {
        let mut steps: Vec<String> = Vec::new();

        if config.defer_provisioning {
            steps.push("Configuring system image in deferred provisioning mode (no user credentials stored).".to_string());
        } else if let Some(ref username) = config.username {
            steps.push(format!("Provisioning user account '{}' with encrypted password hash.", username));
        }

        if config.sshd_enabled {
            steps.push(format!(
                "Injecting {} SSH authorized key(s) into ~/.ssh/authorized_keys, enabling sshd, and opening firewall port 22.",
                config.ssh_authorized_keys.len()
            ));
        }

        if config.tailscale_auto_join {
            steps.push("Enabling Tailscale service and background daemon for automated tailnet registration.".to_string());
        }

        if config.disk_encryption_requested {
            steps.push("Enabling LUKS disk encryption (passphrase prompt required on initial reboot).".to_string());
        }

        let summary = format!(
            "Unattended Omarchy cidata installation completed successfully for hostname '{}' across {} steps.",
            config.hostname,
            steps.len()
        );

        self.status_logs.push(summary.clone());
        Ok(summary)
    }

    fn parse_json_user_config(
        &self,
        json: &str,
        config: &mut CiDataUnattendedConfig,
    ) -> Result<(), String> {
        // Simple string key parser for JSON compatibility without heavy dependencies
        if let Some(host) = extract_json_value(json, "hostname") {
            config.hostname = host;
        } else {
            config.hostname = "omarchy-unattended".to_string();
        }

        if let Some(disk) = extract_json_value(json, "target_disk") {
            config.target_disk = disk;
        } else if let Some(disk) = extract_json_value(json, "disk") {
            config.target_disk = disk;
        } else {
            config.target_disk = "/dev/sda".to_string();
        }

        if let Some(tz) = extract_json_value(json, "timezone") {
            config.timezone = tz;
        } else {
            config.timezone = "UTC".to_string();
        }

        if let Some(kb) = extract_json_value(json, "keyboard") {
            config.keyboard_layout = kb;
        } else {
            config.keyboard_layout = "us".to_string();
        }

        Ok(())
    }

    fn parse_json_user_credentials(
        &self,
        json: &str,
        config: &mut CiDataUnattendedConfig,
    ) -> Result<(), String> {
        if let Some(user) = extract_json_value(json, "username") {
            config.username = Some(user);
        }
        if let Some(pass_hash) = extract_json_value(json, "password_hash") {
            config.password_hash = Some(pass_hash);
        } else if let Some(pass) = extract_json_value(json, "password") {
            config.password_hash = Some(pass);
        }
        Ok(())
    }
}

impl Default for CiDataUnattendedEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cidata_drive_detection() {
        let mut engine = CiDataUnattendedEngine::new();
        let labels = vec!["sda1", "CIDATA", "iso9660"];
        assert!(engine.detect_cidata_drive(&labels));
        assert_eq!(engine.detected_drive_label.as_deref(), Some("CIDATA"));
    }

    #[test]
    fn test_cidata_parsing_full_configuration() {
        let mut engine = CiDataUnattendedEngine::new();
        engine.detect_cidata_drive(&["cidata"]);

        let mut files = BTreeMap::new();
        files.insert(
            "user_configuration.json".to_string(),
            r#"{"hostname": "omarchy-vm", "target_disk": "/dev/vda", "timezone": "America/New_York", "keyboard": "us"}"#.to_string(),
        );
        files.insert(
            "user_credentials.json".to_string(),
            r#"{"username": "admin", "password_hash": "$6$rounds=50000$saltsalt$hashhash"}"#.to_string(),
        );
        files.insert("user_full_name.txt".to_string(), "Omarchy Admin\n".to_string());
        files.insert("user_email_address.txt".to_string(), "admin@omarchy.org\n".to_string());
        files.insert("user_encrypt_installation.txt".to_string(), "true\n".to_string());
        files.insert(
            "authorized_keys".to_string(),
            "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAI... admin@dev\n".to_string(),
        );
        files.insert("tailscale_authkey".to_string(), "tskey-auth-k123456789\n".to_string());

        let cfg = engine.parse_cidata_files(&files).unwrap();
        assert_eq!(cfg.hostname, "omarchy-vm");
        assert_eq!(cfg.target_disk, "/dev/vda");
        assert_eq!(cfg.username.as_deref(), Some("admin"));
        assert_eq!(cfg.git_full_name.as_deref(), Some("Omarchy Admin"));
        assert_eq!(cfg.git_email.as_deref(), Some("admin@omarchy.org"));
        assert!(cfg.disk_encryption_requested);
        assert_eq!(cfg.ssh_authorized_keys.len(), 1);
        assert!(cfg.sshd_enabled);
        assert!(cfg.firewall_ssh_open);
        assert_eq!(cfg.tailscale_authkey.as_deref(), Some("tskey-auth-k123456789"));
        assert!(cfg.tailscale_auto_join);
        assert!(!cfg.defer_provisioning);

        let res = engine.execute_unattended_provisioning(&cfg);
        assert!(res.is_ok());
    }

    #[test]
    fn test_cidata_deferred_provisioning() {
        let mut engine = CiDataUnattendedEngine::new();
        engine.detect_cidata_drive(&["nocloud"]);

        let mut files = BTreeMap::new();
        files.insert("defer-provisioning".to_string(), "".to_string());
        files.insert(
            "user_configuration.json".to_string(),
            r#"{"hostname": "omarchy-imaging-rig"}"#.to_string(),
        );

        let cfg = engine.parse_cidata_files(&files).unwrap();
        assert!(cfg.defer_provisioning);
        assert_eq!(cfg.username, None);

        let res = engine.execute_unattended_provisioning(&cfg);
        assert!(res.is_ok());
    }
}

/// Helper function to extract string value for key from simple JSON object
fn extract_json_value(json: &str, key: &str) -> Option<String> {
    let pattern = format!("\"{}\"", key);
    if let Some(pos) = json.find(&pattern) {
        let remainder = &json[pos + pattern.len()..];
        if let Some(colon_pos) = remainder.find(':') {
            let val_part = remainder[colon_pos + 1..].trim();
            if val_part.starts_with('"') {
                let inner = &val_part[1..];
                if let Some(end_quote) = inner.find('"') {
                    return Some(inner[..end_quote].to_string());
                }
            }
        }
    }
    None
}
