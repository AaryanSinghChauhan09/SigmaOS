// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// desktop/zenith_accessibility.rs — Zenith Accessibility Features
//
// Implements accessibility features including high-contrast themes (WCAG AAA compliant),
// screen magnifier (2-16x zoom), and keyboard-only navigation.
//
// Language: Rust (std for userland services)

// ─── Theme Types ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Theme {
    Default,
    HighContrast,
    HighContrastDark,
    HighContrastLight,
}

#[derive(Debug, Clone)]
pub struct ThemeColors {
    pub background: [u8; 4],
    pub foreground: [u8; 4],
    pub accent: [u8; 4],
    pub border: [u8; 4],
    pub selection: [u8; 4],
}

// ─── Magnifier State ───────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Magnifier {
    pub enabled: bool,
    pub zoom_level: f32, // 2.0 to 16.0
    pub x: i32,
    pub y: i32,
    pub follow_cursor: bool,
}

// ─── Keyboard Navigation State ─────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct KeyboardNav {
    pub enabled: bool,
    pub focus_ring_visible: bool,
    pub focus_ring_color: [u8; 4],
    pub focus_ring_width: u32,
    pub tab_navigation: bool,
}

// ─── Screen Reader Implementation ───────────────────────────────────────────────

/// Screen reader text-to-speech engine
pub struct ScreenReader {
    pub enabled: bool,
    pub voice: String,
    pub rate: f32, // 0.5 to 2.0
    pub pitch: f32, // 0.5 to 2.0
    pub volume: f32, // 0.0 to 1.0
}

impl ScreenReader {
    pub fn new() -> Self {
        ScreenReader {
            enabled: false,
            voice: "default".to_string(),
            rate: 1.0,
            pitch: 1.0,
            volume: 1.0,
        }
    }

    /// Speak text
    pub fn speak(&self, text: &str) {
        if !self.enabled {
            return;
        }
        // In real implementation, use TTS engine to speak text
        // For now, this is a placeholder
    }

    /// Stop speaking
    pub fn stop(&self) {
        // In real implementation, stop TTS engine
    }

    /// Set voice
    pub fn set_voice(&mut self, voice: &str) {
        self.voice = voice.to_string();
    }

    /// Set speech rate
    pub fn set_rate(&mut self, rate: f32) {
        self.rate = rate.clamp(0.5, 2.0);
    }

    /// Set speech pitch
    pub fn set_pitch(&mut self, pitch: f32) {
        self.pitch = pitch.clamp(0.5, 2.0);
    }

    /// Set volume
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }
}

/// Add screen reader to accessibility manager
impl AccessibilityManager {
    pub fn new() -> Self {
        AccessibilityManager {
            theme: Theme::Default,
            magnifier: Magnifier {
                enabled: false,
                zoom_level: 2.0,
                x: 0,
                y: 0,
                follow_cursor: true,
            },
            keyboard_nav: KeyboardNav {
                enabled: false,
                focus_ring_visible: true,
                focus_ring_color: [0x00, 0xFF, 0x00, 0xFF], // Bright green
                focus_ring_width: 3,
                tab_navigation: true,
            },
            screen_reader_enabled: false,
            reduced_motion: false,
            initialized: false,
        }
    }

    /// Initialize accessibility manager
    pub fn init(&mut self) {
        self.initialized = true;
    }

