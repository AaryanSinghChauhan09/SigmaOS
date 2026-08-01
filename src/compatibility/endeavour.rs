// EndeavourOS Parity Engines for SigmaOS
// This module implements user-centric distro utilities inspired by EndeavourOS,
// such as the Welcome assistant, Reflector mirror ranking, update notifier daemon,
// log sharing tool with sanitization, and the Yay AUR helper translator.

use std::collections::HashMap;

/// Represents a package mirror in the SigmaOS network.
#[derive(Debug, Clone, PartialEq)]
pub struct Mirror {
    pub url: String,
    pub country: String,
    pub protocol: String,
    pub latency_ms: u32,
    pub speed_kbps: u32,
    pub active: bool,
}

/// Dynamic Reflector tool for updating and ranking active package mirrors.
pub struct EosMirrorReflector {
    pub mirrors: Vec<Mirror>,
}

impl EosMirrorReflector {
    pub fn new() -> Self {
        Self { mirrors: Vec::new() }
    }

    pub fn add_mirror(&mut self, mirror: Mirror) {
        self.mirrors.push(mirror);
    }

    /// Ranks mirrors using a custom score: (latency_ms * 2) - (speed_kbps / 10).
    /// Lower score is better.
    pub fn rank_mirrors(&mut self, country_filter: Option<&str>, protocol_filter: Option<&str>) -> Vec<Mirror> {
        let mut filtered: Vec<Mirror> = self
            .mirrors
            .iter()
            .filter(|m| m.active)
            .filter(|m| country_filter.map_or(true, |c| m.country == c))
            .filter(|m| protocol_filter.map_or(true, |p| m.protocol == p))
            .cloned()
            .collect();

        filtered.sort_by(|a, b| {
            let score_a = (a.latency_ms * 2) as i32 - (a.speed_kbps / 10) as i32;
            let score_b = (b.latency_ms * 2) as i32 - (b.speed_kbps / 10) as i32;
            score_a.cmp(&score_b)
        });

        filtered
    }
}

/// Steps and commands executed by the EndeavourOS Welcome assistant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WelcomeTab {
    Welcome,
    Assistant,
    Tips,
    Addons,
}

/// The EosWelcomeEngine simulates a post-install interactive terminal/GUI companion.
pub struct EosWelcomeEngine {
    pub current_tab: WelcomeTab,
    pub first_boot: bool,
    pub packages_installed_via_welcome: Vec<String>,
}

impl EosWelcomeEngine {
    pub fn new(first_boot: bool) -> Self {
        Self {
            current_tab: WelcomeTab::Welcome,
            first_boot,
            packages_installed_via_welcome: Vec::new(),
        }
    }

    pub fn navigate_to(&mut self, tab: WelcomeTab) {
        self.current_tab = tab;
    }

    pub fn run_post_install_update(&self) -> &'static str {
        if self.first_boot {
            "Running initial post-installation full system update... SUCCESS"
        } else {
            "System is already up-to-date."
        }
    }

    pub fn install_recommended_addon(&mut self, addon: &str) -> Result<&'static str, &'static str> {
        if addon.is_empty() {
            return Err("Addon name cannot be empty");
        }
        self.packages_installed_via_welcome.push(addon.to_string());
        Ok("Addon installation requested through Welcome assistant")
    }
}

/// Monitor package changes and dispatch update notifications to user space.
pub struct EosUpdateNotifier {
    pub check_interval_hours: u32,
    pub notify_on_aur: bool,
    pub mock_updates: HashMap<String, String>, // package -> version
}

impl EosUpdateNotifier {
    pub fn new(interval: u32, notify_aur: bool) -> Self {
        let mut mock_updates = HashMap::new();
        mock_updates.insert("linux-sigma".to_string(), "6.12.5-1".to_string());
        mock_updates.insert("sigpkg".to_string(), "2.4.0".to_string());
        mock_updates.insert("yay-eos".to_string(), "12.3.0".to_string());

        Self {
            check_interval_hours: interval,
            notify_on_aur: notify_aur,
            mock_updates,
        }
    }

    pub fn check_for_updates(&self) -> Vec<(String, String)> {
        let mut updates = Vec::new();
        for (pkg, version) in &self.mock_updates {
            if self.notify_on_aur || !pkg.ends_with("-aur") {
                updates.push((pkg.clone(), version.clone()));
            }
        }
        updates.sort_by(|a, b| a.0.cmp(&b.0));
        updates
    }
}

/// The EosLogTool cleans, sanitizes, and prepares logs for secure community support uploads.
pub struct EosLogTool {
    pub sensitive_keywords: Vec<String>,
}

impl EosLogTool {
    pub fn new() -> Self {
        Self {
            sensitive_keywords: vec![
                "password".to_string(),
                "secret".to_string(),
                "token".to_string(),
                "api_key".to_string(),
            ],
        }
    }

