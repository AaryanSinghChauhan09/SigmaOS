// SigmaOS — Omarchy Linux Dynamic Workspace & Hardware Productivity Suite
//
// Native implementations of Omarchy Linux's signature keyboard-first workflow:
// 1. Scratchpad Manager: Modal drop-down terminal, AI companion & system HUD
// 2. Audio & MPRIS Controller: Per-app stream routing, volume limits & media HUD
// 3. Capture & Instant OCR: Screen region clipping with optical character recognition
// 4. Power & Battery Governor: AC/Battery dynamic switching & 80% longevity limiter
// 5. Tiling Layout Engine: Dwindle Fibonacci bisection & Master-Stack layouts

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

// ============================================================================
// 1. Omarchy Scratchpad & Dropdown HUD Manager
// ============================================================================

/// Animation state of a scratchpad dropdown
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScratchpadVisibility {
    Hidden,
    SlidingIn,
    Visible,
    SlidingOut,
}

/// A persistent scratchpad window instance
#[derive(Debug, Clone)]
pub struct ScratchpadInstance {
    pub name: String,
    pub command: String,
    pub hotkey: String,
    pub width_percent: u32,
    pub height_percent: u32,
    pub visibility: ScratchpadVisibility,
    pub window_id: Option<u64>,
}

/// Scratchpad orchestrator
pub struct OmarchyScratchpadManager {
    pub scratchpads: BTreeMap<String, ScratchpadInstance>,
    pub active_scratchpad: Option<String>,
}

impl OmarchyScratchpadManager {
    pub fn new() -> Self {
        let mut mgr = Self {
            scratchpads: BTreeMap::new(),
            active_scratchpad: None,
        };

        // Standard Omarchy default scratchpads
        mgr.register_scratchpad("terminal", "ghostty --class=scratchpad-term", "Super+Return", 80, 50);
        mgr.register_scratchpad("ai_assistant", "sigma-ai-hud", "Super+Space", 40, 70);
        mgr.register_scratchpad("monitor", "btop", "Super+M", 85, 60);

        mgr
    }

    pub fn register_scratchpad(
        &mut self,
        name: &str,
        command: &str,
        hotkey: &str,
        width_pct: u32,
        height_pct: u32,
    ) {
        self.scratchpads.insert(
            String::from(name),
            ScratchpadInstance {
                name: String::from(name),
                command: String::from(command),
                hotkey: String::from(hotkey),
                width_percent: width_pct,
                height_percent: height_pct,
                visibility: ScratchpadVisibility::Hidden,
                window_id: None,
            },
        );
    }

    /// Toggle visibility of a scratchpad by name
    pub fn toggle_scratchpad(&mut self, name: &str) -> ScratchpadVisibility {
        if let Some(sp) = self.scratchpads.get_mut(name) {
            match sp.visibility {
                ScratchpadVisibility::Hidden | ScratchpadVisibility::SlidingOut => {
                    sp.visibility = ScratchpadVisibility::Visible;
                    self.active_scratchpad = Some(String::from(name));
                }
                ScratchpadVisibility::Visible | ScratchpadVisibility::SlidingIn => {
                    sp.visibility = ScratchpadVisibility::Hidden;
                    if self.active_scratchpad.as_deref() == Some(name) {
                        self.active_scratchpad = None;
                    }
                }
            }
            sp.visibility
        } else {
            ScratchpadVisibility::Hidden
        }
    }
}

// ============================================================================
// 2. Audio & MPRIS Media Player Controller
// ============================================================================

/// Playback status of active media player
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackStatus {
    Playing,
    Paused,
    Stopped,
}

/// MPRIS track metadata
#[derive(Debug, Clone)]
pub struct MediaTrackMetadata {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub length_seconds: u64,
    pub current_position_seconds: u64,
}

/// Audio output sink
#[derive(Debug, Clone)]
pub struct AudioSink {
    pub name: String,
    pub description: String,
    pub volume_percent: u32,
    pub is_muted: bool,
    pub is_default: bool,
}

/// Omarchy Audio & MPRIS Media Controller
pub struct OmarchyAudioMediaController {
    pub sinks: Vec<AudioSink>,
    pub playback_status: PlaybackStatus,
    pub current_track: Option<MediaTrackMetadata>,
}

