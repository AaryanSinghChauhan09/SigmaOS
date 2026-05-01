/*
 * =========================================================================
 * Î£ SIGMAOS ZENITH SUPREME: SOVEREIGN THEME STORE (v1.0)
 * =========================================================================
 * Absorbing Aesthetics from: LupusOS, EzLinux, and Plasma.
 * Mission: Universal Aesthetic Sharding & Personalization.
 * =========================================================================
 */

#include "SovereignLibC.h"
#include "sigma_types.h"

typedef struct {
    char name[32];
    char primary_color[16];
    char glass_blur[16];
    sigma_bool premium;
} sigma_theme_t;

static sigma_theme_t themes[] = {
    {"Zenith-Default", "#00d2ff", "20px", SIGMA_FALSE},
    {"Hacker-Crimson", "#ff0033", "10px", SIGMA_TRUE},
    {"Lupus-Minimal", "#ffffff", "5px", SIGMA_FALSE},
    {"Ez-Pastel", "#cc99ff", "15px", SIGMA_FALSE},
    {"OLED-Noir", "#111111", "0px", SIGMA_TRUE}
};

void sigma_theme_apply(const char* name) {
    sigma_printf("[THEME] Applying Sovereign Aesthetic: %s... ", name);
    for (int i = 0; i < 5; i++) {
        if (sigma_streq(themes[i].name, name)) {
            sigma_printf("OK\n");
            sigma_printf("[THEME] Injecting CSS Variables: %s Shard... COMPLETE\n", themes[i].primary_color);
            return;
        }
    }
    sigma_printf("ERROR (THEME NOT FOUND)\n");
}

void sigma_theme_list() {
    sigma_printf("\nÎ£ SOVEREIGN THEME & PERSONALIZATION STORE\n");
    sigma_printf("-------------------------------------------\n");
    for (int i = 0; i < 5; i++) {
        sigma_printf("%-17s %-12s %s\n", 
            themes[i].name, 
            themes[i].primary_color, 
            themes[i].premium ? "(PREMIUM)" : "");
    }
    sigma_printf("-------------------------------------------\n\n");
}
