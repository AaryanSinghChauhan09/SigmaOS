#![allow(non_camel_case_types)]
// SPDX-License-Identifier: MIT
// SigmaOS Linux & BSD Inspired Trackpad & Multi-Touch Subsystem
// (`src/drivers/sovereign_trackpad_synthesis.rs`)
//
// Zero-dependency, `#![no_std]` compliant Rust trackpad components inspired by:
// - Linux libinput (multi-finger pinch, swipe, tap-to-click, palm rejection, pointer accel)
// - Asahi Linux / Apple Magic Trackpad (multi-finger raw touch tracking & haptic force feedback)
// - Fusuma / Touchegg (Gestures-to-action daemon mapping 3/4 finger swipes to workspace/WM actions)
// - FreeBSD wmt(4) / psm(4) (Synaptics / Elantech hardware packet protocol decoder)
// - OpenBSD wsmouse(4) (Gesture filtering, palm suppression, and edge scrolling governor)
// - SovereignTrackpadSubsystemSuite (Master coordinator unifying all trackpad engines)

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::collections::BTreeMap;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::format;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::collections::BTreeMap;
#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

// ============================================================================
// 1. LIBINPUT MULTI-TOUCH GESTURE ENGINE
// ============================================================================

/// Libinput Gesture Event Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LibinputGestureType {
    SwipeBegin,
    SwipeUpdate,
    SwipeEnd,
    PinchBegin,
    PinchUpdate,
    PinchEnd,
    TapToClick,
}

/// Libinput Trackpad Gesture Event
#[derive(Debug, Clone)]
pub struct LibinputGestureEvent {
    pub gesture_type: LibinputGestureType,
    pub finger_count: u8,
    pub delta_x: f32,
    pub delta_y: f32,
    pub scale: f32,    // Pinch zoom scale (e.g. 1.0 = 100%)
    pub rotation: f32, // Pinch rotation in degrees
}

/// Libinput Trackpad Configuration
#[derive(Debug, Clone)]
pub struct LibinputTrackpadConfig {
    pub tap_to_click_enabled: bool,
    pub natural_scrolling: bool,
    pub accel_profile_adaptive: bool,
    pub palm_rejection_enabled: bool,
    pub accel_speed: f32, // -1.0 to +1.0
}

/// Linux libinput Trackpad & Multi-Touch Processing Engine
pub struct LibinputTrackpadGestureEngine {
    pub config: LibinputTrackpadConfig,
    pub active_gestures: Vec<LibinputGestureEvent>,
    pub palm_suppressed_count: usize,
}

impl LibinputTrackpadGestureEngine {
    pub fn new() -> Self {
        Self {
            config: LibinputTrackpadConfig {
                tap_to_click_enabled: true,
                natural_scrolling: true,
                accel_profile_adaptive: true,
                palm_rejection_enabled: true,
                accel_speed: 0.0,
            },
            active_gestures: Vec::new(),
            palm_suppressed_count: 0,
        }
    }

    pub fn process_raw_touch_contact(&mut self, fingers: u8, touch_major_mm: f32, dx: f32, dy: f32) -> Option<LibinputGestureEvent> {
        // Palm rejection: large contact area (> 18mm) triggers palm suppression
        if self.config.palm_rejection_enabled && touch_major_mm > 18.0 {
            self.palm_suppressed_count += 1;
            return None;
        }

        let direction_mult = if self.config.natural_scrolling { -1.0 } else { 1.0 };
        let event = LibinputGestureEvent {
            gesture_type: if fingers >= 3 { LibinputGestureType::SwipeUpdate } else { LibinputGestureType::TapToClick },
            finger_count: fingers,
            delta_x: dx * direction_mult,
            delta_y: dy * direction_mult,
            scale: 1.0,
            rotation: 0.0,
        };

        self.active_gestures.push(event.clone());
        Some(event)
    }
}

impl Default for LibinputTrackpadGestureEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 2. ASAHI APPLE MAGIC TRACKPAD & HAPTIC ENGINE
// ============================================================================

/// Apple Magic Trackpad Haptic Actuator Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MagicTrackpadHapticMode {
    Off,
    LightClick,
    MediumClick,
    HeavyClick,
    ForcePress,
}

/// Asahi Linux / Apple Magic Trackpad Raw Touch Slot
#[derive(Debug, Clone)]
pub struct MagicTrackpadTouchSlot {
    pub slot_id: u8,
    pub pos_x: u32,
    pub pos_y: u32,
    pub force_grams: u16, // Pressure reading in grams
}

/// Asahi Linux Apple Magic Trackpad SPI/I2C & Haptic Engine
pub struct AsahiAppleMagicTrackpadEngine {
    pub slots: BTreeMap<u8, MagicTrackpadTouchSlot>,
    pub haptic_mode: MagicTrackpadHapticMode,
    pub force_threshold_grams: u16,
}

impl AsahiAppleMagicTrackpadEngine {
    pub fn new() -> Self {
        Self {
            slots: BTreeMap::new(),
            haptic_mode: MagicTrackpadHapticMode::MediumClick,
            force_threshold_grams: 120, // 120g force triggers click
        }
    }

    pub fn update_touch_slot(&mut self, slot: u8, x: u32, y: u32, force: u16) -> Option<MagicTrackpadHapticMode> {
        let touch = MagicTrackpadTouchSlot {
            slot_id: slot,
            pos_x: x,
            pos_y: y,
            force_grams: force,
        };
        self.slots.insert(slot, touch);

        if force >= self.force_threshold_grams * 2 {
            Some(MagicTrackpadHapticMode::ForcePress)
        } else if force >= self.force_threshold_grams {
            Some(self.haptic_mode)
        } else {
            None
        }
    }
}

impl Default for AsahiAppleMagicTrackpadEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 3. FUSUMA / TOUCHEGG GESTURE MAPPER DAEMON
// ============================================================================

/// Fusuma / Touchegg Directional Gesture Command
#[derive(Debug, Clone)]
pub struct FusumaGestureBinding {
    pub fingers: u8,          // 3 or 4
    pub direction: String,     // "up", "down", "left", "right", "in", "out"
    pub action_command: String, // e.g. "hyprctl dispatch workspace e+1"
}

/// Fusuma / Touchegg Multi-Touch Gesture Mapper
pub struct FusumaToucheggGestureMapper {
    pub bindings: Vec<FusumaGestureBinding>,
}

impl FusumaToucheggGestureMapper {
    pub fn new() -> Self {
        let mut mapper = Self { bindings: Vec::new() };
        mapper.load_defaults();
        mapper
    }

    fn load_defaults(&mut self) {
        self.add_binding(3, "up", "workspace_overview");
        self.add_binding(3, "down", "minimize_window");
        self.add_binding(3, "left", "workspace_next");
        self.add_binding(3, "right", "workspace_prev");
        self.add_binding(4, "up", "toggle_fullscreen");
    }

    pub fn add_binding(&mut self, fingers: u8, dir: &str, cmd: &str) {
        self.bindings.push(FusumaGestureBinding {
            fingers,
            direction: dir.to_string(),
            action_command: cmd.to_string(),
        });
    }

    pub fn lookup_action(&self, fingers: u8, dir: &str) -> Option<String> {
        self.bindings
            .iter()
            .find(|b| b.fingers == fingers && b.direction == dir)
            .map(|b| b.action_command.clone())
    }
}

impl Default for FusumaToucheggGestureMapper {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 4. FREEBSD WMT / PSM HARDWARE TRACKPAD DECODER
// ============================================================================

/// FreeBSD Hardware Trackpad Protocol Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BsdTrackpadProtocol {
    SynapticsPs2,
    ElantechI2c,
    WmtWindowsPrecision,
}

