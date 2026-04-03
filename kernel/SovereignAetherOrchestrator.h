#ifndef SOVEREIGN_AETHER_ORCHESTRATOR_H
#define SOVEREIGN_AETHER_ORCHESTRATOR_H

#include "SovereignOmniShard.h"

/**
 * Σ Dispatch Mission
 * Routes a mission ID to the appropriate system shard.
 */
void SovereignAetherDispatch(const char* mission_id);

/**
 * Σ Register Mission
 */
void SovereignAetherRegisterMission(const SovereignMission* mission);

#endif // SOVEREIGN_AETHER_ORCHESTRATOR_H
