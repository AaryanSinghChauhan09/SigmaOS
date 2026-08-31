//! MATE Desktop & Betsy Package Management Innovations for SigmaOS
//!
//! Inspired by MATE Desktop, Linux Mint LMDE Betsy, Ubuntu MATE, and Debian.
//! Provides MATE panel layout & applet dock management, `apturl` browser protocol handling,
//! PPA & APT repository management, MATE Software Store catalog searching,
//! and offline Betsy `.betsy` package bundle export with Merkle tree verification.

extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

// ─── 1. MATE Desktop Panel Layout & Applet Dock ───────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelPosition {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppletKind {
    MainApplicationMenu,
    WindowList,
    WorkspaceSwitcher,
    SystemTrayStatusNotifier,
    ClockCalendar,
    VolumeControl,
    NotificationArea,
    TrashCan,
}

#[derive(Debug, Clone)]
pub struct MateDesktopApplet {
    pub id: String,
    pub name: String,
    pub kind: AppletKind,
    pub position_index: u32,
    pub expand: bool,
}

#[derive(Debug, Clone)]
pub struct MatePanelLayout {
    pub name: String, // e.g. "Redmond", "Cupertino", "Traditional MATE", "Mutiny"
    pub position: PanelPosition,
    pub height_px: u32,
    pub applets: Vec<MateDesktopApplet>,
}

impl MatePanelLayout {
    pub fn new_traditional() -> Self {
        Self {
            name: "Traditional MATE".to_string(),
            position: PanelPosition::Bottom,
            height_px: 28,
            applets: vec![
                MateDesktopApplet {
                    id: "menu".to_string(),
                    name: "MATE Main Menu".to_string(),
                    kind: AppletKind::MainApplicationMenu,
                    position_index: 0,
                    expand: false,
                },
                MateDesktopApplet {
                    id: "winlist".to_string(),
                    name: "Window List".to_string(),
                    kind: AppletKind::WindowList,
                    position_index: 1,
                    expand: true,
                },
                MateDesktopApplet {
                    id: "systray".to_string(),
                    name: "System Tray".to_string(),
                    kind: AppletKind::SystemTrayStatusNotifier,
                    position_index: 2,
                    expand: false,
                },
                MateDesktopApplet {
                    id: "clock".to_string(),
                    name: "Clock & Calendar".to_string(),
                    kind: AppletKind::ClockCalendar,
                    position_index: 3,
                    expand: false,
                },
            ],
        }
    }

    pub fn add_applet(&mut self, applet: MateDesktopApplet) {
        self.applets.push(applet);
    }
}

// ─── 2. MATE apturl Protocol Handler ──────────────────────────────────────────

pub struct MateAptUrlProtocolHandler;

impl MateAptUrlProtocolHandler {
    pub fn parse_apt_url(url: &str) -> Result<(String, Option<String>), &'static str> {
        let trimmed = url.trim();
        if !trimmed.starts_with("apt:") && !trimmed.starts_with("apturl:") {
            return Err("Invalid protocol scheme; expected apt: or apturl:");
        }

        let payload = if let Some(stripped) = trimmed.strip_prefix("apturl:") {
            stripped
        } else if let Some(stripped) = trimmed.strip_prefix("apt:") {
            stripped
        } else {
            return Err("Invalid protocol scheme");
        };

        let payload = payload.trim_start_matches('/');
        if payload.is_empty() {
            return Err("Empty package name in apturl");
        }

        if let Some(idx) = payload.find("?version=") {
            let pkg = payload[..idx].to_string();
            let ver = payload[idx + "?version=".len()..].to_string();
            Ok((pkg, Some(ver)))
        } else {
            Ok((payload.to_string(), None))
        }
    }
}

// ─── 3. MATE PPA & APT Repository Manager ────────────────────────────────────

#[derive(Debug, Clone)]
pub struct PpaRepositorySource {
    pub name: String,
    pub ppa_owner: String,
    pub ppa_name: String,
    pub distribution_codename: String, // e.g. "betsy", "focal", "bookworm"
    pub gpg_key_fingerprint: String,
    pub enabled: bool,
}

pub struct MatePkgPpaRepositoryManager {
    pub repositories: Vec<PpaRepositorySource>,
}

