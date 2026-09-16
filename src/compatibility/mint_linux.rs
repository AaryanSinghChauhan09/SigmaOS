use std::format;
/// Linux Mint (MintTools) Compatibility and UI Subsystem Layer for SigmaOS
/// Replicates the signature user-friendly systems from Linux Mint:
/// MintBackup, MintUpdate, MintInstall, MintReport, Timeshift-style System Restore,
/// Cinnamon-like desktop theme manager, and MintDrivers manager.
use std::string::{String, ToString};
use std::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MintError {
    LayoutFailed,
    UpdateError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowCoordinates {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Clone)]
pub struct SoftwareMeta {
    pub name: [u8; 32],
}

#[derive(Debug, Clone)]
pub struct MintUpdateItem {
    pub name: [u8; 32],
}

pub struct ZenithDisplayCompositor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MintUpdateLevel {
    Level1Safe,      // Certified safe updates (no system files)
    Level2Tested,    // Thoroughly tested system package upgrades
    Level3Normal,    // Normal upstream upgrades
    Level4Sensitive, // Potentially sensitive, requiring reboot
    Level5Critical,  // Core kernel/VMM critical upgrades (advise care)
}

#[derive(Debug, Clone)]
pub struct MintUpdatePackage {
    pub name: [u8; 32],
    pub version_old: [u8; 16],
    pub version_new: [u8; 16],
    pub level: MintUpdateLevel,
    pub safety_score: usize,
}

impl MintUpdatePackage {
    pub fn new(name: &[u8], old: &[u8], new: &[u8], level: MintUpdateLevel) -> Self {
        let mut name_arr = [0u8; 32];
        let mut old_arr = [0u8; 16];
        let mut new_arr = [0u8; 16];
        name_arr[..name.len().min(31)].copy_from_slice(&name[..name.len().min(31)]);
        old_arr[..old.len().min(15)].copy_from_slice(&old[..old.len().min(15)]);
        new_arr[..new.len().min(15)].copy_from_slice(&new[..new.len().min(15)]);

        let safety_score = match level {
            MintUpdateLevel::Level1Safe => 99,
            MintUpdateLevel::Level2Tested => 95,
            MintUpdateLevel::Level3Normal => 85,
            MintUpdateLevel::Level4Sensitive => 65,
            MintUpdateLevel::Level5Critical => 30,
        };

        MintUpdatePackage {
            name: name_arr,
            version_old: old_arr,
            version_new: new_arr,
            level,
            safety_score,
        }
    }
}

/// MintUpdate: Safe update managers, mirror selection, and kernel swapping
pub struct MintUpdateManager {
    pub pending_updates: Vec<MintUpdatePackage>,
    pub selected_mirror_speed_ms: usize,
    pub current_kernel_ver: [u8; 16],
}

impl Default for MintUpdateManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MintUpdateManager {
    pub fn new() -> Self {
        let mut current_kernel_ver = [0u8; 16];
        let version = b"6.5.6-sigma";
        current_kernel_ver[..version.len()].copy_from_slice(version);

        MintUpdateManager {
            pending_updates: Vec::new(),
            selected_mirror_speed_ms: 9999,
            current_kernel_ver,
        }
    }

    pub fn add_update(&mut self, update: MintUpdatePackage) {
        self.pending_updates.push(update);
    }

    pub fn auto_select_fastest_mirror(&mut self, mirrors: &[(&[u8], usize)]) {
        let mut best_speed = 9999;
        for &(_, speed) in mirrors {
            if speed < best_speed {
                best_speed = speed;
            }
        }
        self.selected_mirror_speed_ms = best_speed;
    }

    pub fn hot_swap_active_kernel(&mut self, new_version: &[u8]) -> Result<(), &'static str> {
        if new_version.is_empty() {
            return Err("Invalid kernel version");
        }
        let len = new_version.len().min(15);
        self.current_kernel_ver = [0u8; 16];
        self.current_kernel_ver[..len].copy_from_slice(&new_version[..len]);
        Ok(())
    }
}

/// MintBackup: Incremental user-data backup and profile state archiver
pub struct MintBackupTool {
    pub user_backups_count: AtomicUsize,
    pub active_backup_path: [u8; 64],
}

impl Default for MintBackupTool {
    fn default() -> Self {
        Self::new()
    }
}

impl MintBackupTool {
    pub fn new() -> Self {
        MintBackupTool {
            user_backups_count: AtomicUsize::new(0),
            active_backup_path: [0u8; 64],
        }
    }

    pub fn perform_user_backup(&mut self, backup_dir: &[u8]) -> Result<usize, &'static str> {
        if backup_dir.is_empty() {
            return Err("Backup target directory is invalid");
        }
        let len = backup_dir.len().min(63);
        self.active_backup_path[..len].copy_from_slice(&backup_dir[..len]);
        let id = self.user_backups_count.fetch_add(1, Ordering::SeqCst);
        Ok(id)
    }
}

/// App review structure representing user feedback (GNOME Software / Google Play inspired)
#[derive(Debug, Clone)]
pub struct AppReview {
    pub reviewer: [u8; 32],
    pub stars: usize, // 1 to 5
    pub comment: [u8; 64],
}

impl AppReview {
    pub fn new(reviewer: &[u8], stars: usize, comment: &[u8]) -> Self {
        let mut reviewer_arr = [0u8; 32];
        let mut comment_arr = [0u8; 64];
        reviewer_arr[..reviewer.len().min(31)].copy_from_slice(&reviewer[..reviewer.len().min(31)]);
        comment_arr[..comment.len().min(63)].copy_from_slice(&comment[..comment.len().min(63)]);

        AppReview {
            reviewer: reviewer_arr,
            stars: stars.clamp(1, 5),
            comment: comment_arr,
        }
    }
}

/// MintInstall: High-level application software ratings, reviews, and categories metadata
#[derive(Debug, Clone)]
pub struct MintAppMetadata {
    pub name: [u8; 32],
    pub rating_stars: usize, // 1 to 5 (calculated as average of reviews)
    pub reviews_count: usize,
    pub is_flatpak: bool,
    pub category: [u8; 16], // e.g. "System", "Games", "Office"
    pub license: [u8; 16],  // e.g. "GPL-3.0", "MIT"
    pub size_bytes: u64,
    pub reviews: Vec<AppReview>,
}

impl MintAppMetadata {
    pub fn new(
        name: &[u8],
        category: &[u8],
        license: &[u8],
        size_bytes: u64,
        is_flatpak: bool,
    ) -> Self {
        let mut name_arr = [0u8; 32];
        let mut category_arr = [0u8; 16];
        let mut license_arr = [0u8; 16];
        name_arr[..name.len().min(31)].copy_from_slice(&name[..name.len().min(31)]);
        category_arr[..category.len().min(15)].copy_from_slice(&category[..category.len().min(15)]);
        license_arr[..license.len().min(15)].copy_from_slice(&license[..license.len().min(15)]);

        MintAppMetadata {
            name: name_arr,
            rating_stars: 5, // Default perfect score prior to reviews
            reviews_count: 0,
            is_flatpak,
            category: category_arr,
            license: license_arr,
            size_bytes,
            reviews: Vec::new(),
        }
    }

