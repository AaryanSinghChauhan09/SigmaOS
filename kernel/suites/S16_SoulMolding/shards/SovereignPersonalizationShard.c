/*
 * =========================================================================
 * Σ SIGMAOS ZENITH: SOVEREIGN PERSONALIZATION (v2.0 — DEEP)
 * =========================================================================
 * Mission: Dynamic Identity Mapping & Sentient UI Personalization.
 * Principles: Identity Sovereignty, Profile Management, Hot-Swap Themes.
 *
 * v2.0: Real profile store with theme engine, locale settings,
 *       accessibility flags, and multi-user switching.
 * =========================================================================
 */

#include "../../include/sigma_kernel.h"

/* --- Theme Definition --- */

typedef struct {
    char   name[32];
    sigma_u32 bg_color;       /* 24-bit RGB background  */
    sigma_u32 fg_color;       /* 24-bit RGB foreground   */
    sigma_u32 accent_color;   /* 24-bit RGB accent       */
    sigma_u32 opacity;        /* 0-100 window opacity    */
} SigmaTheme_t;

/* --- User Profile --- */

typedef struct {
    char           username[32];
    char           tier[16];
    char           locale[8];         /* e.g. "en_US"           */
    sigma_u32      accessibility;     /* bitfield: 0x01=high-contrast, 0x02=large-font */
    SigmaTheme_t   theme;
    sigma_u32      login_count;
} SovereignProfile_t;

/* --- Profile Registry --- */

#define MAX_PROFILES 8
static SovereignProfile_t s_profiles[MAX_PROFILES];
static sigma_u32 s_profile_count = 0;
static sigma_u32 s_active_profile = 0;

/**
 * sigma_profile_create: Registers a new user profile.
 */
sigma_err_t sigma_profile_create(const char* username, const char* tier,
                                 sigma_u32 bg, sigma_u32 fg, sigma_u32 accent) {
    if (s_profile_count >= MAX_PROFILES) return SIGMA_ENOSPC;

    SovereignProfile_t* p = &s_profiles[s_profile_count++];
    sigma_strncpy(p->username, username, 32);
    sigma_strncpy(p->tier, tier, 16);
    sigma_strncpy(p->locale, "en_US", 8);
    p->accessibility = 0;
    p->login_count   = 0;

    sigma_strncpy(p->theme.name, "Sentient-Chroma", 32);
    p->theme.bg_color     = bg;
    p->theme.fg_color     = fg;
    p->theme.accent_color = accent;
    p->theme.opacity      = 95;

    sigma_printf("[IDENTITY]: Created profile '%s' (tier: %s, theme: 0x%06X/0x%06X)\n",
                 username, tier, bg, fg);
    return SIGMA_OK;
}

/**
 * sigma_profile_switch: Switches the active user context.
 * All system aesthetics and behavior adapt to the new profile.
 */
sigma_err_t sigma_profile_switch(sigma_u32 profile_index) {
    if (profile_index >= s_profile_count) return SIGMA_EINVAL;

    s_active_profile = profile_index;
    SovereignProfile_t* p = &s_profiles[profile_index];
    p->login_count++;

    sigma_printf("[IDENTITY]: Switched to profile '%s' (login #%u)\n",
                 p->username, p->login_count);
    sigma_printf("  [CHROMA]: BG=0x%06X FG=0x%06X ACCENT=0x%06X OPACITY=%u%%\n",
                 p->theme.bg_color, p->theme.fg_color,
                 p->theme.accent_color, p->theme.opacity);
    sigma_printf("  [LOCALE]: %s | Accessibility: 0x%02X\n",
                 p->locale, p->accessibility);
    return SIGMA_OK;
}

/**
 * sigma_profile_set_theme: Hot-swaps theme colors for the active user.
 */
void sigma_profile_set_theme(sigma_u32 bg, sigma_u32 fg, sigma_u32 accent) {
    SovereignProfile_t* p = &s_profiles[s_active_profile];
    p->theme.bg_color     = bg;
    p->theme.fg_color     = fg;
    p->theme.accent_color = accent;

    sigma_printf("[CHROMA]: Theme updated for '%s' -> BG=0x%06X FG=0x%06X\n",
                 p->username, bg, fg);
}

/**
 * sigma_profile_set_accessibility: Toggles accessibility flags.
 */
void sigma_profile_set_accessibility(sigma_u32 flags) {
    SovereignProfile_t* p = &s_profiles[s_active_profile];
    p->accessibility = flags;
    sigma_printf("[IDENTITY]: Accessibility flags set to 0x%02X for '%s'\n",
                 flags, p->username);
}

/* --- Audit --- */

void SovereignPersonalization_Audit(void) {
    sigma_printf("\n--- SOVEREIGN PERSONALIZATION AUDIT ---\n");
    sigma_printf("%-20s %-14s %-10s %-8s\n", "USER", "TIER", "LOCALE", "LOGINS");
    sigma_printf("------------------------------------------------------\n");
    for (sigma_u32 i = 0; i < s_profile_count; i++) {
        SovereignProfile_t* p = &s_profiles[i];
        sigma_printf("%-20s %-14s %-10s %-8u %s\n",
                     p->username, p->tier, p->locale, p->login_count,
                     (i == s_active_profile) ? "<- ACTIVE" : "");
    }
    sigma_printf("------------------------------------------------------\n");
}

/* --- Module Factory --- */

void SovereignPersonalization_Register(void) {
    sigma_printf("[REGISTRY]: Sovereign Personalization v2.0 (Deep) active.\n");

    /* Seed the primary user profile */
    sigma_profile_create("AaryanSinghChauhan09", "Zenith Supreme",
                         0x0A0A2E, 0xE0E0FF, 0xFF00FF);
    sigma_profile_switch(0);
}



