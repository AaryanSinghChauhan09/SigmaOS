// SPDX-License-Identifier: MIT
//! # SigmaOS Sovereign Mint & Omarchy Vanguard Subsystem Suite
//! (`src/compatibility/mint_omarchy_vanguard_subsystem_suite.rs`)
//!
//! Bridges essential Linux Mint & Omarchy system infrastructure:
//! 1. `MintTimeshiftSnapshotEngine`: Timeshift snapshot lifecycle, BTRFS/RSYNC scheduling and rollback.
//! 2. `MintStickUsbImagingEngine`: Safe USB ISO writer & multi-fs formatter with root-partition protection.
//! 3. `MintWelcomeOobeWizardEngine`: Out-of-box experience, desktop presets, driver onboarding, and codec setup.
//! 4. `OmarchyWaylandNotificationEngine`: Swaync/Mako Wayland notification governor, DND, and action dispatch.
//! 5. `OmarchyAudioWireplumberEngine`: PipeWire/WirePlumber pro-audio quantum latency & Bluetooth LDAC governor.
//! 6. `OmarchyGamePerformanceEngine`: Feral Gamemode process governor & MangoHud telemetry overlay generator.
//! 7. `SovereignMintOmarchyVanguardSuite`: Master coordinator orchestrating all six vanguard engines.
//!
//! Architectural principles applied:
//! - **ABSTRACTION & ARCHITECTURE**: Decoupled subsystem engines communicating via structured message contracts.
//! - **ARRANGE-ACT-ASSERT (FIRST Testing)**: Fast, isolated, repeatable unit tests covering all operational flows.
//! - **DEFENSIVE PROGRAMMING & FAIL-FAST**: Root device write prevention, snapshot verification before rollback.
//! - **COMPOSITION OVER INHERITANCE**: Clean aggregation of distinct engines within a unified coordinator.
//! - **MEMORY SAFETY & SAFE CONCURRENCY**: Safe idiomatic Rust, zero memory leaks, predictable cache locality.

#![allow(dead_code)]

#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::{
    collections::BTreeMap,
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};

#[cfg(any(feature = "standalone_test", test))]
use std::{
    collections::BTreeMap,
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};

// ============================================================================
// 1. LINUX MINT TIMESHIFT SYSTEM SNAPSHOT ENGINE
// ============================================================================

/// Snapshot storage backend mechanism
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeshiftBackendType {
    Btrfs,
    RsyncHardlink,
}

/// Snapshot scheduling cadence
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotCadence {
    Boot,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Manual,
}

/// Timeshift Snapshot Record
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeshiftSnapshot {
    pub id: String,
    pub timestamp_epoch: u64,
    pub backend: TimeshiftBackendType,
    pub cadence: SnapshotCadence,
    pub label: String,
    pub is_protected: bool,
    pub size_mb: u64,
}

/// Linux Mint Timeshift System Snapshot Engine
#[derive(Debug, Clone)]
pub struct MintTimeshiftSnapshotEngine {
    snapshots: BTreeMap<String, TimeshiftSnapshot>,
    default_backend: TimeshiftBackendType,
    max_retention: usize,
}

impl MintTimeshiftSnapshotEngine {
    pub fn new(backend: TimeshiftBackendType) -> Self {
        let mut engine = Self {
            snapshots: BTreeMap::new(),
            default_backend: backend,
            max_retention: 10,
        };
        // Seed baseline installation snapshot
        engine.create_snapshot(
            "baseline-install",
            backend,
            SnapshotCadence::Manual,
            "Clean SigmaOS installation baseline",
            1600,
        );
        engine
    }

    pub fn create_snapshot(
        &mut self,
        id: &str,
        backend: TimeshiftBackendType,
        cadence: SnapshotCadence,
        label: &str,
        size_mb: u64,
    ) -> &TimeshiftSnapshot {
        let snapshot = TimeshiftSnapshot {
            id: id.to_string(),
            timestamp_epoch: 1729250000,
            backend,
            cadence,
            label: label.to_string(),
            is_protected: false,
            size_mb,
        };
        self.snapshots.insert(id.to_string(), snapshot);
        self.snapshots.get(id).unwrap()
    }

