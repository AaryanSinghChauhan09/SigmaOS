// Lubuntu-Inspired Lightweight System Configuration & Hardware Optimizer
// Focuses on extreme memory conservation, diagnostics, and running flawlessly on legacy/low-end systems.

use core::sync::atomic::{AtomicU8, AtomicUsize, Ordering};
use std::string::String;
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuGovernor {
    Performance,
    Balanced,
    Powersave,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemPressure {
    Low,
    Medium,
    High,
    Critical,
}

/// Lightweight system health report suitable for low-end machines
pub struct LubuntuHealthReport {
    pub memory_util_pct: u8,
    pub active_processes_count: usize,
    pub storage_free_mb: usize,
    pub system_pressure: SystemPressure,
}

/// Sovereign Lubuntu System Manager and Hardware Profile Tuner
pub struct LubuntuSystemManager {
    pub cpu_governor: AtomicU8, // CpuGovernor as u8
    pub max_task_queue_size: AtomicUsize,
    pub background_effects_enabled: AtomicUsize, // 1 if enabled, 0 if disabled
}

impl LubuntuSystemManager {
    pub fn new() -> Self {
        Self {
            cpu_governor: AtomicU8::new(CpuGovernor::Balanced as u8),
            max_task_queue_size: AtomicUsize::new(1000),
            background_effects_enabled: AtomicUsize::new(1),
        }
    }

    /// Retrieve Cpu Governor status
    pub fn get_governor(&self) -> CpuGovernor {
        match self.cpu_governor.load(Ordering::SeqCst) {
            0 => CpuGovernor::Performance,
            2 => CpuGovernor::Powersave,
            _ => CpuGovernor::Balanced,
        }
    }

    /// Run non-intrusive lightweight system health diagnostics
    pub fn diagnose_system_health(
        &self,
        current_memory_pct: u8,
        process_count: usize,
        free_storage: usize,
    ) -> LubuntuHealthReport {
        let system_pressure = if current_memory_pct > 90 || process_count > 500 {
            SystemPressure::Critical
        } else if current_memory_pct > 75 || process_count > 300 {
            SystemPressure::High
        } else if current_memory_pct > 40 {
            SystemPressure::Medium
        } else {
            SystemPressure::Low
        };

        LubuntuHealthReport {
            memory_util_pct: current_memory_pct,
            active_processes_count: process_count,
            storage_free_mb: free_storage,
            system_pressure,
        }
    }

    /// Dynamically optimizes the entire OS to run flawlessly on low-end/legacy physical computers
    pub fn optimize_for_low_end_hardware(
        &mut self,
        current_memory_pct: u8,
    ) -> Result<(), &'static str> {
        if current_memory_pct > 60 {
            // Low-end machine detected or system under load!
            // 1. Force CPU governor to Powersave to prevent thermal throttling
            self.cpu_governor
                .store(CpuGovernor::Powersave as u8, Ordering::SeqCst);

            // 2. Shrink maximum task queue capacities to prevent system queue starvation
            self.max_task_queue_size.store(200, Ordering::SeqCst);

            // 3. Disable resource-heavy visual background compositor shadow and glow effects
            self.background_effects_enabled.store(0, Ordering::SeqCst);
        } else {
            // Restore balanced state
            self.cpu_governor
                .store(CpuGovernor::Balanced as u8, Ordering::SeqCst);
            self.max_task_queue_size.store(1000, Ordering::SeqCst);
            self.background_effects_enabled.store(1, Ordering::SeqCst);
        }
        Ok(())
    }
}

impl Default for LubuntuSystemManager {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 1. LXQT SESSION MANAGER (`LxqtSessionManager`)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LxqtSessionState {
    Uninitialized,
    Starting,
    Running,
    Stopping,
    Stopped,
}

pub struct LxqtSessionManager {
    pub state: LxqtSessionState,
    pub auto_start_apps: Vec<String>,
    pub panel_enabled: bool,
    pub notification_daemon_active: bool,
    pub theme_name: String,
}

impl LxqtSessionManager {
    pub fn new() -> Self {
        Self {
            state: LxqtSessionState::Uninitialized,
            auto_start_apps: Vec::new(),
            panel_enabled: true,
            notification_daemon_active: true,
            theme_name: String::from("lubuntu-arc-dark"),
        }
    }

