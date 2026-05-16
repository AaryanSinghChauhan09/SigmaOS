#include "../include/hal/sigma_hal.h"
#include "../include/libc/SovereignLibC.h"
#ifndef SOVEREIGN_THEME_MARKET_HPP
#define SOVEREIGN_THEME_MARKET_HPP

#include "../include/core/sigma_types.h"

class SovereignThemeMarketEngine {
public:
    static SovereignThemeMarketEngine& getInstance();
    void init();
    void publishTheme(const char* theme_name, const char* author);
    bool applyTheme(const char* theme_name);
    void listThemes();

private:
    SovereignThemeMarketEngine() : available_themes(0), active_theme_idx(0) {}
    char theme_names[64][48];
    char theme_authors[64][32];
    sigma_u32 available_themes;
    sigma_u32 active_theme_idx;
};

extern "C" {
    void theme_market_init();
    void theme_market_publish(const char* name, const char* author);
    bool theme_market_apply(const char* name);
    void theme_market_list();
}

#endif

