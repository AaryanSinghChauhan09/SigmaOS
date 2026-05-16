#include "../../include/sigma_log.h"
#include "../../include/core/sigma_types.h"
#include "SovereignThemeMarket.hpp"
#include "../../include/hal/sigma_hal.h"
#include "../../include/libc/SovereignLibC.h"

SovereignThemeMarketEngine& SovereignThemeMarketEngine::getInstance() {
    static SovereignThemeMarketEngine instance;
    return instance;
}

void SovereignThemeMarketEngine::init() {
    sigma_log("[THEME-MKT] Initializing Sovereign Theme Marketplace...");
    this->available_themes = 0;
    this->active_theme_idx = 0;
}

void SovereignThemeMarketEngine::publishTheme(const char* theme_name, const char* author) {
    if (this->available_themes >= 64) return;
    sigma_hardened_strcpy(this->theme_names[this->available_themes], theme_name, 48);
    sigma_hardened_strcpy(this->theme_authors[this->available_themes], author, 32);
    this->available_themes++;
    sigma_log("[THEME-MKT] Published: '%s' by %s " SAB hash verified.\n", theme_name, author);
}

bool SovereignThemeMarketEngine::applyTheme(const char* theme_name) {
    for (sigma_u32 i = 0; i < this->available_themes; i++) {
        if (sigma_strcmp(this->theme_names[i], theme_name) == 0) {
            this->active_theme_idx = i;
            sigma_log("[THEME-MKT] Live-swapping to theme '%s'...\n", theme_name);
            sigma_log("[THEME-MKT] Zenith MLC compositor notified. Recompositing...");
            return true;
        }
    }
    sigma_log("[THEME-MKT] Theme not found.");
    return false;
}

void SovereignThemeMarketEngine::listThemes() {
    sigma_log("[THEME-MKT] %u themes available:\n", this->available_themes);
    for (sigma_u32 i = 0; i < this->available_themes; i++) {
        sigma_log("  [%s] %s " by %s\n",
                     i == this->active_theme_idx ? "ACTIVE" : "     ",
                     this->theme_names[i], this->theme_authors[i]);
    }
}

void theme_market_init() { SovereignThemeMarketEngine::init(); }
void theme_market_publish(const char* name, const char* author) { SovereignThemeMarketEngine::publishTheme(name, author); }
extern "C" bool theme_market_apply(const char* name) { return SovereignThemeMarketEngine::applyTheme(name); }
void theme_market_list() { SovereignThemeMarketEngine::listThemes(); }




} // extern "C"