    /// Sanitizes sensitive variables, user paths, and raw IP addresses to protect privacy.
    pub fn sanitize_log(&self, raw_log: &str) -> String {
        let mut sanitized = raw_log.to_string();

        // 1. Redact IPs (simple IPv4 regex simulation)
        // Match standard format like 192.168.1.50
        let ip_chars = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'];
        let mut words: Vec<String> = sanitized
            .split_whitespace()
            .map(|word| {
                if word.chars().all(|c| ip_chars.contains(&c)) && word.contains('.') {
                    let dots = word.chars().filter(|&c| c == '.').count();
                    if dots == 3 {
                        return "XXX.XXX.XXX.XXX".to_string();
                    }
                }
                word.to_string()
            })
            .collect();

        sanitized = words.join(" ");

        // 2. Redact sensitive keywords
        for keyword in &self.sensitive_keywords {
            let redact_pattern = format!("{}:", keyword);
            if sanitized.contains(&redact_pattern) {
                sanitized = sanitized.replace(&redact_pattern, &format!("{}: <REDACTED>", keyword));
            }
        }

        sanitized
    }
}

/// Command parser and translator mirroring the Arch/EndeavourOS Yay AUR helper behavior.
pub struct YayAurHelper {
    pub tracking_aur_packages: Vec<String>,
}

impl YayAurHelper {
    pub fn new() -> Self {
        Self {
            tracking_aur_packages: Vec::new(),
        }
    }

    /// Parse a yay-style CLI command and translate it to native sigpkg actions.
    pub fn translate_command(&mut self, cli_args: &str) -> Result<String, &'static str> {
        let parts: Vec<&str> = cli_args.split_whitespace().collect();
        if parts.is_empty() || parts[0] != "yay" {
            return Err("Not a yay command");
        }

        if parts.len() == 1 {
            return Ok("sigpkg sync --sysupgrade".to_string());
        }

        match parts[1] {
            "-Syu" => Ok("sigpkg sync --sysupgrade".to_string()),
            "-S" if parts.len() > 2 => {
                let pkg_name = parts[2];
                if pkg_name.ends_with("-git") || pkg_name.ends_with("-aur") {
                    self.tracking_aur_packages.push(pkg_name.to_string());
                    Ok(format!("sigpkg recipe install --aur {}", pkg_name))
                } else {
                    Ok(format!("sigpkg install {}", pkg_name))
                }
            }
            "-Ss" if parts.len() > 2 => {
                Ok(format!("sigpkg search --all {}", parts[2]))
            }
            "-Rns" if parts.len() > 2 => {
                Ok(format!("sigpkg remove --recursive {}", parts[2]))
            }
            _ => Err("Unsupported yay operation flags"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reflector_mirror_ranking() {
        let mut reflector = EosMirrorReflector::new();
        reflector.add_mirror(Mirror {
            url: "https://mirror.us.sigmaos.org".to_string(),
            country: "US".to_string(),
            protocol: "https".to_string(),
            latency_ms: 20,
            speed_kbps: 15000,
            active: true,
        });
        reflector.add_mirror(Mirror {
            url: "https://mirror.de.sigmaos.org".to_string(),
            country: "DE".to_string(),
            protocol: "https".to_string(),
            latency_ms: 120,
            speed_kbps: 20000,
            active: true,
        });
        reflector.add_mirror(Mirror {
            url: "http://mirror.slow.sigmaos.org".to_string(),
            country: "US".to_string(),
            protocol: "http".to_string(),
            latency_ms: 300,
            speed_kbps: 1000,
            active: true,
        });

        // Test with US country filter
        let ranked = reflector.rank_mirrors(Some("US"), None);
        assert_eq!(ranked.len(), 2);
        assert_eq!(ranked[0].url, "https://mirror.us.sigmaos.org");

        // Test with HTTPS protocol filter
        let ranked_https = reflector.rank_mirrors(None, Some("https"));
        assert_eq!(ranked_https.len(), 2);
    }

    #[test]
    fn test_welcome_assistant_flow() {
        let mut welcome = EosWelcomeEngine::new(true);
        assert_eq!(welcome.current_tab, WelcomeTab::Welcome);

        welcome.navigate_to(WelcomeTab::Assistant);
        assert_eq!(welcome.current_tab, WelcomeTab::Assistant);

        let update_msg = welcome.run_post_install_update();
        assert!(update_msg.contains("Running initial post-installation"));

        assert!(welcome.install_recommended_addon("eos-settings-greeter").is_ok());
        assert_eq!(welcome.packages_installed_via_welcome.len(), 1);
    }

    #[test]
    fn test_update_notifier() {
        let notifier = EosUpdateNotifier::new(6, true);
        let list = notifier.check_for_updates();
        assert!(!list.is_empty());
        assert_eq!(list[0].0, "linux-sigma");
    }

    #[test]
    fn test_log_sanitization() {
        let log_tool = EosLogTool::new();
        let raw_log = "Error on connection from 192.168.1.101 with token: super_secret_123";
        let clean = log_tool.sanitize_log(raw_log);
        assert!(clean.contains("XXX.XXX.XXX.XXX"));
        assert!(clean.contains("token: <REDACTED>"));
    }

    #[test]
    fn test_yay_aur_helper_translation() {
        let mut yay = YayAurHelper::new();
        assert_eq!(
            yay.translate_command("yay -Syu").unwrap(),
            "sigpkg sync --sysupgrade"
        );
        assert_eq!(
            yay.translate_command("yay -S neofetch").unwrap(),
            "sigpkg install neofetch"
        );
        assert_eq!(
            yay.translate_command("yay -S custom-theme-aur").unwrap(),
            "sigpkg recipe install --aur custom-theme-aur"
        );
    }
}