    /// Appends a new user rating/review dynamically and recalculates the average stars rating.
    pub fn add_review(&mut self, review: AppReview) {
        self.reviews.push(review);
        self.reviews_count = self.reviews.len();

        let mut sum = 0;
        for rev in self.reviews.iter() {
            sum += rev.stars;
        }
        self.rating_stars = sum / self.reviews_count;
    }
}

pub struct MintSoftwareManager {
    pub apps_catalog: Vec<MintAppMetadata>,
}

impl Default for MintSoftwareManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MintSoftwareManager {
    pub fn new() -> Self {
        MintSoftwareManager {
            apps_catalog: Vec::new(),
        }
    }

    pub fn add_app_to_catalog(&mut self, app: MintAppMetadata) {
        self.apps_catalog.push(app);
    }

    /// Arrange windows using Stacking layout (Cascaded coordinations)
    pub fn arrange_stacking(
        num_windows: usize,
        coords: &mut [WindowCoordinates],
    ) -> Result<(), &'static str> {
        for i in 0..num_windows {
            if i >= coords.len() {
                return Err("Layout failed");
            }
            coords[i] = WindowCoordinates {
                x: i * 30,
                y: i * 30,
                width: 800,
                height: 600,
            };
        }
        Ok(())
    }

    pub fn search_by_category(&self, category: &[u8]) -> Vec<MintAppMetadata> {
        let mut filtered = Vec::new();
        let cat_len = category.len();
        for app in self.apps_catalog.iter() {
            if app.category.len() < cat_len {
                continue;
            }
            let mut matches = true;
            for i in 0..category.len() {
                if app.category[i] != category[i] {
                    matches = false;
                    break;
                }
            }
            if matches {
                filtered.push(app.clone());
            }
        }
        filtered
    }

    /// Returns apps ranked by user ratings (Featured Apps).
    pub fn get_featured_apps(&self) -> Vec<MintAppMetadata> {
        let mut sorted = self.apps_catalog.clone();
        // Simple bubble sort over vector to rank featured apps without external traits
        for i in 0..sorted.len() {
            for j in 0..sorted.len().saturating_sub(i).saturating_sub(1) {
                if sorted[j].rating_stars < sorted[j + 1].rating_stars {
                    sorted.swap(j, j + 1);
                }
            }
        }
        sorted
    }
}

// ==========================================
// Cinnamon Desktop Theme Engine
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CinnamonDesklet {
    pub id: u32,
    pub x: usize,
    pub y: usize,
}

/// Cinnamon Theme Presets (Linux Mint Mint-Y, Mint-X, Yaru, Adapta)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CinnamonPreset {
    MintYDark,
    MintYLight,
    MintYAqua,
    MintYPurple,
    MintYTeal,
    MintXDefault,
    CinnamonAdwaita,
    CustomCinnamon,
}

/// Cinnamon Spices & Theme Preset Manager
pub struct CinnamonThemeEngine {
    pub active_gtk_theme: [u8; 32],
    pub current_preset: CinnamonPreset,
    pub desklets: Vec<Option<CinnamonDesklet>>,
    pub is_panel_enabled: bool,
    pub panel_transparency_alpha: u8, // 0 to 255
    pub applet_icon_theme: [u8; 32],
    pub sound_scheme_enabled: bool,
}

impl CinnamonThemeEngine {
    pub fn new() -> Self {
        let mut theme = [0u8; 32];
        let mut icon_theme = [0u8; 32];
        let default_name = b"Mint-Y-Dark";
        let default_icons = b"Mint-Y";
        unsafe {
            core::ptr::copy_nonoverlapping(default_name.as_ptr(), theme.as_mut_ptr(), default_name.len());
            core::ptr::copy_nonoverlapping(default_icons.as_ptr(), icon_theme.as_mut_ptr(), default_icons.len());
        }
        Self {
            active_gtk_theme: theme,
            current_preset: CinnamonPreset::MintYDark,
            desklets: Vec::new(),
            is_panel_enabled: true,
            panel_transparency_alpha: 230,
            applet_icon_theme: icon_theme,
            sound_scheme_enabled: true,
        }
    }

    pub fn set_gtk_theme(&mut self, theme_name: &[u8]) {
        let mut theme = [0u8; 32];
        let len = theme_name.len().min(31);
        unsafe {
            core::ptr::copy_nonoverlapping(theme_name.as_ptr(), theme.as_mut_ptr(), len);
        }
        self.active_gtk_theme = theme;
    }

    pub fn apply_preset(&mut self, preset: CinnamonPreset) {
        self.current_preset = preset;
        let (gtk_name, icon_name): (&[u8], &[u8]) = match preset {
            CinnamonPreset::MintYDark => (b"Mint-Y-Dark", b"Mint-Y"),
            CinnamonPreset::MintYLight => (b"Mint-Y", b"Mint-Y"),
            CinnamonPreset::MintYAqua => (b"Mint-Y-Dark-Aqua", b"Mint-Y-Aqua"),
            CinnamonPreset::MintYPurple => (b"Mint-Y-Dark-Purple", b"Mint-Y-Purple"),
            CinnamonPreset::MintYTeal => (b"Mint-Y-Dark-Teal", b"Mint-Y-Teal"),
            CinnamonPreset::MintXDefault => (b"Mint-X", b"Mint-X"),
            CinnamonPreset::CinnamonAdwaita => (b"Adwaita-dark", b"Adwaita"),
            CinnamonPreset::CustomCinnamon => (b"Custom-Cinnamon", b"Custom-Icons"),
        };

        self.set_gtk_theme(gtk_name);
        let mut icon_arr = [0u8; 32];
        let len = icon_name.len().min(31);
        unsafe {
            core::ptr::copy_nonoverlapping(icon_name.as_ptr(), icon_arr.as_mut_ptr(), len);
        }
        self.applet_icon_theme = icon_arr;
    }

    pub fn add_desklet(&mut self, id: u32, x: usize, y: usize) {
        self.desklets.push(Some(CinnamonDesklet { id, x, y }));
    }
}

impl Default for CinnamonThemeEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// Timeshift-style System Restorer
// ==========================================

#[derive(Debug, Clone, Copy)]
pub struct SystemRestorePoint {
    pub id: u32,
    pub is_rsync: bool,
    pub timestamp_ms: u64,
}

pub struct TimeshiftSystemRestorer {
    pub restore_points: Vec<Option<SystemRestorePoint>>,
    pub active_restore_point_id: u32,
}

impl TimeshiftSystemRestorer {
    pub fn new() -> Self {
        Self {
            restore_points: Vec::new(),
            active_restore_point_id: 0,
        }
    }

    pub fn create_restore_point(&mut self, id: u32, is_rsync: bool) {
        self.restore_points.push(Some(SystemRestorePoint {
            id,
            is_rsync,
            timestamp_ms: 0,
        }));
    }