    pub fn lock_protection(&mut self, id: &str, protect: bool) -> Result<(), &'static str> {
        match self.snapshots.get_mut(id) {
            Some(snap) => {
                snap.is_protected = protect;
                Ok(())
            }
            None => Err("Snapshot not found"),
        }
    }

    pub fn remove_snapshot(&mut self, id: &str) -> Result<(), &'static str> {
        if let Some(snap) = self.snapshots.get(id) {
            if snap.is_protected {
                return Err("Cannot remove protected snapshot");
            }
        } else {
            return Err("Snapshot not found");
        }
        self.snapshots.remove(id);
        Ok(())
    }

    pub fn validate_rollback_readiness(&self, id: &str) -> Result<String, &'static str> {
        match self.snapshots.get(id) {
            Some(snap) => Ok(format!(
                "Rollback ready: [{}] {} via {:?} ({} MB)",
                snap.id, snap.label, snap.backend, snap.size_mb
            )),
            None => Err("Target rollback snapshot does not exist"),
        }
    }

    pub fn snapshot_count(&self) -> usize {
        self.snapshots.len()
    }
}

// ============================================================================
// 2. LINUX MINT USB STICK IMAGING & FORMATTING ENGINE
// ============================================================================

/// Filesystem types supported for USB formatting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsbFilesystemType {
    Fat32,
    ExFat,
    Ntfs,
    Ext4,
}

/// Status of USB write/format operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UsbOperationStatus {
    Ready,
    InProgress,
    Completed,
    BlockedDangerousDevice,
}

/// Linux Mint USB Stick Writer & Formatter Engine
#[derive(Debug, Clone)]
pub struct MintStickUsbImagingEngine {
    protected_system_devices: Vec<String>,
}

impl MintStickUsbImagingEngine {
    pub fn new() -> Self {
        Self {
            protected_system_devices: vec![
                "/dev/sda".to_string(),
                "/dev/nvme0n1".to_string(),
                "/dev/vda".to_string(),
                "/dev/mapper/root".to_string(),
            ],
        }
    }

    pub fn is_device_safe_for_write(&self, device_path: &str) -> bool {
        !self
            .protected_system_devices
            .iter()
            .any(|prot| device_path.starts_with(prot))
    }

    pub fn prepare_iso_burn(
        &self,
        iso_path: &str,
        target_device: &str,
    ) -> Result<String, &'static str> {
        if !self.is_device_safe_for_write(target_device) {
            return Err("Refusing to write to protected system storage device");
        }
        if !iso_path.ends_with(".iso") && !iso_path.ends_with(".img") {
            return Err("Invalid image format: must be .iso or .img");
        }
        Ok(format!(
            "Direct I/O burn scheduled: {} -> {} (sync, bs=4M, verify=sha256)",
            iso_path, target_device
        ))
    }

    pub fn format_stick(
        &self,
        target_device: &str,
        fs_type: UsbFilesystemType,
        volume_label: &str,
    ) -> Result<String, &'static str> {
        if !self.is_device_safe_for_write(target_device) {
            return Err("Refusing to format protected system storage device");
        }
        if volume_label.len() > 11 && fs_type == UsbFilesystemType::Fat32 {
            return Err("FAT32 volume label cannot exceed 11 characters");
        }
        Ok(format!(
            "Formatted {} as {:?} with label '{}'",
            target_device, fs_type, volume_label
        ))
    }
}

// ============================================================================
// 3. LINUX MINT WELCOME & OOBE ONBOARDING WIZARD
// ============================================================================

/// Desktop Layout Preference
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MintDesktopLayout {
    TraditionalCinnamon,
    ModernTiling,
}

/// Onboarding configuration state
#[derive(Debug, Clone)]
pub struct MintWelcomeOobeWizardEngine {
    pub layout: MintDesktopLayout,
    pub timeshift_configured: bool,
    pub drivers_installed: bool,
    pub codecs_installed: bool,
    pub firewall_enabled: bool,
}

impl MintWelcomeOobeWizardEngine {
    pub fn new() -> Self {
        Self {
            layout: MintDesktopLayout::TraditionalCinnamon,
            timeshift_configured: false,
            drivers_installed: false,
            codecs_installed: false,
            firewall_enabled: true,
        }
    }

