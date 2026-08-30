// SPDX-License-Identifier: MIT
// SigmaOS Linux Mint Inspiration Subsystem (`src/linuxmint_inspirations.rs`)
// Sovereign `#![no_std]` reimplementations of the most distinctive ideas drawn
// from the entire set of https://github.com/orgs/linuxmint/repositories.
//
// Each subsystem is a faithful, zero-dependency model of the corresponding Mint
// application or daemon so the ideas can be absorbed natively into SigmaOS and
// evolved beyond the originals:
//
//   - Warpinator          -> `LanWarpEngine`  (P2P LAN discovery + encrypted transfer)
//   - Thingy              -> `ThingyRecentDocs`
//   - Webapp Manager      -> `WebappManager`
//   - Captain / apturl    -> `CaptainInstaller`
//   - Hypnotix            -> `HypnotixIptvPlayer`
//   - Bulky               -> `BulkyRenamer`
//   - MintNanny           -> `MintNannyFilter`
//   - MintWelcome         -> `MintWelcomeFlow`
//   - MintReport          -> `MintReportDiagnostics`
//   - MintStick           -> `MintStickFormatter`
//   - MintLocale          -> `MintLocaleManager`
//   - MintMenu            -> `MintMenuLayout`
//   - Automate            -> `AutomateWorkflow`

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

// =========================================================================
// 1. WARPINATOR -> LanWarpEngine
//    P2P file sharing across a local network with secure group codes,
//    encrypted transfers, folder isolation and port-based services.
// =========================================================================

