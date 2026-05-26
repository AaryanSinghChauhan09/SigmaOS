/**
 * ===========================================================================
 * Σ SIGMAOS: SOVEREIGN PROFILE ENGINE (S-PROFILE) v1.0
 * ===========================================================================
 * Mission: Arch/KDE Plasma-grade extreme customization with dynamic profile
 *          switching, .sigmatheme spec, keybind engine, and AI personalization.
 *
 * Inspired by: KDE Plasma / i3wm / Arch Linux / macOS Stage Manager
 * ZERO-DEPENDENCY: Profile state managed at kernel level.
 * ===========================================================================
 */

#include "../../../include/sigma_log.h"
#include "../../../include/sigma_kernel_types.h"

#define PROFILE_MAX_ENTRIES   32
#define PROFILE_MAX_KEYBINDS  64
#define PROFILE_MAX_THEMES    16

namespace SigmaOS {
namespace Kernel {
namespace Profiles {

/* =========================================================================
 * PROFILE TYPES — System-wide behavior presets
 * ========================================================================= */
enum ProfileType {
    PROFILE_GAMING       = 0,
    PROFILE_CODING       = 1,
    PROFILE_AI_WORKSTATION = 2,
    PROFILE_CYBERSECURITY = 3,
    PROFILE_CREATOR      = 4,
    PROFILE_ENTERPRISE   = 5,
    PROFILE_MINIMAL      = 6,
    PROFILE_PRESENTATION = 7
};

struct SystemProfile {
    sigma_u32   id;
    ProfileType type;
    char        name[64];
    char        description[128];
    sigma_u32   scheduler_priority;  /* 0=balanced, 1=latency, 2=throughput */
    sigma_u32   power_mode;          /* Maps to PowerState */
    sigma_u32   gpu_boost;           /* 0=off, 1=balanced, 2=max */
    bool        ai_agents_active;
    bool        strict_security;
    bool        notifications_muted;
    bool        active;
};

static SystemProfile s_profiles[PROFILE_MAX_ENTRIES];
static sigma_u32     s_profile_count = 0;
static sigma_u32     s_active_profile = 0;

/* =========================================================================
 * KEYBIND ENGINE — i3-style keyboard shortcut mapping
 * ========================================================================= */
struct Keybind {
    sigma_u32 id;
    char      combo[32];     /* e.g. "Super+Shift+G" */
    char      action[64];    /* e.g. "profile_switch gaming" */
    bool      enabled;
};

static Keybind   s_keybinds[PROFILE_MAX_KEYBINDS];
static sigma_u32 s_keybind_count = 0;

/* =========================================================================
 * THEME SPEC — .sigmatheme format
 * ========================================================================= */
struct SigmaTheme {
    sigma_u32 id;
    char      name[32];
    sigma_u32 accent_color;    /* ARGB */
    sigma_u32 bg_color;
    sigma_u32 text_color;
    sigma_u32 border_radius;
    bool      glassmorphism;
    bool      dark_mode;
};

static SigmaTheme s_themes[PROFILE_MAX_THEMES];
static sigma_u32  s_theme_count = 0;
static sigma_u32  s_active_theme = 0;

/* ---- Helpers ---- */
static void add_profile(ProfileType type, const char* name, const char* desc,
                         sigma_u32 sched, sigma_u32 power, sigma_u32 gpu,
                         bool ai, bool sec, bool mute) {
    if (s_profile_count >= PROFILE_MAX_ENTRIES) return;
    SystemProfile* p = &s_profiles[s_profile_count];
    p->id = s_profile_count + 1;
    p->type = type;
    sigma_strncpy(p->name, name, 64);
    sigma_strncpy(p->description, desc, 128);
    p->scheduler_priority = sched;
    p->power_mode = power;
    p->gpu_boost = gpu;
    p->ai_agents_active = ai;
    p->strict_security = sec;
    p->notifications_muted = mute;
    p->active = false;
    s_profile_count++;
}

static void add_keybind(const char* combo, const char* action) {
    if (s_keybind_count >= PROFILE_MAX_KEYBINDS) return;
    Keybind* k = &s_keybinds[s_keybind_count];
    k->id = s_keybind_count + 1;
    sigma_strncpy(k->combo, combo, 32);
    sigma_strncpy(k->action, action, 64);
    k->enabled = true;
    s_keybind_count++;
}

static void add_theme(const char* name, sigma_u32 accent, sigma_u32 bg,
                       sigma_u32 text, sigma_u32 radius, bool glass, bool dark) {
    if (s_theme_count >= PROFILE_MAX_THEMES) return;
    SigmaTheme* t = &s_themes[s_theme_count];
    t->id = s_theme_count + 1;
    sigma_strncpy(t->name, name, 32);
    t->accent_color = accent;
    t->bg_color = bg;
    t->text_color = text;
    t->border_radius = radius;
    t->glassmorphism = glass;
    t->dark_mode = dark;
    s_theme_count++;
}

/* =========================================================================
 * SovereignProfileEngine
 * ========================================================================= */
class SovereignProfileEngine {
public:
    static SovereignProfileEngine& getInstance() {
        static SovereignProfileEngine instance;
        return instance;
    }

