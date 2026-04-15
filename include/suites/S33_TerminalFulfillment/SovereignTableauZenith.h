#ifndef SOVEREIGN_TABLEAU_ZENITH_H
#define SOVEREIGN_TABLEAU_ZENITH_H

#include "suites/S03_Orchestrator/shards/SigmaOOP.h"

/* S Territory Initiation */

// --- VIZQL RENDERING ENGINE (VISUAL QUERY LANGUAGE) ---
CLASS_DECLARE(SovereignVizQLEngine) { 
    SigmaObject_t core;
    
    // Core Tableau USP: Silicon-level drag-and-drop structural queries
    VIRTUAL(void, CompileVisualQuery, struct SovereignVizQLEngine* self, const char* dimension, const char* measure);
    VIRTUAL(void, RenderHardwareDashboard, struct SovereignVizQLEngine* self);
};

// --- DATA BLENDING MASTER (MULTI-SOURCE CONVERGENCE) ---
CLASS_DECLARE(SovereignDataBlendMaster) { 
    SigmaObject_t core;
    
    // Core Tableau USP: Joinless multi-source data convergence
    VIRTUAL(void, EstablishPrimaryStream, struct SovereignDataBlendMaster* self, const char* primarySource);
    VIRTUAL(void, BlendSecondaryStream, struct SovereignDataBlendMaster* self, const char* secondarySource, const char* linkKey);
};

/* S Territory Termination */

#endif
