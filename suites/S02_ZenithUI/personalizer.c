#include "../../include/libc/SovereignLibC.h"
#include "../../include/core/sigma_types.h"
/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: PERSONALIZER-ZENITH (v1.0 - CUSTOMISATION & IDENTITY)
 * =============================================================================
 * Algorithm: Sovereign Identity Mapping (SIM)
 * Principles:
 *   - Unified kernel-level theme and personal identity management.
 *   - Direct VBE-to-Shard-Color mapping for absolute GUI sovereignty.
 *   - Personalised resource allocation based on user 'Sovereign ID'.
 * =============================================================================
 */

#include "../../include/sigma_kernel_types.h"

#define MAX_PERSONALITIES 16

typedef struct SovereignIdentity {
    sigma_u64 id;
    sigma_u32 theme_color;      /* VBE hex color code (e.g., 0x0093FF) */
    char user_tag[32];
    sigma_bool active;
} SovereignIdentity;

static SovereignIdentity g_identities[MAX_PERSONALITIES];
static sigma_u32 g_identity_count = 0;

/* =========================================================================
 * CORE IDENTITY Engine (The Personalizer Shard)
 * ========================================================================= */

void personalizer_init(void) {
<<<<<<<< HEAD:suites/S02_ZenithUI/personalizer.c
    for (int i = 0; i < MAX_PERSONALITIES; i++) g_identities[i].active = FALSE;
    // ksigma_printf("[PERSONALIZER]: Sovereign Customisation & Identity Shard Online.\n");
========
    for (int i = 0; i < MAX_PERSONALITIES; i++) g_identities[i].active = SIGMA_FALSE;
    // kprintf("[PERSONALIZER]: Sovereign Customisation & Identity Shard Online.\n");
>>>>>>>> ad8016503ce074e8980abb23e1a44b78be830645:kernel/shards/security/personalizer.c
}

void register_identity(const char* tag, sigma_u32 color) {
    if (g_identity_count >= MAX_PERSONALITIES) return;
    
    SovereignIdentity* id = &g_identities[g_identity_count++];
    id->id = (sigma_u64)g_identity_count;
    id->theme_color = color;
    
    sigma_u32 i = 0;
    while (i < 31 && tag[i]) { id->user_tag[i] = tag[i]; i++; }
    id->user_tag[i] = '\0';
    id->active = SIGMA_TRUE;
    
    // ksigma_printf("[PERSONALIZER]: Sovereign-ID Registered: %s (Theme: 0x%x)\n", 
    //         tag, color);
}

sigma_u32 get_current_theme_color(void) {
    if (g_identity_count == 0) return 0xFFFFFF; // Default white
    return g_identities[0].theme_color;
}
