/**
 * @file sigma_theme_engine.cpp
 * @brief SigmaOS Personalization Core — Theme & UX Engine
 *
 * Competitor Inspiration:
 *  - KDE Plasma: Global Color Scheme + Kvantum SVG theme engine
 *  - GNOME: Libadwaita token-based stylesheet system
 *  - elementaryOS: Pantheon design language consistency
 *  - macOS: Accent color system + auto dark/light mode
 *  - Windows 11: Mica/Acrylic blur-behind materials
 *
 * Provides a centralized registry for global UI settings that
 * broadcasts live updates to all Zenith compositor clients
 * via the Sovereign IPC bus.
 */

#include "../../include/sigma_theme.h"
#include "../../include/sigma_kernel_types.h"

namespace sigma {
namespace theme {

// ─── Global Theme State ───────────────────────────────────────────────────────
static SigmaTheme g_active_theme = THEME_SIGMA_DARK;
static sigma_bool g_auto_dark_mode = SIGMA_TRUE;  // Switch at sunrise/sunset

// ─── Registered Theme Slots (user-defined + built-ins) ───────────────────────
#define SIGMA_MAX_THEMES 32
static SigmaTheme g_theme_registry[SIGMA_MAX_THEMES];
static sigma_u32  g_theme_count = 0;

// ─── Theme Engine Init ────────────────────────────────────────────────────────
sigma_status theme_engine_init() {
    // Pre-load built-in themes into registry
    g_theme_registry[0] = THEME_SIGMA_DARK;
    g_theme_registry[1] = THEME_SIGMA_LIGHT;
    g_theme_count = 2;
    g_active_theme = THEME_SIGMA_DARK;
    return SIGMA_SUCCESS;
}

// ─── Apply a Theme by Name ────────────────────────────────────────────────────
sigma_status apply_theme(const char* name) {
    for (sigma_u32 i = 0; i < g_theme_count; ++i) {
        // Manual string compare (no libc)
        const char* a = g_theme_registry[i].name;
        const char* b = name;
        sigma_bool match = SIGMA_TRUE;
        while (*a && *b) {
            if (*a++ != *b++) { match = SIGMA_FALSE; break; }
        }
        if (match && *a == '\0' && *b == '\0') {
            g_active_theme = g_theme_registry[i];
            // Broadcast to all Zenith compositor clients via IPC
            broadcast_theme_update();
            return SIGMA_SUCCESS;
        }
    }
    return SIGMA_ERROR;  // Theme not found
}

// ─── Register a Custom Theme ──────────────────────────────────────────────────
sigma_status register_theme(const SigmaTheme* theme) {
    if (!theme || g_theme_count >= SIGMA_MAX_THEMES) return SIGMA_ERROR;
    g_theme_registry[g_theme_count++] = *theme;
    return SIGMA_SUCCESS;
}

// ─── Auto Dark Mode (sunrise/sunset switching) ────────────────────────────────
sigma_status auto_dark_mode_tick(sigma_u32 hour_24) {
    if (!g_auto_dark_mode) return SIGMA_SUCCESS;
    // 06:00–19:00 = light mode, else dark (macOS-inspired)
    const sigma_bool should_be_light = (hour_24 >= 6 && hour_24 < 19);
    const sigma_bool currently_light = !g_active_theme.dark_mode;

    if (should_be_light && !currently_light) {
        return apply_theme("Sigma Light");
    } else if (!should_be_light && currently_light) {
        return apply_theme("Sigma Dark");
    }
    return SIGMA_SUCCESS;
}

// ─── Update Single Color Token (KDE-style live recolor) ──────────────────────
sigma_status set_accent_color(sigma_u32 argb) {
    g_active_theme.colors.accent = argb;
    broadcast_theme_update();
    return SIGMA_SUCCESS;
}

// ─── Get Current Theme ────────────────────────────────────────────────────────
const SigmaTheme* get_active_theme() {
    return &g_active_theme;
}

// ─── IPC Broadcast to Compositor Clients ─────────────────────────────────────
sigma_status broadcast_theme_update() {
    // In a real implementation this writes to the Sovereign IPC ring buffer
    // so all connected Zenith GUI clients receive the new token set and
    // redraw their chrome without restarting.
    return SIGMA_SUCCESS;
}

// ─── CSS-like Stylesheet Generator (for GTK/Qt compat) ───────────────────────
sigma_status export_gtk_css(char* buf, sigma_u32 buflen) {
    if (!buf || buflen < 128) return SIGMA_ERROR;
    // Write a minimal GTK4-compatible CSS snippet from the active token set
    const ColorPalette& c = g_active_theme.colors;
    const WindowStyle&  w = g_active_theme.windows;
    // Minimal templated write (no sprintf)
    const char header[] = "@define-color accent #";
    sigma_u32 pos = 0;
    for (; header[pos] && pos < buflen - 1; ++pos)
        buf[pos] = header[pos];
    // Hex-encode accent color (simplified)
    const char hex[] = "0123456789ABCDEF";
    sigma_u32 a = c.accent;
    buf[pos++] = hex[(a >> 20) & 0xF];
    buf[pos++] = hex[(a >> 16) & 0xF];
    buf[pos++] = hex[(a >> 12) & 0xF];
    buf[pos++] = hex[(a >>  8) & 0xF];
    buf[pos++] = hex[(a >>  4) & 0xF];
    buf[pos++] = hex[(a >>  0) & 0xF];
    buf[pos++] = ';';
    buf[pos]   = '\0';
    return SIGMA_SUCCESS;
}

} // namespace theme
} // namespace sigma

extern "C" {
    sigma_status sigma_theme_init(void)                         { return sigma::theme::theme_engine_init(); }
    sigma_status sigma_theme_apply(const char* name)            { return sigma::theme::apply_theme(name); }
    sigma_status sigma_theme_set_accent(sigma_u32 argb)         { return sigma::theme::set_accent_color(argb); }
    sigma_status sigma_theme_auto_tick(sigma_u32 hour)          { return sigma::theme::auto_dark_mode_tick(hour); }
    sigma_status sigma_theme_export_gtk(char* b, sigma_u32 l)   { return sigma::theme::export_gtk_css(b, l); }
}
