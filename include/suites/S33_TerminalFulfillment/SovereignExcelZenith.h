#ifndef SOVEREIGN_EXCEL_ZENITH_H
#define SOVEREIGN_EXCEL_ZENITH_H

#include "SigmaOOP.h"

/* Σ Territory Initiation */

// --- CELL DEPENDENCY RENDERER (DAG CORE) ---
CLASS_DECLARE(SovereignSpreadsheetDAG) { 
    SigmaObject_t core;
    
    // Core Excel USP: Real-time cell recalculation map
    VIRTUAL(void, SetCellFormula, struct SovereignSpreadsheetDAG* self, const char* cell, const char* formula);
    VIRTUAL(sigma_f64, EvaluateCell, struct SovereignSpreadsheetDAG* self, const char* cell);
    VIRTUAL(void, TriggerCascadeUpdate, struct SovereignSpreadsheetDAG* self);
};

// --- MACRO EXECUTION ENGINE (VBA PARITY) ---
CLASS_DECLARE(SovereignMacroVM) { 
    SigmaObject_t core;
    
    // Core Excel USP: Automated business logic execution
    VIRTUAL(void, LoadMacroBytecode, struct SovereignMacroVM* self, const sigma_u8* bytecode, sigma_size_t size);
    VIRTUAL(void, ExecuteMacroSovereign, struct SovereignMacroVM* self);
};

/* Σ Territory Termination */

#endif
