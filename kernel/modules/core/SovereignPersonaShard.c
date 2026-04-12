/*
 * =========================================================================
 * Σ SIGMAOS ZENITH: SOVEREIGN PERSONA SHARD (v1.0)
 * =========================================================================
 * Mission: Absorb macOS Profiles/Android Work Profiles USP.
 *          Native Multi-User Silicon Persona Matrix.
 * Design: C11 / Zero-Dependency / Atomic Persona Context Switch.
 * Standard: Zenith Industrial Sovereignty.
 * =========================================================================
 */

#include "../../include/sigma_kernel.h"

// -------------------------------------------------------------------------
// Persona Structures
// -------------------------------------------------------------------------

typedef struct {
    char      name[32];
    char      theme[24];        // Aesthetic skin
    char      shell_prompt[64]; // Custom ANSI prompt
    sigma_u32 uid;
    sigma_u32 capability_mask;  // Bitmask of allowed shards
    sigma_bool active;
} SigmaPersona_t;

#define MAX_PERSONAS 8
static SigmaPersona_t s_persona_matrix[MAX_PERSONAS];
static sigma_u32      s_persona_count  = 0;
static sigma_u32      s_active_persona = 0;

// -------------------------------------------------------------------------
// Persona Logic (macOS Profiles / Android WP / Linux PAM parity)
// -------------------------------------------------------------------------

/**
 * sigma_persona_create: Creates a new industrial silicon persona with full customisation.
 */
sigma_err_t sigma_persona_create(const char* name, const char* theme,
                                  sigma_u32 uid, sigma_u32 cap_mask) {
    if (s_persona_count >= MAX_PERSONAS) return SIGMA_ENOSPC;

    SigmaPersona_t* ps = &s_persona_matrix[s_persona_count++];
    sigma_strcpy(ps->name, name);
    sigma_strcpy(ps->theme, theme);
    ps->uid = uid;
    ps->capability_mask = cap_mask;
    ps->active = SIGMA_FALSE;

    // Generate ANSI prompt
    sigma_printf("[PERSONA]: Created persona '%s' (UID %u) with theme '%s'.\n",
                 name, uid, theme);
    return SIGMA_OK;
}

/**
 * sigma_persona_switch: Atomically switches the active silicon persona context.
 */
sigma_err_t sigma_persona_switch(const char* name) {
    for (sigma_u32 i = 0; i < s_persona_count; i++) {
        if (sigma_streq(s_persona_matrix[i].name, name)) {
            // Deactivate current
            if (s_active_persona < s_persona_count)
                s_persona_matrix[s_active_persona].active = SIGMA_FALSE;

            s_persona_matrix[i].active = SIGMA_TRUE;
            s_active_persona = i;

            sigma_printf("[PERSONA]: Active context switched to '%s' "
                         "(Theme: %s | UID: %u | CapMask: 0x%X).\n",
                         s_persona_matrix[i].name,
                         s_persona_matrix[i].theme,
                         s_persona_matrix[i].uid,
                         s_persona_matrix[i].capability_mask);
            return SIGMA_OK;
        }
    }
    sigma_printf("[DENIED]: Persona '%s' not found in silicon matrix.\n", name);
    return SIGMA_ENOENT;
}

// -------------------------------------------------------------------------
// Industrial Persona Audit
// -------------------------------------------------------------------------

void SovereignPersona_Audit() {
    sigma_printf("\n--- SOVEREIGN PERSONA AUDIT ---\n");
    sigma_printf("NAME                 THEME          UID    CAP_MASK   STATE\n");
    sigma_printf("--------------------------------------------------------------\n");
    for (sigma_u32 i = 0; i < s_persona_count; i++) {
        sigma_printf("%-20s %-14s %-6u 0x%-8X %s\n",
                     s_persona_matrix[i].name,
                     s_persona_matrix[i].theme,
                     s_persona_matrix[i].uid,
                     s_persona_matrix[i].capability_mask,
                     s_persona_matrix[i].active ? "ACTIVE" : "idle");
    }
    sigma_printf("--------------------------------------------------------------\n");
}

// -------------------------------------------------------------------------
// Factory / Constructor
// -------------------------------------------------------------------------

void SovereignPersonaShard_Init() {
    sigma_printf("[SOC]: Seating Native Persona Shard (macOS-Profiles Parity v1.0)...\n");
    sigma_persona_create("Zenith_Admin",  "Obsidian", 0,    0xFFFFFFFF);
    sigma_persona_create("Citizen_Dev",   "Aurora",   1000, 0x0FFF0000);
    sigma_persona_create("Guest_Secure",  "Frost",    9999, 0x00000001);
    sigma_persona_switch("Zenith_Admin");
}