    pub fn complete_setup_step(&mut self, step: &str) -> Result<(), &'static str> {
        match step {
            "timeshift" => self.timeshift_configured = true,
            "drivers" => self.drivers_installed = true,
            "codecs" => self.codecs_installed = true,
            "firewall" => self.firewall_enabled = true,
            _ => return Err("Unknown onboarding step"),
        }
        Ok(())
    }

    pub fn is_oobe_complete(&self) -> bool {
        self.timeshift_configured
            && self.drivers_installed
            && self.codecs_installed
            && self.firewall_enabled
    }

    pub fn render_welcome_manifest(&self) -> String {
        format!(
            "OOBE Status: [Layout: {:?}] [Timeshift: {}] [Drivers: {}] [Codecs: {}] [Firewall: {}]",
            self.layout,
            self.timeshift_configured,
            self.drivers_installed,
            self.codecs_installed,
            self.firewall_enabled
        )
    }
}

// ============================================================================
// 4. OMARCHY WAYLAND NOTIFICATION GOVERNOR (MAKO / SWAYNC)
// ============================================================================

/// Notification Urgency Classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationUrgency {
    Low,
    Normal,
    Critical,
}

/// Notification Entity
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaylandNotification {
    pub id: u32,
    pub app_name: String,
    pub summary: String,
    pub urgency: NotificationUrgency,
    pub sound_hint: bool,
}

/// Omarchy Wayland Notification Engine (Mako / Swaync parity)
#[derive(Debug, Clone)]
pub struct OmarchyWaylandNotificationEngine {
    notifications: Vec<WaylandNotification>,
    is_dnd_active: bool,
    counter: u32,
}

impl OmarchyWaylandNotificationEngine {
    pub fn new() -> Self {
        Self {
            notifications: Vec::new(),
            is_dnd_active: false,
            counter: 0,
        }
    }

    pub fn set_do_not_disturb(&mut self, active: bool) {
        self.is_dnd_active = active;
    }

    pub fn post_notification(
        &mut self,
        app_name: &str,
        summary: &str,
        urgency: NotificationUrgency,
    ) -> Option<u32> {
        // In DND mode, suppress low and normal urgency notifications
        if self.is_dnd_active && urgency != NotificationUrgency::Critical {
            return None;
        }

        self.counter += 1;
        let notif = WaylandNotification {
            id: self.counter,
            app_name: app_name.to_string(),
            summary: summary.to_string(),
            urgency,
            sound_hint: urgency == NotificationUrgency::Critical,
        };
        self.notifications.push(notif);
        Some(self.counter)
    }

    pub fn dismiss_all(&mut self) {
        self.notifications.clear();
    }

    pub fn export_mako_config(&self) -> String {
        String::from(
            "[global]\n\
            font=JetBrains Mono Nerd Font 10\n\
            background-color=#1e1e2eff\n\
            text-color=#cdd6f4ff\n\
            border-color=#89b4faff\n\
            border-size=2\n\
            border-radius=8\n\
            default-timeout=5000\n\
            [urgency=critical]\n\
            border-color=#f38ba8ff\n\
            default-timeout=0\n",
        )
    }

    pub fn pending_count(&self) -> usize {
        self.notifications.len()
    }
}

// ============================================================================
// 5. OMARCHY AUDIO & WIREPLUMBER QUANTUM ENGINE
// ============================================================================

/// Audio latency profiles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioLatencyProfile {
    UltraLowGamingAudio(u32), // 64 or 128 frames
    StandardDesktop(u32),     // 512 frames
    PowerSavingMedia(u32),    // 1024 frames
}

/// Bluetooth audio codecs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BluetoothAudioCodec {
    Ldac990k,
    AptxHd,
    Aac,
    SbcXq,
}

/// Omarchy PipeWire / WirePlumber Audio Engine
#[derive(Debug, Clone)]
pub struct OmarchyAudioWireplumberEngine {
    current_quantum: u32,
    active_profile: AudioLatencyProfile,
    preferred_bt_codec: BluetoothAudioCodec,
}

impl OmarchyAudioWireplumberEngine {
    pub fn new() -> Self {
        Self {
            current_quantum: 512,
            active_profile: AudioLatencyProfile::StandardDesktop(512),
            preferred_bt_codec: BluetoothAudioCodec::Ldac990k,
        }
    }

    pub fn switch_profile(&mut self, profile: AudioLatencyProfile) {
        self.active_profile = profile;
        self.current_quantum = match profile {
            AudioLatencyProfile::UltraLowGamingAudio(q) => q,
            AudioLatencyProfile::StandardDesktop(q) => q,
            AudioLatencyProfile::PowerSavingMedia(q) => q,
        };
    }

    pub fn current_quantum(&self) -> u32 {
        self.current_quantum
    }