/// FreeBSD wmt(4) / psm(4) Hardware Trackpad Decoder
pub struct FreeBsdWmtPsmTrackpadTranslator {
    pub protocol: BsdTrackpadProtocol,
    pub decoded_packets: usize,
}

impl FreeBsdWmtPsmTrackpadTranslator {
    pub fn new(proto: BsdTrackpadProtocol) -> Self {
        Self {
            protocol: proto,
            decoded_packets: 0,
        }
    }

    pub fn decode_raw_packet(&mut self, packet_bytes: &[u8]) -> Result<(i16, i16, bool, bool), String> {
        if packet_bytes.len() < 3 {
            return Err("Incomplete trackpad hardware packet".to_string());
        }

        self.decoded_packets += 1;
        match self.protocol {
            BsdTrackpadProtocol::SynapticsPs2 => {
                let dx = packet_bytes[1] as i16;
                let dy = packet_bytes[2] as i16;
                let left = (packet_bytes[0] & 0x01) != 0;
                let right = (packet_bytes[0] & 0x02) != 0;
                Ok((dx, dy, left, right))
            }
            BsdTrackpadProtocol::ElantechI2c | BsdTrackpadProtocol::WmtWindowsPrecision => {
                let dx = ((packet_bytes[1] as i16) << 8) | (packet_bytes[2] as i16);
                let left = (packet_bytes[0] & 0x01) != 0;
                Ok((dx, 0, left, false))
            }
        }
    }
}

impl Default for FreeBsdWmtPsmTrackpadTranslator {
    fn default() -> Self {
        Self::new(BsdTrackpadProtocol::SynapticsPs2)
    }
}

// ============================================================================
// 5. OPENBSD WSMOUSE GESTURE FILTER & EDGE SCROLLER
// ============================================================================

/// OpenBSD wsmouse(4) Gesture & Edge Scroller Governor
pub struct OpenBsdWsmouseGestureFilter {
    pub edge_scroll_enabled: bool,
    pub right_edge_threshold_x: u32, // X-coordinate boundary for edge scroll
    pub filtered_event_count: usize,
}

impl OpenBsdWsmouseGestureFilter {
    pub fn new(screen_width: u32) -> Self {
        Self {
            edge_scroll_enabled: true,
            right_edge_threshold_x: (screen_width as f32 * 0.92) as u32,
            filtered_event_count: 0,
        }
    }

    pub fn filter_mouse_movement(&mut self, pos_x: u32, dy: i16) -> (i16, bool) {
        self.filtered_event_count += 1;
        if self.edge_scroll_enabled && pos_x >= self.right_edge_threshold_x {
            (dy, true) // Converted to vertical scroll event
        } else {
            (0, false) // Standard movement
        }
    }
}

impl Default for OpenBsdWsmouseGestureFilter {
    fn default() -> Self {
        Self::new(1920)
    }
}

// ============================================================================
// MASTER TRACKPAD COORDINATOR SUITE
// ============================================================================

/// Sovereign Master Trackpad & Multi-Touch Subsystem Suite
pub struct SovereignTrackpadSubsystemSuite {
    pub libinput: LibinputTrackpadGestureEngine,
    pub magic_trackpad: AsahiAppleMagicTrackpadEngine,
    pub fusuma: FusumaToucheggGestureMapper,
    pub freebsd_pckt: FreeBsdWmtPsmTrackpadTranslator,
    pub openbsd_wsmouse: OpenBsdWsmouseGestureFilter,
}

impl SovereignTrackpadSubsystemSuite {
    pub fn new() -> Self {
        Self {
            libinput: LibinputTrackpadGestureEngine::new(),
            magic_trackpad: AsahiAppleMagicTrackpadEngine::new(),
            fusuma: FusumaToucheggGestureMapper::new(),
            freebsd_pckt: FreeBsdWmtPsmTrackpadTranslator::new(BsdTrackpadProtocol::SynapticsPs2),
            openbsd_wsmouse: OpenBsdWsmouseGestureFilter::new(1920),
        }
    }

