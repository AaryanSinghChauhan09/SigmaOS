// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// userland/desktop/sigma_theme.rs — Zenith Theme Engine (glassmorphism)
// Language: Rust (std) — OOP via ThemeEngine + Theme trait

use std::collections::BTreeMap;

// ── Color ─────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color { pub r: u8, pub g: u8, pub b: u8, pub a: u8 }

impl Color {
    pub const fn rgba(r:u8,g:u8,b:u8,a:u8) -> Self { Self{r,g,b,a} }
    pub const fn rgb (r:u8,g:u8,b:u8)     -> Self { Self{r,g,b,a:255} }
    pub const fn hex(v: u32) -> Self {
        Self{ r:((v>>16)&0xFF) as u8, g:((v>>8)&0xFF) as u8,
              b:(v&0xFF) as u8, a:255 }
    }
    pub fn blend(&self, other: &Color, t: f32) -> Color {
        let lerp = |a:u8,b:u8| -> u8 { ((a as f32)*(1.0-t) + (b as f32)*t) as u8 };
        Color::rgba(lerp(self.r,other.r), lerp(self.g,other.g),
                    lerp(self.b,other.b), lerp(self.a,other.a))
    }
    pub fn with_alpha(&self, a: u8) -> Color { Color::rgba(self.r,self.g,self.b,a) }
    pub fn to_bgra_u32(&self) -> u32 {
        ((self.a as u32)<<24)|((self.r as u32)<<16)|((self.g as u32)<<8)|(self.b as u32)
    }
}

// ── Built-in Color Palette ────────────────────────────────────────────────────

pub struct Palette;
impl Palette {
    pub const BG_DARK:   Color = Color::hex(0x07080C);
    pub const CYAN:      Color = Color::hex(0x45F3FF);
    pub const PURPLE:    Color = Color::hex(0xA855F7);
    pub const GREEN:     Color = Color::hex(0x34D399);
    pub const YELLOW:    Color = Color::hex(0xFBBF24);
    pub const RED:       Color = Color::hex(0xF87171);
    pub const WHITE:     Color = Color::hex(0xF0F2F8);
    pub const MUTED:     Color = Color::hex(0x6B7280);
    pub const GLASS_BG:  Color = Color::rgba(31,33,42, 153);  // 60% opacity
    pub const GLASS_BDR: Color = Color::rgba(255,255,255, 23); // 9% opacity
}

// ── Theme Trait ───────────────────────────────────────────────────────────────

pub trait Theme: Send + Sync {
    fn name(&self)          -> &'static str;
    fn background(&self)    -> Color;
    fn surface(&self)       -> Color;
    fn border(&self)        -> Color;
    fn accent(&self)        -> Color;
    fn accent_secondary(&self) -> Color;
    fn text(&self)          -> Color;
    fn text_muted(&self)    -> Color;
    fn success(&self)       -> Color;
    fn warning(&self)       -> Color;
    fn error(&self)         -> Color;
    fn corner_radius(&self) -> u32;
    fn blur_radius(&self)   -> u32;
    fn gap(&self)           -> u32;
    fn is_dark(&self)       -> bool;
}

// ── Zenith Dark Theme ─────────────────────────────────────────────────────────

pub struct ZenithDark;
impl Theme for ZenithDark {
    fn name(&self)           -> &'static str { "zenith-dark" }
    fn background(&self)     -> Color { Palette::BG_DARK }
    fn surface(&self)        -> Color { Palette::GLASS_BG }
    fn border(&self)         -> Color { Palette::GLASS_BDR }
    fn accent(&self)         -> Color { Palette::CYAN }
    fn accent_secondary(&self) -> Color { Palette::PURPLE }
    fn text(&self)           -> Color { Palette::WHITE }
    fn text_muted(&self)     -> Color { Palette::MUTED }
    fn success(&self)        -> Color { Palette::GREEN }
    fn warning(&self)        -> Color { Palette::YELLOW }
    fn error(&self)          -> Color { Palette::RED }
    fn corner_radius(&self)  -> u32 { 12 }
    fn blur_radius(&self)    -> u32 { 16 }
    fn gap(&self)            -> u32 { 8 }
    fn is_dark(&self)        -> bool { true }
}

// ── Zenith Light Theme ────────────────────────────────────────────────────────

