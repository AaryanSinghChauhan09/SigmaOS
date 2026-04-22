#ifndef SOVEREIGN_POWERBI_ZENITH_H
#define SOVEREIGN_POWERBI_ZENITH_H

#include "suites/S03_Orchestrator/shards/SigmaOOP.h"

/* S Territory Initiation */

// --- DAX EVALUATION ENGINE (DATA MODELING) ---
CLASS_DECLARE(SovereignDAXEngine) { 
    SigmaObject_t core;
    
    // Core PowerBI USP: In-memory tabular processing and DAX compilation
    VIRTUAL(void, IngestStarSchema, struct SovereignDAXEngine* self, const char* schemaName);
    VIRTUAL(sigma_f64, ExecuteDAXQuery, struct SovereignDAXEngine* self, const char* daxExpression);
};

// --- MULTI-STREAM INGESTION (POWER QUERY / M-LANG PARITY) ---
CLASS_DECLARE(SovereignPowerQueryMaster) { 
    SigmaObject_t core;
    
    // Core PowerBI USP: Advanced M-language data transformations
    VIRTUAL(void, ConnectDataSource, struct SovereignPowerQueryMaster* self, const char* connectionString);
    VIRTUAL(void, ApplyTransformationFilter, struct SovereignPowerQueryMaster* self, const char* filterLogic);
    VIRTUAL(void, FinalizeIngestion, struct SovereignPowerQueryMaster* self);
};

/* S Territory Termination */

#endif