    pub fn initialize(&mut self) -> Result<(), &'static str> {
        self.state = LxqtSessionState::Starting;
        // Start essential LXQt lightweight components
        self.panel_enabled = true;
        self.notification_daemon_active = true;
        self.state = LxqtSessionState::Running;
        Ok(())
    }

    pub fn register_autostart_app(&mut self, app: &str) {
        self.auto_start_apps.push(String::from(app));
    }

    pub fn set_theme(&mut self, new_theme: &str) {
        self.theme_name = String::from(new_theme);
    }
}

impl Default for LxqtSessionManager {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 2. PCMANFM-QT FILE MANAGER (`PcmanfmQtAdapter`)
// =========================================================================

#[derive(Debug, Clone)]
pub struct FileNode {
    pub name: String,
    pub is_directory: bool,
    pub size_bytes: usize,
}

pub struct PcmanfmQtAdapter {
    pub current_directory: String,
    pub file_list: Vec<FileNode>,
    pub desktop_icons_enabled: bool,
}

impl PcmanfmQtAdapter {
    pub fn new() -> Self {
        Self {
            current_directory: String::from("/home/lubuntu"),
            file_list: Vec::new(),
            desktop_icons_enabled: true,
        }
    }

    pub fn navigate_to(&mut self, path: &str) {
        self.current_directory = String::from(path);
        self.file_list.clear(); // Simulating changing directory scans
    }

    pub fn add_file(&mut self, name: &str, is_dir: bool, size: usize) {
        self.file_list.push(FileNode {
            name: String::from(name),
            is_directory: is_dir,
            size_bytes: size,
        });
    }

    pub fn toggle_desktop_icons(&mut self) {
        self.desktop_icons_enabled = !self.desktop_icons_enabled;
    }
}

impl Default for PcmanfmQtAdapter {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. DISCOVER PACKAGE ADAPTER (`DiscoverPackageAdapter`)
// =========================================================================

#[derive(Debug, Clone)]
pub struct AptPackage {
    pub name: String,
    pub version: String,
    pub installed: bool,
    pub size_kb: usize,
}

pub struct DiscoverPackageAdapter {
    pub available_packages: Vec<AptPackage>,
    pub active_ppas: Vec<String>,
}

impl DiscoverPackageAdapter {
    pub fn new() -> Self {
        Self {
            available_packages: Vec::new(),
            active_ppas: Vec::new(),
        }
    }

    pub fn register_package(&mut self, name: &str, version: &str, size: usize) {
        self.available_packages.push(AptPackage {
            name: String::from(name),
            version: String::from(version),
            installed: false,
            size_kb: size,
        });
    }

    pub fn add_ppa(&mut self, ppa_uri: &str) {
        self.active_ppas.push(String::from(ppa_uri));
    }

    pub fn install_package(&mut self, name: &str) -> Result<(), &'static str> {
        for pkg in &mut self.available_packages {
            if pkg.name == name {
                pkg.installed = true;
                return Ok(());
            }
        }
        Err("Package not found in APT cache")
    }
}

impl Default for DiscoverPackageAdapter {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. FEATHERPAD EDITOR (`FeatherpadEditor`)
// =========================================================================

pub struct FeatherpadEditor {
    pub file_path: Option<String>,
    pub buffer: String,
    pub is_dirty: bool,
    pub tab_size: usize,
    pub search_history: Vec<String>,
}

impl FeatherpadEditor {
    pub fn new() -> Self {
        Self {
            file_path: None,
            buffer: String::new(),
            is_dirty: false,
            tab_size: 4,
            search_history: Vec::new(),
        }
    }

    pub fn open_file(&mut self, path: &str, contents: &str) {
        self.file_path = Some(String::from(path));
        self.buffer = String::from(contents);
        self.is_dirty = false;
    }

    pub fn insert_text(&mut self, text: &str) {
        self.buffer.push_str(text);
        self.is_dirty = true;
    }

    pub fn save(&mut self) -> Result<(), &'static str> {
        if self.file_path.is_some() {
            self.is_dirty = false;
            Ok(())
        } else {
            Err("No file path specified for save")
        }
    }
}

impl Default for FeatherpadEditor {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 5. QTERMINAL EMULATOR (`QTerminalEmulator`)
// =========================================================================

#[derive(Debug, Clone)]
pub struct TerminalTab {
    pub id: usize,
    pub title: String,
    pub active_process: String,
}

pub struct QTerminalEmulator {
    pub tabs: Vec<TerminalTab>,
    pub next_tab_id: usize,
    pub command_history: Vec<String>,
    pub font_size: u32,
}

impl QTerminalEmulator {
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            next_tab_id: 1,
            command_history: Vec::new(),
            font_size: 10, // lightweight default font size
        }
    }