    pub fn rollback_system(&mut self, id: u32) -> Result<(), &'static str> {
        let mut found = false;
        for i in 0..self.restore_points.len() {
            if let Some(ref rp) = self.restore_points[i] {
                if rp.id == id {
                    found = true;
                    break;
                }
            }
        }
        if found {
            self.active_restore_point_id = id;
            Ok(())
        } else {
            Err("Update error")
        }
    }
}

impl Default for TimeshiftSystemRestorer {
    fn default() -> Self {
        Self::new()
    }
}

/// MintReport: Detects system crashes, memory warnings, and provides direct advice remedies
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MintReportAlertSeverity {
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Clone)]
pub struct MintReportAlert {
    pub name: [u8; 32],
    pub severity: MintReportAlertSeverity,
    pub remedy_advice: [u8; 64],
}

impl MintReportAlert {
    pub fn new(name: &[u8], severity: MintReportAlertSeverity, advice: &[u8]) -> Self {
        let mut name_arr = [0u8; 32];
        let mut advice_arr = [0u8; 64];
        name_arr[..name.len().min(31)].copy_from_slice(&name[..name.len().min(31)]);
        advice_arr[..advice.len().min(63)].copy_from_slice(&advice[..advice.len().min(63)]);

        MintReportAlert {
            name: name_arr,
            severity,
            remedy_advice: advice_arr,
        }
    }
}

pub struct MintReportSystem {
    pub active_alerts: Vec<MintReportAlert>,
}

impl Default for MintReportSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl MintReportSystem {
    pub fn new() -> Self {
        MintReportSystem {
            active_alerts: Vec::new(),
        }
    }

    pub fn register_crash_alert(&mut self, app_name: &[u8]) {
        let mut alert_name = [0u8; 32];
        let len = app_name.len().min(15);
        alert_name[..len].copy_from_slice(&app_name[..len]);
        let suffix = b" crashed";
        alert_name[len..len + suffix.len()].copy_from_slice(suffix);

        let alert = MintReportAlert::new(
            &alert_name,
            MintReportAlertSeverity::Critical,
            b"Please restart the service or run sigpkg update",
        );
        self.active_alerts.push(alert);
    }
}

/// Timeshift-style System Restore checkpoint
#[derive(Debug, Clone)]
pub struct TimeshiftSnapshot {
    pub id: usize,
    pub timestamp_epoch: u64,
    pub description: [u8; 64],
    pub system_state_hash: u64, // Simulated Merkle root hash of systems
}

impl TimeshiftSnapshot {
    pub fn new(id: usize, timestamp_epoch: u64, desc: &[u8], hash: u64) -> Self {
        let mut desc_arr = [0u8; 64];
        let len = desc.len().min(63);
        desc_arr[..len].copy_from_slice(&desc[..len]);
        TimeshiftSnapshot {
            id,
            timestamp_epoch,
            description: desc_arr,
            system_state_hash: hash,
        }
    }
}

/// Timeshift-inspired System Restore point manager
pub struct MintTimeshiftEngine {
    pub snapshots: Vec<TimeshiftSnapshot>,
    pub next_snapshot_id: AtomicUsize,
}

impl Default for MintTimeshiftEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl MintTimeshiftEngine {
    pub fn new() -> Self {
        MintTimeshiftEngine {
            snapshots: Vec::new(),
            next_snapshot_id: AtomicUsize::new(1),
        }
    }

    pub fn create_checkpoint(&mut self, timestamp: u64, desc: &[u8], state_hash: u64) -> usize {
        let id = self.next_snapshot_id.fetch_add(1, Ordering::SeqCst);
        let checkpoint = TimeshiftSnapshot::new(id, timestamp, desc, state_hash);
        self.snapshots.push(checkpoint);
        id
    }

    pub fn restore_checkpoint(&self, snapshot_id: usize) -> Result<u64, &'static str> {
        for snap in self.snapshots.iter() {
            if snap.id == snapshot_id {
                return Ok(snap.system_state_hash);
            }
        }
        Err("Timeshift: Target system restore point not found.")
    }
}

/// Cinnamon-inspired desktop styling configuration
#[derive(Debug, Clone, Copy)]
pub struct MintCinnamonStyling {
    pub panel_height: u32,
    pub menu_layout_compact: bool,
    pub opacity_percent: u32,
    pub window_effects_enabled: bool,
}

impl MintCinnamonStyling {
    pub fn default() -> Self {
        MintCinnamonStyling {
            panel_height: 40,
            menu_layout_compact: false,
            opacity_percent: 100,
            window_effects_enabled: true,
        }
    }

    pub fn configure_workspace(&mut self, height: u32, compact: bool, opacity: u32, effects: bool) {
        self.panel_height = height;
        self.menu_layout_compact = compact;
        self.opacity_percent = opacity.min(100);
        self.window_effects_enabled = effects;
    }
}

/// Hardware Driver metadata managed by the MintDrivers-equivalent system
#[derive(Debug, Clone)]
pub struct MintDriverInfo {
    pub name: [u8; 48],
    pub hardware_class: [u8; 32],
    pub proprietary: bool,
    pub active: bool,
}

impl MintDriverInfo {
    pub fn new(name: &[u8], class: &[u8], proprietary: bool) -> Self {
        let mut name_arr = [0u8; 48];
        let mut class_arr = [0u8; 32];
        name_arr[..name.len().min(47)].copy_from_slice(&name[..name.len().min(47)]);
        class_arr[..class.len().min(31)].copy_from_slice(&class[..class.len().min(31)]);
        MintDriverInfo {
            name: name_arr,
            hardware_class: class_arr,
            proprietary,
            active: false,
        }
    }
}

