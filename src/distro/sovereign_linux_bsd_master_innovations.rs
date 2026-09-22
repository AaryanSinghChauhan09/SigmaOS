use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

/// Command Metadata Record parsed from binary header comments
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CommandMetadata {
    pub binary: String,
    pub group: String,
    pub name: String,
    pub summary: String,
    pub hidden: bool,
    pub requires_sudo: bool,
    pub args: String,
    pub examples: Vec<String>,
    pub canonical_route: String,
    pub filename_route: String,
}

/// Omarchy Spaced CLI Router & Developer Tmux Layout Engine
#[derive(Debug, Clone, Default)]
pub struct SovereignOmarchyCliRouterAndTmuxEngine {
    pub route_table: BTreeMap<String, CommandMetadata>,
    pub group_descriptions: BTreeMap<String, String>,
    pub collision_log: Vec<String>,
}

impl SovereignOmarchyCliRouterAndTmuxEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            route_table: BTreeMap::new(),
            group_descriptions: BTreeMap::new(),
            collision_log: Vec::new(),
        };

        engine.group_descriptions.insert("theme".to_string(), "Desktop & Application Styling".to_string());
        engine.group_descriptions.insert("network".to_string(), "Wi-Fi, DNS & Connectivity".to_string());
        engine.group_descriptions.insert("tailscale".to_string(), "Mesh VPN & File Transfer".to_string());
        engine.group_descriptions.insert("dns".to_string(), "DNS Provider Resolver".to_string());
        engine.group_descriptions.insert("dev".to_string(), "Developer Tooling & Benchmarks".to_string());
        engine.group_descriptions.insert("commands".to_string(), "Command Registry & Inspection".to_string());

        engine.register_binary(
            "omarchy-theme-set",
            "# omarchy:group=theme\n# omarchy:name=set\n# omarchy:summary=Set active system theme\n# omarchy:args=<theme_name>",
        );
        engine.register_binary(
            "omarchy-network-band",
            "# omarchy:group=network\n# omarchy:name=band\n# omarchy:summary=Pin Wi-Fi frequency band\n# omarchy:args=[2.4|5|6|auto]",
        );
        engine.register_binary(
            "omarchy-tailscale-send",
            "# omarchy:group=tailscale\n# omarchy:name=send\n# omarchy:summary=Send files over Taildrop\n# omarchy:args=<machine> [files...]",
        );
        engine.register_binary(
            "omarchy-apply-hardware",
            "# omarchy:group=apply\n# omarchy:name=hardware\n# omarchy:summary=Apply hardware quirk repairs\n# omarchy:hidden=true",
        );

        engine
    }

    /// Register binary and parse metadata comments
    pub fn register_binary(&mut self, binary_name: &str, header_comments: &str) -> String {
        let stem = binary_name.strip_prefix("omarchy-").unwrap_or(binary_name);
        let mut parts = stem.splitn(2, '-');
        let default_group = parts.next().unwrap_or(stem).to_string();
        let default_name = parts.next().unwrap_or("").replace('-', " ");

        let mut meta = CommandMetadata {
            binary: binary_name.to_string(),
            group: default_group,
            name: default_name,
            summary: String::new(),
            hidden: false,
            requires_sudo: false,
            args: String::new(),
            examples: Vec::new(),
            canonical_route: String::new(),
            filename_route: String::new(),
        };

        for line in header_comments.lines().take(80) {
            let line = line.trim();
            if !line.starts_with('#') {
                break;
            }
            let text = line.trim_start_matches('#').trim();
            if let Some(kv) = text.strip_prefix("omarchy:") {
                let mut kv_parts = kv.splitn(2, '=');
                let key = kv_parts.next().unwrap_or("").trim();
                let val = kv_parts.next().unwrap_or("").trim();

                match key {
                    "group" => meta.group = val.to_string(),
                    "name" => meta.name = val.to_string(),
                    "summary" => meta.summary = val.to_string(),
                    "hidden" => meta.hidden = val == "true",
                    "requires-sudo" => meta.requires_sudo = val == "true",
                    "args" => meta.args = val.to_string(),
                    "example" => meta.examples.push(val.to_string()),
                    _ => {}
                }
            }
        }

        let canonical_route = if meta.name.is_empty() {
            format!("omarchy {}", meta.group)
        } else {
            format!("omarchy {} {}", meta.group, meta.name)
        };
        let filename_route = format!("omarchy {}", stem.replace('-', " "));

        meta.canonical_route = canonical_route.clone();
        meta.filename_route = filename_route.clone();

        if let Some(existing) = self.route_table.get(&canonical_route) {
            if existing.binary != binary_name {
                self.collision_log.push(format!(
                    "Collision on route '{}': {} vs {}",
                    canonical_route, existing.binary, binary_name
                ));
            }
        } else {
            self.route_table.insert(canonical_route.clone(), meta.clone());
        }

        if filename_route != canonical_route && !self.route_table.contains_key(&filename_route) {
            self.route_table.insert(filename_route, meta);
        }

        canonical_route
    }

    /// Resolve spaced CLI route via longest-prefix matching
    pub fn resolve_route(&self, args: &[&str]) -> Result<(CommandMetadata, Vec<String>), String> {
        let full_cmd = format!("omarchy {}", args.join(" "));

        for len in (1..=args.len()).rev() {
            let prefix_route = format!("omarchy {}", args[..len].join(" "));
            if let Some(meta) = self.route_table.get(&prefix_route) {
                let leftovers: Vec<String> = args[len..].iter().map(|s| s.to_string()).collect();
                return Ok((meta.clone(), leftovers));
            }
        }

        Err(format!("Unknown command route: '{}'", full_cmd))
    }

    /// Run CLI metadata lint check (omarchy commands --check)
    pub fn check_metadata(&self) -> Result<usize, Vec<String>> {
        let mut errors = Vec::new();

        for (route, meta) in &self.route_table {
            if meta.summary.is_empty() {
                errors.push(format!("Missing explicit summary for binary '{}' at route '{}'", meta.binary, route));
            }
        }

        for col in &self.collision_log {
            errors.push(col.clone());
        }

        if errors.is_empty() {
            Ok(self.route_table.len())
        } else {
            Err(errors)
        }
    }

    /// Generate Developer Tmux IDE Layout script (tdl / tds / tdlm / tsl)
    pub fn generate_tmux_layout(&self, layout_type: &str, agent: &str, num_panes: usize) -> String {
        match layout_type {
            "tdl" => format!(
                "tmux new-session -d -s tdl '$EDITOR'\n\
                 tmux split-window -h -t tdl '{}'\n\
                 tmux split-window -v -t tdl.0\n\
                 tmux select-layout -t tdl main-vertical",
                agent
            ),
            "tds" => format!(
                "tmux new-session -d -s tds '$EDITOR'\n\
                 tmux split-window -h -t tds 'git diff --watch'\n\
                 tmux split-window -v -t tds.0\n\
                 tmux split-window -v -t tds.1 '{}'\n\
                 tmux select-layout -t tds tiled",
                agent
            ),
            "tsl" => format!(
                "tmux new-session -d -s tsl '{}'\n\
                 for i in $(seq 2 {}); do tmux split-window -t tsl '{}'; done\n\
                 tmux select-layout -t tsl tiled",
                agent, num_panes, agent
            ),
            _ => format!("tmux new-session -d -s dev '$SHELL'"),
        }
    }
}