    /// Set theme
    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }

    /// Get theme colors (WCAG AAA compliant)
    pub fn get_theme_colors(&self) -> ThemeColors {
        match self.theme {
            Theme::Default => ThemeColors {
                background: [0x20, 0x20, 0x30, 0xFF],
                foreground: [0xFF, 0xFF, 0xFF, 0xFF],
                accent: [0x00, 0xFF, 0xFF, 0xFF],
                border: [0x40, 0x40, 0x50, 0xFF],
                selection: [0x00, 0x80, 0xFF, 0xFF],
            },
            Theme::HighContrast => ThemeColors {
                background: [0x00, 0x00, 0x00, 0xFF],
                foreground: [0xFF, 0xFF, 0xFF, 0xFF],
                accent: [0xFF, 0xFF, 0x00, 0xFF],
                border: [0xFF, 0xFF, 0xFF, 0xFF],
                selection: [0x00, 0xFF, 0x00, 0xFF],
            },
            Theme::HighContrastDark => ThemeColors {
                background: [0x00, 0x00, 0x00, 0xFF],
                foreground: [0xFF, 0xFF, 0xFF, 0xFF],
                accent: [0x00, 0xFF, 0xFF, 0xFF],
                border: [0xFF, 0xFF, 0xFF, 0xFF],
                selection: [0x00, 0xFF, 0x00, 0xFF],
            },
            Theme::HighContrastLight => ThemeColors {
                background: [0xFF, 0xFF, 0xFF, 0xFF],
                foreground: [0x00, 0x00, 0x00, 0xFF],
                accent: [0x00, 0x00, 0xFF, 0xFF],
                border: [0x00, 0x00, 0x00, 0xFF],
                selection: [0x00, 0x00, 0xFF, 0xFF],
            },
        }
    }

    /// Check if colors meet WCAG AAA contrast ratio (7:1)
    pub fn check_wcag_aaa_contrast(&self, fg: [u8; 4], bg: [u8; 4]) -> bool {
        let fg_luminance = self.luminance(fg);
        let bg_luminance = self.luminance(bg);
        
        let lighter = fg_luminance.max(bg_luminance);
        let darker = fg_luminance.min(bg_luminance);
        
        if darker == 0.0 {
            return false;
        }
        
        let contrast_ratio = (lighter + 0.05) / (darker + 0.05);
        contrast_ratio >= 7.0
    }

    /// Calculate relative luminance (WCAG 2.0)
    fn luminance(&self, color: [u8; 4]) -> f64 {
        let r = self.srgb_to_linear(color[0] as f64 / 255.0);
        let g = self.srgb_to_linear(color[1] as f64 / 255.0);
        let b = self.srgb_to_linear(color[2] as f64 / 255.0);
        
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }

    /// Convert sRGB to linear RGB
    fn srgb_to_linear(&self, c: f64) -> f64 {
        if c <= 0.03928 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    }

    /// Enable/disable magnifier
    pub fn set_magnifier_enabled(&mut self, enabled: bool) {
        self.magnifier.enabled = enabled;
    }

    /// Set magnifier zoom level (2.0 to 16.0)
    pub fn set_magnifier_zoom(&mut self, zoom: f32) {
        self.magnifier.zoom_level = zoom.clamp(2.0, 16.0);
    }

    /// Increase magnifier zoom
    pub fn magnifier_zoom_in(&mut self) {
        self.magnifier.zoom_level = (self.magnifier.zoom_level * 1.5).clamp(2.0, 16.0);
    }

    /// Decrease magnifier zoom
    pub fn magnifier_zoom_out(&mut self) {
        self.magnifier.zoom_level = (self.magnifier.zoom_level / 1.5).clamp(2.0, 16.0);
    }

    /// Set magnifier position
    pub fn set_magnifier_position(&mut self, x: i32, y: i32) {
        self.magnifier.x = x;
        self.magnifier.y = y;
    }

    /// Enable/disable keyboard navigation
    pub fn set_keyboard_nav_enabled(&mut self, enabled: bool) {
        self.keyboard_nav.enabled = enabled;
    }

    /// Set focus ring visibility
    pub fn set_focus_ring_visible(&mut self, visible: bool) {
        self.keyboard_nav.focus_ring_visible = visible;
    }

    /// Set focus ring color
    pub fn set_focus_ring_color(&mut self, color: [u8; 4]) {
        self.keyboard_nav.focus_ring_color = color;
    }

    /// Set focus ring width
    pub fn set_focus_ring_width(&mut self, width: u32) {
        self.keyboard_nav.focus_ring_width = width;
    }

    /// Enable/disable screen reader
    pub fn set_screen_reader_enabled(&mut self, enabled: bool) {
        self.screen_reader_enabled = enabled;
    }

    /// Enable/disable reduced motion
    pub fn set_reduced_motion(&mut self, enabled: bool) {
        self.reduced_motion = enabled;
    }

    /// Get magnifier viewport (area to magnify)

    pub fn get_magnifier_viewport(&self, screen_width: u32, screen_height: u32) -> (i32, i32, u32, u32) {
        if !self.magnifier.enabled {
            return (0, 0, screen_width, screen_height);
        }

        let viewport_width = (screen_width as f32 / self.magnifier.zoom_level) as u32;
        let viewport_height = (screen_height as f32 / self.magnifier.zoom_level) as u32;

        let x = self.magnifier.x - (viewport_width / 2) as i32;
        let y = self.magnifier.y - (viewport_height / 2) as i32;

        (x, y, viewport_width, viewport_height)
    }

    /// Check if reduced motion is enabled
    pub fn is_reduced_motion(&self) -> bool {
        self.reduced_motion
    }

    /// Check if screen reader is enabled
    pub fn is_screen_reader_enabled(&self) -> bool {
        self.screen_reader_enabled
    }

    /// Check if keyboard navigation is enabled
    pub fn is_keyboard_nav_enabled(&self) -> bool {
        self.keyboard_nav.enabled
    }

    /// Get focus ring settings
    pub fn get_focus_ring_settings(&self) -> (bool, [u8; 4], u32) {
        (
            self.keyboard_nav.focus_ring_visible,
            self.keyboard_nav.focus_ring_color,
            self.keyboard_nav.focus_ring_width,
        )
    }

    /// Announce UI element to screen reader
    pub fn announce(&self, text: &str) {
        if self.screen_reader_enabled {
            // In real implementation, use screen reader to announce text
            // For now, this is a placeholder
        }
    }

    /// Read focused element
    pub fn read_focused_element(&self) {
        if self.screen_reader_enabled {
            // In real implementation, read the currently focused element
            // For now, this is a placeholder
        }
    }
}
