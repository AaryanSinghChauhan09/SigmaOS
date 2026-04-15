/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN AUTOMATION ENGINE HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_AUTOMATION_ENGINE_H
#define SOVEREIGN_AUTOMATION_ENGINE_H

#include "SigmaOOP.h"

CLASS_DECLARE(SovereignAutomationEngine) {
    SigmaObject_t core;
    sigma_u32     scripts_run;
    sigma_u32     total_commands;
    
    // Virtual Methods
    VIRTUAL(sigma_err_t, execute_script, struct SovereignAutomationEngine* self, const char* script_content);
    VIRTUAL(void, audit_automation, struct SovereignAutomationEngine* self);
};

SovereignAutomationEngine_t SovereignAutomationEngine_Create(void);
void SovereignAutomationEngine_Init(void);

#endif /* SOVEREIGN_AUTOMATION_ENGINE_H */