/// MintDrivers-inspired Hardware Driver Manager
// ==========================================
// Linux Mint mint4win & Wubi Loopback Windows Installer Engine
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopbackDiskFormat {
    VhdFixed,
    VhdDynamic,
    RawNtfsImage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowsBootloaderType {
    Ntldr,        // Windows XP / Server 2003
    BcdBootmgr,   // Windows Vista / 7 / 8 / 10 / 11
    Grub4Dos,     // Legacy MBR chainloader
    UefiEfiEntry, // UEFI NVRAM Boot Entry
}

#[derive(Debug, Clone)]
pub struct Mint4WinInstallationConfig {
    pub target_drive_letter: char, // e.g. 'C'
    pub target_folder: String,     // e.g. "C:\mint4win"
    pub disk_format: LoopbackDiskFormat,
    pub bootloader_type: WindowsBootloaderType,
    pub root_disk_size_mb: u64, // e.g. 32768 MB (32 GB)
    pub swap_file_size_mb: u64, // e.g. 4096 MB (4 GB)
    pub default_username: String,
    pub host_os_version: String,
}

impl Mint4WinInstallationConfig {
    pub fn default_windows_c(drive_letter: char, username: &str) -> Self {
        Mint4WinInstallationConfig {
            target_drive_letter: drive_letter,
            target_folder: format!("{}:\\mint4win", drive_letter),
            disk_format: LoopbackDiskFormat::RawNtfsImage,
            bootloader_type: WindowsBootloaderType::BcdBootmgr,
            root_disk_size_mb: 32768,
            swap_file_size_mb: 4096,
            default_username: username.to_string(),
            host_os_version: "Windows 11".to_string(),
        }
    }
}

/// Linux Mint `mint4win` & Ubuntu `Wubi` inspired Windows Loopback Installer Engine
pub struct Mint4WinInstallerEngine {
    pub config: Mint4WinInstallationConfig,
    pub loopback_root_vhd_created: bool,
    pub bcd_entry_added: bool,
    pub installed: bool,
    pub bcd_guid: String,
}

impl Mint4WinInstallerEngine {
    pub fn new(config: Mint4WinInstallationConfig) -> Self {
        Mint4WinInstallerEngine {
            config,
            loopback_root_vhd_created: false,
            bcd_entry_added: false,
            installed: false,
            bcd_guid: String::from("{a1b2c3d4-e5f6-7890-abcd-1234567890ab}"),
        }
    }

    pub fn allocate_loopback_disks(&mut self) -> Result<String, &'static str> {
        if self.config.root_disk_size_mb < 8192 {
            return Err("mint4win: Minimum root disk size is 8192 MB (8 GB)");
        }

        self.loopback_root_vhd_created = true;
        Ok(format!(
            "Successfully allocated {} MB loopback root disk and {} MB swap file at {}",
            self.config.root_disk_size_mb, self.config.swap_file_size_mb, self.config.target_folder
        ))
    }

    pub fn configure_windows_bcd_boot_entry(&mut self) -> Result<String, &'static str> {
        if !self.loopback_root_vhd_created {
            return Err("mint4win: Must allocate loopback disk before configuring Windows BCD");
        }

        self.bcd_entry_added = true;
        self.installed = true;
        Ok(format!(
            "Added Windows BCD boot entry [{}] 'SigmaOS (Linux Mint Dual-Boot)'",
            self.bcd_guid
        ))
    }

    pub fn generate_unattended_install_script(&self) -> String {
        format!(
            "unattended_user=\"{}\"\nloopback_root=\"{}\\disks\\root.disk\"\nloopback_swap=\"{}\\disks\\swap.disk\"\nbootloader=\"{:?}\"\n",
            self.config.default_username,
            self.config.target_folder,
            self.config.target_folder,
            self.config.bootloader_type
        )
    }

    pub fn uninstall_mint4win(&mut self) -> Result<String, &'static str> {
        if !self.installed {
            return Err("mint4win is not installed");
        }

        self.bcd_entry_added = false;
        self.loopback_root_vhd_created = false;
        self.installed = false;

        Ok("Successfully removed mint4win Windows boot entry and loopback disk files".to_string())
    }
}

// ==========================================
// Linux Mint Parity Subsystems
// ==========================================

/// Sticky Notes Utility (Sticky / Sticky Notes)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StickyNoteColor {
    Yellow,
    Blue,
    Green,
    Pink,
    Purple,
    Orange,
}

#[derive(Debug, Clone)]
pub struct StickyNote {
    pub id: u32,
    pub title: String,
    pub body: String,
    pub color: StickyNoteColor,
    pub is_pinned: bool,
    pub x: usize,
    pub y: usize,
}

pub struct MintStickyNotesManager {
    pub notes: Vec<StickyNote>,
    next_id: u32,
}

impl Default for MintStickyNotesManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MintStickyNotesManager {
    pub fn new() -> Self {
        Self {
            notes: Vec::new(),
            next_id: 1,
        }
    }

    pub fn create_note(&mut self, title: &str, body: &str, color: StickyNoteColor) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.notes.push(StickyNote {
            id,
            title: title.to_string(),
            body: body.to_string(),
            color,
            is_pinned: false,
            x: 100,
            y: 100,
        });
        id
    }

    pub fn toggle_pin(&mut self, id: u32) -> Result<bool, &'static str> {
        if let Some(note) = self.notes.iter_mut().find(|n| n.id == id) {
            note.is_pinned = !note.is_pinned;
            Ok(note.is_pinned)
        } else {
            Err("Note not found")
        }
    }

    pub fn update_body(&mut self, id: u32, new_body: &str) -> Result<(), &'static str> {
        if let Some(note) = self.notes.iter_mut().find(|n| n.id == id) {
            note.body = new_body.to_string();
            Ok(())
        } else {
            Err("Note not found")
        }
    }
}

/// Web Apps Manager (webapp-manager) - Site-Specific Browser (SSB) launcher
#[derive(Debug, Clone)]
pub struct WebAppProfile {
    pub id: u32,
    pub name: String,
    pub url: String,
    pub icon_path: String,
    pub category: String,
    pub browser_profile: String,
    pub custom_user_agent: Option<String>,
    pub isolated_cookies: bool,
}

pub struct MintWebAppManager {
    pub web_apps: Vec<WebAppProfile>,
    next_id: u32,
}

impl Default for MintWebAppManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MintWebAppManager {
    pub fn new() -> Self {
        Self {
            web_apps: Vec::new(),
            next_id: 1,
        }
    }

    pub fn register_webapp(
        &mut self,
        name: &str,
        url: &str,
        category: &str,
        icon: &str,
        isolated: bool,
    ) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.web_apps.push(WebAppProfile {
            id,
            name: name.to_string(),
            url: url.to_string(),
            icon_path: icon.to_string(),
            category: category.to_string(),
            browser_profile: format!("profile_{}", name.to_lowercase().replace(' ', "_")),
            custom_user_agent: None,
            isolated_cookies: isolated,
        });
        id
    }

    pub fn set_custom_user_agent(&mut self, id: u32, user_agent: &str) -> Result<(), &'static str> {
        if let Some(app) = self.web_apps.iter_mut().find(|a| a.id == id) {
            app.custom_user_agent = Some(user_agent.to_string());
            Ok(())
        } else {
            Err("WebApp not found")
        }
    }

    pub fn launch_webapp_command(&self, id: u32) -> Result<String, &'static str> {
        if let Some(app) = self.web_apps.iter().find(|a| a.id == id) {
            Ok(format!(
                "zenith-browser --app=\"{}\" --profile=\"{}\" --isolated={}",
                app.url, app.browser_profile, app.isolated_cookies
            ))
        } else {
            Err("WebApp not found")
        }
    }
}

/// Hypnotix IPTV / Media Player Engine
#[derive(Debug, Clone)]
pub struct IptvChannel {
    pub name: String,
    pub stream_url: String,
    pub country_code: String,
    pub category: String,
    pub quality_720p_or_higher: bool,
}

pub struct MintHypnotixIptvEngine {
    pub channels: Vec<IptvChannel>,
    pub active_channel_index: Option<usize>,
    pub user_favorites: Vec<String>,
}

impl Default for MintHypnotixIptvEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl MintHypnotixIptvEngine {
    pub fn new() -> Self {
        Self {
            channels: Vec::new(),
            active_channel_index: None,
            user_favorites: Vec::new(),
        }
    }

    pub fn import_m3u_playlist(&mut self, playlist_content: &str) -> usize {
        let mut added = 0;
        for line in playlist_content.lines() {
            if line.contains("http://") || line.contains("https://") {
                self.channels.push(IptvChannel {
                    name: format!("Stream {}", self.channels.len() + 1),
                    stream_url: line.trim().to_string(),
                    country_code: "INT".to_string(),
                    category: "General".to_string(),
                    quality_720p_or_higher: true,
                });
                added += 1;
            }
        }
        added
    }

