//! Comprehensive tests for SigmaOS-settings (≥ 15 tests).

use sigmaos_settings::{
    SettingsStore, SettingsPanel, launch_settings,
    display::{DisplaySettingsPanel, Orientation},
    sound::SoundPanel,
    network::{NetworkPanel, ConnectionState},
};

// ── Display tests ─────────────────────────────────────────────────────────────

#[test]
fn test_display_resolution_set() {
    let mut panel = DisplaySettingsPanel::default();
    panel.resolution = (2560, 1440);
    let mut store = SettingsStore::default();
    panel.apply(&mut store).unwrap();
    assert_eq!(store.get("display.resolution"), Some("2560x1440"));
}

#[test]
fn test_display_orientation() {
    let mut panel = DisplaySettingsPanel::default();
    panel.orientation = Orientation::Left;
    let mut store = SettingsStore::default();
    panel.apply(&mut store).unwrap();
    assert_eq!(store.get("display.orientation"), Some("left"));
}

#[test]
fn test_display_brightness_validation() {
    let mut panel = DisplaySettingsPanel::default();
    panel.brightness = 150; // invalid
    let mut store = SettingsStore::default();
    assert!(panel.apply(&mut store).is_err());
}

#[test]
fn test_display_night_light() {
    let mut panel = DisplaySettingsPanel::default();
    panel.night_light = true;
    let mut store = SettingsStore::default();
    panel.apply(&mut store).unwrap();
    assert_eq!(store.get("display.night_light"), Some("true"));
}

#[test]
fn test_display_hidpi_scale() {
    let mut panel = DisplaySettingsPanel::default();
    panel.scale = 2.0;
    let mut store = SettingsStore::default();
    panel.apply(&mut store).unwrap();
    assert_eq!(store.get("display.scale"), Some("2"));
}

// ── Sound tests ───────────────────────────────────────────────────────────────

#[test]
fn test_sound_volume() {
    let mut panel = SoundPanel::default();
    panel.output_volume = 60;
    let mut store = SettingsStore::default();
    panel.apply(&mut store).unwrap();
    assert_eq!(store.get("sound.output_volume"), Some("60"));
}

#[test]
fn test_sound_device_select() {
    let mut panel = SoundPanel::default();
    panel.add_output_device("alsa-0", "Built-in Audio");
    panel.add_output_device("usb-dac", "USB DAC");
    panel.set_default_output("usb-dac");
    let mut store = SettingsStore::default();
    panel.apply(&mut store).unwrap();
    assert_eq!(store.get("sound.default_output"), Some("usb-dac"));
}

#[test]
fn test_sound_mute() {
    let mut panel = SoundPanel::default();
    panel.muted = true;
    let mut store = SettingsStore::default();
    panel.apply(&mut store).unwrap();
    assert_eq!(store.get("sound.muted"), Some("true"));
}

// ── Network tests ─────────────────────────────────────────────────────────────

#[test]
fn test_network_connect() {
    let mut panel = NetworkPanel::default();
    panel.wifi_enabled = true;
    panel.scan();
    panel.connect("SigmaOS-5G").unwrap();
    assert_eq!(panel.wifi_state, ConnectionState::Connected);
    assert_eq!(panel.connected_ssid.as_deref(), Some("SigmaOS-5G"));
}

#[test]
fn test_network_scan() {
    let mut panel = NetworkPanel::default();
    panel.scan();
    assert!(!panel.available_networks.is_empty());
}

#[test]
fn test_network_connect_unknown_fails() {
    let mut panel = NetworkPanel::default();
    panel.scan();
    assert!(panel.connect("nonexistent-ssid").is_err());
}

#[test]
fn test_network_disconnect() {
    let mut panel = NetworkPanel::default();
    panel.scan();
    panel.connect("SigmaOS-5G").unwrap();
    panel.disconnect();
    assert_eq!(panel.wifi_state, ConnectionState::Disconnected);
    assert!(panel.connected_ssid.is_none());
}

// ── Privacy / accessibility placeholders ─────────────────────────────────────

#[test]
fn test_privacy_mic_toggle() {
    // Mic toggle stored in settings store.
    let mut store = SettingsStore::default();
    store.set("privacy.microphone_enabled", "false");
    assert_eq!(store.get("privacy.microphone_enabled"), Some("false"));
}

#[test]
fn test_accessibility_font_scale() {
    let mut store = SettingsStore::default();
    store.set("accessibility.font_scale", "1.25");
    assert_eq!(store.get("accessibility.font_scale"), Some("1.25"));
}

#[test]
fn test_accessibility_high_contrast() {
    let mut store = SettingsStore::default();
    store.set("accessibility.high_contrast", "true");
    assert_eq!(store.get("accessibility.high_contrast"), Some("true"));
}

#[test]
fn test_keyboard_layout() {
    let mut store = SettingsStore::default();
    store.set("keyboard.layout", "us");
    assert_eq!(store.get("keyboard.layout"), Some("us"));
}

#[test]
fn test_timezone_set() {
    let mut store = SettingsStore::default();
    store.set("datetime.timezone", "America/New_York");
    assert_eq!(store.get("datetime.timezone"), Some("America/New_York"));
}

#[test]
fn test_power_profile() {
    let mut store = SettingsStore::default();
    store.set("power.profile", "balanced");
    assert_eq!(store.get("power.profile"), Some("balanced"));
}
