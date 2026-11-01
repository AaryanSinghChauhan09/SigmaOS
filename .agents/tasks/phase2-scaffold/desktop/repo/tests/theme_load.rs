//! Theme loading and synthesis tests.
//!
//! Covers `ThemeEngine` registration, activation, removal, and the
//! wallpaper-based colour-scheme synthesis path.

use sigmaos_desktop::{Color, ColorScheme, Theme, ThemeEngine, ThemeError};

// ── helpers ───────────────────────────────────────────────────────────────────

fn custom_theme(name: &str) -> Theme {
    Theme {
        name: name.to_owned(),
        colors: ColorScheme::light_default(),
        font_family: "JetBrains Mono".to_owned(),
        font_size_base: 13,
        border_radius: 4,
        animation_duration_ms: 150,
    }
}

// ── default state ─────────────────────────────────────────────────────────────

#[test]
fn engine_starts_with_sigma_default() {
    let e = ThemeEngine::new();
    assert_eq!(e.active_theme().name, "SigmaDefault");
}

#[test]
fn engine_lists_sigma_default() {
    let e = ThemeEngine::new();
    assert!(e.theme_names().contains(&"SigmaDefault"));
}

// ── register & activate ───────────────────────────────────────────────────────

#[test]
fn register_makes_theme_available() {
    // Arrange
    let mut e = ThemeEngine::new();
    // Act
    e.register(custom_theme("Gruvbox"));
    // Assert
    assert!(e.theme_names().contains(&"Gruvbox"));
}

#[test]
fn activate_registered_theme_succeeds() {
    // Arrange
    let mut e = ThemeEngine::new();
    e.register(custom_theme("Gruvbox"));
    // Act
    let result = e.activate("Gruvbox");
    // Assert
    assert!(result.is_ok());
    assert_eq!(e.active_theme().name, "Gruvbox");
}

#[test]
fn activate_unknown_theme_returns_error() {
    // Arrange
    let mut e = ThemeEngine::new();
    // Act
    let result = e.activate("NonExistent");
    // Assert
    assert_eq!(result, Err(ThemeError::ThemeNotFound));
}

#[test]
fn active_theme_unchanged_after_failed_activate() {
    let mut e = ThemeEngine::new();
    let _ = e.activate("ghost");
    assert_eq!(e.active_theme().name, "SigmaDefault");
}

// ── remove ────────────────────────────────────────────────────────────────────

#[test]
fn remove_custom_theme_succeeds() {
    // Arrange
    let mut e = ThemeEngine::new();
    e.register(custom_theme("Nord"));
    // Act
    let result = e.remove("Nord");
    // Assert
    assert!(result.is_ok());
    assert!(!e.theme_names().contains(&"Nord"));
}

#[test]
fn remove_builtin_returns_error() {
    let mut e = ThemeEngine::new();
    assert_eq!(e.remove("SigmaDefault"), Err(ThemeError::CannotRemoveBuiltin));
}

#[test]
fn remove_unknown_theme_returns_error() {
    let mut e = ThemeEngine::new();
    assert_eq!(e.remove("Ghost"), Err(ThemeError::ThemeNotFound));
}

#[test]
fn remove_active_theme_reverts_to_default() {
    // Arrange
    let mut e = ThemeEngine::new();
    e.register(custom_theme("Monokai"));
    e.activate("Monokai").unwrap();
    // Act
    e.remove("Monokai").unwrap();
    // Assert — active falls back to built-in
    assert_eq!(e.active_theme().name, "SigmaDefault");
}

// ── open-closed: extend without modifying core ────────────────────────────────

#[test]
fn multiple_custom_themes_coexist() {
    let mut e = ThemeEngine::new();
    for name in &["A", "B", "C", "D"] {
        e.register(custom_theme(name));
    }
    let names = e.theme_names();
    for n in &["A", "B", "C", "D", "SigmaDefault"] {
        assert!(names.contains(n), "missing {n}");
    }
}

#[test]
fn register_overwrites_existing_theme_with_same_name() {
    let mut e = ThemeEngine::new();
    e.register(custom_theme("X"));
    let mut replacement = custom_theme("X");
    replacement.font_size_base = 20;
    e.register(replacement);
    // Activate and check property
    e.activate("X").unwrap();
    assert_eq!(e.active_theme().font_size_base, 20);
}

// ── colour helpers ────────────────────────────────────────────────────────────

#[test]
fn color_black_is_not_light() {
    assert!(!Color::rgb(0, 0, 0).is_light());
}

#[test]
fn color_white_is_light() {
    assert!(Color::rgb(255, 255, 255).is_light());
}

#[test]
fn color_blend_midpoint() {
    let black = Color::rgb(0, 0, 0);
    let white = Color::rgb(255, 255, 255);
    let mid = black.blend(white, 0.5);
    // Result should be near grey (not exact due to integer rounding)
    assert!(mid.r > 100 && mid.r < 155);
    assert!(mid.g > 100 && mid.g < 155);
    assert!(mid.b > 100 && mid.b < 155);
}

#[test]
fn color_blend_zero_returns_self() {
    let c = Color::rgb(100, 150, 200);
    let other = Color::rgb(0, 0, 0);
    let result = c.blend(other, 0.0);
    assert_eq!(result, c);
}

#[test]
fn color_blend_one_returns_other() {
    let c = Color::rgb(100, 150, 200);
    let other = Color::rgb(0, 0, 0);
    let result = c.blend(other, 1.0);
    assert_eq!(result, other);
}

// ── image synthesis ───────────────────────────────────────────────────────────

#[test]
fn synthesise_from_minimal_image_succeeds() {
    let e = ThemeEngine::new();
    // 4 bytes = one ARGB8888 pixel
    let data = [0xFFu8, 0x20, 0x80, 0xC0];
    let result = e.synthesise_from_image(&data);
    assert!(result.is_ok());
}

#[test]
fn synthesise_from_empty_image_returns_error() {
    let e = ThemeEngine::new();
    let result = e.synthesise_from_image(&[]);
    assert_eq!(result, Err(ThemeError::InvalidImage));
}

#[test]
fn synthesise_from_gradient_produces_distinct_bg_fg() {
    let e = ThemeEngine::new();
    // Two pixels: near-black and near-white (ARGB)
    let data = [0xFF, 0x10, 0x10, 0x10, 0xFF, 0xF0, 0xF0, 0xF0];
    let scheme = e.synthesise_from_image(&data).unwrap();
    // Background should be darker than foreground
    assert!(scheme.background.luminance() < scheme.foreground.luminance());
}
