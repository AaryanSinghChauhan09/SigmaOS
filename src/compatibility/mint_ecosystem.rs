use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;

// =========================================================================
// 1. CINNAMON SPICES ENGINE (desklets, themes, actions)
// =========================================================================

#[derive(Debug, Clone)]
pub struct CinnamonSpiceDesklet {
    pub id: String,
    pub name: String,
    pub version: String,
    pub is_enabled: bool,
}

#[derive(Debug, Clone)]
pub struct CinnamonSpiceTheme {
    pub name: String,
    pub is_dark_variant: bool,
    pub accent_color: String,
}

pub struct CinnamonSpicesEngine {
    pub desklets: Vec<CinnamonSpiceDesklet>,
    pub active_theme: CinnamonSpiceTheme,
    pub custom_actions: BTreeMap<String, String>,
}

impl CinnamonSpicesEngine {
    pub fn new() -> Self {
        Self {
            desklets: Vec::new(),
            active_theme: CinnamonSpiceTheme {
                name: String::from("Mint-Y-Dark-Aqua"),
                is_dark_variant: true,
                accent_color: String::from("#16a085"),
            },
            custom_actions: BTreeMap::new(),
        }
    }

    pub fn register_desklet(&mut self, id: &str, name: &str, version: &str) {
        self.desklets.push(CinnamonSpiceDesklet {
            id: id.to_string(),
            name: name.to_string(),
            version: version.to_string(),
            is_enabled: true,
        });
    }

    pub fn register_action(&mut self, action_name: &str, command: &str) {
        self.custom_actions
            .insert(action_name.to_string(), command.to_string());
    }
}

impl Default for CinnamonSpicesEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 2. MINTUPGRADE & MINTSOURCES MIRROR SELECTOR
// =========================================================================

#[derive(Debug, Clone)]
pub struct MintRepoMirror {
    pub url: String,
    pub speed_kbps: u32,
    pub latency_ms: u32,
}

pub struct MintUpgradeSourcesEngine {
    pub current_os_release: String,
    pub target_os_release: String,
    pub mirrors: Vec<MintRepoMirror>,
}

impl MintUpgradeSourcesEngine {
    pub fn new(current_ver: &str, target_ver: &str) -> Self {
        Self {
            current_os_release: current_ver.to_string(),
            target_os_release: target_ver.to_string(),
            mirrors: Vec::new(),
        }
    }

    pub fn add_mirror(&mut self, url: &str, speed_kbps: u32, latency_ms: u32) {
        self.mirrors.push(MintRepoMirror {
            url: url.to_string(),
            speed_kbps,
            latency_ms,
        });
    }

    pub fn select_fastest_mirror(&self) -> Option<&MintRepoMirror> {
        self.mirrors.iter().min_by_key(|m| m.latency_ms)
    }

    pub fn simulate_major_upgrade(&mut self) -> Result<String, &'static str> {
        self.current_os_release = self.target_os_release.clone();
        Ok(format!("Upgrade successful to {}", self.current_os_release))
    }
}

// =========================================================================
// 3. SLICK-GREETER & XAPP PORTAL (LightDM & Portal Parity)
// =========================================================================

pub struct SlickGreeterXappPortal {
    pub greeter_theme: String,
    pub background_image: String,
    pub user_sessions: Vec<String>,
    pub xapp_status_icons: BTreeMap<String, String>,
}

impl SlickGreeterXappPortal {
    pub fn new() -> Self {
        Self {
            greeter_theme: String::from("slick-greeter"),
            background_image: String::from("/usr/share/backgrounds/linuxmint/default.jpg"),
            user_sessions: vec![String::from("Cinnamon"), String::from("Xfce")],
            xapp_status_icons: BTreeMap::new(),
        }
    }

    pub fn register_status_icon(&mut self, app_id: &str, icon_name: &str) {
        self.xapp_status_icons
            .insert(app_id.to_string(), icon_name.to_string());
    }
}

impl Default for SlickGreeterXappPortal {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. WARPINATOR LAN FILE TRANSFER ENGINE
// =========================================================================

#[derive(Debug, Clone)]
pub struct WarpinatorPeer {
    pub hostname: String,
    pub ip_address: String,
    pub port: u16,
    pub is_authenticated: bool,
}

pub struct WarpinatorLanShare {
    pub pin_code: String,
    pub known_peers: Vec<WarpinatorPeer>,
    pub total_transferred_bytes: u64,
}

impl WarpinatorLanShare {
    pub fn new(pin: &str) -> Self {
        Self {
            pin_code: pin.to_string(),
            known_peers: Vec::new(),
            total_transferred_bytes: 0,
        }
    }

    pub fn discover_peer(&mut self, hostname: &str, ip: &str, port: u16) {
        self.known_peers.push(WarpinatorPeer {
            hostname: hostname.to_string(),
            ip_address: ip.to_string(),
            port,
            is_authenticated: false,
        });
    }

    pub fn authenticate_peer(&mut self, hostname: &str, pin: &str) -> bool {
        if pin == self.pin_code {
            if let Some(peer) = self.known_peers.iter_mut().find(|p| p.hostname == hostname) {
                peer.is_authenticated = true;
                return true;
            }
        }
        false
    }

    pub fn send_file(&mut self, hostname: &str, size_bytes: u64) -> Result<(), &'static str> {
        let peer = self
            .known_peers
            .iter()
            .find(|p| p.hostname == hostname)
            .ok_or("Warpinator: Peer not found")?;

