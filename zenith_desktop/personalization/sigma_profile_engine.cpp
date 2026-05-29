/**
 * Zenith personalization engine — declarative ~/.sigma_profile support.
 * Competitor inspiration: Solus Budgie settings, GNOME dconf, NixOS home-manager style keys.
 */
#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_theme.h"

namespace Zenith {
namespace Personalization {

struct SigmaUserProfile {
    char name[32];
    char theme[32];          // zenith-dark | zenith-light
    char accent[16];         // hex without #
    char wm_layout[32];      // master-stack | bsp | floating
    sigma_u32 gap_inner;
    sigma_u32 gap_outer;
    sigma_bool animations;
    sigma_bool auto_tile;
};

static SigmaUserProfile g_profile;

static void load_defaults() {
    sigma_strncpy(g_profile.name, "default", 31);
    sigma_strncpy(g_profile.theme, "zenith-dark", 31);
    sigma_strncpy(g_profile.accent, "007AFF", 15);
    sigma_strncpy(g_profile.wm_layout, "master-stack", 31);
    g_profile.gap_inner = 4;
    g_profile.gap_outer = 8;
    g_profile.animations = SIGMA_TRUE;
    g_profile.auto_tile = SIGMA_TRUE;
}

/* Minimal key=value parser for ~/.sigma_profile */
static void apply_kv(const char* key, const char* value) {
    if (sigma_strcmp(key, "theme") == 0) sigma_strncpy(g_profile.theme, value, 31);
    else if (sigma_strcmp(key, "accent") == 0) sigma_strncpy(g_profile.accent, value, 15);
    else if (sigma_strcmp(key, "wm_layout") == 0) sigma_strncpy(g_profile.wm_layout, value, 31);
    else if (sigma_strcmp(key, "gap_inner") == 0) g_profile.gap_inner = (sigma_u32)sigma_atoi(value);
    else if (sigma_strcmp(key, "gap_outer") == 0) g_profile.gap_outer = (sigma_u32)sigma_atoi(value);
    else if (sigma_strcmp(key, "animations") == 0) g_profile.animations = (value[0] == '1');
    else if (sigma_strcmp(key, "auto_tile") == 0) g_profile.auto_tile = (value[0] == '1');
}

sigma_status load_profile_file(const char* path) {
    (void)path;
    /* TODO: VFS read of ~/.sigma_profile; for now use defaults + sample keys */
    load_defaults();
    apply_kv("theme", "zenith-dark");
    apply_kv("wm_layout", "master-stack");
    apply_kv("gap_inner", "4");
    apply_kv("gap_outer", "8");
    apply_kv("auto_tile", "1");
    return SIGMA_SUCCESS;
}

const SigmaUserProfile* get_active_profile() {
    return &g_profile;
}

void apply_to_desktop() {
    sys_print("[Profile] theme=%s accent=#%s layout=%s gaps=%u/%u auto_tile=%u\n",
              g_profile.theme, g_profile.accent, g_profile.wm_layout,
              g_profile.gap_inner, g_profile.gap_outer, g_profile.auto_tile);
    /* IPC hooks: theme engine + tiling WM */
}

} // namespace Personalization
} // namespace Zenith

extern "C" void zenith_profile_init() {
    Zenith::Personalization::load_profile_file("~/.sigma_profile");
    Zenith::Personalization::apply_to_desktop();
}
