#include "sigma_libc.h"
#include "sigma_log.h"
#include "core/sigma_types.h"

// ---------------------------------------------------------
// SigmaOS Zenith Theme Engine
// Mission: Context-aware personalization for industrial shards.
// ---------------------------------------------------------

typedef struct {
    sigma_u8 r, g, b, a;
} zenith_color_t;

typedef struct {
    zenith_color_t accent;
    zenith_color_t background;
    sigma_u8 corner_radius;
    sigma_u8 glass_alpha;
} zenith_theme_spec_t;

static zenith_theme_spec_t current_spec;

void zenith_theme_load_industrial(const char* shard_name) {
    sigma_log_info("[THEME] Loading industrial persona for shard: %s", shard_name);

    if (sigma_strcmp(shard_name, "Medical") == 0) {
        current_spec.accent = (zenith_color_t){0, 200, 150, 255}; // Clinical Teal
        current_spec.background = (zenith_color_t){240, 245, 245, 255}; // Light Mode
        current_spec.glass_alpha = 200;
    } else if (sigma_strcmp(shard_name, "Finance") == 0) {
        current_spec.accent = (zenith_color_t){255, 215, 0, 255}; // Gold
        current_spec.background = (zenith_color_t){10, 10, 20, 255}; // Deep Navy
        current_spec.glass_alpha = 180;
    } else if (sigma_strcmp(shard_name, "Cyber") == 0) {
        current_spec.accent = (zenith_color_t){0, 255, 255, 255}; // Neon Cyan
        current_spec.background = (zenith_color_t){5, 5, 5, 255}; // OLED Black
        current_spec.glass_alpha = 120;
    } else if (sigma_strcmp(shard_name, "Space") == 0) {
        current_spec.accent = (zenith_color_t){200, 100, 255, 255}; // Cosmic Purple
        current_spec.background = (zenith_color_t){2, 0, 15, 255}; // Deep Space
        current_spec.glass_alpha = 200;
    } else {
        // Default Sovereign Dark
        current_spec.accent = (zenith_color_t){0, 120, 215, 255};
        current_spec.background = (zenith_color_t){18, 18, 18, 240};
        current_spec.glass_alpha = 240;
    }
    
    current_spec.corner_radius = 12;
    sigma_log_info("[THEME] Zenith UI re-skinned for %s.", shard_name);
}

zenith_theme_spec_t* zenith_theme_get_current() {
    return &current_spec;
}
void personalization_sync_ui(const char* profile_name) {
    sigma_log_info("[THEME] Synchronizing UI with kernel profile: %s", profile_name);
    zenith_theme_load_industrial(profile_name);
}