impl OmarchyAudioMediaController {
    pub fn new() -> Self {
        Self {
            sinks: vec![
                AudioSink {
                    name: String::from("alsa_output.pci-0000_00_1f.3.analog-stereo"),
                    description: String::from("Built-in Analog Audio"),
                    volume_percent: 65,
                    is_muted: false,
                    is_default: true,
                },
                AudioSink {
                    name: String::from("bluez_output.WH-1000XM4.a2dp-sink"),
                    description: String::from("Sony WH-1000XM4 Bluetooth"),
                    volume_percent: 80,
                    is_muted: false,
                    is_default: false,
                },
            ],
            playback_status: PlaybackStatus::Playing,
            current_track: Some(MediaTrackMetadata {
                title: String::from("Resonance"),
                artist: String::from("HOME"),
                album: String::from("Odyssey"),
                length_seconds: 212,
                current_position_seconds: 48,
            }),
        }
    }

    /// Set volume on default sink with safety cap
    pub fn set_volume(&mut self, percent: u32) -> u32 {
        let capped = percent.min(100); // 100% ear protection cap
        if let Some(sink) = self.sinks.iter_mut().find(|s| s.is_default) {
            sink.volume_percent = capped;
            capped
        } else {
            0
        }
    }

    /// Route audio stream to a different output device
    pub fn switch_default_sink(&mut self, sink_name: &str) -> bool {
        let exists = self.sinks.iter().any(|s| s.name == sink_name);
        if exists {
            for sink in &mut self.sinks {
                sink.is_default = sink.name == sink_name;
            }
            true
        } else {
            false
        }
    }

    /// Toggle media playback
    pub fn toggle_playback(&mut self) -> PlaybackStatus {
        self.playback_status = match self.playback_status {
            PlaybackStatus::Playing => PlaybackStatus::Paused,
            PlaybackStatus::Paused | PlaybackStatus::Stopped => PlaybackStatus::Playing,
        };
        self.playback_status
    }
}

// ============================================================================
// 3. Screen Capture & Instant OCR Pipeline
// ============================================================================

/// Selected rectangular screen region
#[derive(Debug, Clone, Copy)]
pub struct ScreenRectangle {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

/// Instant Screen Capture & OCR Pipeline
pub struct OmarchyCaptureOcrEngine;

impl OmarchyCaptureOcrEngine {
    pub fn new() -> Self {
        Self
    }

    /// Take screenshot of region and return raw pixel buffer
    pub fn capture_region(&self, rect: ScreenRectangle) -> Result<Vec<u8>, &'static str> {
        if rect.width == 0 || rect.height == 0 {
            return Err("Invalid selection geometry: zero area");
        }
        let total_pixels = (rect.width as usize) * (rect.height as usize);
        // RGBA mock framebuffer capture
        let buffer = vec![0xFF; total_pixels * 4];
        Ok(buffer)
    }

    /// Perform OCR on image region, returning recognized text
    pub fn extract_text_from_region(&self, rect: ScreenRectangle) -> Result<String, &'static str> {
        let _ = self.capture_region(rect)?;
        // Simulated OCR engine: returns recognized text for screen region
        Ok(format!(
            "SigmaOS sovereign terminal output at ({},{}) [{}x{}]",
            rect.x, rect.y, rect.width, rect.height
        ))
    }
}

// ============================================================================
// 4. Power & Battery Longevity Governor
// ============================================================================

/// Power profile modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerProfile {
    Performance,
    Balanced,
    PowerSaver,
}

/// Power source state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerSource {
    AcMains,
    BatteryDischarge,
}

/// Dynamic power profile governor & lithium battery longevity limiter
pub struct OmarchyPowerGovernor {
    pub current_source: PowerSource,
    pub battery_charge_percent: u8,
    pub current_profile: PowerProfile,
    pub charge_threshold_limit_percent: u8, // e.g. 80% to protect battery health
    pub is_threshold_active: bool,
}

impl OmarchyPowerGovernor {
    pub fn new() -> Self {
        Self {
            current_source: PowerSource::AcMains,
            battery_charge_percent: 85,
            current_profile: PowerProfile::Performance,
            charge_threshold_limit_percent: 80,
            is_threshold_active: true,
        }
    }

