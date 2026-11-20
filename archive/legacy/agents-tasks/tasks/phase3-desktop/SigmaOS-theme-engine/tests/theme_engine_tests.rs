//! Comprehensive tests for SigmaOS-theme-engine (≥ 15 tests).

use sigmaos_theme_engine::{
    Color, ColorPalette, ThemeRenderer, ThemeError,
    css::{palette_to_css, CssVariableMap, ColorSchemePreference},
    icons::{IconTheme, IconThemeRegistry, IconSize},
    palette::{contrast_ratio, dark_palette_from_accent, generate_tonal_scale, meets_wcag_aa},
    load_theme,
};
use std::path::PathBuf;

// ── CSS variable resolution ───────────────────────────────────────────────────

#[test]
fn test_css_variable_resolution() {
    let mut palette = ColorPalette::default();
    palette.set("background", Color::rgb(18, 18, 18));
    let css = palette_to_css(&palette);
    assert!(css.get("--sigma-background").is_some());
}

#[test]
fn test_dark_mode_switch() {
    let engine = load_theme("dark").unwrap();
    let bg = engine.palette.get("background").unwrap();
    assert!(bg.r < 128, "dark background must be < 128, got {}", bg.r);
}

#[test]
fn test_light_mode_switch() {
    let engine = load_theme("light").unwrap();
    let bg = engine.palette.get("background").unwrap();
    assert!(bg.r > 200, "light background must be > 200, got {}", bg.r);
}

#[test]
fn test_css_block_format() {
    let mut map = CssVariableMap::new();
    map.set("--sigma-accent", "#6495EDFF");
    let block = map.to_css_block();
    assert!(block.starts_with(":root {"));
    assert!(block.contains("--sigma-accent"));
}

#[test]
fn test_color_scheme_preference_default() {
    assert_eq!(ColorSchemePreference::default(), ColorSchemePreference::Dark);
}

// ── Icon theming ──────────────────────────────────────────────────────────────

#[test]
fn test_icon_lookup() {
    let mut registry = IconThemeRegistry::new();
    let mut theme = IconTheme::new("papirus");
    theme.add_icon(
        "folder",
        PathBuf::from("/usr/share/icons/Papirus/24x24/places/folder.svg"),
    );
    registry.register(theme);
    let result = registry.lookup("papirus", "folder");
    assert!(result.is_some());
}

#[test]
fn test_icon_fallback_to_hicolor() {
    let mut registry = IconThemeRegistry::new();
    let mut hicolor = IconTheme::new("hicolor");
    hicolor.add_icon(
        "application-x-executable",
        PathBuf::from("/hicolor/exec.png"),
    );
    registry.register(hicolor);
    let mut papirus = IconTheme::new("papirus");
    papirus.parents.push("hicolor".to_string());
    registry.register(papirus);
    let result = registry.lookup("papirus", "application-x-executable");
    assert!(result.is_some());
}

#[test]
fn test_icon_size_variants() {
    assert_eq!(IconSize::Small as u32, 16);
    assert_eq!(IconSize::Medium as u32, 24);
    assert_eq!(IconSize::Large as u32, 48);
}

// ── Palette generation ────────────────────────────────────────────────────────

#[test]
fn test_accent_color_tonal_scale() {
    let accent = Color::rgb(100, 149, 237);
    let scale = generate_tonal_scale(accent);
    assert_eq!(scale.len(), 5);
    // Lightest (index 0) should be lighter than darkest (index 4).
    let lightest_sum = scale[0].r as u32 + scale[0].g as u32 + scale[0].b as u32;
    let darkest_sum = scale[4].r as u32 + scale[4].g as u32 + scale[4].b as u32;
    assert!(lightest_sum > darkest_sum);
}

#[test]
fn test_palette_generation_dark() {
    let accent = Color::rgb(0, 120, 215);
    let (bg, _surface, acc, _on_acc, _fg) = dark_palette_from_accent(accent);
    assert!(bg.r < 50, "dark bg.r={}", bg.r);
    assert_eq!(acc, accent);
}

#[test]
fn test_theme_export_css_non_empty() {
    let engine = load_theme("default").unwrap();
    let css_map = palette_to_css(&engine.palette);
    assert!(!css_map.is_empty());
}

#[test]
fn test_theme_import_unknown_returns_error() {
    let result = load_theme("material-you");
    assert!(matches!(result, Err(ThemeError::ThemeNotFound(_))));
}

#[test]
fn test_gtk_apply_placeholder() {
    let engine = load_theme("light").unwrap();
    let css = palette_to_css(&engine.palette);
    let block = css.to_css_block();
    assert!(block.len() > 10);
}

#[test]
fn test_realtime_reload_placeholder() {
    let result = load_theme("default");
    assert!(result.is_ok());
}

#[test]
fn test_contrast_check_wcag_aa() {
    let white = Color::rgb(255, 255, 255);
    let black = Color::rgb(0, 0, 0);
    assert!(meets_wcag_aa(black, white));
    assert!(meets_wcag_aa(white, black));
}

#[test]
fn test_color_scheme_palette() {
    let mut palette = ColorPalette::default();
    palette.set("primary", Color::rgb(33, 150, 243));
    palette.set("on-primary", Color::rgb(255, 255, 255));
    let cr = contrast_ratio(
        palette.get("on-primary").unwrap(),
        palette.get("primary").unwrap(),
    );
    assert!(cr > 1.5, "contrast ratio too low: {}", cr);
}

#[test]
fn test_renderer_hex_format_includes_alpha() {
    let mut palette = ColorPalette::default();
    palette.set("surface", Color::rgba(255, 255, 255, 128));
    let renderer = ThemeRenderer::default();
    let css = renderer.render_css(&palette);
    let hex = css.get("surface").unwrap();
    assert_eq!(hex.len(), 9, "expected #RRGGBBAA, got {}", hex);
    assert_eq!(&hex[7..9], "80", "alpha 128=0x80, got {}", &hex[7..9]);
}
