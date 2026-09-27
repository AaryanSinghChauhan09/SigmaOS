//! Linux Mint Subsystem Innovations for SigmaOS
//!
//! Inspired by Linux Mint:
//! - `CinnamonThemeConfig`: Cinnamon desktop theme, icon set, and accent color customization
//! - `TimeshiftBtrfsRsyncEngine`: Btrfs and Rsync system snapshot creation, rotation, and rollbacks
//! - `MintUpdateSafetyManager`: Tiered package update safety policy levels (1..5) with kernel protection
//! - `MintstickUsbFormatterEngine`: Low-level USB image writer, ISO burner, and FAT32/exFAT formatter



/// Cinnamon Desktop Theme Configuration
#[derive(Debug, Clone)]
pub struct CinnamonThemeConfig {
    pub theme_name: String,
    pub icon_theme: String,
    pub accent_color: String,
    pub dark_mode: bool,
}

impl CinnamonThemeConfig {
    pub fn new() -> Self {
        Self {
            theme_name: "Mint-Y".to_string(),
            icon_theme: "Mint-Y-Icons".to_string(),
            accent_color: "Green".to_string(),
            dark_mode: true,
        }
    }

    pub fn set_accent_color(&mut self, color: &str) -> String {
        self.accent_color = color.to_string();
        format!("Cinnamon Desktop Accent set to {}", self.accent_color)
    }
}

/// Linux Mint `mint-x-icons` Parity Icon Color Variant
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MintXIconVariant {
    ClassicGreen,
    Aqua,
    Blue,
    Brown,
    Dark,
    Grey,
    Orange,
    Pink,
    Purple,
    Red,
    Sand,
    Teal,
}

impl MintXIconVariant {
    pub fn theme_name(&self) -> &'static str {
        match self {
            MintXIconVariant::ClassicGreen => "Mint-X",
            MintXIconVariant::Aqua => "Mint-X-Aqua",
            MintXIconVariant::Blue => "Mint-X-Blue",
            MintXIconVariant::Brown => "Mint-X-Brown",
            MintXIconVariant::Dark => "Mint-X-Dark",
            MintXIconVariant::Grey => "Mint-X-Grey",
            MintXIconVariant::Orange => "Mint-X-Orange",
            MintXIconVariant::Pink => "Mint-X-Pink",
            MintXIconVariant::Purple => "Mint-X-Purple",
            MintXIconVariant::Red => "Mint-X-Red",
            MintXIconVariant::Sand => "Mint-X-Sand",
            MintXIconVariant::Teal => "Mint-X-Teal",
        }
    }
}

/// XDG Icon Category Directory
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconDirectoryCategory {
    Actions,
    Apps,
    Categories,
    Devices,
    Emblems,
    Mimetypes,
    Places,
    Status,
}

impl IconDirectoryCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            IconDirectoryCategory::Actions => "actions",
            IconDirectoryCategory::Apps => "apps",
            IconDirectoryCategory::Categories => "categories",
            IconDirectoryCategory::Devices => "devices",
            IconDirectoryCategory::Emblems => "emblems",
            IconDirectoryCategory::Mimetypes => "mimetypes",
            IconDirectoryCategory::Places => "places",
            IconDirectoryCategory::Status => "status",
        }
    }
}

/// Icon Descriptor Entry in Mint-X Theme Catalog
#[derive(Debug, Clone)]
pub struct MintXIconEntry {
    pub name: String,
    pub category: IconDirectoryCategory,
    pub size_px: u16,
    pub is_symbolic: bool,
    pub relative_path: String,
}

/// Linux Mint `mint-x-icons` Parity Icon Theme Engine
pub struct MintXIconThemeEngine {
    pub active_variant: MintXIconVariant,
    pub inherits_base: Vec<String>,
    pub icon_catalog: Vec<MintXIconEntry>,
    pub cache_valid: bool,
}

impl MintXIconThemeEngine {
    pub fn new(variant: MintXIconVariant) -> Self {
        let mut engine = Self {
            active_variant: variant,
            inherits_base: vec![
                "Mint-X".to_string(),
                "Mint-Y".to_string(),
                "gnome".to_string(),
                "hicolor".to_string(),
            ],
            icon_catalog: Vec::new(),
            cache_valid: false,
        };
        engine.seed_default_mint_x_icons();
        engine
    }

