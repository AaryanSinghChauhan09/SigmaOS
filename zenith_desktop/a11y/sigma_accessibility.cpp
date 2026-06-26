/**
 * @file sigma_accessibility.cpp
 * @brief Zenith Accessibility Engine — Screen reader, zoom, high contrast
 *
 * Competitor Inspiration:
 *  - GNOME: Orca screen reader, AT-SPI2 accessibility API
 *  - KDE: KAccessibility modules, screen magnifier
 *  - macOS: VoiceOver, Zoom, Switch Control, Reduce Motion
 *  - Windows: Narrator, Magnifier, High Contrast themes
 *  - elementaryOS: Universal Access settings
 *
 * Provides an accessibility subsystem that hooks into the compositor
 * and input layer to support users with visual, motor, or auditory needs.
 */

#include "../../include/sigma_kernel_types.h"

namespace sigma {
namespace a11y {

// ─── Accessibility Features ──────────────────────────────────────────────────
struct A11yConfig {
    // Visual
    sigma_bool screen_reader_enabled;    // Orca/VoiceOver-style TTS
    sigma_bool high_contrast;            // Boost contrast ratio
    sigma_bool reduce_motion;            // Disable animations
    sigma_bool reduce_transparency;      // Disable blur-behind
    sigma_bool invert_colors;            // Full color inversion
    sigma_bool color_filter;             // Deuteranopia, protanopia, etc.
    sigma_u32  color_filter_type;        // 0=none, 1=deutan, 2=protan, 3=tritan

    // Zoom
    sigma_bool zoom_enabled;
    sigma_u32  zoom_level;               // 100 = 1x, 200 = 2x, etc.
    sigma_u32  zoom_style;               // 0=fullscreen, 1=lens, 2=split

    // Motor
    sigma_bool sticky_keys;              // Modifiers stay active until next key
    sigma_bool slow_keys;                // Ignore brief keypresses
    sigma_u32  slow_key_delay_ms;        // Minimum press duration to register
    sigma_bool bounce_keys;              // Ignore rapid repeat presses
    sigma_u32  bounce_delay_ms;
    sigma_bool mouse_keys;               // Numpad controls pointer
    sigma_u32  cursor_size;              // 1=small, 2=medium, 3=large, 4=xlarge

