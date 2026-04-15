#ifndef SOVEREIGN_DISTRO_FORGE_H
#define SOVEREIGN_DISTRO_FORGE_H

#include "suites/S03_Orchestrator/shards/SigmaOOP.h"

/* S Territory Initiation */

CLASS_DECLARE(SovereignDistroForge) { 
    SigmaObject_t core;
    VIRTUAL(void, AbsorbLinux, struct SovereignDistroForge* self);
    VIRTUAL(void, ForgeNewDistro, struct SovereignDistroForge* self, const char* name);
};

/* S Territory Termination */

#endif