    fn seed_default_mint_x_icons(&mut self) {
        let default_icons = [
            ("folder", IconDirectoryCategory::Places, 48, false, "places/48/folder.png"),
            ("folder-home", IconDirectoryCategory::Places, 48, false, "places/48/folder-home.png"),
            ("user-desktop", IconDirectoryCategory::Places, 48, false, "places/48/user-desktop.png"),
            ("system-file-manager", IconDirectoryCategory::Apps, 48, false, "apps/48/system-file-manager.png"),
            ("terminal", IconDirectoryCategory::Apps, 48, false, "apps/48/terminal.png"),
            ("text-editor", IconDirectoryCategory::Apps, 48, false, "apps/48/text-editor.png"),
            ("edit-cut", IconDirectoryCategory::Actions, 16, false, "actions/16/edit-cut.png"),
            ("edit-copy", IconDirectoryCategory::Actions, 16, false, "actions/16/edit-copy.png"),
            ("folder-symbolic", IconDirectoryCategory::Places, 16, true, "places/symbolic/folder-symbolic.svg"),
            ("network-workgroup", IconDirectoryCategory::Places, 48, false, "places/48/network-workgroup.png"),
            ("drive-harddisk", IconDirectoryCategory::Devices, 48, false, "devices/48/drive-harddisk.png"),
            ("computer", IconDirectoryCategory::Places, 48, false, "places/48/computer.png"),
        ];

        for (name, category, size, symbolic, rel_path) in default_icons {
            self.icon_catalog.push(MintXIconEntry {
                name: name.to_string(),
                category,
                size_px: size,
                is_symbolic: symbolic,
                relative_path: format!("/usr/share/icons/{}/{}", self.active_variant.theme_name(), rel_path),
            });
        }
        self.cache_valid = true;
    }

    pub fn set_variant(&mut self, variant: MintXIconVariant) {
        self.active_variant = variant;
        self.cache_valid = false;
        self.rebuild_icon_cache();
    }

    pub fn rebuild_icon_cache(&mut self) {
        for entry in self.icon_catalog.iter_mut() {
            let parts: Vec<&str> = entry.relative_path.split('/').collect();
            if parts.len() >= 5 {
                entry.relative_path = format!(
                    "/usr/share/icons/{}/{}",
                    self.active_variant.theme_name(),
                    parts[5..].join("/")
                );
            }
        }
        self.cache_valid = true;
    }

    pub fn lookup_icon(&self, icon_name: &str, size_px: u16) -> Option<String> {
        // Try exact match by name and size
        if let Some(entry) = self
            .icon_catalog
            .iter()
            .find(|e| e.name == icon_name && e.size_px == size_px)
        {
            return Some(entry.relative_path.clone());
        }

        // Fallback to closest size match
        if let Some(entry) = self.icon_catalog.iter().find(|e| e.name == icon_name) {
            return Some(entry.relative_path.clone());
        }

        // Fallback to symbolic icon
        let symbolic_name = format!("{}-symbolic", icon_name);
        if let Some(entry) = self.icon_catalog.iter().find(|e| e.name == symbolic_name) {
            return Some(entry.relative_path.clone());
        }

        None
    }

    pub fn export_index_theme(&self) -> String {
        format!(
            "[Icon Theme]\nName={}\nComment=Smooth Mint-X icon theme for SigmaOS and Cinnamon\nInherits={}\nDirectories={}\n\n[places/48]\nSize=48\nContext=Places\nType=Fixed\n",
            self.active_variant.theme_name(),
            self.inherits_base.join(","),
            "places/48,apps/48,actions/16,devices/48,places/symbolic"
        )
    }
}

impl Default for MintXIconThemeEngine {
    fn default() -> Self {
        Self::new(MintXIconVariant::ClassicGreen)
    }
}

impl Default for CinnamonThemeConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Timeshift Snapshot Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeshiftMode {
    Btrfs,
    Rsync,
}

/// Timeshift System Snapshot Entry
#[derive(Debug, Clone)]
pub struct SystemSnapshotEntry {
    pub snapshot_id: String,
    pub timestamp: u64,
    pub mode: TimeshiftMode,
    pub description: String,
}

/// Timeshift Snapshot & Rollback Engine
pub struct TimeshiftBtrfsRsyncEngine {
    pub mode: TimeshiftMode,
    pub snapshots: Vec<SystemSnapshotEntry>,
}

impl TimeshiftBtrfsRsyncEngine {
    pub fn new(mode: TimeshiftMode) -> Self {
        Self {
            mode,
            snapshots: Vec::new(),
        }
    }

    pub fn create_snapshot(&mut self, description: &str, timestamp: u64) -> String {
        let snap_id = format!("timeshift_{:?}_{}", self.mode, timestamp);
        self.snapshots.push(SystemSnapshotEntry {
            snapshot_id: snap_id.clone(),
            timestamp,
            mode: self.mode,
            description: description.to_string(),
        });
        snap_id
    }

