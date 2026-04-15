/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN PERSONALIZER HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_PERSONALIZER_H
#define SOVEREIGN_PERSONALIZER_H

#include "SigmaOOP.h"

CLASS_DECLARE(SovereignPersonalizer) {
    SigmaObject_t core;
    char user_name[64];
    char theme_id[32];
    sigma_u32 automation_level;
    
    // Virtual Methods
    VIRTUAL(void, apply_theme, struct SovereignPersonalizer* self, const char* name);
    VIRTUAL(void, set_automation_policy, struct SovereignPersonalizer* self, sigma_u32 level);
    VIRTUAL(void, trigger_self_healing, struct SovereignPersonalizer* self);
    VIRTUAL(void, audit_customizations, struct SovereignPersonalizer* self);
};

SovereignPersonalizer_t SovereignPersonalizer_Create(const char* user);
void SovereignPersonalizer_Init(void);

#endif /* SOVEREIGN_PERSONALIZER_H */