    pub fn open_tab(&mut self, title: &str) -> usize {
        let tab_id = self.next_tab_id;
        self.tabs.push(TerminalTab {
            id: tab_id,
            title: String::from(title),
            active_process: String::from("sh"),
        });
        self.next_tab_id += 1;
        tab_id
    }

    pub fn execute_command(&mut self, tab_id: usize, cmd: &str) -> Result<(), &'static str> {
        for tab in &mut self.tabs {
            if tab.id == tab_id {
                self.command_history.push(String::from(cmd));
                tab.active_process = String::from(cmd);
                return Ok(());
            }
        }
        Err("Tab not found")
    }
}

impl Default for QTerminalEmulator {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 6. CALAMARES INSTALLER (`CalamaresInstallerShim`)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalamaresStage {
    Welcome,
    Location,
    Keyboard,
    Partitions,
    Users,
    Summary,
    Install,
    Finish,
}

pub struct CalamaresInstallerShim {
    pub current_stage: CalamaresStage,
    pub target_disk_name: String,
    pub partition_size_mb: usize,
    pub username: String,
    pub progress_pct: u8,
}

impl CalamaresInstallerShim {
    pub fn new() -> Self {
        Self {
            current_stage: CalamaresStage::Welcome,
            target_disk_name: String::from("/dev/sda"),
            partition_size_mb: 20480, // default 20 GB
            username: String::new(),
            progress_pct: 0,
        }
    }

    pub fn set_user_details(&mut self, username: &str) {
        self.username = String::from(username);
    }

    pub fn configure_partitions(&mut self, disk: &str, size_mb: usize) {
        self.target_disk_name = String::from(disk);
        self.partition_size_mb = size_mb;
    }

    pub fn next_stage(&mut self) {
        self.current_stage = match self.current_stage {
            CalamaresStage::Welcome => CalamaresStage::Location,
            CalamaresStage::Location => CalamaresStage::Keyboard,
            CalamaresStage::Keyboard => CalamaresStage::Partitions,
            CalamaresStage::Partitions => CalamaresStage::Users,
            CalamaresStage::Users => CalamaresStage::Summary,
            CalamaresStage::Summary => CalamaresStage::Install,
            CalamaresStage::Install => CalamaresStage::Finish,
            CalamaresStage::Finish => CalamaresStage::Finish,
        };
    }

    pub fn step_install_progress(&mut self) {
        if self.current_stage == CalamaresStage::Install {
            if self.progress_pct < 100 {
                self.progress_pct += 10;
            }
            if self.progress_pct == 100 {
                self.next_stage();
            }
        }
    }
}

impl Default for CalamaresInstallerShim {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lubuntu_system_diagnostics() {
        let manager = LubuntuSystemManager::new();

        // Low utilization scenario
        let report_low = manager.diagnose_system_health(30, 15, 102400);
        assert_eq!(report_low.system_pressure, SystemPressure::Low);

        // Critical utilization scenario
        let report_critical = manager.diagnose_system_health(95, 600, 5000);
        assert_eq!(report_critical.system_pressure, SystemPressure::Critical);
        assert_eq!(report_critical.memory_util_pct, 95);
    }

    #[test]
    fn test_lubuntu_low_end_optimization() {
        let mut manager = LubuntuSystemManager::new();
        assert_eq!(manager.get_governor(), CpuGovernor::Balanced);
        assert_eq!(manager.background_effects_enabled.load(Ordering::SeqCst), 1);

        // Trigger low-end optimization due to high memory use
        manager.optimize_for_low_end_hardware(85).unwrap();

        assert_eq!(manager.get_governor(), CpuGovernor::Powersave);
        assert_eq!(manager.max_task_queue_size.load(Ordering::SeqCst), 200);
        assert_eq!(manager.background_effects_enabled.load(Ordering::SeqCst), 0);
        // Background effects disabled!
    }

    #[test]
    fn test_lxqt_session_manager() {
        let mut session = LxqtSessionManager::new();
        assert_eq!(session.state, LxqtSessionState::Uninitialized);
        assert!(session.initialize().is_ok());
        assert_eq!(session.state, LxqtSessionState::Running);
        assert_eq!(session.theme_name, "lubuntu-arc-dark");

        session.register_autostart_app("lxqt-runner");
        assert_eq!(session.auto_start_apps.len(), 1);
        assert_eq!(session.auto_start_apps[0], "lxqt-runner");

        session.set_theme("lubuntu-light");
        assert_eq!(session.theme_name, "lubuntu-light");
    }

