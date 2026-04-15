#ifndef SOVEREIGN_R_ZENITH_H
#define SOVEREIGN_R_ZENITH_H

#include "SigmaOOP.h"

/* S Territory Initiation */

// --- VECTORIZED COMPUTATION (R BASE PARITY) ---
CLASS_DECLARE(SovereignRVectorMath) { 
    SigmaObject_t core;
    
    // Core R USP: Element-wise mathematical scaling without iterative overhead
    VIRTUAL(void, ExecuteSIMDVectorOp, struct SovereignRVectorMath* self, const sigma_f64* vecA, const sigma_f64* vecB, sigma_f64* result, sigma_sz_t len);
    VIRTUAL(sigma_f64, ExecuteStatisticalInference, struct SovereignRVectorMath* self, const sigma_f64* dataSet, sigma_sz_t len);
};

// --- DATAFRAME MEMORY MAP (R DATAFRAME PARITY) ---
CLASS_DECLARE(SovereignDataframeMatrix) { 
    SigmaObject_t core;
    
    // Core R USP: Native heterogenous data tables built directly into the grammar
    VIRTUAL(void, InitializeTabularFrame, struct SovereignDataframeMatrix* self, const char* colNames[], sigma_sz_t numCols);
    VIRTUAL(void, ColumnarBind, struct SovereignDataframeMatrix* self, const sigma_f64* dataColumn);
};

// --- GRAMMAR OF GRAPHICS (GGPLOT2 PARITY) ---
CLASS_DECLARE(SovereignGrammarOfGraphics) { 
    SigmaObject_t core;
    
    // Core R USP: Layered coordinate geometry definition
    VIRTUAL(void, EstablishAesthetics, struct SovereignGrammarOfGraphics* self, const char* aesX, const char* aesY);
    VIRTUAL(void, ApplyGeometricTopology, struct SovereignGrammarOfGraphics* self, const char* geomType);
};

/* S Territory Termination */

#endif
