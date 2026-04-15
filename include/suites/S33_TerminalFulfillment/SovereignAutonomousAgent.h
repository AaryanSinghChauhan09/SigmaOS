/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN AUTONOMOUS AGENT HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_AUTONOMOUS_AGENT_H
#define SOVEREIGN_AUTONOMOUS_AGENT_H

#include "SigmaOOP.h"

CLASS_DECLARE(SovereignAutonomousAgent) {
    SigmaObject_t core;
    sigma_u32     agent_id;
    sigma_bool    prowling;
    sigma_u32     missions_completed;
    
    // Virtual Methods
    VIRTUAL(void, bootstrap_mission, struct SovereignAutonomousAgent* self);
    VIRTUAL(void, prowl_sector, struct SovereignAutonomousAgent* self, const char* sector);
    VIRTUAL(void, execute_autonomous_audit, struct SovereignAutonomousAgent* self);
};

SovereignAutonomousAgent_t SovereignAutonomousAgent_Create(sigma_u32 id);
void SovereignAutonomousAgent_Init(void);

#endif /* SOVEREIGN_AUTONOMOUS_AGENT_H */