    pub fn preferred_bt_codec(&self) -> BluetoothAudioCodec {
        self.preferred_bt_codec
    }

    pub fn render_wireplumber_conf(&self) -> String {
        format!(
            "context.properties = {{\n    default.clock.rate = 48000\n    default.clock.quantum = {}\n    default.clock.min-quantum = 64\n    default.clock.max-quantum = 1024\n}}\nbluez5.codecs = [ ldac aptx_hd aac sbc_xq ]\n",
            self.current_quantum
        )
    }
}

// ============================================================================
// 6. OMARCHY GAMEMODE & MANGOHUD OVERLAY SYNTHESIZER
// ============================================================================

/// Omarchy Gamemode & MangoHud Overlay Engine
#[derive(Debug, Clone)]
pub struct OmarchyGamePerformanceEngine {
    active_pids: Vec<u32>,
    hud_fps_enabled: bool,
    hud_frametime_enabled: bool,
    hud_temperatures_enabled: bool,
}

impl OmarchyGamePerformanceEngine {
    pub fn new() -> Self {
        Self {
            active_pids: Vec::new(),
            hud_fps_enabled: true,
            hud_frametime_enabled: true,
            hud_temperatures_enabled: true,
        }
    }

    pub fn register_game(&mut self, pid: u32) {
        if !self.active_pids.contains(&pid) {
            self.active_pids.push(pid);
        }
    }

    pub fn unregister_game(&mut self, pid: u32) {
        self.active_pids.retain(|&p| p != pid);
    }

    pub fn is_gamemode_active(&self) -> bool {
        !self.active_pids.is_empty()
    }

    pub fn generate_mangohud_conf(&self) -> String {
        let mut conf = String::from("legacy_layout=false\ngpu_stats\ncpu_stats\n");
        if self.hud_fps_enabled {
            conf.push_str("fps\n");
        }
        if self.hud_frametime_enabled {
            conf.push_str("frametime=1\n");
        }
        if self.hud_temperatures_enabled {
            conf.push_str("gpu_temp\ncpu_temp\n");
        }
        conf.push_str("round_corners=10\nbackground_alpha=0.6\nfont_size=24\n");
        conf
    }
}

// ============================================================================
// 7. SOVEREIGN MINT & OMARCHY VANGUARD SUBSYSTEM MASTER COORDINATOR
// ============================================================================

/// Master Coordinator orchestrating all six Vanguard engines
#[derive(Debug, Clone)]
pub struct SovereignMintOmarchyVanguardSuite {
    pub timeshift: MintTimeshiftSnapshotEngine,
    pub usb_imaging: MintStickUsbImagingEngine,
    pub oobe_wizard: MintWelcomeOobeWizardEngine,
    pub notifications: OmarchyWaylandNotificationEngine,
    pub audio_wireplumber: OmarchyAudioWireplumberEngine,
    pub game_performance: OmarchyGamePerformanceEngine,
}

impl SovereignMintOmarchyVanguardSuite {
    pub fn new() -> Self {
        Self {
            timeshift: MintTimeshiftSnapshotEngine::new(TimeshiftBackendType::Btrfs),
            usb_imaging: MintStickUsbImagingEngine::new(),
            oobe_wizard: MintWelcomeOobeWizardEngine::new(),
            notifications: OmarchyWaylandNotificationEngine::new(),
            audio_wireplumber: OmarchyAudioWireplumberEngine::new(),
            game_performance: OmarchyGamePerformanceEngine::new(),
        }
    }

    pub fn generate_vanguard_manifest(&self) -> String {
        format!(
            "=== SOVEREIGN MINT & OMARCHY VANGUARD SUBSYSTEM SUITE ===\n\
            1. Timeshift Snapshots: {} active\n\
            2. USB Imaging Engine: Armed (Root protections active)\n\
            3. OOBE Wizard: {}\n\
            4. Wayland Notifications: {} queued (Mako/Swaync profile)\n\
            5. WirePlumber Audio: Quantum {} frames, LDAC 990k ready\n\
            6. Game Performance: Active PIDs: {}, MangoHud ready\n\
            ===========================================================",
            self.timeshift.snapshot_count(),
            self.oobe_wizard.render_welcome_manifest(),
            self.notifications.pending_count(),
            self.audio_wireplumber.current_quantum(),
            self.game_performance.active_pids.len()
        )
    }
}