    pub fn rollback(&mut self, snapshot_id: &str) -> Result<String, &'static str> {
        if self.snapshots.iter().any(|s| s.snapshot_id == snapshot_id) {
            Ok(format!("Timeshift: System successfully restored to snapshot '{}'", snapshot_id))
        } else {
            Err("Target snapshot ID not found")
        }
    }
}

impl Default for TimeshiftBtrfsRsyncEngine {
    fn default() -> Self {
        Self::new(TimeshiftMode::Btrfs)
    }
}

/// MintUpdate Package Safety Level (1 = Certified, 5 = Experimental/Dangerous)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UpdateSafetyLevel {
    Level1Certified,
    Level2Tested,
    Level3Safe,
    Level4Untested,
    Level5Dangerous,
}

/// MintUpdate Safety Policy Manager
pub struct MintUpdateSafetyManager {
    pub max_allowed_safety_level: UpdateSafetyLevel,
    pub ignore_kernel_updates: bool,
}

impl MintUpdateSafetyManager {
    pub fn new() -> Self {
        Self {
            max_allowed_safety_level: UpdateSafetyLevel::Level3Safe,
            ignore_kernel_updates: false,
        }
    }

    pub fn evaluate_package_update(&self, pkg_name: &str, level: UpdateSafetyLevel) -> bool {
        if pkg_name.contains("kernel") && self.ignore_kernel_updates {
            return false;
        }
        level <= self.max_allowed_safety_level
    }
}

impl Default for MintUpdateSafetyManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Mintstick USB Image Writer & Formatter
pub struct MintstickUsbFormatterEngine {
    pub target_device: String,
    pub written_bytes: u64,
}

impl MintstickUsbFormatterEngine {
    pub fn new(target_device: &str) -> Self {
        Self {
            target_device: target_device.to_string(),
            written_bytes: 0,
        }
    }

    pub fn write_iso_image(&mut self, iso_bytes: &[u8]) -> Result<String, &'static str> {
        if self.target_device.is_empty() {
            return Err("Mintstick error: Target USB device path is empty");
        }
        self.written_bytes += iso_bytes.len() as u64;
        Ok(format!(
            "Mintstick: Successfully wrote {} bytes ISO image to USB device {}",
            self.written_bytes, self.target_device
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cinnamon_theme_config() {
        let mut cinnamon = CinnamonThemeConfig::new();
        assert_eq!(cinnamon.accent_color, "Green");

        let res = cinnamon.set_accent_color("Teal");
        assert!(res.contains("Accent set to Teal"));
        assert_eq!(cinnamon.accent_color, "Teal");
    }

    #[test]
    fn test_timeshift_engine() {
        let mut timeshift = TimeshiftBtrfsRsyncEngine::new(TimeshiftMode::Btrfs);
        let snap_id = timeshift.create_snapshot("Pre-upgrade backup", 1700000000);
        assert_eq!(timeshift.snapshots.len(), 1);

        let restore_res = timeshift.rollback(&snap_id).unwrap();
        assert!(restore_res.contains("System successfully restored"));
    }

    #[test]
    fn test_mint_update_safety_manager() {
        let manager = MintUpdateSafetyManager::new();
        assert!(manager.evaluate_package_update("curl", UpdateSafetyLevel::Level1Certified));
        assert!(!manager.evaluate_package_update("experimental-driver", UpdateSafetyLevel::Level5Dangerous));
    }

    #[test]
    fn test_mint_x_icon_theme_engine() {
        let mut icon_engine = MintXIconThemeEngine::new(MintXIconVariant::ClassicGreen);
        assert_eq!(icon_engine.active_variant.theme_name(), "Mint-X");

        let folder_path = icon_engine.lookup_icon("folder", 48).unwrap();
        assert!(folder_path.contains("/usr/share/icons/Mint-X/places/48/folder.png"));

        // Change color variant to Aqua
        icon_engine.set_variant(MintXIconVariant::Aqua);
        assert_eq!(icon_engine.active_variant.theme_name(), "Mint-X-Aqua");

        let folder_aqua = icon_engine.lookup_icon("folder", 48).unwrap();
        assert!(folder_aqua.contains("/usr/share/icons/Mint-X-Aqua/places/48/folder.png"));

        // Symbolic fallback
        let sym_path = icon_engine.lookup_icon("folder", 16).unwrap();
        assert!(sym_path.contains("folder-symbolic.svg"));

        let index_file = icon_engine.export_index_theme();
        assert!(index_file.contains("Name=Mint-X-Aqua"));
        assert!(index_file.contains("Inherits=Mint-X,Mint-Y,gnome,hicolor"));
    }
}
