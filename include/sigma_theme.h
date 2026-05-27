/**
 * @file sigma_theme.h
 * @brief SigmaOS Personalization & Theme Engine API
 *
 * Defines the core data types for the theme registry, color palette,
 * icon packs, and font rendering — inspired by KDE Plasma's token system
 * and ElementaryOS's Pantheon design language.
 */

#pragma once
#include "sigma_kernel_types.h"

namespace sigma {
namespace theme {

// ─── Color Tokens (KDE/GTK-style semantic colors) ───────────────────────────
struct ColorPalette {
    sigma_u32 accent;         // Primary brand color (HSL encoded)
    sigma_u32 background;     // Window/desktop background
    sigma_u32 surface;        // Cards, panels, secondary surfaces
    sigma_u32 on_background;  // Text on background
    sigma_u32 on_surface;     // Text on surfaces
    sigma_u32 error;          // Error states
    sigma_u32 success;        // Success states
    sigma_u32 warning;        // Warning states
};

// ─── Typography (inspired by GNOME HIG and Fluent Design) ───────────────────
struct FontConfig {
    char family[64];          // e.g. "Inter", "JetBrains Mono"
    sigma_u32 size_body;      // Body text size (pt)
    sigma_u32 size_heading;   // Heading size (pt)
    sigma_u32 size_mono;      // Monospace/terminal size (pt)
    sigma_bool antialiased;   // Subpixel rendering
    sigma_bool hinting;       // Grid-fit hinting
};

// ─── Window Decoration ───────────────────────────────────────────────────────
struct WindowStyle {
    sigma_u32 corner_radius;  // px — 0 = sharp, 12 = modern rounded
    sigma_u32 shadow_blur;    // px drop shadow
    sigma_u32 border_width;   // px
    sigma_bool blur_behind;   // Frosted glass (Aero/Plasma-style)
};

// ─── Animation Tokens (inspired by macOS Spring Animations) ─────────────────
struct AnimationConfig {
    sigma_u32 duration_fast;    // ms — micro interactions (150ms)
    sigma_u32 duration_normal;  // ms — panel transitions (280ms)
    sigma_u32 duration_slow;    // ms — page turns (500ms)
    sigma_bool reduced_motion;  // Accessibility: disable animations
};

// ─── Complete Theme Descriptor ───────────────────────────────────────────────
struct SigmaTheme {
    char name[64];
    sigma_bool dark_mode;
    ColorPalette  colors;
    FontConfig    fonts;
    WindowStyle   windows;
    AnimationConfig animations;
};

// ─── Built-in Presets ────────────────────────────────────────────────────────
static const SigmaTheme THEME_SIGMA_DARK = {
    "Sigma Dark",
    /*dark_mode=*/SIGMA_TRUE,
    /*colors=*/{
        0xFF6C63FF,  // accent: Violet
        0xFF1A1A2E,  // background: Deep Navy
        0xFF16213E,  // surface
        0xFFE0E0FF,  // on_background
        0xFFB0B0CC,  // on_surface
        0xFFFF4444,  // error
        0xFF44FF88,  // success
        0xFFFFCC44   // warning
    },
    /*fonts=*/{"Inter", 14, 20, 13, SIGMA_TRUE, SIGMA_TRUE},
    /*windows=*/{12, 24, 0, SIGMA_TRUE},
    /*animations=*/{150, 280, 500, SIGMA_FALSE}
};

static const SigmaTheme THEME_SIGMA_LIGHT = {
    "Sigma Light",
    /*dark_mode=*/SIGMA_FALSE,
    /*colors=*/{
        0xFF5A54E5,  // accent: Indigo
        0xFFF5F5F5,  // background
        0xFFFFFFFF,  // surface
        0xFF111111,  // on_background
        0xFF444444,  // on_surface
        0xFFD32F2F,  // error
        0xFF2E7D32,  // success
        0xFFE65100   // warning
    },
    /*fonts=*/{"Inter", 14, 20, 13, SIGMA_TRUE, SIGMA_TRUE},
    /*windows=*/{12, 16, 1, SIGMA_FALSE},
    /*animations=*/{150, 280, 500, SIGMA_FALSE}
};

} // namespace theme
} // namespace sigma