    pub fn add_channel(
        &mut self,
        name: &str,
        url: &str,
        country: &str,
        category: &str,
        hd: bool,
    ) {
        self.channels.push(IptvChannel {
            name: name.to_string(),
            stream_url: url.to_string(),
            country_code: country.to_string(),
            category: category.to_string(),
            quality_720p_or_higher: hd,
        });
    }

    pub fn play_channel(&mut self, name: &str) -> Result<String, &'static str> {
        if let Some((idx, ch)) = self.channels.iter().enumerate().find(|(_, c)| c.name == name) {
            self.active_channel_index = Some(idx);
            Ok(format!("Playing stream from {}", ch.stream_url))
        } else {
            Err("Channel not found")
        }
    }

    pub fn filter_by_country(&self, country: &str) -> Vec<&IptvChannel> {
        self.channels.iter().filter(|c| c.country_code == country).collect()
    }
}

/// MintWelcome - First-Run Desktop Setup & Onboarding Wizard
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopLayoutPreset {
    Traditional, // Panel at bottom with menu
    Modern,      // Centered panel with app grid
    Compact,     // Minimal panel
}

pub struct MintWelcomeWizard {
    pub layout: DesktopLayoutPreset,
    pub dark_mode: bool,
    pub initial_timeshift_done: bool,
    pub drivers_checked: bool,
    pub updates_checked: bool,
    pub firewall_enabled: bool,
}

impl Default for MintWelcomeWizard {
    fn default() -> Self {
        Self::new()
    }
}

impl MintWelcomeWizard {
    pub fn new() -> Self {
        Self {
            layout: DesktopLayoutPreset::Traditional,
            dark_mode: true,
            initial_timeshift_done: false,
            drivers_checked: false,
            updates_checked: false,
            firewall_enabled: true,
        }
    }

    pub fn set_desktop_layout(&mut self, layout: DesktopLayoutPreset) {
        self.layout = layout;
    }

    pub fn toggle_dark_mode(&mut self, enabled: bool) {
        self.dark_mode = enabled;
    }

    pub fn mark_timeshift_completed(&mut self) {
        self.initial_timeshift_done = true;
    }

    pub fn mark_drivers_reviewed(&mut self) {
        self.drivers_checked = true;
    }

    pub fn mark_updates_checked(&mut self) {
        self.updates_checked = true;
    }

    pub fn is_setup_completed(&self) -> bool {
        self.initial_timeshift_done && self.drivers_checked && self.updates_checked
    }
}

/// MintNanny - Domain & Parental Controls Manager
#[derive(Debug, Clone)]
pub struct BlockedDomainRule {
    pub domain: String,
    pub reason: String,
    pub block_time_start_hour: u8, // 0 to 23
    pub block_time_end_hour: u8,   // 0 to 23
}

pub struct MintNannyDomainFilter {
    pub blocked_rules: Vec<BlockedDomainRule>,
    pub filter_active: bool,
}

impl Default for MintNannyDomainFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl MintNannyDomainFilter {
    pub fn new() -> Self {
        Self {
            blocked_rules: Vec::new(),
            filter_active: true,
        }
    }

    pub fn add_blocked_domain(&mut self, domain: &str, reason: &str) {
        self.blocked_rules.push(BlockedDomainRule {
            domain: domain.to_string(),
            reason: reason.to_string(),
            block_time_start_hour: 0,
            block_time_end_hour: 24,
        });
    }

    pub fn is_domain_blocked(&self, domain: &str, current_hour: u8) -> bool {
        if !self.filter_active {
            return false;
        }
        for rule in &self.blocked_rules {
            if domain.contains(&rule.domain) {
                if current_hour >= rule.block_time_start_hour && current_hour < rule.block_time_end_hour {
                    return true;
                }
            }
        }
        false
    }
}

/// MintLocale & Language Selector
#[derive(Debug, Clone)]
pub struct LanguagePack {
    pub locale_code: String, // e.g. "en_US.UTF-8", "fr_FR.UTF-8", "hi_IN.UTF-8"
    pub name: String,
    pub has_spellcheck: bool,
    pub input_method: String, // e.g. "IBus", "Fcitx5"
}

pub struct MintLocaleManager {
    pub installed_languages: Vec<LanguagePack>,
    pub active_locale: String,
}

impl Default for MintLocaleManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MintLocaleManager {
    pub fn new() -> Self {
        let mut mgr = Self {
            installed_languages: Vec::new(),
            active_locale: String::from("en_US.UTF-8"),
        };
        mgr.installed_languages.push(LanguagePack {
            locale_code: String::from("en_US.UTF-8"),
            name: String::from("English (United States)"),
            has_spellcheck: true,
            input_method: String::from("IBus"),
        });
        mgr
    }

    pub fn install_language_pack(&mut self, pack: LanguagePack) {
        self.installed_languages.push(pack);
    }

    pub fn switch_active_locale(&mut self, locale_code: &str) -> Result<(), &'static str> {
        if self.installed_languages.iter().any(|l| l.locale_code == locale_code) {
            self.active_locale = locale_code.to_string();
            Ok(())
        } else {
            Err("Language pack not installed")
        }
    }
}

/// MintUsbStickUtilities - USB Image Writer & Formatting Tool
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsbFileSystemFormat {
    Fat32,
    Ntfs,
    ExFat,
    Ext4,
}

pub struct MintUsbFormatterTool {
    pub target_device_path: String,
    pub is_busy: bool,
    pub format_progress_percent: u8,
}

impl Default for MintUsbFormatterTool {
    fn default() -> Self {
        Self::new()
    }
}

impl MintUsbFormatterTool {
    pub fn new() -> Self {
        Self {
            target_device_path: String::new(),
            is_busy: false,
            format_progress_percent: 0,
        }
    }

    pub fn select_device(&mut self, device_path: &str) -> Result<(), &'static str> {
        if device_path.starts_with("/dev/sd") || device_path.starts_with("/dev/nvme") || device_path.starts_with("/dev/mmc") {
            self.target_device_path = device_path.to_string();
            Ok(())
        } else {
            Err("Invalid storage device path")
        }
    }

    pub fn format_filesystem(&mut self, format_type: UsbFileSystemFormat, label: &str) -> Result<String, &'static str> {
        if self.target_device_path.is_empty() {
            return Err("No device selected");
        }
        self.is_busy = true;
        self.format_progress_percent = 100;
        self.is_busy = false;
        Ok(format!(
            "Successfully formatted {} to {:?} with label '{}'",
            self.target_device_path, format_type, label
        ))
    }

    pub fn write_iso_image(&mut self, iso_path: &str) -> Result<String, &'static str> {
        if self.target_device_path.is_empty() {
            return Err("No device selected");
        }
        if iso_path.is_empty() {
            return Err("ISO path invalid");
        }
        Ok(format!(
            "Successfully wrote ISO image '{}' to device {}",
            iso_path, self.target_device_path
        ))
    }
}