pub struct ZenithLight;
impl Theme for ZenithLight {
    fn name(&self)           -> &'static str { "zenith-light" }
    fn background(&self)     -> Color { Color::hex(0xF5F7FA) }
    fn surface(&self)        -> Color { Color::rgba(255,255,255,230) }
    fn border(&self)         -> Color { Color::rgba(0,0,0,18) }
    fn accent(&self)         -> Color { Color::hex(0x0EA5E9) }
    fn accent_secondary(&self) -> Color { Color::hex(0x8B5CF6) }
    fn text(&self)           -> Color { Color::hex(0x0F172A) }
    fn text_muted(&self)     -> Color { Color::hex(0x64748B) }
    fn success(&self)        -> Color { Color::hex(0x10B981) }
    fn warning(&self)        -> Color { Color::hex(0xF59E0B) }
    fn error(&self)          -> Color { Color::hex(0xEF4444) }
    fn corner_radius(&self)  -> u32 { 10 }
    fn blur_radius(&self)    -> u32 { 12 }
    fn gap(&self)            -> u32 { 8 }
    fn is_dark(&self)        -> bool { false }
}

// ── High Contrast Theme (accessibility) ──────────────────────────────────────

pub struct HighContrast;
impl Theme for HighContrast {
    fn name(&self)           -> &'static str { "high-contrast" }
    fn background(&self)     -> Color { Color::hex(0x000000) }
    fn surface(&self)        -> Color { Color::hex(0x1A1A1A) }
    fn border(&self)         -> Color { Color::hex(0xFFFFFF) }
    fn accent(&self)         -> Color { Color::hex(0xFFFF00) }
    fn accent_secondary(&self) -> Color { Color::hex(0xFF8000) }
    fn text(&self)           -> Color { Color::hex(0xFFFFFF) }
    fn text_muted(&self)     -> Color { Color::hex(0xCCCCCC) }
    fn success(&self)        -> Color { Color::hex(0x00FF00) }
    fn warning(&self)        -> Color { Color::hex(0xFFFF00) }
    fn error(&self)          -> Color { Color::hex(0xFF0000) }
    fn corner_radius(&self)  -> u32 { 0 }
    fn blur_radius(&self)    -> u32 { 0 }
    fn gap(&self)            -> u32 { 4 }
    fn is_dark(&self)        -> bool { true }
}

// ── Theme Engine ──────────────────────────────────────────────────────────────

pub struct ThemeEngine {
    current:       Box<dyn Theme>,
    custom_colors: BTreeMap<String, Color>,
    auto_switch:   bool,    // switch based on time of day
    scale:         f32,     // UI scale factor
}

impl ThemeEngine {
    pub fn new() -> Self {
        Self {
            current:       Box::new(ZenithDark),
            custom_colors: BTreeMap::new(),
            auto_switch:   false,
            scale:         1.0,
        }
    }

    pub fn set_theme(&mut self, name: &str) {
        self.current = match name {
            "zenith-light"   => Box::new(ZenithLight),
            "high-contrast"  => Box::new(HighContrast),
            _                => Box::new(ZenithDark),
        };
    }

    pub fn set_accent(&mut self, color: Color) {
        self.custom_colors.insert("accent".to_owned(), color);
    }

    pub fn get_color(&self, role: &str) -> Color {
        if let Some(&c) = self.custom_colors.get(role) { return c; }
        match role {
            "background" => self.current.background(),
            "surface"    => self.current.surface(),
            "border"     => self.current.border(),
            "accent"     => self.current.accent(),
            "accent2"    => self.current.accent_secondary(),
            "text"       => self.current.text(),
            "muted"      => self.current.text_muted(),
            "success"    => self.current.success(),
            "warning"    => self.current.warning(),
            "error"      => self.current.error(),
            _            => Palette::WHITE,
        }
    }

    pub fn set_scale(&mut self, scale: f32) { self.scale = scale.clamp(0.5, 3.0); }
    pub fn scale(&self) -> f32 { self.scale }
    pub fn theme(&self) -> &dyn Theme { self.current.as_ref() }

    /// Apply auto-switch: dark from 19:00–07:00, light otherwise
    pub fn auto_switch_by_hour(&mut self, hour: u8) {
        if !self.auto_switch { return; }
        if hour >= 19 || hour < 7 { self.set_theme("zenith-dark"); }
        else                      { self.set_theme("zenith-light"); }
    }

    /// Generate CSS variables string for web UI
    pub fn to_css_vars(&self) -> String {
        let t = self.current.as_ref();
        let c = |col: Color| format!("rgb({},{},{})", col.r, col.g, col.b);
        format!(
            ":root {{--bg:{};--surface:{};--accent:{};--text:{};--muted:{};--border:{};--radius:{}px;}}",
            c(t.background()), c(t.surface()), c(t.accent()),
            c(t.text()), c(t.text_muted()), c(t.border()), t.corner_radius()
        )
    }
}
