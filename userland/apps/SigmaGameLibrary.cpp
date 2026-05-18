/*
 * =========================================================================
 * Σ SIGMAOS: SIGMA GAME LIBRARY (SigmaGameLibrary) v1.0
 * =========================================================================
 * Mission: Curated open-source gaming hub.
 * Inspiration: Steam client + Lutris.
 * Principle: Zero-overhead launching with direct GPU HAL access.
 * =========================================================================
 */

#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "SigmaOOP.hpp"

namespace SigmaOS {
namespace Apps {

struct GameEntry {
    char      title[64];
    char      executable_path[128];
    sigma_u8  is_installed;
    sigma_u8  requires_esports_mode;
};

class SigmaGameLibrary : public SigmaObject, public SigmaSingleton<SigmaGameLibrary> {
    friend class SigmaSingleton<SigmaGameLibrary>;
public:
    const char* type_name() const noexcept override { return "SigmaGameLibrary"; }

    void init() {
        m_game_count = 0;
        sigma_log_info("[GAMELIB] Sigma Game Library v1.0 initialized.");
        
        /* Pre-populate library */
        add_game("0 A.D.", "/usr/games/0ad", 1, 0);
        add_game("Xonotic", "/usr/games/xonotic", 1, 1);
        add_game("SuperTuxKart", "/usr/games/supertuxkart", 1, 0);
        add_game("Minetest", "/usr/games/minetest", 0, 0);
    }

    void add_game(const char* title, const char* path, sigma_u8 installed, sigma_u8 esports) {
        if (m_game_count >= MAX_GAMES) return;
        GameEntry& g = m_games[m_game_count++];
        sigma_u32 i = 0;
        while (title[i] && i < 63) { g.title[i] = title[i]; i++; } g.title[i] = '\0';
        i = 0;
        while (path[i] && i < 127) { g.executable_path[i] = path[i]; i++; } g.executable_path[i] = '\0';
        g.is_installed = installed;
        g.requires_esports_mode = esports;
    }

    void launch_game(const char* title) {
        for (sigma_u32 i = 0; i < m_game_count; i++) {
            sigma_u32 j = 0;
            while (m_games[i].title[j] == title[j] && title[j]) j++;
            if (!title[j] && !m_games[i].title[j]) {
                if (!m_games[i].is_installed) {
                    sigma_log_error("[GAMELIB] Cannot launch '%s': Not installed.", title);
                    return;
                }
                
                sigma_log_info("[GAMELIB] Launching '%s'...", title);
                if (m_games[i].requires_esports_mode) {
                    sigma_log_info("[GAMELIB] Auto-enabling eSports latency profile for %s.", title);
                    /* In reality, this would call latency_set_esports() */
                }
                sigma_log_info("[GAMELIB] Executing: %s (Direct GPU HAL Access)", m_games[i].executable_path);
                return;
            }
        }
        sigma_log_error("[GAMELIB] Game '%s' not found in library.", title);
    }

    void list_games() const {
        sigma_log_info("[GAMELIB] ===== Sigma Game Library =====");
        for (sigma_u32 i = 0; i < m_game_count; i++) {
            sigma_log_info("[GAMELIB] [%s] %-20s %s", 
                m_games[i].is_installed ? "INSTALL" : " CLOUD ",
                m_games[i].title,
                m_games[i].requires_esports_mode ? "[eSports]" : "");
        }
    }

private:
    static constexpr sigma_u32 MAX_GAMES = 256;
    SigmaGameLibrary() : m_game_count(0) {}
    GameEntry m_games[MAX_GAMES];
    sigma_u32 m_game_count;
};

} // namespace Apps
} // namespace SigmaOS

extern "C" {
void gamelib_init()                        { SigmaOS::Apps::SigmaGameLibrary::getInstance().init(); }
void gamelib_launch(const char* title)     { SigmaOS::Apps::SigmaGameLibrary::getInstance().launch_game(title); }
void gamelib_list()                        { SigmaOS::Apps::SigmaGameLibrary::getInstance().list_games(); }
}