/// Sovereign NetworkManager & Mesh VPN Engine
#[derive(Debug, Clone)]
pub struct SovereignNetworkManagerMeshEngine {
    pub active_interface: String,
    pub pinned_band: String,
    pub dns_provider: String,
    pub sshd_enabled: bool,
    pub tailscale_connected: bool,
    pub taildrop_downloads_dir: String,
    pub file_transfer_log: Vec<String>,
}

impl Default for SovereignNetworkManagerMeshEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SovereignNetworkManagerMeshEngine {
    pub fn new() -> Self {
        Self {
            active_interface: "wlan0".to_string(),
            pinned_band: "auto".to_string(),
            dns_provider: "DHCP".to_string(),
            sshd_enabled: false,
            tailscale_connected: false,
            taildrop_downloads_dir: "~/Downloads".to_string(),
            file_transfer_log: Vec::new(),
        }
    }

    pub fn generate_wifi_qr_code(&self, ssid: &str, pass: &str) -> String {
        format!("WIFI:S:{};T:WPA;P:{};;", ssid, pass)
    }

    pub fn pin_wifi_band(&mut self, band: &str) -> Result<String, String> {
        match band {
            "2.4" | "5" | "6" | "auto" => {
                self.pinned_band = band.to_string();
                let nm_band = match band {
                    "2.4" => "a",
                    "5" => "bg",
                    "6" => "6ghz",
                    _ => "auto",
                };
                Ok(format!("nmcli connection modify '{}' 802-11-wireless.band {}", self.active_interface, nm_band))
            }
            _ => Err("Invalid band. Choose 2.4, 5, 6, or auto".to_string()),
        }
    }

