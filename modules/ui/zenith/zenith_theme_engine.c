/*
 * ============================================================
 * SigmaOS  modules/ui/zenith  zenith_theme_engine.c  v2.0
 * Context-aware Zenith UI personalisation for industrial shards
 * ============================================================
 */
#include "../../../include/libc/SovereignLibC.h"
#include "../../../include/sigma_log.h"
#include "../../../include/sigma_kernel_types.h"

typedef struct {
    sigma_u8 r, g, b, a;
} zenith_color_t;

typedef struct {
    zenith_color_t accent;
    zenith_color_t background;
    zenith_color_t text;
    sigma_u8 corner_radius;
    sigma_u8 glass_alpha;
    const char* font_family;
} zenith_theme_spec_t;

static zenith_theme_spec_t g_current_spec;

/* Predefined palette tokens */
static const zenith_color_t COLOR_CLINICAL_TEAL  = {  0, 200, 150, 255 };
static const zenith_color_t COLOR_GOLD           = {255, 215,   0, 255 };
static const zenith_color_t COLOR_NEON_CYAN      = {  0, 255, 255, 255 };
static const zenith_color_t COLOR_COSMIC_PURPLE  = {200, 100, 255, 255 };
static const zenith_color_t COLOR_SOVEREIGN_BLUE = {  0, 120, 215, 255 };
static const zenith_color_t COLOR_INDIA_ORANGE   = {255, 153,  51, 255 };
static const zenith_color_t COLOR_LAW_BROWN      = {139,  90,  43, 255 };
static const zenith_color_t COLOR_AGRI_GREEN     = { 76, 175,  80, 255 };

static const zenith_color_t BG_LIGHT     = {240, 245, 245, 255};
static const zenith_color_t BG_NAVY      = { 10,  10,  20, 255};
static const zenith_color_t BG_OLED      = {  5,   5,   5, 255};
static const zenith_color_t BG_DEEP_SPACE= {  2,   0,  15, 255};
static const zenith_color_t BG_SOVEREIGN = { 18,  18,  18, 240};

static const zenith_color_t TEXT_WHITE   = {255, 255, 255, 255};
static const zenith_color_t TEXT_DARK    = { 20,  20,  20, 255};

void zenith_theme_load_shard(const char* shard_name) {
    sigma_log_info("[THEME] Loading Zenith persona for shard: %s\n", shard_name);

    if      (sigma_hardened_strcmp(shard_name, "Medical")   == 0) {
        g_current_spec = (zenith_theme_spec_t){COLOR_CLINICAL_TEAL,  BG_LIGHT,      TEXT_DARK,  12, 200, "Outfit"       };
    } else if (sigma_hardened_strcmp(shard_name, "Finance") == 0) {
        g_current_spec = (zenith_theme_spec_t){COLOR_GOLD,           BG_NAVY,       TEXT_WHITE, 8,  180, "JetBrains Mono"};
    } else if (sigma_hardened_strcmp(shard_name, "Cyber")   == 0) {
        g_current_spec = (zenith_theme_spec_t){COLOR_NEON_CYAN,      BG_OLED,       TEXT_WHITE, 4,  120, "JetBrains Mono"};
    } else if (sigma_hardened_strcmp(shard_name, "Space")   == 0) {
        g_current_spec = (zenith_theme_spec_t){COLOR_COSMIC_PURPLE,  BG_DEEP_SPACE, TEXT_WHITE, 16, 200, "Outfit"       };
    } else if (sigma_hardened_strcmp(shard_name, "India")   == 0) {
        g_current_spec = (zenith_theme_spec_t){COLOR_INDIA_ORANGE,   BG_NAVY,       TEXT_WHITE, 10, 190, "Outfit"       };
    } else if (sigma_hardened_strcmp(shard_name, "Legal")   == 0) {
        g_current_spec = (zenith_theme_spec_t){COLOR_LAW_BROWN,      BG_LIGHT,      TEXT_DARK,  6,  220, "Outfit"       };
    } else if (sigma_hardened_strcmp(shard_name, "Agri")    == 0) {
        g_current_spec = (zenith_theme_spec_t){COLOR_AGRI_GREEN,     BG_SOVEREIGN,  TEXT_WHITE, 12, 200, "Outfit"       };
    } else {
        g_current_spec = (zenith_theme_spec_t){COLOR_SOVEREIGN_BLUE, BG_SOVEREIGN,  TEXT_WHITE, 12, 240, "Outfit"       };
    }

    sigma_log_info("[THEME] Zenith re-skinned: accent=(%u,%u,%u) glass=%u font=%s\n",
        g_current_spec.accent.r, g_current_spec.accent.g, g_current_spec.accent.b,
        g_current_spec.glass_alpha, g_current_spec.font_family);
}

void zenith_theme_load_industrial(const char* shard_name) {
    zenith_theme_load_shard(shard_name);
}

zenith_theme_spec_t* zenith_theme_get_current(void) {
    return &g_current_spec;
}

void personalization_sync_ui(const char* profile_name) {
    sigma_log_info("[THEME] Syncing UI with kernel profile: %s\n", profile_name);
    zenith_theme_load_shard(profile_name);
}