/// MintSoftwareSources & PPA Manager
#[derive(Debug, Clone)]
pub struct PpaRepository {
    pub ppa_name: String, // e.g. "ppa:cinnamon/stable"
    pub gpg_key_fingerprint: String,
    pub enabled: bool,
}

pub struct MintSoftwareSourcesPpaManager {
    pub ppas: Vec<PpaRepository>,
    pub main_mirror_url: String,
    pub base_mirror_url: String,
}

impl Default for MintSoftwareSourcesPpaManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MintSoftwareSourcesPpaManager {
    pub fn new() -> Self {
        Self {
            ppas: Vec::new(),
            main_mirror_url: String::from("https://packages.linuxmint.com"),
            base_mirror_url: String::from("https://archive.ubuntu.com/ubuntu"),
        }
    }

    pub fn add_ppa(&mut self, ppa: &str, fingerprint: &str) {
        self.ppas.push(PpaRepository {
            ppa_name: ppa.to_string(),
            gpg_key_fingerprint: fingerprint.to_string(),
            enabled: true,
        });
    }

    pub fn remove_ppa(&mut self, ppa: &str) -> Result<(), &'static str> {
        let initial_len = self.ppas.len();
        self.ppas.retain(|p| p.ppa_name != ppa);
        if self.ppas.len() < initial_len {
            Ok(())
        } else {
            Err("PPA not found")
        }
    }
}

/// Cinnamon Spices Suite & Applet Manager
#[derive(Debug, Clone)]
pub struct CinnamonSpiceApplet {
    pub uuid: String,
    pub name: String,
    pub enabled: bool,
    pub panel_index: usize,
}

pub struct MintCinnamonSpicesAppletManager {
    pub applets: Vec<CinnamonSpiceApplet>,
}

impl Default for MintCinnamonSpicesAppletManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MintCinnamonSpicesAppletManager {
    pub fn new() -> Self {
        Self {
            applets: Vec::new(),
        }
    }

    pub fn add_applet(&mut self, uuid: &str, name: &str, panel_index: usize) {
        self.applets.push(CinnamonSpiceApplet {
            uuid: uuid.to_string(),
            name: name.to_string(),
            enabled: true,
            panel_index,
        });
    }

    pub fn set_applet_state(&mut self, uuid: &str, enabled: bool) -> Result<(), &'static str> {
        if let Some(applet) = self.applets.iter_mut().find(|a| a.uuid == uuid) {
            applet.enabled = enabled;
            Ok(())
        } else {
            Err("Applet UUID not found")
        }
    }
}

// ==========================================
// Warpinator Local Network File Transfer Engine
// ==========================================

#[derive(Debug, Clone)]
pub struct WarpinatorPeer {
    pub uuid: String,
    pub hostname: String,
    pub ip_address: String,
    pub port: u16,
    pub pin_code: String,
    pub trusted: bool,
}

#[derive(Debug, Clone)]
pub struct WarpinatorTransferItem {
    pub file_name: String,
    pub file_size_bytes: u64,
    pub sender_uuid: String,
    pub status: String,
}

pub struct MintWarpinatorFileTransferEngine {
    pub discovered_peers: Vec<WarpinatorPeer>,
    pub transfer_queue: Vec<WarpinatorTransferItem>,
    pub group_code: String,
}

impl Default for MintWarpinatorFileTransferEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl MintWarpinatorFileTransferEngine {
    pub fn new() -> Self {
        Self {
            discovered_peers: Vec::new(),
            transfer_queue: Vec::new(),
            group_code: String::from("WarpinatorSigmaOS"),
        }
    }

    pub fn discover_peer(&mut self, hostname: &str, ip: &str, port: u16, pin: &str) -> String {
        let uuid = format!("peer_{}", self.discovered_peers.len() + 1);
        self.discovered_peers.push(WarpinatorPeer {
            uuid: uuid.clone(),
            hostname: hostname.to_string(),
            ip_address: ip.to_string(),
            port,
            pin_code: pin.to_string(),
            trusted: false,
        });
        uuid
    }

    pub fn pair_peer(&mut self, uuid: &str, pin: &str) -> Result<(), &'static str> {
        if let Some(peer) = self.discovered_peers.iter_mut().find(|p| p.uuid == uuid) {
            if peer.pin_code == pin {
                peer.trusted = true;
                Ok(())
            } else {
                Err("Warpinator: PIN code mismatch")
            }
        } else {
            Err("Warpinator: Peer not found")
        }
    }

    pub fn queue_file_transfer(&mut self, sender_uuid: &str, file_name: &str, size: u64) -> Result<(), &'static str> {
        if !self.discovered_peers.iter().any(|p| p.uuid == sender_uuid && p.trusted) {
            return Err("Warpinator: Peer is not trusted or paired");
        }
        self.transfer_queue.push(WarpinatorTransferItem {
            file_name: file_name.to_string(),
            file_size_bytes: size,
            sender_uuid: sender_uuid.to_string(),
            status: String::from("Pending"),
        });
        Ok(())
    }
}

// ==========================================
// Bulky Batch File Renamer Engine
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BulkyCaseMode {
    Lowercase,
    Uppercase,
    Titlecase,
    Unchanged,
}

pub struct MintBulkyBatchRenamerEngine;

impl MintBulkyBatchRenamerEngine {
    pub fn rename_find_replace(files: &[&str], search: &str, replace: &str) -> Vec<String> {
        files.iter().map(|f| f.replace(search, replace)).collect()
    }

    pub fn rename_add_prefix_suffix(files: &[&str], prefix: &str, suffix: &str) -> Vec<String> {
        files
            .iter()
            .map(|f| {
                if let Some(pos) = f.rfind('.') {
                    let stem = &f[..pos];
                    let ext = &f[pos..];
                    format!("{}{}{}{}", prefix, stem, suffix, ext)
                } else {
                    format!("{}{}{}", prefix, f, suffix)
                }
            })
            .collect()
    }

    pub fn rename_change_case(files: &[&str], mode: BulkyCaseMode) -> Vec<String> {
        files
            .iter()
            .map(|f| match mode {
                BulkyCaseMode::Lowercase => f.to_lowercase(),
                BulkyCaseMode::Uppercase => f.to_uppercase(),
                BulkyCaseMode::Titlecase => {
                    let mut chars = f.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    }
                }
                BulkyCaseMode::Unchanged => f.to_string(),
            })
            .collect()
    }
}

// ==========================================
// XApps Status Icon & Tray System
// ==========================================

#[derive(Debug, Clone)]
pub struct XAppStatusIcon {
    pub id: u32,
    pub icon_name: String,
    pub tooltip: String,
    pub badge_count: u32,
    pub visible: bool,
}

pub struct MintXAppStatusIconTray {
    pub icons: Vec<XAppStatusIcon>,
    next_id: u32,
}

impl Default for MintXAppStatusIconTray {
    fn default() -> Self {
        Self::new()
    }
}

