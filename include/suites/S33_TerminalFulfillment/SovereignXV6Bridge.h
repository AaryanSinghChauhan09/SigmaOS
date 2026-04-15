/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN XV6 BRIDGE (v12.0 - PURE C11)
 * =========================================================================
 * Mission: XV6-Style Process & IPC Parity for industrial reliability.
 * Design: C11 / Zero-Dependency / Struct-based OOP.
 * =========================================================================
 */

#ifndef SOVEREIGN_XV6_BRIDGE_H
#define SOVEREIGN_XV6_BRIDGE_H

#include "sigma_libc.h"
#include "SigmaOOP.h"

// -------------------------------------------------------------------------
// XV6 Style Process & IPC Shards
// -------------------------------------------------------------------------

CLASS_DECLARE(SovereignTrapHandler) {
    SigmaObject_t core;
    VIRTUAL(void, HandleTrap, struct SovereignTrapHandler* self, int trapno);
};

CLASS_DECLARE(SovereignPipeNode) {
    SigmaObject_t core;
    int fds[2];
    VIRTUAL(void, CreatePipe, struct SovereignPipeNode* self);
    VIRTUAL(void, RedirectStdout, struct SovereignPipeNode* self, int fd);
};

CLASS_DECLARE(SovereignSleepWakeup) {
    SigmaObject_t core;
    VIRTUAL(void, Sleep, struct SovereignSleepWakeup* self, void* chan);
    VIRTUAL(void, Wakeup, struct SovereignSleepWakeup* self, void* chan);
};

// -------------------------------------------------------------------------
// Advanced Network Subsystem Shards
// -------------------------------------------------------------------------

CLASS_DECLARE(SovereignSocketMesh) {
    SigmaObject_t core;
    VIRTUAL(void, EstablishTCP, struct SovereignSocketMesh* self, const char* host, int port);
    VIRTUAL(void, EpollWaitShard, struct SovereignSocketMesh* self);
};

#endif // SOVEREIGN_XV6_BRIDGE_H