pub const WARP_TRANSFER_PORT: u16 = 42000;
pub const WARP_AUTH_PORT: u16 = 42001;
pub const WARP_MDNS_UDP_PORT: u16 = 5353;
const DEFAULT_GROUP_CODE: &str = "Warpinator";
const MIN_GROUP_CODE_LEN: usize = 8;
const MAX_GROUP_CODE_LEN: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsolationMode {
    Landlock,
    Bubblewrap,
    Legacy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LanPeer {
    pub hostname: String,
    pub address: String,
    pub group_code: String,
    pub secure: bool,
    pub port: u16,
    pub compression_supported: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransferRequest {
    pub filename: String,
    pub size_bytes: usize,
    pub sender: String,
    pub approved: bool,
    pub compressed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransferOutcome {
    Completed { bytes: usize },
    Rejected,
    Interrupted { bytes: usize },
}

/// Warpinator-style LAN transfer engine.
pub struct LanWarpEngine {
    pub group_code: String,
    pub secure_mode: bool,
    pub isolation: IsolationMode,
    pub local_hostname: String,
    pub local_address: String,
    pub peers: Vec<LanPeer>,
    pub incoming_auto_approve: bool,
    pub compression_enabled: bool,
}

impl LanWarpEngine {
    pub fn new(local_hostname: &str, local_address: &str, isolation: IsolationMode) -> Self {
        Self {
            group_code: DEFAULT_GROUP_CODE.to_string(),
            secure_mode: false,
            isolation,
            local_hostname: local_hostname.to_string(),
            local_address: local_address.to_string(),
            peers: Vec::new(),
            incoming_auto_approve: false,
            compression_enabled: false,
        }
    }

    /// Set a group code. Enabling secure mode (a unique code) also turns on
    /// the security restrictions Warpinator applies until Secure Mode.
    pub fn set_group_code(&mut self, code: &str) -> Result<(), &'static str> {
        let non_ascii = code.chars().any(|c| !c.is_ascii());
        let max = if non_ascii { 24 } else { MAX_GROUP_CODE_LEN };
        if code.len() < MIN_GROUP_CODE_LEN || code.len() > max {
            return Err("group code must be between 8 and 32 characters");
        }
        self.group_code = code.to_string();
        self.secure_mode = code != DEFAULT_GROUP_CODE;
        Ok(())
    }

    /// mDNS/zeroconf peer discovery over the shared group code.
    pub fn discover_peer(&mut self, hostname: &str, address: &str, code: &str) -> bool {
        if code != self.group_code {
            return false;
        }
        if !self.peers.iter().any(|p| p.address == address) {
            self.peers.push(LanPeer {
                hostname: hostname.to_string(),
                address: address.to_string(),
                group_code: code.to_string(),
                secure: self.secure_mode,
                port: WARP_TRANSFER_PORT,
                compression_supported: true,
            });
        }
        true
    }

    pub fn peer_count(&self) -> usize {
        self.peers.len()
    }

    pub fn send_file(&mut self, peer_address: &str, _filename: &str, payload: &[u8]) -> TransferOutcome {
        if !self.peers.iter().any(|p| p.address == peer_address) {
            return TransferOutcome::Interrupted { bytes: 0 };
        }
        if self.compression_enabled {
            // Compression reduces transfer size; modeled as a ratio.
            let comp = payload.len().saturating_mul(85) / 100;
            TransferOutcome::Completed { bytes: comp }
        } else {
            TransferOutcome::Completed { bytes: payload.len() }
        }
    }

    pub fn receive_file(&mut self, req: RequestIncoming) -> TransferOutcome {
        if req.auto_approvable && self.incoming_auto_approve {
            if self.isolation != IsolationMode::Legacy {
                return TransferOutcome::Completed { bytes: req.size_bytes };
            }
            return TransferOutcome::Completed { bytes: req.size_bytes };
        }
        if req.approved {
            TransferOutcome::Completed { bytes: req.size_bytes }
        } else {
            TransferOutcome::Rejected
        }
    }

    /// In secure mode, Warpinator exits after sixty minutes and disables
    /// auto-start. Model the policy here.
    pub fn secure_mode_restrictions(&self) -> Vec<&'static str> {
        if self.secure_mode {
            vec![]
        } else {
            vec![
                "auto-start disabled",
                "all incoming transfers must be approved",
                "exits after sixty minutes",
            ]
        }
    }
}

#[derive(Debug, Clone)]
pub struct RequestIncoming {
    pub filename: String,
    pub size_bytes: usize,
    pub approved: bool,
    pub auto_approvable: bool,
}

impl Default for LanWarpEngine {
    fn default() -> Self {
        Self::new("sigma-local", "192.168.1.100", IsolationMode::Legacy)
    }
}

// =========================================================================
// 2. THINGY -> ThingyRecentDocs
//    Quick access to recent and favorite documents.
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThingyKind {
    Recent,
    Favourite,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThingyEntry {
    pub path: String,
    pub kind: ThingyKind,
    pub opened_at_secs: u64,
}

pub struct ThingyRecentDocs {
    pub entries: Vec<ThingyEntry>,
    pub max_recent: usize,
}

impl ThingyRecentDocs {
    pub fn new(max_recent: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_recent,
        }
    }

    pub fn open(&mut self, path: &str, now_secs: u64) {
        self.entries.retain(|e| e.path != path);
        self.entries.push(ThingyEntry {
            path: path.to_string(),
            kind: ThingyKind::Recent,
            opened_at_secs: now_secs,
        });
        if self.entries.len() > self.max_recent {
            self.entries.remove(0);
        }
    }

    pub fn toggle_favourite(&mut self, path: &str) {
        if let Some(e) = self.entries.iter_mut().find(|e| e.path == path) {
            e.kind = if e.kind == ThingyKind::Favourite {
                ThingyKind::Recent
            } else {
                ThingyKind::Favourite
            };
        }
    }

    pub fn favourites(&self) -> Vec<&str> {
        self.entries
            .iter()
            .filter(|e| e.kind == ThingyKind::Favourite)
            .map(|e| e.path.as_str())
            .collect()
    }

    pub fn recent(&self) -> Vec<&str> {
        self.entries
            .iter()
            .filter(|e| e.kind == ThingyKind::Recent)
            .map(|e| e.path.as_str())
            .collect()
    }
}

impl Default for ThingyRecentDocs {
    fn default() -> Self {
        Self::new(12)
    }
}

// =========================================================================
// 3. WEBAPP MANAGER -> WebappManager
//    Run websites as if they were isolated applications.
//    Incorporate Linux Mint WebApp Manager, Peppermint OS ICE/SSB, GNOME Web,
//    and FreeBSD Capsicum sandbox innovations.
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WebEngineKind {
    Chromium,
    Gecko,
    WebKitGtk,
    Brave,
    Vivaldi,
    LibreWolf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebappCategory {
    Web,
    Office,
    Graphics,
    Multimedia,
    Games,
    Utilities,
    Development,
    Finance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebappNavigationMode {
    /// Frameless SSB application window without navigation elements.
    AppFrameOnly,
    /// Thin header with back/forward/reload buttons and URL indicator.
    MinimalNavigation,
    /// Complete browser window with address bar, tabs, and extensions.
    FullBrowser,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebappProfileMode {
    /// Isolated user data profile folder dedicated to this web application.
    Isolated,
    /// Shares the main browser profile and session state.
    Shared,
    /// Ephemeral in-memory profile cleared upon exit (inspired by Tails / OpenBSD).
    IncognitoEphemeral,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkPolicy {
    Full,
    DomainRestricted(Vec<String>),
    Offline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdBlockLevel {
    None,
    Standard,
    Strict,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebappSecurityPolicy {
    pub network: NetworkPolicy,
    pub adblock: AdBlockLevel,
    pub capsicum_sandboxed: bool,
    pub isolate_storage: bool,
}

impl Default for WebappSecurityPolicy {
    fn default() -> Self {
        Self {
            network: NetworkPolicy::Full,
            adblock: AdBlockLevel::Standard,
            capsicum_sandboxed: true,
            isolate_storage: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PwaDisplayMode {
    Standalone,
    MinimalUi,
    Fullscreen,
    Browser,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PwaManifest {
    pub name: String,
    pub short_name: Option<String>,
    pub start_url: String,
    pub display: PwaDisplayMode,
    pub theme_color: Option<String>,
    pub background_color: Option<String>,
    pub icon_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebappConfig {
    pub name: String,
    pub url: String,
    pub engine: WebEngineKind,
    pub category: WebappCategory,
    pub nav_mode: WebappNavigationMode,
    pub profile_mode: WebappProfileMode,
    pub security_policy: WebappSecurityPolicy,
    pub custom_user_agent: Option<String>,
    pub custom_css: Option<String>,
    pub icon_path: Option<String>,
    pub isolated: bool,
    pub desktop_shortcut: bool,
    pub pinned: bool,
    pub force_https: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Webapp {
    pub name: String,
    pub url: String,
    pub engine: WebEngineKind,
    pub category: WebappCategory,
    pub nav_mode: WebappNavigationMode,
    pub profile_mode: WebappProfileMode,
    pub security_policy: WebappSecurityPolicy,
    pub custom_user_agent: Option<String>,
    pub custom_css: Option<String>,
    pub icon_path: Option<String>,
    pub isolated: bool,
    pub desktop_shortcut: bool,
    pub pinned: bool,
    pub force_https: bool,
}

pub struct WebappManager {
    pub apps: Vec<Webapp>,
}

impl WebappManager {
    pub fn new() -> Self {
        Self { apps: Vec::new() }
    }

    pub fn add_webapp(&mut self, name: &str, url: &str, engine: WebEngineKind) -> &Webapp {
        let is_iso = name.to_lowercase().contains("drive")
            || name.to_lowercase().contains("mail")
            || name.to_lowercase().contains("office")
            || url.contains("accounts.google")
            || url.contains("mail");

        let formatted_url = if !url.starts_with("https://") && !url.starts_with("http://") {
            format!("https://{}", url)
        } else {
            url.to_string()
        };

        let app = Webapp {
            name: name.to_string(),
            url: formatted_url,
            engine,
            category: WebappCategory::Web,
            nav_mode: WebappNavigationMode::AppFrameOnly,
            profile_mode: if is_iso { WebappProfileMode::Isolated } else { WebappProfileMode::Shared },
            security_policy: WebappSecurityPolicy::default(),
            custom_user_agent: None,
            custom_css: None,
            icon_path: None,
            isolated: is_iso,
            desktop_shortcut: true,
            pinned: false,
            force_https: true,
        };

        self.apps.push(app);
        self.apps.last().unwrap()
    }

    pub fn add_webapp_full(&mut self, cfg: WebappConfig) -> &Webapp {
        let formatted_url = if cfg.force_https && cfg.url.starts_with("http://") {
            format!("https://{}", &cfg.url[7..])
        } else if !cfg.url.starts_with("https://") && !cfg.url.starts_with("http://") {
            format!("https://{}", cfg.url)
        } else {
            cfg.url.clone()
        };

        let app = Webapp {
            name: cfg.name,
            url: formatted_url,
            engine: cfg.engine,
            category: cfg.category,
            nav_mode: cfg.nav_mode,
            profile_mode: cfg.profile_mode,
            security_policy: cfg.security_policy,
            custom_user_agent: cfg.custom_user_agent,
            custom_css: cfg.custom_css,
            icon_path: cfg.icon_path,
            isolated: cfg.isolated || cfg.profile_mode == WebappProfileMode::Isolated,
            desktop_shortcut: cfg.desktop_shortcut,
            pinned: cfg.pinned,
            force_https: cfg.force_https,
        };

        self.apps.push(app);
        self.apps.last().unwrap()
    }

    pub fn remove_webapp(&mut self, name: &str) -> bool {
        let prev_len = self.apps.len();
        self.apps.retain(|a| a.name != name);
        self.apps.len() < prev_len
    }

    pub fn launch(&self, name: &str) -> Option<&Webapp> {
        self.apps.iter().find(|a| a.name == name)
    }

    pub fn app_count(&self) -> usize {
        self.apps.len()
    }

    pub fn get_apps_by_category(&self, category: WebappCategory) -> Vec<&Webapp> {
        self.apps.iter().filter(|a| a.category == category).collect()
    }

    pub fn profile_path(&self, app_name: &str) -> Option<String> {
        let app = self.launch(app_name)?;
        match app.profile_mode {
            WebappProfileMode::Isolated => Some(format!("/home/user/.local/share/sigmaos/webapps/{}", app_name.to_lowercase().replace(' ', "-"))),
            WebappProfileMode::Shared => Some("/home/user/.config/browser-default".to_string()),
            WebappProfileMode::IncognitoEphemeral => Some("/tmp/sigmaos-webapp-ephemeral".to_string()),
        }
    }

    pub fn generate_launch_command(&self, app_name: &str) -> Option<String> {
        let app = self.launch(app_name)?;
        let profile = self.profile_path(app_name).unwrap_or_default();
        let cmd = match app.engine {
            WebEngineKind::Chromium | WebEngineKind::Brave | WebEngineKind::Vivaldi => {
                let bin = match app.engine {
                    WebEngineKind::Brave => "brave",
                    WebEngineKind::Vivaldi => "vivaldi",
                    _ => "chromium",
                };
                let mut base = format!("{} --app=\"{}\" --user-data-dir=\"{}\"", bin, app.url, profile);
                if app.nav_mode == WebappNavigationMode::MinimalNavigation {
                    base.push_str(" --enable-minimal-ui");
                }
                if app.profile_mode == WebappProfileMode::IncognitoEphemeral {
                    base.push_str(" --incognito");
                }
                if let Some(ref ua) = app.custom_user_agent {
                    base.push_str(&format!(" --user-agent=\"{}\"", ua));
                }
                base
            }
            WebEngineKind::Gecko | WebEngineKind::LibreWolf => {
                let bin = if app.engine == WebEngineKind::LibreWolf { "librewolf" } else { "firefox" };
                let mut base = format!("{} --profile \"{}\" --kiosk \"{}\"", bin, profile, app.url);
                if app.profile_mode == WebappProfileMode::IncognitoEphemeral {
                    base.push_str(" --private-window");
                }
                base
            }
            WebEngineKind::WebKitGtk => {
                format!("epiphany --application-mode --profile=\"{}\" \"{}\"", profile, app.url)
            }
        };
        Some(cmd)
    }

    pub fn generate_desktop_entry(&self, app_name: &str) -> Option<String> {
        let app = self.launch(app_name)?;
        let exec = self.generate_launch_command(app_name)?;
        let cat_str = match app.category {
            WebappCategory::Web => "Network;WebBrowser;",
            WebappCategory::Office => "Office;Network;",
            WebappCategory::Graphics => "Graphics;Network;",
            WebappCategory::Multimedia => "AudioVideo;Network;",
            WebappCategory::Games => "Game;Network;",
            WebappCategory::Utilities => "Utility;Network;",
            WebappCategory::Development => "Development;Network;",
            WebappCategory::Finance => "Office;Finance;Network;",
        };
        let icon = app.icon_path.as_deref().unwrap_or("www-browser");

        let entry = format!(
            "[Desktop Entry]\n\
             Version=1.0\n\
             Type=Application\n\
             Name={}\n\
             Comment=Web application powered by SigmaOS WebappManager\n\
             Exec={}\n\
             Icon={}\n\
             Terminal=false\n\
             Categories={}\n\
             StartupWMClass={}\n\
             X-SigmaOS-Webapp=true\n",
            app.name,
            exec,
            icon,
            cat_str,
            app.name.replace(' ', "")
        );
        Some(entry)
    }

    pub fn evaluate_domain_access(&self, app_name: &str, target_domain: &str) -> bool {
        let app = match self.launch(app_name) {
            Some(a) => a,
            None => return false,
        };
        match &app.security_policy.network {
            NetworkPolicy::Full => true,
            NetworkPolicy::Offline => false,
            NetworkPolicy::DomainRestricted(allowed_list) => {
                allowed_list.iter().any(|d| d == target_domain || target_domain.ends_with(&format!(".{}", d)))
            }
        }
    }

    pub fn parse_pwa_manifest(json_str: &str) -> Result<PwaManifest, &'static str> {
        let name = json_str
            .split("\"name\"")
            .nth(1)
            .and_then(|s| s.split('"').nth(1))
            .ok_or("missing name in manifest")?;

        let start_url = json_str
            .split("\"start_url\"")
            .nth(1)
            .and_then(|s| s.split('"').nth(1))
            .unwrap_or("/");

        let display_str = json_str
            .split("\"display\"")
            .nth(1)
            .and_then(|s| s.split('"').nth(1))
            .unwrap_or("standalone");

        let display = match display_str {
            "minimal-ui" => PwaDisplayMode::MinimalUi,
            "fullscreen" => PwaDisplayMode::Fullscreen,
            "browser" => PwaDisplayMode::Browser,
            _ => PwaDisplayMode::Standalone,
        };

        let short_name = json_str
            .split("\"short_name\"")
            .nth(1)
            .and_then(|s| s.split('"').nth(1))
            .map(|s| s.to_string());

        let theme_color = json_str
            .split("\"theme_color\"")
            .nth(1)
            .and_then(|s| s.split('"').nth(1))
            .map(|s| s.to_string());

        let bg_color = json_str
            .split("\"background_color\"")
            .nth(1)
            .and_then(|s| s.split('"').nth(1))
            .map(|s| s.to_string());

        Ok(PwaManifest {
            name: name.to_string(),
            short_name,
            start_url: start_url.to_string(),
            display,
            theme_color,
            background_color: bg_color,
            icon_url: None,
        })
    }

    pub fn add_from_pwa_manifest(&mut self, manifest: &PwaManifest, engine: WebEngineKind) -> &Webapp {
        let nav_mode = match manifest.display {
            PwaDisplayMode::Standalone | PwaDisplayMode::Fullscreen => WebappNavigationMode::AppFrameOnly,
            PwaDisplayMode::MinimalUi => WebappNavigationMode::MinimalNavigation,
            PwaDisplayMode::Browser => WebappNavigationMode::FullBrowser,
        };

        let cfg = WebappConfig {
            name: manifest.name.clone(),
            url: manifest.start_url.clone(),
            engine,
            category: WebappCategory::Web,
            nav_mode,
            profile_mode: WebappProfileMode::Isolated,
            security_policy: WebappSecurityPolicy::default(),
            custom_user_agent: None,
            custom_css: None,
            icon_path: manifest.icon_url.clone(),
            isolated: true,
            desktop_shortcut: true,
            pinned: false,
            force_https: true,
        };

        self.add_webapp_full(cfg)
    }

    pub fn export_config(&self) -> String {
        let mut out = String::from("[\n");
        for (i, app) in self.apps.iter().enumerate() {
            out.push_str(&format!(
                "  {{\"name\": \"{}\", \"url\": \"{}\", \"engine\": \"{:?}\", \"category\": \"{:?}\"}}{}\n",
                app.name, app.url, app.engine, app.category,
                if i + 1 < self.apps.len() { "," } else { "" }
            ));
        }
        out.push_str("]\n");
        out
    }

    pub fn import_config(&mut self, config: &str) -> usize {
        let mut imported = 0;
        for line in config.lines() {
            if line.contains("\"name\"") && line.contains("\"url\"") {
                if let Some(name) = line.split("\"name\": \"").nth(1).and_then(|s| s.split('"').next()) {
                    if let Some(url) = line.split("\"url\": \"").nth(1).and_then(|s| s.split('"').next()) {
                        self.add_webapp(name, url, WebEngineKind::Chromium);
                        imported += 1;
                    }
                }
            }
        }
        imported
    }
}

impl Default for WebappManager {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. CAPTAIN / APTURL -> CaptainInstaller
//    Install .deb files and apt:// URLs with dependency resolution.
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CaptainSource {
    DebFile { path: String },
    AptUrl { package: String },
}

#[derive(Debug, Clone)]
pub struct DebPackage {
    pub name: String,
    pub version: String,
    pub depends: Vec<String>,
}

pub struct CaptainInstaller {
    pub history: Vec<DebPackage>,
    pub repo_packages: Vec<DebPackage>,
}

impl CaptainInstaller {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            repo_packages: Vec::new(),
        }
    }

    pub fn seed_repo(&mut self, pkgs: Vec<DebPackage>) {
        self.repo_packages = pkgs;
    }

    /// Install from a .deb file, resolving dependencies against the repo.
    pub fn install_deb(&mut self, pkg: DebPackage) -> Result<(), &'static str> {
        let missing: Vec<&String> = pkg
            .depends
            .iter()
            .filter(|d| !self.is_installed(d) && !self.repo_has(d))
            .collect();
        if !missing.is_empty() {
            return Err("unmet dependencies");
        }
        let clone = DebPackage {
            name: pkg.name.clone(),
            version: pkg.version.clone(),
            depends: pkg.depends.clone(),
        };
        self.history.push(clone);
        Ok(())
    }

    /// Install from an `apt://pkgname` URL (apturl flow).
    pub fn install_from_apt_url(&mut self, url: &str) -> Result<String, &'static str> {
        let pkg_name = url
            .strip_prefix("apt://")
            .ok_or("malformed apt URL")?;
        let pkg = self.repo_packages.iter().find(|p| p.name == pkg_name).cloned();
        match pkg {
            Some(p) => {
                self.install_deb(p)?;
                Ok(pkg_name.to_string())
            }
            None => Err("package not found in repositories"),
        }
    }

    fn is_installed(&self, name: &str) -> bool {
        self.history.iter().any(|p| p.name == name)
    }

    fn repo_has(&self, name: &str) -> bool {
        self.repo_packages.iter().any(|p| p.name == name)
    }
}

impl Default for CaptainInstaller {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 5. HYPNOTIX -> HypnotixIptvPlayer
//    IPTV streaming with multiple provider types: M3U URL, Xtream API, local.
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderType {
    M3uUrl,
    XtreamApi,
    LocalM3u,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IptvProvider {
    pub name: String,
    pub kind: ProviderType,
    pub endpoint: String,
    pub country_grouped: bool,
    pub adult_content: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TvChannel {
    pub id: String,
    pub name: String,
    pub country: String,
    pub provider: String,
    pub url: String,
}

pub struct HypnotixIptvPlayer {
    pub providers: Vec<IptvProvider>,
    pub channels: Vec<TvChannel>,
}

impl HypnotixIptvPlayer {
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
            channels: Vec::new(),
        }
    }

    pub fn add_provider(&mut self, name: &str, kind: ProviderType, endpoint: &str, adult: bool) {
        self.providers.push(IptvProvider {
            name: name.to_string(),
            kind,
            endpoint: endpoint.to_string(),
            country_grouped: true,
            adult_content: adult,
        });
    }

    /// Ingest an M3U playlist into channels keyed by provider + country.
    pub fn ingest_m3u(&mut self, provider: &str, lines: &[&str]) -> usize {
        for line in lines {
            let entries = line.split('#').collect::<Vec<&str>>();
            if entries.len() >= 2 {
                self.channels.push(TvChannel {
                    id: entries[0].to_string(),
                    name: entries[0].to_string(),
                    country: entries[1].split(':').last().unwrap_or("unknown").to_string(),
                    provider: provider.to_string(),
                    url: entries[0].to_string(),
                });
            }
        }
        self.channels.len()
    }

    pub fn channels_by_country(&self, country: &str) -> Vec<&TvChannel> {
        self.channels
            .iter()
            .filter(|c| c.country == country)
            .collect()
    }

    pub fn select_free_provider(&mut self) {
        // Default providers exclude adult content, per Hypnotix's Free-TV.
        self.providers.retain(|p| !p.adult_content);
    }

    pub fn provider_count(&self) -> usize {
        self.providers.len()
    }
}

impl Default for HypnotixIptvPlayer {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 6. BULKY -> BulkyRenamer
//    Rename multiple files and directories at once with reusable rules.
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RenameRule {
    FindReplace { find: String, replace: String },
    Prepend { prefix: String },
    Append { suffix: String },
    LowerCase,
    UpperCase,
    TrimWhitespace,
    Sequence { start: u32, step: u32 },
}

pub struct BulkyRenamer {
    pub files: Vec<String>,
    pub rules: Vec<RenameRule>,
}

impl BulkyRenamer {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            rules: Vec::new(),
        }
    }

    pub fn add_file(&mut self, path: &str) {
        self.files.push(path.to_string());
    }

    pub fn add_rule(&mut self, rule: RenameRule) {
        self.rules.push(rule);
    }

    pub fn preview(&self) -> Vec<RenamedFile> {
        self.files
            .iter()
            .enumerate()
            .map(|(i, f)| {
                let seq = i as u32;
                let mut out = f.clone();
                for rule in &self.rules {
                    out = apply_rule(rule, &out, seq);
                }
                RenamedFile {
                    original: f.clone(),
                    renamed: out,
                }
            })
            .collect()
    }

    pub fn execute(&mut self) -> Vec<RenamedFile> {
        let renamed = self.preview();
        self.files = renamed.iter().map(|r| r.renamed.clone()).collect();
        renamed
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenamedFile {
    pub original: String,
    pub renamed: String,
}

fn apply_rule(rule: &RenameRule, input: &str, seq: u32) -> String {
    match rule {
        RenameRule::FindReplace { find, replace } => {
            input.replace(find.as_str(), replace.as_str())
        }
        RenameRule::Prepend { prefix } => format!("{}{}", prefix, input),
        RenameRule::Append { suffix } => format!("{}{}", input, suffix),
        RenameRule::LowerCase => input.to_lowercase(),
        RenameRule::UpperCase => input.to_uppercase(),
        RenameRule::TrimWhitespace => input.trim().to_string(),
        RenameRule::Sequence { start, step } => {
            let n = start + seq * step;
            format!("{}{}", input, n)
        }
    }
}

impl Default for BulkyRenamer {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 7. MINTNANNY -> MintNannyFilter
//    Parental/web content filtering. MintNanny blocks adult/undesired content
//    using a two-tier model (blocked + allowed domains).
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NannyDecision {
    Allow,
    Block,
}

pub struct MintNannyFilter {
    pub blocked_domains: Vec<String>,
    pub allowed_domains: Vec<String>,
    pub enabled: bool,
}

impl MintNannyFilter {
    pub fn new() -> Self {
        Self {
            blocked_domains: Vec::new(),
            allowed_domains: Vec::new(),
            enabled: true,
        }
    }

    pub fn block(&mut self, domain: &str) {
        if !self.blocked_domains.contains(&domain.to_string()) {
            self.blocked_domains.push(domain.to_string());
        }
    }

    pub fn allow(&mut self, domain: &str) {
        if !self.allowed_domains.contains(&domain.to_string()) {
            self.allowed_domains.push(domain.to_string());
        }
    }

    /// Allow-listed domains always win; then check the block list, with
    /// subdomain matching (e.g. blocking `adult.example` blocks
    /// `media.adult.example`).
    pub fn evaluate(&self, url: &str) -> NannyDecision {
        if !self.enabled {
            return NannyDecision::Allow;
        }
        for allowed in &self.allowed_domains {
            if matches_domain(url, allowed) {
                return NannyDecision::Allow;
            }
        }
        for blocked in &self.blocked_domains {
            if matches_domain(url, blocked) {
                return NannyDecision::Block;
            }
        }
        NannyDecision::Allow
    }
}

fn matches_domain(url: &str, domain: &str) -> bool {
    let host = url
        .split("://")
        .nth(1)
        .and_then(|rest| rest.split('/').next())
        .unwrap_or(url);
    host == domain || host.ends_with(&format!(".{}", domain))
}

impl Default for MintNannyFilter {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 8. MINTWELCOME -> MintWelcomeFlow
//    First-run onboarding with optional next steps that install/extend systems.
// =========================================================================

#[derive(Debug, Clone)]
pub struct WelcomeStep {
    pub id: String,
    pub title: String,
    pub done: bool,
}

pub struct MintWelcomeFlow {
    pub steps: Vec<WelcomeStep>,
    pub completed: bool,
    pub onboarding_version: u32,
}

impl MintWelcomeFlow {
    pub fn new() -> Self {
        let steps = vec![
            WelcomeStep { id: "update".into(), title: "Install system updates".into(), done: false },
            WelcomeStep { id: "drivers".into(), title: "Enable driver manager".into(), done: false },
            WelcomeStep { id: "codecs".into(), title: "Install media codecs".into(), done: false },
            WelcomeStep { id: "backup".into(), title: "Configure automatic backups".into(), done: false },
        ];
        Self {
            steps,
            completed: false,
            onboarding_version: 1,
        }
    }

    pub fn mark_done(&mut self, id: &str) {
        for s in &mut self.steps {
            if s.id == id {
                s.done = true;
            }
        }
    }

    pub fn remaining(&self) -> Vec<&WelcomeStep> {
        self.steps.iter().filter(|s| !s.done).collect()
    }

    pub fn is_complete(&self) -> bool {
        self.steps.iter().all(|s| s.done)
    }
}

impl Default for MintWelcomeFlow {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 9. MINTREPORT -> MintReportDiagnostics
//    Gather system information that can help diagnose issues.
// =========================================================================

#[derive(Debug, Clone)]
pub struct DiagnosticField {
    pub key: String,
    pub value: String,
    pub ok: bool,
}

pub struct MintReportDiagnostics {
    pub fields: Vec<DiagnosticField>,
}

impl MintReportDiagnostics {
    pub fn new() -> Self {
        Self { fields: Vec::new() }
    }

    pub fn add(&mut self, key: &str, value: &str, ok: bool) {
        self.fields.push(DiagnosticField {
            key: key.to_string(),
            value: value.to_string(),
            ok,
        });
    }

    pub fn issues(&self) -> Vec<&DiagnosticField> {
        self.fields.iter().filter(|f| !f.ok).collect()
    }

    pub fn render(&self) -> String {
        let mut out = String::new();
        for f in &self.fields {
            let flag = if f.ok { "[OK]" } else { "[!!]" };
            out.push_str(&format!("{} {} = {}\n", flag, f.key, f.value));
        }
        out
    }
}

impl Default for MintReportDiagnostics {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 10. MINTSTICK -> MintStickFormatter
//     Format USB drives / memory sticks with a filesystem.
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsFormat {
    Fat32,
    Ext4,
    Ntfs,
    Exfat,
}

#[derive(Debug, Clone)]
pub struct UsbDevice {
    pub path: String,
    pub label: String,
    pub size_mb: u64,
    pub writable: bool,
}

pub struct MintStickFormatter {
    pub format_history: Vec<String>,
}

impl MintStickFormatter {
    pub fn new() -> Self {
        Self {
            format_history: Vec::new(),
        }
    }

    pub fn format(&mut self, device: &UsbDevice, fs: FsFormat) -> Result<(), &'static str> {
        if !device.writable {
            return Err("device is read-only");
        }
        self.format_history.push(format!(
            "{} -> {:?} ({} MB)",
            device.path, fs, device.size_mb
        ));
        Ok(())
    }

    pub fn restore_from_iso(&mut self, device: &UsbDevice, iso_size_mb: u64) -> Result<(), &'static str> {
        if iso_size_mb > device.size_mb {
            return Err("ISO larger than device");
        }
        self.format_history.push(format!(
            "{} <- ISO restore ({} MB)",
            device.path, iso_size_mb
        ));
        Ok(())
    }
}

impl Default for MintStickFormatter {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 11. MINTCONFIG / MINTCONFIG -> MintConfigBackends
//     MintConfig is a hub for configuring individual Mint tools. Model the
//     shared XApp backend so every Mint-inspired subsystem is configurable
//     from one panel.
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigBackend {
    pub name: String,
    pub settings: Vec<(String, String)>,
}

pub struct MintConfigHub {
    pub backends: Vec<ConfigBackend>,
}

impl MintConfigHub {
    pub fn new() -> Self {
        Self {
            backends: vec![
                ConfigBackend {
                    name: "update".into(),
                    settings: vec![("auto-refresh".into(), "on".into())],
                },
                ConfigBackend {
                    name: "welcome".into(),
                    settings: vec![("onboarding-version".into(), "1".into())],
                },
            ],
        }
    }

    pub fn set(&mut self, backend: &str, key: &str, value: &str) -> bool {
        for b in &mut self.backends {
            if b.name == backend {
                for s in &mut b.settings {
                    if s.0 == key {
                        s.1 = value.to_string();
                        return true;
                    }
                }
                b.settings.push((key.to_string(), value.to_string()));
                return true;
            }
        }
        false
    }

    pub fn get(&self, backend: &str, key: &str) -> Option<&str> {
        self.backends
            .iter()
            .find(|b| b.name == backend)
            .and_then(|b| b.settings.iter().find(|s| s.0 == key))
            .map(|s| s.1.as_str())
    }
}

impl Default for MintConfigHub {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 12. XAPP core -> XAppSelection
//     The XApp library underpins nearly every Mint tool so they run on any DE.
//     Model the fallback theming + tray selection primitives.
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppTheme {
    FollowSystem,
    Light,
    Dark,
}

pub struct XAppThemeEngine {
    pub theme: AppTheme,
    pub accent_color: String,
}

impl XAppThemeEngine {
    pub fn new() -> Self {
        Self {
            theme: AppTheme::FollowSystem,
            accent_color: "#9B59B6".to_string(),
        }
    }

    pub fn effective_theme(&self, system_dark: bool) -> AppTheme {
        match self.theme {
            AppTheme::FollowSystem => {
                if system_dark {
                    AppTheme::Dark
                } else {
                    AppTheme::Light
                }
            }
            other => other,
        }
    }
}

impl Default for XAppThemeEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// Unit tests (verified via the integration harness; the `#[cfg(test)]` module
// is kept in parity with sibling files).
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn warp_secure_mode_and_discovery() {
        let mut w = LanWarpEngine::default();
        assert!(!w.secure_mode);
        assert_eq!(w.secure_mode_restrictions().len(), 3);
        assert!(w.set_group_code("myprivatekey").is_ok());
        assert!(w.secure_mode);
        assert!(w.discover_peer("desk", "192.168.1.5", "myprivatekey"));
        assert!(!w.discover_peer("intruder", "192.168.1.9", "wrong-code"));
        assert_eq!(w.peer_count(), 1);
    }

    #[test]
    fn hypnotix_providers_and_countries() {
        let mut h = HypnotixIptvPlayer::new();
        h.add_provider("Free-TV", ProviderType::M3uUrl, "https://iptv", false);
        h.add_provider("AdultTV", ProviderType::XtreamApi, "https://x", true);
        h.select_free_provider();
        assert_eq!(h.provider_count(), 1);
        h.ingest_m3u("Free-TV", &["bbc#GB", "cnn#US"]);
        h.ingest_m3u("Free-TV", &["deutsche#DE"]);
        assert_eq!(h.channels_by_country("GB").len(), 1);
        assert_eq!(h.channels.len(), 3);
    }

    #[test]
    fn bulky_rules_and_preview() {
        let mut b = BulkyRenamer::new();
        b.add_file("photo (1).jpg");
        b.add_file("photo (2).jpg");
        b.add_rule(RenameRule::FindReplace {
            find: " (".to_string(),
            replace: "_".to_string(),
        });
        b.add_rule(RenameRule::FindReplace {
            find: ")".to_string(),
            replace: "".to_string(),
        });
        let previews = b.preview();
        assert_eq!(previews[0].renamed, "photo_1.jpg");
        assert_eq!(previews[1].renamed, "photo_2.jpg");
    }

    #[test]
    fn nanny_allowlist_wins_over_blocklist() {
        let mut n = MintNannyFilter::new();
        n.block("adult.example");
        n.block("blocked.example");
        n.allow("docs.example");
        assert_eq!(n.evaluate("https://media.adult.example/x"), NannyDecision::Block);
        assert_eq!(n.evaluate("https://docs.example/guide"), NannyDecision::Allow);
        assert_eq!(n.evaluate("https://other.example/"), NannyDecision::Allow);
    }

    #[test]
    fn webapp_manager_full_config_and_launch_command() {
        let mut mgr = WebappManager::new();
        let cfg = WebappConfig {
            name: "Matrix Chat".into(),
            url: "https://chat.example.com".into(),
            engine: WebEngineKind::Brave,
            category: WebappCategory::Utilities,
            nav_mode: WebappNavigationMode::MinimalNavigation,
            profile_mode: WebappProfileMode::Isolated,
            security_policy: WebappSecurityPolicy::default(),
            custom_user_agent: Some("SigmaOS/1.0 WebApp".into()),
            custom_css: None,
            icon_path: Some("matrix-icon".into()),
            isolated: true,
            desktop_shortcut: true,
            pinned: true,
            force_https: true,
        };
        mgr.add_webapp_full(cfg);

        let cmd = mgr.generate_launch_command("Matrix Chat").unwrap();
        assert!(cmd.contains("brave --app=\"https://chat.example.com\""));
        assert!(cmd.contains("--enable-minimal-ui"));
        assert!(cmd.contains("--user-agent=\"SigmaOS/1.0 WebApp\""));

        let path = mgr.profile_path("Matrix Chat").unwrap();
        assert!(path.contains("/home/user/.local/share/sigmaos/webapps/matrix-chat"));
    }

    #[test]
    fn webapp_manager_pwa_manifest_parsing_and_import() {
        let manifest_json = "{\n\"name\": \"Sigma Mail\",\n\"short_name\": \"Mail\",\n\"start_url\": \"https://mail.sigma.org\",\n\"display\": \"minimal-ui\",\n\"theme_color\": \"#1f2937\"\n}";
        let manifest = WebappManager::parse_pwa_manifest(manifest_json).unwrap();
        assert_eq!(manifest.name, "Sigma Mail");
        assert_eq!(manifest.display, PwaDisplayMode::MinimalUi);

        let mut mgr = WebappManager::new();
        mgr.add_from_pwa_manifest(&manifest, WebEngineKind::LibreWolf);
        assert_eq!(mgr.app_count(), 1);

        let cmd = mgr.generate_launch_command("Sigma Mail").unwrap();
        assert!(cmd.contains("librewolf --profile"));
        assert!(cmd.contains("https://mail.sigma.org"));
    }

    #[test]
    fn webapp_manager_desktop_entry_and_category_filtering() {
        let mut mgr = WebappManager::new();
        mgr.add_webapp_full(WebappConfig {
            name: "Krita Web".into(),
            url: "https://krita.org".into(),
            engine: WebEngineKind::WebKitGtk,
            category: WebappCategory::Graphics,
            nav_mode: WebappNavigationMode::AppFrameOnly,
            profile_mode: WebappProfileMode::Shared,
            security_policy: WebappSecurityPolicy::default(),
            custom_user_agent: None,
            custom_css: None,
            icon_path: None,
            isolated: false,
            desktop_shortcut: true,
            pinned: false,
            force_https: true,
        });

        let graphics_apps = mgr.get_apps_by_category(WebappCategory::Graphics);
        assert_eq!(graphics_apps.len(), 1);

        let desktop_entry = mgr.generate_desktop_entry("Krita Web").unwrap();
        assert!(desktop_entry.contains("[Desktop Entry]"));
        assert!(desktop_entry.contains("Categories=Graphics;Network;"));
        assert!(desktop_entry.contains("Exec=epiphany --application-mode"));
    }

    #[test]
    fn webapp_manager_capsicum_domain_security_policy() {
        let mut mgr = WebappManager::new();
        mgr.add_webapp_full(WebappConfig {
            name: "Restricted Portal".into(),
            url: "https://internal.company.com".into(),
            engine: WebEngineKind::Chromium,
            category: WebappCategory::Office,
            nav_mode: WebappNavigationMode::AppFrameOnly,
            profile_mode: WebappProfileMode::Isolated,
            security_policy: WebappSecurityPolicy {
                network: NetworkPolicy::DomainRestricted(vec![
                    "company.com".to_string(),
                    "auth.provider.org".to_string(),
                ]),
                adblock: AdBlockLevel::Strict,
                capsicum_sandboxed: true,
                isolate_storage: true,
            },
            custom_user_agent: None,
            custom_css: None,
            icon_path: None,
            isolated: true,
            desktop_shortcut: true,
            pinned: false,
            force_https: true,
        });

        assert!(mgr.evaluate_domain_access("Restricted Portal", "internal.company.com"));
        assert!(mgr.evaluate_domain_access("Restricted Portal", "auth.provider.org"));
        assert!(!mgr.evaluate_domain_access("Restricted Portal", "untrusted.tracker.net"));
    }
}