    // Auditory
    sigma_bool visual_alerts;            // Flash screen instead of system beep
    sigma_bool captions;                 // Show captions for audio content
    sigma_bool mono_audio;               // Merge stereo to mono
};

static A11yConfig g_config;

// ─── Init ────────────────────────────────────────────────────────────────────
sigma_status a11y_init() {
    // All accessibility features off by default
    g_config.screen_reader_enabled  = SIGMA_FALSE;
    g_config.high_contrast          = SIGMA_FALSE;
    g_config.reduce_motion          = SIGMA_FALSE;
    g_config.reduce_transparency    = SIGMA_FALSE;
    g_config.invert_colors          = SIGMA_FALSE;
    g_config.color_filter           = SIGMA_FALSE;
    g_config.color_filter_type      = 0;
    g_config.zoom_enabled           = SIGMA_FALSE;
    g_config.zoom_level             = 100;
    g_config.zoom_style             = 0;
    g_config.sticky_keys            = SIGMA_FALSE;
    g_config.slow_keys              = SIGMA_FALSE;
    g_config.slow_key_delay_ms      = 300;
    g_config.bounce_keys            = SIGMA_FALSE;
    g_config.bounce_delay_ms        = 300;
    g_config.mouse_keys             = SIGMA_FALSE;
    g_config.cursor_size            = 2;
    g_config.visual_alerts          = SIGMA_FALSE;
    g_config.captions               = SIGMA_FALSE;
    g_config.mono_audio             = SIGMA_FALSE;
    return SIGMA_SUCCESS;
}

// ─── Screen Reader ───────────────────────────────────────────────────────────
sigma_status screen_reader_toggle() {
    g_config.screen_reader_enabled = g_config.screen_reader_enabled ? SIGMA_FALSE : SIGMA_TRUE;
    if (g_config.screen_reader_enabled) {
        // Initialize TTS engine and register AT-SPI2-style event hooks
        // Announce: "Screen reader enabled"
    } else {
        // Announce: "Screen reader disabled" then shutdown
    }
    return SIGMA_SUCCESS;
}

sigma_status screen_reader_speak(const char* text) {
    if (!g_config.screen_reader_enabled || !text) return SIGMA_ERROR;
    // Send text to TTS engine via IPC
    // In a real implementation, this calls into a TTS synthesis module
    (void)text;
    return SIGMA_SUCCESS;
}

// ─── Zoom ────────────────────────────────────────────────────────────────────
sigma_status zoom_set(sigma_u32 level) {
    if (level < 100) level = 100;
    if (level > 1600) level = 1600;
    g_config.zoom_level = level;
    g_config.zoom_enabled = (level > 100) ? SIGMA_TRUE : SIGMA_FALSE;
    // Notify compositor to re-render at new scale
    return SIGMA_SUCCESS;
}

sigma_status zoom_in() {
    return zoom_set(g_config.zoom_level + 25);
}

sigma_status zoom_out() {
    if (g_config.zoom_level <= 100) return SIGMA_SUCCESS;
    return zoom_set(g_config.zoom_level - 25);
}

// ─── High Contrast ───────────────────────────────────────────────────────────
sigma_status high_contrast_toggle() {
    g_config.high_contrast = g_config.high_contrast ? SIGMA_FALSE : SIGMA_TRUE;
    // Notify theme engine to switch to high contrast palette
    return SIGMA_SUCCESS;
}

// ─── Color Filter (colorblindness correction) ───────────────────────────────
sigma_status color_filter_set(sigma_u32 type) {
    g_config.color_filter_type = type;
    g_config.color_filter = (type > 0) ? SIGMA_TRUE : SIGMA_FALSE;
    // Apply color matrix to compositor output
    return SIGMA_SUCCESS;
}

// ─── Apply Color Transform to Pixel (called per-pixel in compositor) ────────
sigma_u32 apply_color_transform(sigma_u32 argb) {
    sigma_u32 a = (argb >> 24) & 0xFF;
    sigma_u32 r = (argb >> 16) & 0xFF;
    sigma_u32 g = (argb >>  8) & 0xFF;
    sigma_u32 b = (argb >>  0) & 0xFF;

    if (g_config.invert_colors) {
        r = 255 - r;
        g = 255 - g;
        b = 255 - b;
    }

    if (g_config.high_contrast) {
        // Increase contrast by stretching histogram
        sigma_u32 luma = (r * 299 + g * 587 + b * 114) / 1000;
        if (luma > 128) { r = 255; g = 255; b = 255; }
        else            { r = 0;   g = 0;   b = 0;   }
    }

    if (g_config.color_filter) {
        switch (g_config.color_filter_type) {
            case 1: // Deuteranopia (green-blind) — shift green to blue
                g = (g * 60 + b * 40) / 100;
                break;
            case 2: // Protanopia (red-blind) — shift red to green
                r = (r * 40 + g * 60) / 100;
                break;
            case 3: // Tritanopia (blue-blind) — shift blue to red
                b = (b * 40 + r * 60) / 100;
                break;
        }
    }

    return (a << 24) | (r << 16) | (g << 8) | b;
}

// ─── Sticky Keys ─────────────────────────────────────────────────────────────
static sigma_u32 g_sticky_mods = 0;

sigma_status sticky_keys_toggle() {
    g_config.sticky_keys = g_config.sticky_keys ? SIGMA_FALSE : SIGMA_TRUE;
    g_sticky_mods = 0;
    return SIGMA_SUCCESS;
}

sigma_u32 sticky_key_press(sigma_u32 modifier) {
    if (!g_config.sticky_keys) return modifier;
    g_sticky_mods |= modifier;
    return g_sticky_mods;
}

sigma_u32 sticky_key_consume() {
    sigma_u32 mods = g_sticky_mods;
    g_sticky_mods = 0;
    return mods;
}

// ─── Cursor Size ─────────────────────────────────────────────────────────────
sigma_status set_cursor_size(sigma_u32 size) {
    if (size < 1) size = 1;
    if (size > 4) size = 4;
    g_config.cursor_size = size;
    // Notify compositor to reload cursor theme at new size
    return SIGMA_SUCCESS;
}

// ─── Get Config (for settings panel) ─────────────────────────────────────────
const A11yConfig* get_config() {
    return &g_config;
}

} // namespace a11y
} // namespace sigma

extern "C" {
    sigma_status sigma_a11y_init(void)            { return sigma::a11y::a11y_init(); }
    sigma_status sigma_a11y_screen_reader(void)    { return sigma::a11y::screen_reader_toggle(); }
    sigma_status sigma_a11y_speak(const char* t)   { return sigma::a11y::screen_reader_speak(t); }
    sigma_status sigma_a11y_zoom_in(void)          { return sigma::a11y::zoom_in(); }
    sigma_status sigma_a11y_zoom_out(void)         { return sigma::a11y::zoom_out(); }
    sigma_status sigma_a11y_high_contrast(void)    { return sigma::a11y::high_contrast_toggle(); }
    sigma_status sigma_a11y_color_filter(sigma_u32 t) { return sigma::a11y::color_filter_set(t); }
    sigma_status sigma_a11y_sticky_keys(void)      { return sigma::a11y::sticky_keys_toggle(); }
    sigma_status sigma_a11y_cursor_size(sigma_u32 s)  { return sigma::a11y::set_cursor_size(s); }
}
