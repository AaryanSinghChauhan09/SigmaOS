//! Integration tests — panel loading.

use sigmaos_settings::{launch_settings, NetworkPanel, DisplayPanel, SettingsPanel};

#[test]
fn default_launch_has_network_and_display_panels() {
    let app = launch_settings();
    assert!(app.panel("network").is_some());
    assert!(app.panel("display").is_some());
}

#[test]
fn panel_ids_are_correct() {
    let net = NetworkPanel::default();
    let disp = DisplayPanel::default();
    assert_eq!(net.id(), "network");
    assert_eq!(disp.id(), "display");
}

#[test]
fn panel_titles_are_non_empty() {
    let net = NetworkPanel::default();
    let disp = DisplayPanel::default();
    assert!(!net.title().is_empty());
    assert!(!disp.title().is_empty());
}

#[test]
fn unknown_panel_returns_none() {
    let app = launch_settings();
    assert!(app.panel("bluetooth").is_none());
}