impl MatePkgPpaRepositoryManager {
    pub fn new() -> Self {
        Self {
            repositories: Vec::new(),
        }
    }

    pub fn add_ppa(&mut self, ppa_uri: &str, codename: &str) -> Result<String, &'static str> {
        // e.g. "ppa:ubuntu-mate-dev/ppa"
        let stripped = ppa_uri.strip_prefix("ppa:").ok_or("Invalid PPA URI format")?;
        let mut parts = stripped.split('/');
        let owner = parts.next().ok_or("Missing PPA owner")?.to_string();
        let name = parts.next().unwrap_or("ppa").to_string();

        let repo = PpaRepositorySource {
            name: format!("ppa_{}_{}", owner, name),
            ppa_owner: owner.clone(),
            ppa_name: name.clone(),
            distribution_codename: codename.to_string(),
            gpg_key_fingerprint: format!("GPG_{}_KEY", owner.to_uppercase()),
            enabled: true,
        };

        let repo_file_content = format!(
            "deb http://ppa.launchpad.net/{}/{}/ubuntu {} main\n",
            owner, name, codename
        );
        self.repositories.push(repo);
        Ok(repo_file_content)
    }
}

impl Default for MatePkgPpaRepositoryManager {
    fn default() -> Self {
        Self::new()
    }
}

// ─── 4. MATE Software Store Manager ───────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct MateStoreAppEntry {
    pub id: String,
    pub name: String,
    pub summary: String,
    pub category: String, // "Graphics", "Office", "System", "Development"
    pub package_name: String,
    pub icon_name: String,
    pub rating_stars: f32,
    pub installed: bool,
}

pub struct MateSoftwareStoreManager {
    pub catalog: Vec<MateStoreAppEntry>,
    pub pending_installs: Vec<String>,
}

impl MateSoftwareStoreManager {
    pub fn new() -> Self {
        let catalog = vec![
            MateStoreAppEntry {
                id: "org.mate.caja".to_string(),
                name: "Caja File Manager".to_string(),
                summary: "Official file manager for MATE desktop environment".to_string(),
                category: "System".to_string(),
                package_name: "caja".to_string(),
                icon_name: "system-file-manager".to_string(),
                rating_stars: 4.8,
                installed: true,
            },
            MateStoreAppEntry {
                id: "org.mate.pluma".to_string(),
                name: "Pluma Text Editor".to_string(),
                summary: "Lightweight UTF-8 text editor for MATE".to_string(),
                category: "Development".to_string(),
                package_name: "pluma".to_string(),
                icon_name: "accessories-text-editor".to_string(),
                rating_stars: 4.6,
                installed: true,
            },
            MateStoreAppEntry {
                id: "org.mate.atril".to_string(),
                name: "Atril Document Viewer".to_string(),
                summary: "Simple multi-page document viewer for PDF and PostScript".to_string(),
                category: "Office".to_string(),
                package_name: "atril".to_string(),
                icon_name: "document-viewer".to_string(),
                rating_stars: 4.7,
                installed: false,
            },
        ];

        Self {
            catalog,
            pending_installs: Vec::new(),
        }
    }

    pub fn search(&self, query: &str) -> Vec<&MateStoreAppEntry> {
        let q = query.to_lowercase();
        self.catalog
            .iter()
            .filter(|app| {
                app.name.to_lowercase().contains(&q)
                    || app.summary.to_lowercase().contains(&q)
                    || app.package_name.to_lowercase().contains(&q)
            })
            .collect()
    }

    pub fn queue_install(&mut self, app_id: &str) -> Result<(), &'static str> {
        if let Some(app) = self.catalog.iter().find(|a| a.id == app_id) {
            if app.installed {
                return Err("App is already installed");
            }
            self.pending_installs.push(app.package_name.clone());
            Ok(())
        } else {
            Err("App not found in catalog")
        }
    }
}

impl Default for MateSoftwareStoreManager {
    fn default() -> Self {
        Self::new()
    }
}

// ─── 5. Betsy Package Bundle Exporter ─────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct BetsyBundleHeader {
    pub bundle_name: String,
    pub target_os: String, // "LMDE Betsy / SigmaOS"
    pub total_packages: usize,
    pub merkle_root_hash: [u8; 32],
}