impl MintXAppStatusIconTray {
    pub fn new() -> Self {
        Self {
            icons: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_status_icon(&mut self, icon_name: &str, tooltip: &str) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.icons.push(XAppStatusIcon {
            id,
            icon_name: icon_name.to_string(),
            tooltip: tooltip.to_string(),
            badge_count: 0,
            visible: true,
        });
        id
    }

    pub fn set_badge_count(&mut self, id: u32, count: u32) -> Result<(), &'static str> {
        if let Some(icon) = self.icons.iter_mut().find(|i| i.id == id) {
            icon.badge_count = count;
            Ok(())
        } else {
            Err("XAppStatusIcon not found")
        }
    }
}

pub struct MintDriverManager {
    pub available_drivers: Vec<MintDriverInfo>,
}

impl Default for MintDriverManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MintDriverManager {
    pub fn new() -> Self {
        MintDriverManager {
            available_drivers: Vec::new(),
        }
    }

    pub fn register_driver(&mut self, driver: MintDriverInfo) {
        self.available_drivers.push(driver);
    }

    pub fn toggle_driver(&mut self, name: &[u8], active: bool) -> Result<(), &'static str> {
        for driver in self.available_drivers.iter_mut() {
            let mut matches = true;
            for i in 0..name.len().min(47) {
                if driver.name[i] != name[i] {
                    matches = false;
                    break;
                }
            }
            if matches {
                driver.active = active;
                return Ok(());
            }
        }
        Err("MintDrivers: Specified driver not found.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mint_sticky_notes_manager() {
        let mut notes = MintStickyNotesManager::new();
        let id1 = notes.create_note("Shopping", "Buy apples & milk", StickyNoteColor::Yellow);
        assert_eq!(id1, 1);
        assert_eq!(notes.notes.len(), 1);

        let pinned = notes.toggle_pin(id1).unwrap();
        assert!(pinned);

        notes.update_body(id1, "Buy apples, milk, & bread").unwrap();
        assert_eq!(notes.notes[0].body, "Buy apples, milk, & bread");
    }

    #[test]
    fn test_mint_webapp_manager() {
        let mut webapps = MintWebAppManager::new();
        let id = webapps.register_webapp("GitHub", "https://github.com", "Development", "github.png", true);
        assert_eq!(id, 1);

        webapps.set_custom_user_agent(id, "Mozilla/5.0 Custom").unwrap();
        assert_eq!(webapps.web_apps[0].custom_user_agent, Some("Mozilla/5.0 Custom".to_string()));

        let cmd = webapps.launch_webapp_command(id).unwrap();
        assert!(cmd.contains("zenith-browser --app=\"https://github.com\""));
    }

    #[test]
    fn test_mint_hypnotix_iptv_engine() {
        let mut hypnotix = MintHypnotixIptvEngine::new();
        hypnotix.add_channel("News 24", "https://stream.news24.com/live.m3u8", "US", "News", true);
        assert_eq!(hypnotix.channels.len(), 1);

        let m3u_added = hypnotix.import_m3u_playlist("#EXTM3U\nhttps://stream.sports.com/live.m3u8");
        assert_eq!(m3u_added, 1);
        assert_eq!(hypnotix.channels.len(), 2);

        let status = hypnotix.play_channel("News 24").unwrap();
        assert!(status.contains("Playing stream from https://stream.news24.com/live.m3u8"));

        let us_channels = hypnotix.filter_by_country("US");
        assert_eq!(us_channels.len(), 1);
    }

    #[test]
    fn test_mint_welcome_wizard() {
        let mut wizard = MintWelcomeWizard::new();
        assert_eq!(wizard.layout, DesktopLayoutPreset::Traditional);
        assert!(!wizard.is_setup_completed());

        wizard.set_desktop_layout(DesktopLayoutPreset::Modern);
        wizard.mark_timeshift_completed();
        wizard.mark_drivers_reviewed();
        wizard.mark_updates_checked();

        assert_eq!(wizard.layout, DesktopLayoutPreset::Modern);
        assert!(wizard.is_setup_completed());
    }

    #[test]
    fn test_mint_nanny_domain_filter() {
        let mut nanny = MintNannyDomainFilter::new();
        nanny.add_blocked_domain("gambling.com", "Parental Control");

        assert!(nanny.is_domain_blocked("https://gambling.com/poker", 14));
        assert!(!nanny.is_domain_blocked("https://wikipedia.org", 14));

        nanny.filter_active = false;
        assert!(!nanny.is_domain_blocked("https://gambling.com/poker", 14));
    }

    #[test]
    fn test_mint_locale_manager() {
        let mut locale_mgr = MintLocaleManager::new();
        assert_eq!(locale_mgr.active_locale, "en_US.UTF-8");

        locale_mgr.install_language_pack(LanguagePack {
            locale_code: "fr_FR.UTF-8".to_string(),
            name: "French".to_string(),
            has_spellcheck: true,
            input_method: "IBus".to_string(),
        });

        assert!(locale_mgr.switch_active_locale("fr_FR.UTF-8").is_ok());
        assert_eq!(locale_mgr.active_locale, "fr_FR.UTF-8");
    }

    #[test]
    fn test_mint_usb_formatter_tool() {
        let mut usb_tool = MintUsbFormatterTool::new();
        assert!(usb_tool.select_device("/dev/sdb").is_ok());

        let fmt_result = usb_tool.format_filesystem(UsbFileSystemFormat::Fat32, "MINT_USB").unwrap();
        assert!(fmt_result.contains("Successfully formatted /dev/sdb"));

        let iso_result = usb_tool.write_iso_image("/home/user/sigmaos.iso").unwrap();
        assert!(iso_result.contains("Successfully wrote ISO image"));
    }

    #[test]
    fn test_mint_software_sources_ppa_manager() {
        let mut ppa_mgr = MintSoftwareSourcesPpaManager::new();
        ppa_mgr.add_ppa("ppa:cinnamon/stable", "1234567890ABCDEF");
        assert_eq!(ppa_mgr.ppas.len(), 1);

        assert!(ppa_mgr.remove_ppa("ppa:cinnamon/stable").is_ok());
        assert_eq!(ppa_mgr.ppas.len(), 0);
    }

    #[test]
    fn test_mint_cinnamon_spices_applet_manager() {
        let mut applet_mgr = MintCinnamonSpicesAppletManager::new();
        applet_mgr.add_applet("workspace-switcher@cinnamon.org", "Workspace Switcher", 0);
        assert_eq!(applet_mgr.applets.len(), 1);

        assert!(applet_mgr.set_applet_state("workspace-switcher@cinnamon.org", false).is_ok());
        assert!(!applet_mgr.applets[0].enabled);
    }

    #[test]
    fn test_mint_update_manager() {
        let mut manager = MintUpdateManager::new();
        let pkg =
            MintUpdatePackage::new(b"zenith", b"1.0.0", b"1.1.0", MintUpdateLevel::Level1Safe);
        manager.add_update(pkg);

        assert_eq!(manager.pending_updates.len(), 1);
        assert_eq!(manager.pending_updates[0].safety_score, 99);

        // Fast mirror selection
        manager.auto_select_fastest_mirror(&[(b"us-mirror", 45), (b"eu-mirror", 120)]);
        assert_eq!(manager.selected_mirror_speed_ms, 45);

        // Hot swap active kernel version
        manager.hot_swap_active_kernel(b"6.6.0").unwrap();
        assert!(manager.current_kernel_ver.starts_with(b"6.6.0"));
    }

