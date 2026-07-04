// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// userland/desktop/sigma_accessibility.rs — Accessibility Subsystem
// Language: Rust (std) — OOP via AccessibilityManager + A11yFeature trait

use crate::userland::desktop::sigma_theme::{Color, Palette};

// ── A11y Feature Trait ────────────────────────────────────────────────────────
pub trait A11yFeature: Send {
    fn id(&self)     -> &'static str;
    fn name(&self)   -> &'static str;
    fn enabled(&self) -> bool;
    fn enable(&mut self);
    fn disable(&mut self);
    fn toggle(&mut self) { if self.enabled() { self.disable() } else { self.enable() } }
}

// ── High Contrast ─────────────────────────────────────────────────────────────
pub struct HighContrastFeature { pub active: bool }
impl A11yFeature for HighContrastFeature {
    fn id(&self)     -> &'static str { "high_contrast" }
    fn name(&self)   -> &'static str { "High Contrast Mode" }
    fn enabled(&self) -> bool { self.active }
    fn enable(&mut self)  { self.active = true; }
    fn disable(&mut self) { self.active = false; }
}

// ── Large Text ────────────────────────────────────────────────────────────────
pub struct LargeTextFeature { pub active: bool, pub scale: f32 }
impl LargeTextFeature { pub fn new() -> Self { Self { active: false, scale: 1.5 } } }
impl A11yFeature for LargeTextFeature {
    fn id(&self)     -> &'static str { "large_text" }
    fn name(&self)   -> &'static str { "Large Text" }
    fn enabled(&self) -> bool { self.active }
    fn enable(&mut self)  { self.active = true; }
    fn disable(&mut self) { self.active = false; }
}

// ── Reduce Motion ─────────────────────────────────────────────────────────────
pub struct ReduceMotionFeature { pub active: bool }
impl A11yFeature for ReduceMotionFeature {
    fn id(&self)     -> &'static str { "reduce_motion" }
    fn name(&self)   -> &'static str { "Reduce Motion" }
    fn enabled(&self) -> bool { self.active }
    fn enable(&mut self)  { self.active = true; }
    fn disable(&mut self) { self.active = false; }
}

// ── Screen Reader (stub) ──────────────────────────────────────────────────────
pub struct ScreenReaderFeature { pub active: bool, pub speech_rate: f32 }
impl ScreenReaderFeature { pub fn new() -> Self { Self { active: false, speech_rate: 1.0 } } }
impl A11yFeature for ScreenReaderFeature {
    fn id(&self)     -> &'static str { "screen_reader" }
    fn name(&self)   -> &'static str { "Screen Reader" }
    fn enabled(&self) -> bool { self.active }
    fn enable(&mut self)  { self.active = true; eprintln!("[a11y] screen reader activated"); }
    fn disable(&mut self) { self.active = false; }
}

// ── Colour Blind Mode ─────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ColourBlindMode { None, Deuteranopia, Protanopia, Tritanopia }

pub struct ColourBlindFeature { pub mode: ColourBlindMode }
impl A11yFeature for ColourBlindFeature {
    fn id(&self)     -> &'static str { "colour_blind" }
    fn name(&self)   -> &'static str { "Colour Blind Mode" }
    fn enabled(&self) -> bool { self.mode != ColourBlindMode::None }
    fn enable(&mut self)  { self.mode = ColourBlindMode::Deuteranopia; }
    fn disable(&mut self) { self.mode = ColourBlindMode::None; }
}