    /// Notify governor of power source change (e.g. unplugging charger)
    pub fn handle_power_event(&mut self, source: PowerSource, battery_level: u8) -> PowerProfile {
        self.current_source = source;
        self.battery_charge_percent = battery_level;

        self.current_profile = match source {
            PowerSource::AcMains => PowerProfile::Performance,
            PowerSource::BatteryDischarge => {
                if battery_level <= 20 {
                    PowerProfile::PowerSaver
                } else {
                    PowerProfile::Balanced
                }
            }
        };

        self.current_profile
    }

    /// Check if battery charging should be cut off by hardware ACPI controller
    pub fn should_stop_charging(&self) -> bool {
        self.is_threshold_active
            && self.current_source == PowerSource::AcMains
            && self.battery_charge_percent >= self.charge_threshold_limit_percent
    }
}

// ============================================================================
// 5. Tiling Layout Engine (Dwindle & Master-Stack)
// ============================================================================

/// Window geometry in pixels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutGeometry {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

/// Active tiling algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TilingAlgorithm {
    Dwindle,
    MasterStack,
}

/// Smart tiling layout calculation engine
pub struct OmarchyTilingLayoutEngine {
    pub algorithm: TilingAlgorithm,
    pub screen_width: u32,
    pub screen_height: u32,
    pub gap_inner: u32,
    pub gap_outer: u32,
    pub master_ratio: f32, // e.g. 0.55 for 55% screen width
}

impl OmarchyTilingLayoutEngine {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            algorithm: TilingAlgorithm::Dwindle,
            screen_width: width,
            screen_height: height,
            gap_inner: 8,
            gap_outer: 12,
            master_ratio: 0.55,
        }
    }

    /// Compute geometries for N active windows according to the selected layout
    pub fn calculate_layout(&self, window_count: usize) -> Vec<LayoutGeometry> {
        if window_count == 0 {
            return Vec::new();
        }

        let usable_w = self.screen_width.saturating_sub(self.gap_outer * 2);
        let usable_h = self.screen_height.saturating_sub(self.gap_outer * 2);

        // Single window: fills usable area
        if window_count == 1 {
            return vec![LayoutGeometry {
                x: self.gap_outer,
                y: self.gap_outer,
                width: usable_w,
                height: usable_h,
            }];
        }

        match self.algorithm {
            TilingAlgorithm::MasterStack => {
                let mut geoms = Vec::with_capacity(window_count);
                let master_w = ((usable_w as f32 * self.master_ratio) as u32).saturating_sub(self.gap_inner / 2);
                let stack_w = usable_w.saturating_sub(master_w).saturating_sub(self.gap_inner);
                let stack_count = (window_count - 1) as u32;

                // 1. Master Window
                geoms.push(LayoutGeometry {
                    x: self.gap_outer,
                    y: self.gap_outer,
                    width: master_w,
                    height: usable_h,
                });

                // 2. Stack Windows
                let total_gaps = (stack_count - 1) * self.gap_inner;
                let each_h = usable_h.saturating_sub(total_gaps) / stack_count;
                let stack_x = self.gap_outer + master_w + self.gap_inner;

                for i in 0..stack_count {
                    let stack_y = self.gap_outer + i * (each_h + self.gap_inner);
                    geoms.push(LayoutGeometry {
                        x: stack_x,
                        y: stack_y,
                        width: stack_w,
                        height: each_h,
                    });
                }
                geoms
            }
            TilingAlgorithm::Dwindle => {
                // Fibonacci bisection: repeatedly cut in half alternating horizontal / vertical
                let mut geoms = Vec::with_capacity(window_count);
                let mut cur_x = self.gap_outer;
                let mut cur_y = self.gap_outer;
                let mut cur_w = usable_w;
                let mut cur_h = usable_h;

                for i in 0..window_count {
                    if i == window_count - 1 {
                        // Last window takes all remaining area
                        geoms.push(LayoutGeometry {
                            x: cur_x,
                            y: cur_y,
                            width: cur_w,
                            height: cur_h,
                        });
                        break;
                    }

                    if i % 2 == 0 {
                        // Split horizontally (left / right)
                        let half_w = (cur_w.saturating_sub(self.gap_inner)) / 2;
                        geoms.push(LayoutGeometry {
                            x: cur_x,
                            y: cur_y,
                            width: half_w,
                            height: cur_h,
                        });
                        cur_x += half_w + self.gap_inner;
                        cur_w = cur_w.saturating_sub(half_w + self.gap_inner);
                    } else {
                        // Split vertically (top / bottom)
                        let half_h = (cur_h.saturating_sub(self.gap_inner)) / 2;
                        geoms.push(LayoutGeometry {
                            x: cur_x,
                            y: cur_y,
                            width: cur_w,
                            height: half_h,
                        });
                        cur_y += half_h + self.gap_inner;
                        cur_h = cur_h.saturating_sub(half_h + self.gap_inner);
                    }
                }
                geoms
            }
        }
    }
}