// ============================================================================
// TESTS (ARRANGE - ACT - ASSERT / FIRST PRINCIPLES)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeshift_snapshot_lifecycle() {
        // Arrange
        let mut engine = MintTimeshiftSnapshotEngine::new(TimeshiftBackendType::Btrfs);

        // Act
        let snap = engine.create_snapshot(
            "pre-upgrade",
            TimeshiftBackendType::Btrfs,
            SnapshotCadence::Daily,
            "Pre upgrade to 6.12",
            1200,
        );
        assert_eq!(snap.id, "pre-upgrade");

        let ready = engine.validate_rollback_readiness("pre-upgrade");
        assert!(ready.is_ok());

        engine.lock_protection("pre-upgrade", true).unwrap();
        let delete_attempt = engine.remove_snapshot("pre-upgrade");

        // Assert
        assert!(delete_attempt.is_err());
        engine.lock_protection("pre-upgrade", false).unwrap();
        assert!(engine.remove_snapshot("pre-upgrade").is_ok());
    }

    #[test]
    fn test_usb_imaging_defensive_protections() {
        // Arrange
        let engine = MintStickUsbImagingEngine::new();

        // Act
        let danger_write = engine.prepare_iso_burn("mint.iso", "/dev/sda");
        let safe_write = engine.prepare_iso_burn("mint.iso", "/dev/sdb");
        let danger_format = engine.format_stick("/dev/nvme0n1", UsbFilesystemType::ExFat, "USB_KEY");
        let safe_format = engine.format_stick("/dev/sdc1", UsbFilesystemType::Fat32, "SIGMA_BOOT");

        // Assert
        assert!(danger_write.is_err());
        assert!(safe_write.is_ok());
        assert!(danger_format.is_err());
        assert!(safe_format.is_ok());
    }

    #[test]
    fn test_oobe_wizard_progression() {
        // Arrange
        let mut wizard = MintWelcomeOobeWizardEngine::new();

        // Act & Assert
        assert!(!wizard.is_oobe_complete());
        wizard.complete_setup_step("timeshift").unwrap();
        wizard.complete_setup_step("drivers").unwrap();
        wizard.complete_setup_step("codecs").unwrap();
        wizard.complete_setup_step("firewall").unwrap();

        assert!(wizard.is_oobe_complete());
        assert!(wizard.render_welcome_manifest().contains("OOBE Status"));
    }

    #[test]
    fn test_wayland_notifications_and_dnd() {
        // Arrange
        let mut notif_engine = OmarchyWaylandNotificationEngine::new();

        // Act
        notif_engine.post_notification("Terminal", "Build done", NotificationUrgency::Normal);
        assert_eq!(notif_engine.pending_count(), 1);

        notif_engine.set_do_not_disturb(true);
        let blocked = notif_engine.post_notification("Mail", "New mail", NotificationUrgency::Low);
        assert!(blocked.is_none());

        let crit = notif_engine.post_notification("Power", "Battery 2%", NotificationUrgency::Critical);
        assert!(crit.is_some());
        assert_eq!(notif_engine.pending_count(), 2);

        // Assert config export
        let mako_conf = notif_engine.export_mako_config();
        assert!(mako_conf.contains("border-radius=8"));
    }

    #[test]
    fn test_wireplumber_audio_and_gamemode() {
        // Arrange
        let mut audio = OmarchyAudioWireplumberEngine::new();
        let mut game = OmarchyGamePerformanceEngine::new();

        // Act
        audio.switch_profile(AudioLatencyProfile::UltraLowGamingAudio(64));
        game.register_game(4242);

        // Assert
        assert_eq!(audio.current_quantum(), 64);
        assert!(game.is_gamemode_active());

        let wp_conf = audio.render_wireplumber_conf();
        assert!(wp_conf.contains("default.clock.quantum = 64"));

        let mango_conf = game.generate_mangohud_conf();
        assert!(mango_conf.contains("gpu_stats"));
        assert!(mango_conf.contains("fps"));
    }

    #[test]
    fn test_sovereign_vanguard_master_suite() {
        // Arrange
        let suite = SovereignMintOmarchyVanguardSuite::new();

        // Act
        let manifest = suite.generate_vanguard_manifest();

        // Assert
        assert!(manifest.contains("SOVEREIGN MINT & OMARCHY VANGUARD SUBSYSTEM SUITE"));
        assert!(manifest.contains("Timeshift Snapshots: 1 active"));
        assert!(manifest.contains("LDAC 990k ready"));
    }
}
