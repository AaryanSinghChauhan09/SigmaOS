/*
 * Meta-distro hub — initializes competitor-inspired subsystems under one engine.
 */
#include "../../include/sigma_meta_distro.h"
#include "../../include/sigma_game_layer.h"

extern void sigma_sched_init(void);
extern void sigma_immutable_root_init(void);
extern void recovery_init(void);
extern void recovery_gui_init(void);
extern void SovereignPkg_InitRegistry(void);

#ifndef SIGMA_FEATURE_CONTAINERS
/* Containers initialized via sigma-pod / orchestrator at user request */
#endif
extern void zenith_subsystem_init(sigma_u32 w, sigma_u32 h);
extern void sigma_orchestrator_init(void);

static sigma_u32 g_features;
static char g_status[256];

void sigma_meta_distro_init(sigma_u32 feature_mask) {
    g_features = feature_mask;

    if (feature_mask & SIGMA_FEATURE_GAMING) {
        sigma_game_layer_init();
    }
    if (feature_mask & SIGMA_FEATURE_PERFORMANCE) {
        sigma_sched_init();
    }
    if (feature_mask & SIGMA_FEATURE_PACKAGES) {
        SovereignPkg_InitRegistry();
    }
    if (feature_mask & SIGMA_FEATURE_IMMUTABLE) {
        sigma_immutable_root_init();
    }
    if (feature_mask & SIGMA_FEATURE_RECOVERY) {
        recovery_init();
        recovery_gui_init();
    }
    if (feature_mask & SIGMA_FEATURE_CONTAINERS) {
        sigma_orchestrator_init();
    }
    if (feature_mask & SIGMA_FEATURE_DESKTOP) {
        zenith_subsystem_init(1920, 1080);
    }

    g_status[0] = '\0';
    sigma_u32 n = 0;
    const char* prefix = "{\"features\":";
    while (prefix[n] && n < 240) { g_status[n] = prefix[n]; n++; }
    g_status[n++] = (char)('0' + (feature_mask & 0xF));
    g_status[n++] = '}';
    g_status[n] = '\0';
}

const char* sigma_meta_distro_status_json(void) {
    return g_status;
}
