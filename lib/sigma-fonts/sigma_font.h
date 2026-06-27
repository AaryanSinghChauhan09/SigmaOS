// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_font.h — Font rendering subsystem
 *
 * Wraps FreeType2 (rasterisation) + HarfBuzz (text shaping) + fontconfig
 * (font discovery). Without this, the GUI shows boxes instead of text.
 *
 * Usage:
 *   sigma_font_t* font = sigma_font_load("SigmaSans", 16, SIGMA_FONT_REGULAR);
 *   sigma_font_shape(font, "Hello, World!", &glyphs);
 *   sigma_font_render(font, &glyphs, surface, x, y, 0xFFFFFFFF);
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

/* ── Font weight / style ─────────────────────────────────────────────────── */
typedef enum {
    SIGMA_FONT_REGULAR    = 0,
    SIGMA_FONT_BOLD       = 1,
    SIGMA_FONT_ITALIC     = 2,
    SIGMA_FONT_BOLD_ITALIC = 3,
    SIGMA_FONT_MONO       = 4,
} sigma_font_style_t;

/* ── Glyph run (output of text shaping) ─────────────────────────────────── */
typedef struct {
    sigma_u32 glyph_id;
    sigma_i32 x_advance;   /* pixels to advance after this glyph         */
    sigma_i32 y_advance;
    sigma_i32 x_offset;
    sigma_i32 y_offset;
} sigma_glyph_t;

typedef struct {
    sigma_glyph_t* glyphs;
    int            count;
    int            total_width;
    int            ascent;
    int            descent;
} sigma_glyph_run_t;

/* ── Font handle ─────────────────────────────────────────────────────────── */
typedef struct sigma_font sigma_font_t;

/* ── API ──────────────────────────────────────────────────────────────────── */

/* Initialise font subsystem — loads SigmaSans.ttf from /sigma/share/fonts/ */
int sigma_font_init(void);

/* Load a font by family name and pixel size */
sigma_font_t* sigma_font_load(const char* family, int size_px,
                                sigma_font_style_t style);
void sigma_font_unref(sigma_font_t* font);

/*
 * Shape text into a glyph run (uses HarfBuzz for correct RTL/Devanagari etc.)
 * text must be UTF-8. lang: "en", "hi", "ar", "zh" for shaping hints.
 */
int sigma_font_shape(sigma_font_t* font, const char* text_utf8,
                      const char* lang, sigma_glyph_run_t* out);
void sigma_glyph_run_free(sigma_glyph_run_t* run);

/*
 * Render a glyph run onto a 32-bit RGBA surface at (x, y).
 * color: 0xRRGGBBAA
 */
int sigma_font_render(sigma_font_t* font, const sigma_glyph_run_t* run,
                       sigma_u32* surface, int surface_w, int surface_h,
                       int x, int y, sigma_u32 color);

/* Measure text width/height without rendering */
int sigma_font_measure(sigma_font_t* font, const char* text_utf8,
                        int* width_out, int* height_out);

/* Emoji support — sigma_font_t* can be a colour emoji font */
bool sigma_font_has_color_glyph(sigma_font_t* font, sigma_u32 codepoint);

/* Font discovery */
const char** sigma_font_list_families(int* count_out);
int          sigma_font_find(const char* pattern, char* path_out, int path_max);