        if !peer.is_authenticated {
            return Err("Warpinator: Peer is not authenticated");
        }

        self.total_transferred_bytes += size_bytes;
        Ok(())
    }
}

// =========================================================================
// 5. DOCUMENT, IMAGE, AND DOCUMENT HISTORY SUITE (xreader, xviewer, pix, thingy)
// =========================================================================

#[derive(Debug, Clone)]
pub struct RecentDocument {
    pub title: String,
    pub file_path: String,
    pub last_accessed_timestamp: u64,
    pub is_favorite: bool,
}

pub struct XappAppsSuite {
    pub recent_documents: Vec<RecentDocument>, // Thingy parity
    pub opened_images: Vec<String>,            // Pix / Xviewer parity
    pub opened_pdfs: Vec<String>,              // Xreader parity
}

impl XappAppsSuite {
    pub fn new() -> Self {
        Self {
            recent_documents: Vec::new(),
            opened_images: Vec::new(),
            opened_pdfs: Vec::new(),
        }
    }

    pub fn record_document_access(&mut self, title: &str, path: &str, is_fav: bool) {
        self.recent_documents.push(RecentDocument {
            title: title.to_string(),
            file_path: path.to_string(),
            last_accessed_timestamp: 1700000000,
            is_favorite: is_fav,
        });
    }

    pub fn get_favorites(&self) -> Vec<&RecentDocument> {
        self.recent_documents
            .iter()
            .filter(|d| d.is_favorite)
            .collect()
    }
}

impl Default for XappAppsSuite {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 6. CAPTAIN, TIMESHIFT, MINTDRIVERS, AND MINTBACKUP MANAGER
// =========================================================================

pub struct CaptainMintManager {
    pub timeshift_snapshots_count: usize,
    pub installed_drivers_count: usize,
    pub backups_count: usize,
}

impl CaptainMintManager {
    pub fn new() -> Self {
        Self {
            timeshift_snapshots_count: 2,
            installed_drivers_count: 3,
            backups_count: 1,
        }
    }

    pub fn create_timeshift_snapshot(&mut self) -> usize {
        self.timeshift_snapshots_count += 1;
        self.timeshift_snapshots_count
    }
}

impl Default for CaptainMintManager {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 7. LINUX MINT ECOSYSTEM HUB
// =========================================================================

pub struct LinuxMintEcosystemHub {
    pub spices: CinnamonSpicesEngine,
    pub upgrade_sources: MintUpgradeSourcesEngine,
    pub greeter_portal: SlickGreeterXappPortal,
    pub warpinator: WarpinatorLanShare,
    pub apps_suite: XappAppsSuite,
    pub captain_manager: CaptainMintManager,
}

impl LinuxMintEcosystemHub {
    pub fn new() -> Self {
        Self {
            spices: CinnamonSpicesEngine::new(),
            upgrade_sources: MintUpgradeSourcesEngine::new("21.3", "22.0"),
            greeter_portal: SlickGreeterXappPortal::new(),
            warpinator: WarpinatorLanShare::new("1234"),
            apps_suite: XappAppsSuite::new(),
            captain_manager: CaptainMintManager::new(),
        }
    }
}

impl Default for LinuxMintEcosystemHub {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cinnamon_spices_engine() {
        let mut spices = CinnamonSpicesEngine::new();
        spices.register_desklet("clock@cinnamon.org", "Desktop Clock", "1.0");
        assert_eq!(spices.desklets.len(), 1);
        assert_eq!(spices.desklets[0].name, "Desktop Clock");

        spices.register_action("open_terminal", "alacritty");
        assert_eq!(
            spices.custom_actions.get("open_terminal").unwrap(),
            "alacritty"
        );
    }

    #[test]
    fn test_mint_upgrade_sources_engine() {
        let mut sources = MintUpgradeSourcesEngine::new("21.3", "22.0");
        sources.add_mirror("https://mirror.layeronline.com/linuxmint", 50000, 20);
        sources.add_mirror("https://fast.mirror.org/linuxmint", 80000, 10);

        let fastest = sources.select_fastest_mirror().unwrap();
        assert_eq!(fastest.url, "https://fast.mirror.org/linuxmint");

        assert!(sources.simulate_major_upgrade().is_ok());
        assert_eq!(sources.current_os_release, "22.0");
    }

    #[test]
    fn test_warpinator_lan_share() {
        let mut warpinator = WarpinatorLanShare::new("9876");
        warpinator.discover_peer("laptop-work", "192.168.1.50", 42000);

        // Fail authentication
        assert!(!warpinator.authenticate_peer("laptop-work", "1111"));
        assert!(warpinator.send_file("laptop-work", 1024).is_err());

        // Pass authentication
        assert!(warpinator.authenticate_peer("laptop-work", "9876"));
        assert!(warpinator.send_file("laptop-work", 1024).is_ok());
        assert_eq!(warpinator.total_transferred_bytes, 1024);
    }

    #[test]
    fn test_xapp_apps_suite_thingy() {
        let mut suite = XappAppsSuite::new();
        suite.record_document_access("Quarterly Report", "/home/user/docs/report.pdf", true);
        suite.record_document_access("Temp Note", "/tmp/note.txt", false);

        let favs = suite.get_favorites();
        assert_eq!(favs.len(), 1);
        assert_eq!(favs[0].title, "Quarterly Report");
    }
}