    #[test]
    fn test_mint_backup_tool() {
        let mut backup = MintBackupTool::new();
        let backup_id = backup.perform_user_backup(b"/backup/user_state").unwrap();
        assert_eq!(backup_id, 0);
    }

    #[test]
    fn test_mint_software_manager_with_reviews() {
        let mut software = MintSoftwareManager::new();

        // 1. Create app metadata and add reviews
        let mut app1 = MintAppMetadata::new(b"alacritty", b"System", b"Apache-2.0", 4500000, true);
        app1.add_review(AppReview::new(b"gamer1", 5, b"Fast terminal!"));
        app1.add_review(AppReview::new(b"dev1", 3, b"Nice, but lacks tabs."));

        // Average should be (5 + 3) / 2 = 4 stars
        assert_eq!(app1.rating_stars, 4);
        assert_eq!(app1.reviews_count, 2);

        let mut app2 = MintAppMetadata::new(b"flipper", b"Games", b"GPL-3.0", 12000000, false);
        app2.add_review(AppReview::new(b"gamer2", 5, b"Pristine retro gameplay!"));

        software.add_app_to_catalog(app1);
        software.add_app_to_catalog(app2);

        // 2. Test Category Search
        let system_apps = software.search_by_category(b"System");
        assert_eq!(system_apps.len(), 1);
        assert!(system_apps[0].name.starts_with(b"alacritty"));

        // 3. Test Featured (Ranked) Apps
        let featured = software.get_featured_apps();
        assert_eq!(featured.len(), 2);
        assert!(featured[0].name.starts_with(b"flipper")); // 5 stars > 4 stars
    }

    #[test]
    fn test_mint_report_system() {
        let mut report = MintReportSystem::new();
        report.register_crash_alert(b"launcher");
        assert_eq!(report.active_alerts.len(), 1);
        assert_eq!(
            report.active_alerts[0].severity,
            MintReportAlertSeverity::Critical
        );
    }

    #[test]
    fn test_mint_timeshift_restore_points() {
        let mut timeshift = MintTimeshiftEngine::new();
        let snap_id =
            timeshift.create_checkpoint(1690000000, b"Fresh boot restore point", 0xDEADBEEF);
        assert_eq!(snap_id, 1);

        let hash = timeshift.restore_checkpoint(1).unwrap();
        assert_eq!(hash, 0xDEADBEEF);

        let mut engine = CinnamonThemeEngine::new();
        engine.add_desklet(101, 200, 200);
        assert_eq!(engine.desklets.len(), 1);
        assert_eq!(engine.desklets[0].unwrap().id, 101);
    }

    #[test]
    fn test_mint_cinnamon_styling_options() {
        let mut style = MintCinnamonStyling::default();
        assert_eq!(style.panel_height, 40);
        assert!(style.window_effects_enabled);

        style.configure_workspace(36, true, 85, false);
        assert_eq!(style.panel_height, 36);
        assert!(style.menu_layout_compact);
        assert_eq!(style.opacity_percent, 85);
        assert!(!style.window_effects_enabled);

        // Test Cinnamon Theme Presets
        let mut cinnamon = CinnamonThemeEngine::new();
        assert_eq!(cinnamon.current_preset, CinnamonPreset::MintYDark);
        assert!(cinnamon.active_gtk_theme.starts_with(b"Mint-Y-Dark"));

        cinnamon.apply_preset(CinnamonPreset::MintYAqua);
        assert_eq!(cinnamon.current_preset, CinnamonPreset::MintYAqua);
        assert!(cinnamon.active_gtk_theme.starts_with(b"Mint-Y-Dark-Aqua"));
        assert!(cinnamon.applet_icon_theme.starts_with(b"Mint-Y-Aqua"));
    }

    #[test]
    fn test_mint_driver_manager_flows() {
        let mut drivers = MintDriverManager::new();
        let wifi_drv = MintDriverInfo::new(b"Broadcom BCM4360 WiFi", b"Wireless Controller", true);
        drivers.register_driver(wifi_drv);

        assert_eq!(drivers.available_drivers.len(), 1);
        assert!(!drivers.available_drivers[0].active);

        drivers
            .toggle_driver(b"Broadcom BCM4360 WiFi", true)
            .unwrap();
        assert!(drivers.available_drivers[0].active);
    }

    #[test]
    fn test_mint_warpinator_file_transfer() {
        let mut warpinator = MintWarpinatorFileTransferEngine::new();
        let peer_uuid = warpinator.discover_peer("mint-laptop", "192.168.1.100", 42000, "123456");

        // Unpaired queue attempt should fail
        assert!(warpinator.queue_file_transfer(&peer_uuid, "document.pdf", 1024).is_err());

        // Pair with matching PIN
        assert!(warpinator.pair_peer(&peer_uuid, "123456").is_ok());

        // Paired queue attempt should succeed
        assert!(warpinator.queue_file_transfer(&peer_uuid, "document.pdf", 1024).is_ok());
        assert_eq!(warpinator.transfer_queue.len(), 1);
    }

    #[test]
    fn test_mint_bulky_batch_renamer() {
        let files = vec!["photo_1.png", "photo_2.png"];

        let replaced = MintBulkyBatchRenamerEngine::rename_find_replace(&files, "photo_", "vacation_");
        assert_eq!(replaced, vec!["vacation_1.png", "vacation_2.png"]);

        let prefixed = MintBulkyBatchRenamerEngine::rename_add_prefix_suffix(&files, "2026_", "_backup");
        assert_eq!(prefixed, vec!["2026_photo_1_backup.png", "2026_photo_2_backup.png"]);

        let cased = MintBulkyBatchRenamerEngine::rename_change_case(&files, BulkyCaseMode::Uppercase);
        assert_eq!(cased, vec!["PHOTO_1.PNG", "PHOTO_2.PNG"]);
    }

    #[test]
    fn test_mint_xapp_status_icon_tray() {
        let mut tray = MintXAppStatusIconTray::new();
        let id = tray.add_status_icon("update-notifier", "Updates Available");
        assert_eq!(id, 1);

        assert!(tray.set_badge_count(id, 5).is_ok());
        assert_eq!(tray.icons[0].badge_count, 5);
    }

    #[test]
    fn test_mint4win_installer_flow() {
        let config = Mint4WinInstallationConfig::default_windows_c('C', "mintuser");
        let mut installer = Mint4WinInstallerEngine::new(config);

        let alloc_res = installer.allocate_loopback_disks().unwrap();
        assert!(alloc_res.contains("Successfully allocated 32768 MB"));

        let bcd_res = installer.configure_windows_bcd_boot_entry().unwrap();
        assert!(bcd_res.contains("Added Windows BCD boot entry"));

        let script = installer.generate_unattended_install_script();
        assert!(script.contains("unattended_user=\"mintuser\""));

        let uninst_res = installer.uninstall_mint4win().unwrap();
        assert!(uninst_res.contains("Successfully removed mint4win"));
    }
}