impl ColourBlindFeature {
    /// Simulate colour blindness by remapping a colour
    pub fn remap(&self, c: Color) -> Color {
        match self.mode {
            ColourBlindMode::None => c,
            ColourBlindMode::Deuteranopia => {
                // Deuteranopia: red-green weakness — boost blue, reduce green
                Color::rgba(c.r, (c.g as u32 * 70 / 100) as u8, ((c.b as u32 + c.g as u32 * 30 / 100) as u32).min(255) as u8, c.a)
            }
            ColourBlindMode::Protanopia => {
                // Protanopia: red weakness
                Color::rgba((c.r as u32 * 60 / 100) as u8, c.g, ((c.b as u32 + c.r as u32 * 40 / 100) as u32).min(255) as u8, c.a)
            }
            ColourBlindMode::Tritanopia => {
                // Tritanopia: blue-yellow weakness
                Color::rgba(c.r, ((c.g as u32 + c.b as u32 * 30 / 100) as u32).min(255) as u8, (c.b as u32 * 60 / 100) as u8, c.a)
            }
        }
    }
}

// ── Sticky Keys ───────────────────────────────────────────────────────────────
pub struct StickyKeysFeature {
    pub active:  bool,
    modifiers:   u8,   // bitmask of stuck modifiers
}
impl StickyKeysFeature { pub fn new() -> Self { Self { active: false, modifiers: 0 } } }
impl A11yFeature for StickyKeysFeature {
    fn id(&self)     -> &'static str { "sticky_keys" }
    fn name(&self)   -> &'static str { "Sticky Keys" }
    fn enabled(&self) -> bool { self.active }
    fn enable(&mut self)  { self.active = true; }
    fn disable(&mut self) { self.active = false; self.modifiers = 0; }
}
impl StickyKeysFeature {
    /// Called on modifier key press — latch the modifier
    pub fn press_modifier(&mut self, mod_bit: u8) {
        if !self.active { return; }
        self.modifiers |= mod_bit;
    }
    /// Called on normal key press — return latched modifiers and clear
    pub fn consume_modifiers(&mut self) -> u8 {
        let m = self.modifiers; self.modifiers = 0; m
    }
}

// ── Accessibility Manager ─────────────────────────────────────────────────────
pub struct AccessibilityManager {
    pub high_contrast:  HighContrastFeature,
    pub large_text:     LargeTextFeature,
    pub reduce_motion:  ReduceMotionFeature,
    pub screen_reader:  ScreenReaderFeature,
    pub colour_blind:   ColourBlindFeature,
    pub sticky_keys:    StickyKeysFeature,
    pub cursor_size:    u32,
    pub focus_ring_px:  u32,
}

impl AccessibilityManager {
    pub fn new() -> Self {
        Self {
            high_contrast:  HighContrastFeature { active: false },
            large_text:     LargeTextFeature::new(),
            reduce_motion:  ReduceMotionFeature { active: false },
            screen_reader:  ScreenReaderFeature::new(),
            colour_blind:   ColourBlindFeature { mode: ColourBlindMode::None },
            sticky_keys:    StickyKeysFeature::new(),
            cursor_size:    16,
            focus_ring_px:  3,
        }
    }

    /// Apply colour remapping (used in renderer)
    pub fn remap_color(&self, c: Color) -> Color {
        if self.high_contrast.active { return self.high_contrast_remap(c); }
        self.colour_blind.remap(c)
    }

    fn high_contrast_remap(&self, c: Color) -> Color {
        // Threshold to pure black/white/yellow based on luminance
        let luma = (c.r as u32 * 299 + c.g as u32 * 587 + c.b as u32 * 114) / 1000;
        if luma > 128 { Palette::WHITE } else { Color::hex(0x000000) }
    }

    pub fn text_scale(&self) -> f32 {
        if self.large_text.active { self.large_text.scale } else { 1.0 }
    }

    pub fn animations_enabled(&self) -> bool { !self.reduce_motion.active }

    pub fn announce(&self, text: &str) {
        if self.screen_reader.active { eprintln!("[screen-reader] {}", text); }
    }

    pub fn any_active(&self) -> bool {
        self.high_contrast.active || self.large_text.active
            || self.reduce_motion.active || self.screen_reader.active
            || self.sticky_keys.active || self.colour_blind.enabled()
    }
}
