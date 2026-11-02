//! Palette generation for SigmaOS-theme-engine.
//!
//! Given an accent color, generates a full tonal palette (light → dark shades)
//! suitable for populating a `ColorPalette`.

use crate::Color;

/// Generate a 5-step tonal scale from a base accent color.
/// Returns [lightest, light, base, dark, darkest].
pub fn generate_tonal_scale(base: Color) -> [Color; 5] {
    let lighten = |c: u8, amount: i16| -> u8 {
        (c as i16 + amount).clamp(0, 255) as u8
    };
    let darken = |c: u8, amount: i16| -> u8 {
        (c as i16 - amount).clamp(0, 255) as u8
    };

    [
        Color::rgb(lighten(base.r, 80), lighten(base.g, 80), lighten(base.b, 80)),
        Color::rgb(lighten(base.r, 40), lighten(base.g, 40), lighten(base.b, 40)),
        base,
        Color::rgb(darken(base.r, 40), darken(base.g, 40), darken(base.b, 40)),
        Color::rgb(darken(base.r, 80), darken(base.g, 80), darken(base.b, 80)),
    ]
}

/// Perceived luminance (0.0 – 1.0) using the sRGB formula.
pub fn luminance(c: Color) -> f32 {
    let linearize = |v: u8| -> f32 {
        let f = v as f32 / 255.0;
        if f <= 0.03928 { f / 12.92 } else { ((f + 0.055) / 1.055).powf(2.4) }
    };
    0.2126 * linearize(c.r) + 0.7152 * linearize(c.g) + 0.0722 * linearize(c.b)
}

/// WCAG 2.1 contrast ratio between two colors.
/// Returns a value in [1.0, 21.0].
pub fn contrast_ratio(a: Color, b: Color) -> f32 {
    let la = luminance(a);
    let lb = luminance(b);
    let (lighter, darker) = if la > lb { (la, lb) } else { (lb, la) };
    (lighter + 0.05) / (darker + 0.05)
}

/// Check if two colors meet the WCAG AA normal-text threshold (≥ 4.5).
pub fn meets_wcag_aa(fg: Color, bg: Color) -> bool {
    contrast_ratio(fg, bg) >= 4.5
}

/// Generate a dark-mode palette from an accent color.
/// Returns (background, surface, accent, on_accent, foreground).
pub fn dark_palette_from_accent(accent: Color) -> (Color, Color, Color, Color, Color) {
    let scale = generate_tonal_scale(accent);
    let background = Color::rgb(18, 18, 18);
    let surface = Color::rgb(30, 30, 30);
    let on_accent = if luminance(scale[2]) > 0.5 {
        Color::rgb(0, 0, 0)
    } else {
        Color::rgb(255, 255, 255)
    };
    let foreground = Color::rgb(220, 220, 220);
    (background, surface, scale[2], on_accent, foreground)
}