    void init() {
        sigma_log("[PROFILE]: ═══════════════════════════════════════════════\n");
        sigma_log("[PROFILE]: Σ SOVEREIGN PROFILE ENGINE v1.0 — Init...\n");
        sigma_log("[PROFILE]: ═══════════════════════════════════════════════\n");

        /* Register system profiles */
        add_profile(PROFILE_GAMING, "Gaming", "Low-latency GPU-first mode",
                    1, 6, 2, false, false, true);
        add_profile(PROFILE_CODING, "Developer", "IDE-optimized with AI code assist",
                    0, 1, 0, true, false, false);
        add_profile(PROFILE_AI_WORKSTATION, "AI Workstation", "GPU compute + local inference",
                    2, 0, 2, true, false, true);
        add_profile(PROFILE_CYBERSECURITY, "Cybersecurity", "Max isolation, audit logging",
                    0, 1, 0, true, true, false);
        add_profile(PROFILE_CREATOR, "Creator", "Optimized for Blender/DaVinci",
                    2, 0, 2, false, false, true);
        add_profile(PROFILE_ENTERPRISE, "Enterprise", "Compliance + monitoring active",
                    0, 1, 0, true, true, false);
        add_profile(PROFILE_MINIMAL, "Minimal", "Resource-light, IoT-friendly",
                    0, 2, 0, false, false, false);
        add_profile(PROFILE_PRESENTATION, "Presentation", "Clean UI, no notifications",
                    0, 1, 0, false, false, true);

        /* Register keybinds */
        add_keybind("Super+Shift+G", "profile_switch gaming");
        add_keybind("Super+Shift+D", "profile_switch developer");
        add_keybind("Super+Shift+A", "profile_switch ai_workstation");
        add_keybind("Super+Shift+S", "profile_switch cybersecurity");
        add_keybind("Super+Shift+P", "profile_switch presentation");
        add_keybind("Super+Space", "command_palette");
        add_keybind("Super+T", "terminal");
        add_keybind("Super+L", "lock_screen");
        add_keybind("Super+E", "file_manager");

        /* Register themes */
        add_theme("Zenith Dark", 0xFF6C63FF, 0xFF1A1A2E, 0xFFE0E0E0, 12, true, true);
        add_theme("Zenith Light", 0xFF4A90D9, 0xFFF5F5F5, 0xFF1A1A1A, 12, false, false);
        add_theme("Sovereign Gold", 0xFFD4A017, 0xFF0D0D0D, 0xFFDADADA, 8, true, true);
        add_theme("Matrix Green", 0xFF00FF41, 0xFF000000, 0xFF00FF41, 0, false, true);

        /* Activate default profile */
        switchProfile(2); /* Developer */

        sigma_log("[PROFILE]: %d profiles | %d keybinds | %d themes registered.\n",
                  s_profile_count, s_keybind_count, s_theme_count);
        sigma_log("[PROFILE]: Profile Engine READY.\n");
    }

    void switchProfile(sigma_u32 profile_id) {
        if (profile_id == 0 || profile_id > s_profile_count) return;

        /* Deactivate current */
        if (s_active_profile > 0 && s_active_profile <= s_profile_count)
            s_profiles[s_active_profile - 1].active = false;

        SystemProfile* p = &s_profiles[profile_id - 1];
        p->active = true;
        s_active_profile = profile_id;

        sigma_log("[PROFILE]: ┌──────────────────────────────────────────┐\n");
        sigma_log("[PROFILE]: │ SWITCHED TO: %-27s │\n", p->name);
        sigma_log("[PROFILE]: └──────────────────────────────────────────┘\n");
        sigma_log("[PROFILE]:   Scheduler   : %s\n",
                  p->scheduler_priority == 0 ? "BALANCED" :
                  p->scheduler_priority == 1 ? "LATENCY" : "THROUGHPUT");
        sigma_log("[PROFILE]:   GPU Boost   : %s\n",
                  p->gpu_boost == 0 ? "OFF" : p->gpu_boost == 1 ? "BALANCED" : "MAX");
        sigma_log("[PROFILE]:   AI Agents   : %s\n", p->ai_agents_active ? "ON" : "OFF");
        sigma_log("[PROFILE]:   Security    : %s\n", p->strict_security ? "STRICT" : "STANDARD");
        sigma_log("[PROFILE]:   Muted       : %s\n", p->notifications_muted ? "YES" : "NO");
    }

    void reportStatus() {
        sigma_log("\n--- Σ SOVEREIGN PROFILE ENGINE ---\n");
        for (sigma_u32 i = 0; i < s_profile_count; i++) {
            sigma_log("| [%d] %-20s %s\n", s_profiles[i].id, s_profiles[i].name,
                      s_profiles[i].active ? "◄ ACTIVE" : "");
        }
        sigma_log("| Active Theme: %s\n",
                  s_active_theme > 0 ? s_themes[s_active_theme - 1].name : "Zenith Dark");
        sigma_log("| Keybinds: %d registered\n", s_keybind_count);
        sigma_log("----------------------------------\n");
    }

private:
    SovereignProfileEngine() = default;
};

} // namespace Profiles
} // namespace Kernel
} // namespace SigmaOS

extern "C" void profile_engine_init() {
    SigmaOS::Kernel::Profiles::SovereignProfileEngine::getInstance().init();
}
extern "C" void profile_switch(sigma_u32 id) {
    SigmaOS::Kernel::Profiles::SovereignProfileEngine::getInstance().switchProfile(id);
}
extern "C" void profile_status() {
    SigmaOS::Kernel::Profiles::SovereignProfileEngine::getInstance().reportStatus();
}
