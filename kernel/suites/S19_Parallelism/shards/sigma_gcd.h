/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN PARALLELISM (Suite S19)
 * =========================================================================
 * Shard: Sovereign GCD (Grand Central Dispatch parity)
 * Parity: macOS libdispatch, Linux WorkQueues, Windows ThreadPool
 * Design: Zero-dependency, Work-stealing, Lock-free Ring Buffers.
 * =========================================================================
 */

#ifndef SOVEREIGN_GCD_H
#define SOVEREIGN_GCD_H

#include "../../../include/SovereignCommon.h"

#define GCD_MAX_QUEUES    64
#define GCD_RING_SIZE    1024

typedef enum {
    GCD_PRIO_HIGH       = 0,
    GCD_PRIO_DEFAULT    = 1,
    GCD_PRIO_LOW        = 2,
    GCD_PRIO_BACKGROUND = 3
} gcd_priority_t;

typedef void (*gcd_block_t)(void* context);

typedef struct {
    gcd_block_t block;
    void*       context;
} gcd_task_t;

typedef struct {
    gcd_task_t ring[GCD_RING_SIZE];
    volatile sigma_u32 head;
    volatile sigma_u32 tail;
    sigma_u32          queue_id;
    gcd_priority_t     priority;
    char               name[32];
} gcd_queue_t;

/* Public API */
void        sigma_gcd_init(void);

/* Queue management */
gcd_queue_t* sigma_gcd_get_main_queue(void);
gcd_queue_t* sigma_gcd_get_global_queue(gcd_priority_t prio);
gcd_queue_t* sigma_gcd_queue_create(const char* name, gcd_priority_t prio);

/* Async/Sync execution */
void        sigma_gcd_async(gcd_queue_t* queue, gcd_block_t block, void* context);
void        sigma_gcd_sync(gcd_queue_t* queue, gcd_block_t block, void* context);

/* Batch execution (Dispatch Apply) */
void        sigma_gcd_apply(sigma_u32 iterations, gcd_queue_t* queue, void (*block)(sigma_u32 index));

/* Maintenance */
void        sigma_gcd_stats(void);

#endif /* SOVEREIGN_GCD_H */
