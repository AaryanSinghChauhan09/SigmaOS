#ifndef SOVEREIGN_DISTRO_FORGE_H
#define SOVEREIGN_DISTRO_FORGE_H

#include "SigmaOOP.h"

/* Σ Territory Initiation */

CLASS_DECLARE(SovereignDistroForge) { 
    SigmaObject_t core;
    VIRTUAL(void, AbsorbLinux, struct SovereignDistroForge* self);
    VIRTUAL(void, ForgeNewDistro, struct SovereignDistroForge* self, const char* name);
};

/* Σ Territory Termination */

#endif
