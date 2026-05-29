/*
 * SigmaOS gaming subsystem — SteamOS-class compatibility layer (Phase C).
 * Future: Proton/Wine shim IPC, GPU passthrough policy, sigma-pod game profiles.
 */
#include "../../include/sigma_kernel_types.h"

typedef struct {
    sigma_bool proton_enabled;
    sigma_bool gamemode_cpu_boost;
    sigma_u32  nic_priority_weight;
} sigma_game_profile_t;

static sigma_game_profile_t g_game;

void sigma_game_layer_init(void) {
    g_game.proton_enabled = SIGMA_FALSE;
    g_game.gamemode_cpu_boost = SIGMA_TRUE;
    g_game.nic_priority_weight = 200;
}

void sigma_game_layer_set_proton(sigma_bool enabled) {
    g_game.proton_enabled = enabled;
}

sigma_bool sigma_game_layer_is_proton_enabled(void) {
    return g_game.proton_enabled;
}

void sigma_game_layer_apply_gamemode(void) {
    /* Hook: raise scheduler weight + compositor bypass for focused game window */
    g_game.gamemode_cpu_boost = SIGMA_TRUE;
}