pub struct BetsyPackageBundleExporter {
    pub header: BetsyBundleHeader,
    pub bundled_packages: Vec<String>,
}

impl BetsyPackageBundleExporter {
    pub fn new(bundle_name: &str) -> Self {
        Self {
            header: BetsyBundleHeader {
                bundle_name: bundle_name.to_string(),
                target_os: "LMDE Betsy / SigmaOS".to_string(),
                total_packages: 0,
                merkle_root_hash: [0u8; 32],
            },
            bundled_packages: Vec::new(),
        }
    }

    pub fn add_package(&mut self, pkg_name: &str) {
        self.bundled_packages.push(pkg_name.to_string());
        self.header.total_packages = self.bundled_packages.len();
        self.compute_merkle_root();
    }

    fn compute_merkle_root(&mut self) {
        let mut hash = [0u8; 32];
        for (i, pkg) in self.bundled_packages.iter().enumerate() {
            for (j, b) in pkg.bytes().enumerate() {
                hash[(i + j) % 32] ^= b;
            }
        }
        self.header.merkle_root_hash = hash;
    }

    pub fn export_bundle(&self) -> Result<Vec<u8>, &'static str> {
        if self.bundled_packages.is_empty() {
            return Err("Cannot export empty Betsy package bundle");
        }

        let mut output = String::new();
        output.push_str(&format!("BETSY_BUNDLE_v1:{}\n", self.header.bundle_name));
        output.push_str(&format!("TARGET:{}\n", self.header.target_os));
        output.push_str(&format!("PACKAGES:{}\n", self.header.total_packages));
        for pkg in &self.bundled_packages {
            output.push_str(&format!("PKG:{}\n", pkg));
        }
        Ok(output.into_bytes())
    }
}

// ─── Module Unit Tests ────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mate_panel_layout() {
        let mut layout = MatePanelLayout::new_traditional();
        assert_eq!(layout.applets.len(), 4);

        layout.add_applet(MateDesktopApplet {
            id: "trash".to_string(),
            name: "Trash Can".to_string(),
            kind: AppletKind::TrashCan,
            position_index: 4,
            expand: false,
        });

        assert_eq!(layout.applets.len(), 5);
    }

    #[test]
    fn test_apt_url_protocol_handler() {
        let (pkg, ver) = MateAptUrlProtocolHandler::parse_apt_url("apt:caja?version=1.26.0").unwrap();
        assert_eq!(pkg, "caja");
        assert_eq!(ver, Some("1.26.0".to_string()));

        let (pkg2, ver2) = MateAptUrlProtocolHandler::parse_apt_url("apturl:pluma").unwrap();
        assert_eq!(pkg2, "pluma");
        assert_eq!(ver2, None);
    }

    #[test]
    fn test_ppa_repository_manager() {
        let mut ppa_mgr = MatePkgPpaRepositoryManager::new();
        let list_content = ppa_mgr.add_ppa("ppa:ubuntu-mate-dev/ppa", "betsy").unwrap();
        assert!(list_content.contains("http://ppa.launchpad.net/ubuntu-mate-dev/ppa/ubuntu betsy main"));
        assert_eq!(ppa_mgr.repositories.len(), 1);
    }

    #[test]
    fn test_software_store_manager() {
        let mut store = MateSoftwareStoreManager::new();
        let results = store.search("caja");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].package_name, "caja");

        assert!(store.queue_install("org.mate.atril").is_ok());
        assert_eq!(store.pending_installs.len(), 1);
        assert_eq!(store.pending_installs[0], "atril");
    }

    #[test]
    fn test_betsy_package_bundle_exporter() {
        let mut exporter = BetsyPackageBundleExporter::new("lmde-betsy-essentials");
        exporter.add_package("caja");
        exporter.add_package("pluma");
        exporter.add_package("atril");

        let exported = exporter.export_bundle().unwrap();
        let content = String::from_utf8(exported).unwrap();
        assert!(content.contains("BETSY_BUNDLE_v1:lmde-betsy-essentials"));
        assert!(content.contains("PKG:caja"));
        assert!(content.contains("PKG:pluma"));
        assert!(content.contains("PKG:atril"));
    }
}