// ============================================================================
// Unified Suite Coordinator
// ============================================================================

/// Unified coordinator for Omarchy Linux dynamic workspace suite
pub struct OmarchyDynamicWorkspaceSuite {
    pub scratchpads: OmarchyScratchpadManager,
    pub audio: OmarchyAudioMediaController,
    pub capture: OmarchyCaptureOcrEngine,
    pub power: OmarchyPowerGovernor,
    pub layout: OmarchyTilingLayoutEngine,
}

impl OmarchyDynamicWorkspaceSuite {
    pub fn new(display_width: u32, display_height: u32) -> Self {
        Self {
            scratchpads: OmarchyScratchpadManager::new(),
            audio: OmarchyAudioMediaController::new(),
            capture: OmarchyCaptureOcrEngine::new(),
            power: OmarchyPowerGovernor::new(),
            layout: OmarchyTilingLayoutEngine::new(display_width, display_height),
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scratchpad_toggle() {
        let mut sp_mgr = OmarchyScratchpadManager::new();
        assert_eq!(sp_mgr.active_scratchpad, None);

        let vis = sp_mgr.toggle_scratchpad("terminal");
        assert_eq!(vis, ScratchpadVisibility::Visible);
        assert_eq!(sp_mgr.active_scratchpad.as_deref(), Some("terminal"));

        let vis2 = sp_mgr.toggle_scratchpad("terminal");
        assert_eq!(vis2, ScratchpadVisibility::Hidden);
        assert_eq!(sp_mgr.active_scratchpad, None);
    }

    #[test]
    fn test_audio_and_mpris() {
        let mut audio = OmarchyAudioMediaController::new();
        let vol = audio.set_volume(75);
        assert_eq!(vol, 75);

        let status = audio.toggle_playback();
        assert_eq!(status, PlaybackStatus::Paused);

        let switched = audio.switch_default_sink("bluez_output.WH-1000XM4.a2dp-sink");
        assert!(switched);
    }

    #[test]
    fn test_power_governor() {
        let mut gov = OmarchyPowerGovernor::new();
        assert_eq!(gov.current_profile, PowerProfile::Performance);

        // Unplug AC on battery at 85%
        let prof = gov.handle_power_event(PowerSource::BatteryDischarge, 85);
        assert_eq!(prof, PowerProfile::Balanced);

        // Low battery at 15%
        let low_prof = gov.handle_power_event(PowerSource::BatteryDischarge, 15);
        assert_eq!(low_prof, PowerProfile::PowerSaver);

        // Plugged in at 85% -> should stop charging because limit is 80%
        let _ = gov.handle_power_event(PowerSource::AcMains, 85);
        assert!(gov.should_stop_charging());
    }

    #[test]
    fn test_tiling_layouts() {
        let mut engine = OmarchyTilingLayoutEngine::new(1920, 1080);
        let geoms_1 = engine.calculate_layout(1);
        assert_eq!(geoms_1.len(), 1);

        // Dwindle with 3 windows
        let geoms_3 = engine.calculate_layout(3);
        assert_eq!(geoms_3.len(), 3);

        // Master-Stack with 3 windows
        engine.algorithm = TilingAlgorithm::MasterStack;
        let ms_geoms = engine.calculate_layout(3);
        assert_eq!(ms_geoms.len(), 3);
        // Master window should have width around 55%
        assert!(ms_geoms[0].width > ms_geoms[1].width);
    }
}
