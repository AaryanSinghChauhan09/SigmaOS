#ifndef SOVEREIGN_CORE_UTILS_H
#define SOVEREIGN_CORE_UTILS_H

#include "suites/S03_Orchestrator/shards/SigmaOOP.h"

/* S Territory Initiation */

CLASS_DECLARE(SovereignListDir) { 
    SigmaObject_t core;
    VIRTUAL(void, Execute, struct SovereignListDir* self, const char* path);
};

CLASS_DECLARE(SovereignConcatenate) { 
    SigmaObject_t core;
    VIRTUAL(void, Execute, struct SovereignConcatenate* self, const char* file);
};

CLASS_DECLARE(SovereignGrepSearch) { 
    SigmaObject_t core;
    VIRTUAL(void, Execute, struct SovereignGrepSearch* self, const char* pattern, const char* file);
};

CLASS_DECLARE(SovereignProcessMonitor) { 
    SigmaObject_t core;
    VIRTUAL(void, Execute, struct SovereignProcessMonitor* self);
};

CLASS_DECLARE(SovereignPermissionMod) { 
    SigmaObject_t core;
    VIRTUAL(void, Execute, struct SovereignPermissionMod* self, const char* permissions, const char* file);
};

CLASS_DECLARE(AutoAetherOrchestrator) { 
    SigmaObject_t core;
    VIRTUAL(void, DispatchCron, struct AutoAetherOrchestrator* self);
};

CLASS_DECLARE(SovereignDataScienceForge) { 
    SigmaObject_t core;
    VIRTUAL(void, TrainModel, struct SovereignDataScienceForge* self, const char* dataSet);
    VIRTUAL(void, PlotGraph, struct SovereignDataScienceForge* self, const char* metrics);
};

/* S Territory Termination */

#endif
