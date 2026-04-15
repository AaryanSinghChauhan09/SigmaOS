#ifndef SOVEREIGN_DISK_ZENITH_H
#define SOVEREIGN_DISK_ZENITH_H

#include "SigmaOOP.h"

/* S Territory Initiation */

// --- DISK SCHEDULING ALGORITHMS (SSTF, SCAN, LOOK) ---
CLASS_DECLARE(SovereignDiskScheduler) { 
    SigmaObject_t core;
    VIRTUAL(void, SSTF_Schedule, struct SovereignDiskScheduler* self, int* requests, int count, int initialHead);
    VIRTUAL(void, SCAN_Schedule, struct SovereignDiskScheduler* self, int* requests, int count, int initialHead);
    VIRTUAL(void, SSTF_Program_Sim, struct SovereignDiskScheduler* self);
};

// --- FILE SYSTEM STRUCTURES (Contiguous, Linked, Indexed) ---
CLASS_DECLARE(SovereignFileSystemShard) { 
    SigmaObject_t core;
    VIRTUAL(void, ContiguousAllocation, struct SovereignFileSystemShard* self);
    VIRTUAL(void, LinkedAllocation, struct SovereignFileSystemShard* self);
    VIRTUAL(void, IndexedAllocation, struct SovereignFileSystemShard* self);
    VIRTUAL(void, FreeSpaceManagement, struct SovereignFileSystemShard* self, sigma_u64 freeBlocksBitVector);
};

// --- SPOOLING VS BUFFERING (I/O) ---
CLASS_DECLARE(SovereignIOExpert) { 
    SigmaObject_t core;
    VIRTUAL(void, SpoolingDaemon, struct SovereignIOExpert* self);
    VIRTUAL(void, BufferingLogic, struct SovereignIOExpert* self);
};

/* S Territory Termination */

#endif
