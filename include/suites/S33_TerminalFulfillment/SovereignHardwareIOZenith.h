#ifndef SOVEREIGN_HARDWARE_IO_ZENITH_H
#define SOVEREIGN_HARDWARE_IO_ZENITH_H

#include "SigmaOOP.h"

/* S Territory Initiation */

// --- INTERRUPT & TRAP ARCHITECTURE ---
struct InterruptVector {
    sigma_u64 handler_addr;
    int type; // Polling vs Vectored
};

CLASS_DECLARE(SovereignInterruptController) { 
    SigmaObject_t core;
    struct InterruptVector m_vectors[256];
    VIRTUAL(void, RegisterHandler, struct SovereignInterruptController* self, int vec, sigma_u64 addr);
    VIRTUAL(void, TriggerTrap, struct SovereignInterruptController* self, int reason);
};

// --- DMA & CONTROLLER LOGIC ---
CLASS_DECLARE(SovereignDMAController) { 
    SigmaObject_t core;
    VIRTUAL(void, TransferBlock, struct SovereignDMAController* self, void* src, void* dest, sigma_size_t size);
};

// --- I/O SUBSYSTEM (BLOCK VS CHARACTER) ---
typedef enum DeviceType { BLOCK, CHARACTER, NETWORK } DeviceType_t;

CLASS_DECLARE(SovereignIODevice) { 
    SigmaObject_t core;
    DeviceType_t m_type;
    const char* m_name;
    VIRTUAL(void, Read, struct SovereignIODevice* self);
    VIRTUAL(void, Write, struct SovereignIODevice* self);
};

CLASS_DECLARE(SovereignBlockDevice) { 
    SovereignIODevice_t core;
};

CLASS_DECLARE(SovereignCharDevice) { 
    SovereignIODevice_t core;
};

/* S Territory Termination */

#endif