    pub fn set_dns_provider(&mut self, provider: &str) -> String {
        self.dns_provider = provider.to_string();
        let servers = match provider {
            "Cloudflare" => "1.1.1.1 1.0.0.1",
            "Google" => "8.8.8.8 8.8.4.4",
            "Quad9" => "9.9.9.9 149.112.112.112",
            _ => "DHCP",
        };
        format!("nmcli connection modify '{}' ipv4.dns '{}'", self.active_interface, servers)
    }

    pub fn toggle_sshd(&mut self, enable: bool) -> String {
        self.sshd_enabled = enable;
        if enable {
            "systemctl enable --now sshd && iptables -A INPUT -p tcp --dport 22 -m state --state NEW -m recent --set && iptables -A INPUT -p tcp --dport 22 -m state --state NEW -m recent --update --seconds 60 --hitcount 4 -j DROP".to_string()
        } else {
            "systemctl disable --now sshd".to_string()
        }
    }

    pub fn send_taildrop_file(&mut self, target_machine: &str, file_path: &str) -> String {
        let cmd = format!("tailscale file cp {} {}:", file_path, target_machine);
        self.file_transfer_log.push(cmd.clone());
        cmd
    }
}

/// Sovereign Idempotent One-Time Migration Runner Engine
#[derive(Debug, Clone)]
pub struct SovereignAtomicMigrationsEngine {
    pub migrations_dir: String,
    pub state_dir: String,
    pub applied_markers: Vec<String>,
    pub migration_log: Vec<String>,
}

impl Default for SovereignAtomicMigrationsEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SovereignAtomicMigrationsEngine {
    pub fn new() -> Self {
        Self {
            migrations_dir: "migrations".to_string(),
            state_dir: "~/.local/state/omarchy/migrations".to_string(),
            applied_markers: Vec::new(),
            migration_log: Vec::new(),
        }
    }

    pub fn is_applied(&self, migration_name: &str) -> bool {
        self.applied_markers.contains(&migration_name.to_string())
    }

    pub fn apply_migration(&mut self, migration_name: &str, script_content: &str) -> Result<String, String> {
        if self.is_applied(migration_name) {
            return Ok(format!("Migration {} already applied (no-op)", migration_name));
        }

        if script_content.contains("exit 1") || script_content.contains("error") {
            return Err(format!("Migration {} failed execution - remaining pending", migration_name));
        }

        self.applied_markers.push(migration_name.to_string());
        let log_entry = format!("Applied migration {} successfully", migration_name);
        self.migration_log.push(log_entry.clone());
        Ok(log_entry)
    }

    pub fn generate_notifier_service_unit(&self) -> String {
        "[Unit]\n\
         Description=Omarchy User Migration Notifier\n\
         After=graphical-session.target\n\n\
         [Service]\n\
         Type=oneshot\n\
         ExecStart=/usr/bin/omarchy-migrate --pending-notify\n\n\
         [Install]\n\
         WantedBy=graphical-session.target\n"
            .to_string()
    }

