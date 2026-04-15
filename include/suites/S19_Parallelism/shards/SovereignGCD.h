/*
 * =========================================================================
 * S SIGMAOS: S11_VIRTUALIZATION — SovereignGCD_Engine.c
 * =========================================================================
 * Mission: Grand Central Dispatch (GCD) Parity. 
 * Capability: Asynchronous task queues, serial/concurrent dispatch.
 * =========================================================================
 */

#ifndef SOVEREIGN_GCD_H
#define SOVEREIGN_GCD_H

#include "suites/S01_Genesis/shards/sigma_types.h"

typedef void (*sigma_dispatch_fn)(void* context);

typedef enum {
    DISPATCH_QUEUE_SERIAL,
    DISPATCH_QUEUE_CONCURRENT
} sigma_dispatch_type_t;

typedef struct {
    sigma_dispatch_type_t type;
    char label[32];
    sigma_u32 thread_affinity;
} sigma_dispatch_queue_t;

/* GCD Master Interface */
sigma_dispatch_queue_t* sigma_dispatch_get_main_queue(void);
sigma_dispatch_queue_t* sigma_dispatch_queue_create(const char* label, sigma_dispatch_type_t type);
void sigma_dispatch_async(sigma_dispatch_queue_t* queue, sigma_dispatch_fn fn, void* ctx);
void sigma_dispatch_sync(sigma_dispatch_queue_t* queue, sigma_dispatch_fn fn, void* ctx);

#endif
