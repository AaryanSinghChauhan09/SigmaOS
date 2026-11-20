//! Integration tests — settings persistence (store read/write).

use sigmaos_settings::{launch_settings, SettingsStore, NetworkPanel, SettingsPanel};

#[test]
fn apply_all_writes_expected_keys() {
    let mut app = launch_settings();
    app.apply_all().expect("apply_all must succeed");
    assert!(app.store.get("network.wifi_enabled").is_some());
    assert!(app.store.get("display.brightness").is_some());
}

#[test]
fn store_set_get_roundtrip() {
    let mut store = SettingsStore::default();
    store.set("key", "value");
    assert_eq!(store.get("key"), Some("value"));
}

#[test]
fn store_overwrite_returns_new_value() {
    let mut store = SettingsStore::default();
    store.set("x", "old");
    store.set("x", "new");
    assert_eq!(store.get("x"), Some("new"));
    assert_eq!(store.len(), 1);
}

#[test]
fn network_panel_apply_reflects_wifi_state() {
    let mut panel = NetworkPanel::default();
    panel.wifi_enabled = true;
    let mut store = SettingsStore::default();
    panel.apply(&mut store).unwrap();
    assert_eq!(store.get("network.wifi_enabled"), Some("true"));
}