    pub fn get_pending_migrations(&self, available: &[&str]) -> Vec<String> {
        available
            .iter()
            .filter(|m| !self.is_applied(m))
            .map(|s| s.to_string())
            .collect()
    }
}

/// Sovereign Linux & BSD Master Innovations Suite
#[derive(Debug, Clone)]
pub struct SovereignLinuxBsdMasterInnovationsSuite {
    pub cli_router: SovereignOmarchyCliRouterAndTmuxEngine,
    pub network_mesh: SovereignNetworkManagerMeshEngine,
    pub migrations_engine: SovereignAtomicMigrationsEngine,
}

impl Default for SovereignLinuxBsdMasterInnovationsSuite {
    fn default() -> Self {
        Self::new()
    }
}

impl SovereignLinuxBsdMasterInnovationsSuite {
    pub fn new() -> Self {
        Self {
            cli_router: SovereignOmarchyCliRouterAndTmuxEngine::new(),
            network_mesh: SovereignNetworkManagerMeshEngine::new(),
            migrations_engine: SovereignAtomicMigrationsEngine::new(),
        }
    }

    pub fn run_master_distro_health_check(&mut self) -> Result<String, String> {
        let lint_count = self.cli_router.check_metadata().map_err(|e| e.join("; "))?;
        let nm_cmd = self.network_mesh.pin_wifi_band("5")?;
        let mig_res = self.migrations_engine.apply_migration("1781158082.sh", "echo 'Relink theme'; exit 0")?;

        Ok(format!(
            "Master Distro Health Check Passed: {} CLI routes verified, NM: '{}', {}",
            lint_count, nm_cmd, mig_res
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_router_route_resolution_and_lint() {
        let router = SovereignOmarchyCliRouterAndTmuxEngine::new();
        let (meta, leftovers) = router.resolve_route(&["theme", "set", "gruvbox"]).unwrap();
        assert_eq!(meta.binary, "omarchy-theme-set");
        assert_eq!(leftovers, vec!["gruvbox"]);

        assert!(router.check_metadata().is_ok());
    }

    #[test]
    fn test_tmux_layout_generation() {
        let router = SovereignOmarchyCliRouterAndTmuxEngine::new();
        let tdl = router.generate_tmux_layout("tdl", "opencode", 1);
        assert!(tdl.contains("tmux new-session -d -s tdl '$EDITOR'"));
        assert!(tdl.contains("opencode"));

        let tsl = router.generate_tmux_layout("tsl", "claude", 4);
        assert!(tsl.contains("seq 2 4"));
    }

    #[test]
    fn test_network_manager_band_pinning_and_qr() {
        let mut nm = SovereignNetworkManagerMeshEngine::new();
        let qr = nm.generate_wifi_qr_code("HomeNet", "Secret123");
        assert_eq!(qr, "WIFI:S:HomeNet;T:WPA;P:Secret123;;");

        let cmd = nm.pin_wifi_band("5").unwrap();
        assert!(cmd.contains("802-11-wireless.band bg"));

        let td_cmd = nm.send_taildrop_file("desktop-pc", "report.pdf");
        assert_eq!(td_cmd, "tailscale file cp report.pdf desktop-pc:");
    }

    #[test]
    fn test_idempotent_atomic_migrations() {
        let mut mig = SovereignAtomicMigrationsEngine::new();
        assert!(!mig.is_applied("001_theme.sh"));

        let res = mig.apply_migration("001_theme.sh", "echo 'Relink theme'").unwrap();
        assert!(res.contains("successfully"));
        assert!(mig.is_applied("001_theme.sh"));

        let res2 = mig.apply_migration("001_theme.sh", "echo 'Relink theme'").unwrap();
        assert!(res2.contains("no-op"));

        let unit = mig.generate_notifier_service_unit();
        assert!(unit.contains("WantedBy=graphical-session.target"));
    }

    #[test]
    fn test_master_innovations_suite() {
        let mut suite = SovereignLinuxBsdMasterInnovationsSuite::new();
        let health = suite.run_master_distro_health_check().unwrap();
        assert!(health.contains("Master Distro Health Check Passed"));
    }
}