    pub fn verify_suite(&mut self) -> BTreeMap<String, bool> {
        let mut results = BTreeMap::new();

        // 1. Libinput check
        let lib_evt = self.libinput.process_raw_touch_contact(3, 8.0, 10.0, -5.0);
        results.insert("libinput_gestures".to_string(), lib_evt.is_some());

        // 2. Magic trackpad haptic check
        let haptic = self.magic_trackpad.update_touch_slot(0, 500, 400, 150);
        results.insert("magic_trackpad_haptics".to_string(), haptic == Some(MagicTrackpadHapticMode::MediumClick));

        // 3. Fusuma check
        let action = self.fusuma.lookup_action(3, "up");
        results.insert("fusuma_touchegg_gestures".to_string(), action == Some("workspace_overview".to_string()));

        // 4. FreeBSD decoder check
        let decoded = self.freebsd_pckt.decode_raw_packet(&[0x01, 0x05, 0x0A]);
        results.insert("freebsd_wmt_psm_decoder".to_string(), decoded.is_ok());

        // 5. OpenBSD wsmouse check
        let (scroll, is_edge) = self.openbsd_wsmouse.filter_mouse_movement(1800, 12);
        results.insert("openbsd_wsmouse_edgescroll".to_string(), is_edge && scroll == 12);

        results
    }
}

impl Default for SovereignTrackpadSubsystemSuite {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_libinput_gesture_engine() {
        let mut engine = LibinputTrackpadGestureEngine::new();
        let evt = engine.process_raw_touch_contact(3, 10.0, 5.0, 2.0).unwrap();
        assert_eq!(evt.finger_count, 3);
        assert_eq!(evt.delta_x, -5.0); // Natural scrolling inverted

        // Palm rejection test
        let palm = engine.process_raw_touch_contact(1, 22.0, 0.0, 0.0);
        assert!(palm.is_none());
        assert_eq!(engine.palm_suppressed_count, 1);
    }

    #[test]
    fn test_asahi_magic_trackpad() {
        let mut magic = AsahiAppleMagicTrackpadEngine::new();
        let haptic_click = magic.update_touch_slot(0, 100, 100, 130);
        assert_eq!(haptic_click, Some(MagicTrackpadHapticMode::MediumClick));

        let haptic_force = magic.update_touch_slot(0, 100, 100, 250);
        assert_eq!(haptic_force, Some(MagicTrackpadHapticMode::ForcePress));
    }

    #[test]
    fn test_fusuma_touchegg_mapper() {
        let mapper = FusumaToucheggGestureMapper::new();
        assert_eq!(mapper.lookup_action(3, "up"), Some("workspace_overview".to_string()));
        assert_eq!(mapper.lookup_action(4, "up"), Some("toggle_fullscreen".to_string()));
    }

    #[test]
    fn test_freebsd_openbsd_drivers() {
        let mut freebsd = FreeBsdWmtPsmTrackpadTranslator::new(BsdTrackpadProtocol::SynapticsPs2);
        let (dx, dy, left, right) = freebsd.decode_raw_packet(&[0x01, 0x0A, 0x05]).unwrap();
        assert_eq!(dx, 10);
        assert_eq!(dy, 5);
        assert!(left);
        assert!(!right);

        let mut openbsd = OpenBsdWsmouseGestureFilter::new(1920);
        let (scroll, is_edge) = openbsd.filter_mouse_movement(1850, 15);
        assert!(is_edge);
        assert_eq!(scroll, 15);
    }

    #[test]
    fn test_trackpad_subsystem_suite() {
        let mut suite = SovereignTrackpadSubsystemSuite::new();
        let health = suite.verify_suite();
        assert_eq!(health.len(), 5);
        for (k, v) in health {
            assert!(v, "Trackpad suite health check failed for: {}", k);
        }
    }
}
