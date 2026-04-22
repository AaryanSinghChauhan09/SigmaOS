#ifndef SOVEREIGN_COORDINATION_ZENITH_H
#define SOVEREIGN_COORDINATION_ZENITH_H

#include "suites/S03_Orchestrator/shards/SigmaOOP.h"

/* S Territory Initiation */

// --- ATOMIC HARDWARE INSTRUCTIONS ---
static inline sigma_bool SigmaTestAndSet(volatile sigma_bool* target) {
    sigma_bool rv = *target;
    *target = SIGMA_TRUE;
    return rv;
}

static inline void SigmaSwap(volatile sigma_bool* a, volatile sigma_bool* b) {
    sigma_bool temp = *a;
    *a = *b;
    *b = temp;
}

// --- PROCESS COORDINATION & SYNCHRONIZATION ---
CLASS_DECLARE(SovereignPetersonSolution) { 
    SigmaObject_t core;
    volatile sigma_bool m_flag[2];
    volatile int m_turn;

    VIRTUAL(void, Entering, struct SovereignPetersonSolution* self, int i);
    VIRTUAL(void, Leaving, struct SovereignPetersonSolution* self, int i);
};

// --- MONITOR STRUCTURE (SILBERSCHATZ) ---
CLASS_DECLARE(SovereignMonitor) { 
    SigmaObject_t core;
    VIRTUAL(void, InitializationCode, struct SovereignMonitor* self);
    VIRTUAL(void, EnterMonitor, struct SovereignMonitor* self);
    VIRTUAL(void, LeaveMonitor, struct SovereignMonitor* self);
};

/* S Territory Termination */

#endif
