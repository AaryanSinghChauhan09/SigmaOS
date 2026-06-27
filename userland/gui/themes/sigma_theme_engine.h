// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_theme_engine.h — Runtime theme switcher for Zenith Desktop
 *
 * Themes are TOML files stored at /sigma/share/themes/<name>/theme.toml.
 * The engine hot-reloads themes without restarting any app — it emits a
 * sigma-bus THEME_CHANGED event that all Zenith apps subscribe to.
 *
 * CLI:
 *   sigma-theme list
 *   sigma-theme set sigma-light
 *   sigma-theme set high-contrast
 *   sigma-theme create --name "my-dark" --accent "#ff6b6b"
 *   sigma-theme export my-dark > my-dark.sigma-theme
 *   sigma-theme import < downloaded.sigma-theme
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

/* ── Colour token ────────────────────────────────────────────────────────── */
typedef struct {
    sigma_u8 r, g, b, a;
} sigma_color_t;

/* ── Complete theme descriptor ───────────────────────────────────────────── */
typedef struct {
    char  name[64];
    char  author[64];
    char  version[16];
    bool  dark_mode;
    bool  high_contrast;    /* WCAG AAA contrast ratios */

    /* Colour palette */
    sigma_color_t accent;          /* buttons, highlights, focus rings     */
    sigma_color_t background;      /* window / desktop background          */
    sigma_color_t surface;         /* cards, panels, sidebars              */
    sigma_color_t surface_raised;  /* elevated cards (drop shadow)         */
    sigma_color_t border;          /* dividers, input outlines             */
    sigma_color_t text_primary;    /* body text                            */
    sigma_color_t text_muted;      /* captions, placeholders               */
    sigma_color_t text_inverse;    /* text on accent-coloured backgrounds  */
    sigma_color_t success;         /* green — confirmations                */
    sigma_color_t warning;         /* amber — caution                      */
    sigma_color_t error;           /* red — errors                         */
    sigma_color_t info;            /* blue — information                   */

    /* Typography */
    char     font_ui[64];          /* "SigmaSans", "Noto Sans", etc.      */
    char     font_code[64];        /* "SigmaMono", "JetBrains Mono"       */
    char     font_display[64];     /* headings                            */
    sigma_u8 font_size_base;       /* base size in pt (default 14)        */
    float    line_height;          /* default 1.5                         */

    /* Shape */
    sigma_u8 radius_window;        /* corner radius for windows (px)      */
    sigma_u8 radius_button;
    sigma_u8 radius_card;
    sigma_u8 radius_input;

    /* Spacing */
    sigma_u8 spacing_xs;           /* 4px  */
    sigma_u8 spacing_sm;           /* 8px  */
    sigma_u8 spacing_md;           /* 16px */
    sigma_u8 spacing_lg;           /* 24px */
    sigma_u8 spacing_xl;           /* 32px */

    /* Animation */
    sigma_u16 anim_duration_ms;    /* default 200ms                       */
    char      anim_easing[32];     /* "ease-in-out", "spring", "linear"   */

    /* Wallpaper */
    char  wallpaper_path[256];     /* /sigma/share/wallpapers/default.jpg */
    bool  wallpaper_blur;
    float wallpaper_blur_radius;
} sigma_theme_t;

/* ── Built-in theme IDs ──────────────────────────────────────────────────── */
#define SIGMA_THEME_DEFAULT        "sigma-dark"
#define SIGMA_THEME_LIGHT          "sigma-light"
#define SIGMA_THEME_HIGH_CONTRAST  "high-contrast"
#define SIGMA_THEME_SAFFRON        "sigma-saffron"   /* India-inspired warm */
#define SIGMA_THEME_FOREST         "sigma-forest"    /* green, earthy       */

/* ── API ─────────────────────────────────────────────────────────────────── */

/* Load a theme from /sigma/share/themes/<name>/theme.toml. */
int sigma_theme_load(const char *name, sigma_theme_t *out);

/* Apply theme system-wide (emits sigma-bus THEME_CHANGED). */
int sigma_theme_apply(const sigma_theme_t *theme);

/* Get the currently active theme. */
int sigma_theme_current(sigma_theme_t *out);

/* List all installed themes (calls cb for each). */
void sigma_theme_list(void (*cb)(const char *name,
                                  bool dark_mode,
                                  bool high_contrast,
                                  void *userdata),
                      void *userdata);

/* Save a theme to /sigma/share/themes/<theme.name>/theme.toml. */
int sigma_theme_save(const sigma_theme_t *theme);

/* Export theme to a portable .sigma-theme file (tar.gz of theme dir). */
int sigma_theme_export(const char *name, const char *output_path);

/* Import a .sigma-theme file. */
int sigma_theme_import(const char *path);

/* Create a new theme by deriving from an existing one with overrides. */
int sigma_theme_derive(const char *base_name, const char *new_name,
                        sigma_color_t accent, bool dark_mode,
                        sigma_theme_t *out);

/* Subscribe to theme changes (called when sigma-bus THEME_CHANGED fires). */
typedef void (*sigma_theme_change_cb_t)(const sigma_theme_t *new_theme,
                                         void *userdata);
void sigma_theme_subscribe(sigma_theme_change_cb_t cb, void *userdata);
