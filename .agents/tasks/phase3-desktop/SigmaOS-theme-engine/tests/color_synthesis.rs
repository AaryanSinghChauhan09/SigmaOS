//! Integration tests — colour synthesis and rendering.

use sigmaos_theme_engine::{Color, ColorPalette, ThemeRenderer};

#[test]
fn palette_set_and_get_roundtrip() {
    let mut palette = ColorPalette::default();
    let red = Color::rgb(255, 0, 0);
    palette.set("error", red);
    let retrieved = palette.get("error").expect("error token must exist");
    assert_eq!(retrieved, red);
}

#[test]
fn color_blend_symmetric() {
    let a = Color::rgb(100, 0, 0);
    let b = Color::rgb(200, 0, 0);
    let ab = a.blend(&b);
    let ba = b.blend(&a);
    assert_eq!(ab, ba);
}

#[test]
fn renderer_produces_hex_string() {
    let mut palette = ColorPalette::default();
    palette.set("bg", Color::rgb(0, 0, 0));
    let renderer = ThemeRenderer::default();
    let css = renderer.render_css(&palette);
    let hex = css.get("bg").expect("bg must be present");
    assert_eq!(hex, "#000000FF");
}

#[test]
fn palette_len_tracks_insertions() {
    let mut palette = ColorPalette::default();
    assert_eq!(palette.len(), 0);
    palette.set("a", Color::rgb(1, 2, 3));
    palette.set("b", Color::rgb(4, 5, 6));
    assert_eq!(palette.len(), 2);
}

#[test]
fn palette_overwrite_same_key() {
    let mut palette = ColorPalette::default();
    palette.set("key", Color::rgb(10, 10, 10));
    palette.set("key", Color::rgb(20, 20, 20));
    assert_eq!(palette.len(), 1);
    assert_eq!(palette.get("key").unwrap().r, 20);
}
