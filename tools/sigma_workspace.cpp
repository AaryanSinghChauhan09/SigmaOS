/*
 * =========================================================================
 * Î£ SIGMAOS: SIGMA WORKSPACE PROFILES (sigma_workspace) v1.0
 * =========================================================================
 * Mission: Save and restore complete UI/workflow layouts.
 * Inspiration: NixOS declarative configs + macOS Spaces.
 * Principle: Snapshot-backed, rehydrate in <1ms.
 * =========================================================================
 */

#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "SigmaOOP.hpp"

namespace SigmaOS {
namespace Tools {

struct WorkspaceProfile {
    char      name[64];
    char      theme[32];
    sigma_u32 window_count;
    sigma_u32 layout_id;    /* 0=tiling, 1=floating, 2=focus, 3=zen */
    sigma_u8  ai_assist;
    sigma_u8  notifications;
};

class SigmaWorkspaceProfiles : public SigmaObject, public SigmaSingleton<SigmaWorkspaceProfiles> {
    friend class SigmaSingleton<SigmaWorkspaceProfiles>;
public:
    const char* type_name() const noexcept override { return "SigmaWorkspaceProfiles"; }

    void init() {
        m_profile_count  = 0;
        m_active_profile = 0xFFFFFFFFu;
        sigma_log_info("[WORKSPACE] Sigma Workspace Profiles v1.0 initialized.");
        /* Create defaults */
        save_profile("default",    "sigma-dark",  0, 1, 1);
        save_profile("coding",     "sigma-focus", 3, 0, 0);
        save_profile("gaming",     "sigma-neon",  1, 1, 0);
        save_profile("zen",        "sigma-zen",   0, 3, 0);
    }

    void save_profile(const char* name, const char* theme,
                      sigma_u32 win_count, sigma_u32 layout, sigma_u8 ai_assist) {
        if (m_profile_count >= MAX_PROFILES) return;
        WorkspaceProfile& p = m_profiles[m_profile_count];
        sigma_u32 i = 0;
        while (name[i] && i < 63) { p.name[i] = name[i]; i++; } p.name[i] = '\0';
        i = 0;
        while (theme[i] && i < 31) { p.theme[i] = theme[i]; i++; } p.theme[i] = '\0';
        p.window_count   = win_count;
        p.layout_id      = layout;
        p.ai_assist      = ai_assist;
        p.notifications  = 1;
        m_profile_count++;
        sigma_log_info("[WORKSPACE] Profile saved: '%s' (theme=%s, layout=%u, ai=%u)",
                       name, theme, layout, ai_assist);
    }

    void load_profile(const char* name) {
        for (sigma_u32 i = 0; i < m_profile_count; i++) {
            sigma_u32 j = 0;
            while (m_profiles[i].name[j] == name[j] && name[j]) j++;
            if (!name[j] && !m_profiles[i].name[j]) {
                m_active_profile = i;
                sigma_log_info("[WORKSPACE] Loaded profile '%s': theme=%s, layout=%u",
                               m_profiles[i].name, m_profiles[i].theme, m_profiles[i].layout_id);
                return;
            }
        }
        sigma_log_infoor("[WORKSPACE] Profile '%s' not found.", name);
    }

    void list_profiles() const {
        sigma_log_info("[WORKSPACE] ===== Workspace Profiles =====");
        for (sigma_u32 i = 0; i < m_profile_count; i++) {
            sigma_log_info("[WORKSPACE] %s %-16s theme=%-12s layout=%u%s",
                (i == m_active_profile) ? ">" : " ",
                m_profiles[i].name, m_profiles[i].theme,
                m_profiles[i].layout_id,
                (i == m_active_profile) ? " [ACTIVE]" : "");
        }
    }

private:
    static constexpr sigma_u32 MAX_PROFILES = 32;
    SigmaWorkspaceProfiles() : m_profile_count(0), m_active_profile(0xFFFFFFFFu) {}
    WorkspaceProfile m_profiles[MAX_PROFILES];
    sigma_u32        m_profile_count;
    sigma_u32        m_active_profile;
};

} // namespace Tools
} // namespace SigmaOS

extern "C" {
void workspace_init()                         { SigmaOS::Tools::SigmaWorkspaceProfiles::getInstance().init(); }
void workspace_load(const char* name)         { SigmaOS::Tools::SigmaWorkspaceProfiles::getInstance().load_profile(name); }
void workspace_list()                         { SigmaOS::Tools::SigmaWorkspaceProfiles::getInstance().list_profiles(); }
}

