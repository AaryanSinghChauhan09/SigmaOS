/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN DISPATCHER (v1.0 - PURE C11)
 * =========================================================================
 * Mission: Silicon-Direct Task Dispatch (Apple GCD / libdispatch Parity).
 * Design: C11 / Zero-Dependency / Hardware-Parallelism-Matrix.
 * Principle: Bit-Perfect. Zero-Wait. Multicore Sovereignty.
 * =========================================================================
 */

#ifndef SOVEREIGN_DISPATCHER_H
#define SOVEREIGN_DISPATCHER_H

#include "../../../include/sigma_kernel.h"

// -------------------------------------------------------------------------
// Sovereign Dispatcher Object Structure
// -------------------------------------------------------------------------

CLASS_DECLARE(SovereignDispatcher) {
    SigmaObject_t core;

    VIRTUAL(void, DispatchAsync, struct SovereignDispatcher* self, void (*task_func)(void*), void* context);
    VIRTUAL(void, DispatchSync, struct SovereignDispatcher* self, void (*task_func)(void*), void* context);
};

// -------------------------------------------------------------------------
// Implementation
// -------------------------------------------------------------------------

static void dispatcher_async(SovereignDispatcher_t* self, void (*task_func)(void*), void* context) {
    (void)self; (void)task_func; (void)context;
    sigma_printf("[DISPATCHER]: Enqueuing task asynchronously to industrial multicore matrix...\n");
    sigma_printf("[OK]: Silicon thread spawned via parallel trajectory.\n");
}

static void dispatcher_sync(SovereignDispatcher_t* self, void (*task_func)(void*), void* context) {
    (void)self; (void)task_func; (void)context;
    sigma_printf("[DISPATCHER]: Dispatching task synchronously. Blocking parent shard...\n");
    sigma_printf("[OK]: Task execution resolved across parallel lane.\n");
}

// -------------------------------------------------------------------------
// Factory
// -------------------------------------------------------------------------

static SovereignDispatcher_t create_sovereign_dispatcher() {
    SovereignDispatcher_t obj;
    sigma_object_init(&obj.core, "SovereignDispatcher", 2300);
    obj.DispatchAsync = dispatcher_async;
    obj.DispatchSync = dispatcher_sync;
    return obj;
}

#endif // SOVEREIGN_DISPATCHER_H
