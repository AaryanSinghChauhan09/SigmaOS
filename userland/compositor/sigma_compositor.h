// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_compositor.h — Zenith Wayland compositor public API
 *
 * Implements a minimal Wayland compositor for SigmaOS, inspired by:
 *   - Sway (wlroots-based tiling)
 *   - KWin (KDE)
 *   - Mutter (GNOME)
 *   - Mir (Ubuntu)
 *
 * Architecture:
 *   sigma-compositor (this) → DRM/KMS (kernel) → GPU
 *   sigma-compositor ← Wayland clients (apps, Zenith shell)
 *
 * Features:
 *   - Wayland core + xdg-shell + layer-shell + wlr-screencopy
 *   - Hardware-accelerated compositing via Vulkan or OpenGL ES 3.2
 *   - High-DPI / fractional scaling (1× to 4×, per-output)
 *   - Variable refresh rate (VRR/FreeSync/G-SYNC)
 *   - HDR output (BT.2020 + PQ tone mapping)
 *   - Multi-monitor with independent refresh rates
 *   - Screen recording / screenshot via wlr-screencopy
 *   - Input: libinput + sigma_input_filter for gesture recognition
 */

#include <sigma_kernel_types.h>
#include <stdbool.h>

/* ── Output (monitor) ────────────────────────────────────────────────────── */
typedef struct sigma_output sigma_output_t;

typedef struct {
    char     name[32];         /* "eDP-1", "HDMI-A-1", etc.                  */
    sigma_u32 width_px;
    sigma_u32 height_px;
    sigma_u32 refresh_mhz;     /* milli-Hz: 60000 = 60 Hz, 144000 = 144 Hz  */
    float     scale;           /* 1.0, 1.5, 2.0 etc.                        */
    bool      vrr_enabled;
    bool      hdr_enabled;
    sigma_u32 x, y;            /* position in global output layout           */
} sigma_output_info_t;

/* ── Surface ─────────────────────────────────────────────────────────────── */
typedef struct sigma_surface sigma_surface_t;

typedef enum {
    SIGMA_SURF_TOPLEVEL  = 0,  /* regular app window                        */
    SIGMA_SURF_POPUP     = 1,  /* context menus, tooltips                   */
    SIGMA_SURF_OVERLAY   = 2,  /* always-on-top (OSD, notifications)        */
    SIGMA_SURF_BACKGROUND= 3,  /* desktop wallpaper                         */
    SIGMA_SURF_PANEL     = 4,  /* Zenith taskbar / dock                     */
} sigma_surf_role_t;

typedef struct {
    sigma_u32        x, y, width, height;
    float            opacity;           /* 0.0 – 1.0                          */
    bool             fullscreen;
    bool             maximized;
    sigma_surf_role_t role;
    char             app_id[64];
    char             title[256];
} sigma_surface_info_t;

/* ── Compositor lifecycle ────────────────────────────────────────────────── */

int  sigma_compositor_init(const char* wayland_socket); /* "wayland-1" etc.  */
void sigma_compositor_run(void);                         /* blocks forever    */
void sigma_compositor_shutdown(void);

/* ── Output management ───────────────────────────────────────────────────── */

int  sigma_compositor_enumerate_outputs(sigma_output_info_t* out, int max);
int  sigma_compositor_set_output_scale(const char* output_name, float scale);
int  sigma_compositor_set_vrr(const char* output_name, bool enable);
int  sigma_compositor_set_hdr(const char* output_name, bool enable);

/* ── Surface API (used by Zenith shell / window manager) ─────────────────── */

sigma_surface_t* sigma_compositor_surface_create(sigma_surf_role_t role);
int  sigma_compositor_surface_move(sigma_surface_t* s, sigma_u32 x, sigma_u32 y);
int  sigma_compositor_surface_resize(sigma_surface_t* s, sigma_u32 w, sigma_u32 h);
int  sigma_compositor_surface_set_opacity(sigma_surface_t* s, float opacity);
void sigma_compositor_surface_destroy(sigma_surface_t* s);

/* ── Screenshot / screen recording ──────────────────────────────────────── */

/*
 * sigma_compositor_screenshot — capture framebuffer of output to RGBA buffer.
 * Caller must free() *pixels.
 */
int sigma_compositor_screenshot(const char* output_name,
                                 sigma_u8**  pixels,
                                 sigma_u32*  width,
                                 sigma_u32*  height);

/*
 * sigma_compositor_record_start / stop — encode frames to VP9/H.265.
 * path: output file path (e.g. /home/user/screencast.webm)
 */
int  sigma_compositor_record_start(const char* output_name, const char* path);
int  sigma_compositor_record_stop(const char* output_name);

/* ── Animation engine ────────────────────────────────────────────────────── */

typedef enum {
    SIGMA_ANIM_LINEAR      = 0,
    SIGMA_ANIM_EASE_IN_OUT = 1,
    SIGMA_ANIM_SPRING      = 2,   /* spring physics: tension + friction       */
} sigma_anim_curve_t;

typedef struct {
    sigma_surface_t*  target;
    sigma_u32         duration_ms;
    sigma_anim_curve_t curve;
    /* End state */
    sigma_u32         to_x, to_y, to_w, to_h;
    float             to_opacity;
} sigma_animation_t;

int sigma_compositor_animate(const sigma_animation_t* anim);
