#ifndef SOVEREIGN_AETHER_ORCHESTRATOR_H
#define SOVEREIGN_AETHER_ORCHESTRATOR_H

#include "../libc/SovereignLibC.h"
#include "../SovereignOmniShard.h"

/**
 * Σ Sovereign Aether Orchestrator Types
 * (Struct defined in SovereignOmniShard.h)
 */

/**
 * Σ Dispatch Mission
 * Routes a mission ID to the appropriate system shard.
 */
void SovereignAetherOrchestrator_init(SovereignAetherOrchestrator* self);
void SovereignAetherOrchestrator_RouteMission(SovereignAetherOrchestrator* self, const char* mission);
void SovereignAetherDispatch(const char* mission_id);
void SovereignAetherRegisterMission(const SovereignMission* mission);

#endif // SOVEREIGN_AETHER_ORCHESTRATOR_H
