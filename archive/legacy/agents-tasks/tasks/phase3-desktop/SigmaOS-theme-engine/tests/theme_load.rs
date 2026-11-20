//! Integration tests — theme loading.

use sigmaos_theme_engine::{load_theme, ThemeError};

#[test]
fn default_theme_loads() {
    let engine = load_theme("default").expect("default theme must load");
    assert_eq!(engine.name, "default");
}

#[test]
fn dark_theme_alias_loads() {
    let engine = load_theme("dark").expect("dark theme must load");
    assert!(!engine.palette.is_empty());
}

#[test]
fn light_theme_loads() {
    let engine = load_theme("light").expect("light theme must load");
    let bg = engine.palette.get("background").expect("background must exist");
    assert_eq!(bg.r, 255); // white background
}

#[test]
fn unknown_theme_is_error() {
    let result = load_theme("gruvbox-dark-extra");
    assert!(matches!(result, Err(ThemeError::ThemeNotFound(_))));
}

#[test]
fn default_palette_has_required_tokens() {
    let engine = load_theme("default").unwrap();
    assert!(engine.palette.get("background").is_some());
    assert!(engine.palette.get("foreground").is_some());
    assert!(engine.palette.get("accent").is_some());
}
