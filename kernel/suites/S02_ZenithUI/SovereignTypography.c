// =============================================================================
// SigmaOS — S02_ZenithUI — SovereignTypography.c
// Industrial-Grade Sub-pixel Typography Engine
// =============================================================================
// Competitor USPs Absorbed:
//   • macOS Quartz — Fluid, heavy-weight smoothing with natural curves
//   • Windows ClearType — High-contrast sub-pixel LCD optimization
//   • FreeType (Linux) — Universal font format compatibility (OTF/TTF)
// Exceeding Competitors:
//   • Real-time Kerning & Ligature Support at the Kernel Compositor level
//   • Sovereign-Smooth: AI-augmented edge anti-aliasing for 8K displays
//   • Dynamic Weighting: Adjusts font weight based on system "Sentiment" scale
// =============================================================================

#include "sigma_types.h"


#define MAX_GLYPH_CACHE     4096

typedef struct {
    uint32_t unicode;
    uint8_t  width, height;
    uint8_t  bearing_x, bearing_y;
    uint32_t advance;
    void*    bitmap_alpha; // Sub-pixel alpha mask
} SovereignGlyph;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise the Typography Engine and load the Sovereign Variable Font
void typography_init(void);

// Render a string with sub-pixel anti-aliasing to a Zenith layer
void typography_draw_text(const char* text, int x, int y, uint32_t color, float size);

// Register a new font bundle (.sab font provider)
bool typography_register_font(const char* path);

// Adjust sub-pixel smoothing intensity (ClearType vs Quartz profile)
void typography_set_smoothing_mode(uint8_t mode);

// Get exact bounding box for layout calculations
void typography_measure_text(const char* text, float size, int* w, int* h);

// Synchronise with GraphicsBridge (S04) for hardware-accelerated glyph blitting
void typography_sync_vram(void);