    #[test]
    fn test_pcmanfm_qt_adapter() {
        let mut fm = PcmanfmQtAdapter::new();
        assert_eq!(fm.current_directory, "/home/lubuntu");
        assert!(fm.desktop_icons_enabled);

        fm.add_file("Documents", true, 4096);
        fm.add_file("featherpad.desktop", false, 256);
        assert_eq!(fm.file_list.len(), 2);
        assert!(fm.file_list[0].is_directory);
        assert_eq!(fm.file_list[1].name, "featherpad.desktop");

        fm.toggle_desktop_icons();
        assert!(!fm.desktop_icons_enabled);

        fm.navigate_to("/home/lubuntu/Downloads");
        assert_eq!(fm.current_directory, "/home/lubuntu/Downloads");
        assert_eq!(fm.file_list.len(), 0); // Cleared on navigation
    }

    #[test]
    fn test_discover_package_adapter() {
        let mut discover = DiscoverPackageAdapter::new();
        discover.register_package("featherpad", "1.4.0-1", 512);
        discover.register_package("qterminal", "1.4.0-1", 1024);
        assert_eq!(discover.available_packages.len(), 2);

        discover.add_ppa("ppa:lubuntu-dev/stable");
        assert_eq!(discover.active_ppas.len(), 1);

        assert!(!discover.available_packages[0].installed);
        assert!(discover.install_package("featherpad").is_ok());
        assert!(discover.available_packages[0].installed);

        assert!(discover.install_package("nonexistent").is_err());
    }

    #[test]
    fn test_featherpad_editor() {
        let mut editor = FeatherpadEditor::new();
        assert_eq!(editor.file_path, None);
        assert!(!editor.is_dirty);

        editor.open_file("test.txt", "Hello Lubuntu!");
        assert_eq!(editor.file_path.as_deref(), Some("test.txt"));
        assert_eq!(editor.buffer, "Hello Lubuntu!");
        assert!(!editor.is_dirty);

        editor.insert_text("\nNew line.");
        assert_eq!(editor.buffer, "Hello Lubuntu!\nNew line.");
        assert!(editor.is_dirty);

        assert!(editor.save().is_ok());
        assert!(!editor.is_dirty);
    }

    #[test]
    fn test_qterminal_emulator() {
        let mut term = QTerminalEmulator::new();
        assert_eq!(term.tabs.len(), 0);

        let t1 = term.open_tab("Tab 1");
        assert_eq!(t1, 1);
        assert_eq!(term.tabs.len(), 1);
        assert_eq!(term.tabs[0].title, "Tab 1");
        assert_eq!(term.tabs[0].active_process, "sh");

        assert!(term.execute_command(t1, "htop").is_ok());
        assert_eq!(term.command_history.len(), 1);
        assert_eq!(term.command_history[0], "htop");
        assert_eq!(term.tabs[0].active_process, "htop");

        assert!(term.execute_command(999, "ls").is_err());
    }

    #[test]
    fn test_calamares_installer() {
        let mut calamares = CalamaresInstallerShim::new();
        assert_eq!(calamares.current_stage, CalamaresStage::Welcome);
        assert_eq!(calamares.target_disk_name, "/dev/sda");

        calamares.configure_partitions("/dev/nvme0n1", 40960);
        assert_eq!(calamares.target_disk_name, "/dev/nvme0n1");
        assert_eq!(calamares.partition_size_mb, 40960);

        calamares.set_user_details("lubuntu_user");
        assert_eq!(calamares.username, "lubuntu_user");

        // Advance to Summary
        for _ in 0..5 {
            calamares.next_stage();
        }
        assert_eq!(calamares.current_stage, CalamaresStage::Summary);

        // Go to Install stage
        calamares.next_stage();
        assert_eq!(calamares.current_stage, CalamaresStage::Install);
        assert_eq!(calamares.progress_pct, 0);

        // Step up progress
        for _ in 0..10 {
            calamares.step_install_progress();
        }
        assert_eq!(calamares.current_stage, CalamaresStage::Finish);
        assert_eq!(calamares.progress_pct, 100);
    }
}
