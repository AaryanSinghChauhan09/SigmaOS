#ifndef SIGMA_RING_H
#define SIGMA_RING_H

#include "core/sigma_types.h"

/**
 * @file sigma_ring.h
 * @brief High-Performance Shared-Memory Ring Buffer (sigma_ring)
 * 
 * Inspired by io_uring, sigma_ring allows zero-copy, lock-free communication
 * between user-space shards and the Sovereign Kernel.
 */

#define SIGMA_RING_ENTRIES 4096
#define SIGMA_RING_ALIGN 64

typedef enum {
    SIGMA_OP_READ = 0,
    SIGMA_OP_WRITE = 1,
    SIGMA_OP_NOP = 2,
    SIGMA_OP_NETWORK_SEND = 3,
    SIGMA_OP_NETWORK_RECV = 4,
    SIGMA_OP_SHARD_INJECT = 5
} sigma_op_t;

/**
 * @brief Submission Queue Entry (SQE)
 */
typedef struct {
    sigma_op_t opcode;
    sigma_u8 flags;
    sigma_u16 ioprio;
    sigma_s32 fd;
    sigma_u64 addr;
    sigma_u64 len;
    sigma_u64 user_data;
} sigma_sqe_t;

/**
 * @brief Completion Queue Entry (CQE)
 */
typedef struct {
    sigma_u64 user_data;
    sigma_s32 res;
    sigma_u32 flags;
} sigma_cqe_t;

/**
 * @brief The sigma_ring structure, mapped in both user and kernel space.
 */
typedef struct {
    /* Submission Queue (SQ) */
    struct {
        sigma_u32 head;
        sigma_u32 tail;
        sigma_u32 mask;
        sigma_u32 flags;
        sigma_sqe_t entries[SIGMA_RING_ENTRIES];
    } sq;

    /* Completion Queue (CQ) */
    struct {
        sigma_u32 head;
        sigma_u32 tail;
        sigma_u32 mask;
        sigma_u32 flags;
        sigma_cqe_t entries[SIGMA_RING_ENTRIES];
    } cq;
} sigma_ring_t;

/* Memory Barrier Shorthands (Arch-specific implementation in sigma_hal.h) */
#define sigma_smp_rmb() __atomic_thread_fence(__ATOMIC_ACQUIRE)
#define sigma_smp_wmb() __atomic_thread_fence(__ATOMIC_RELEASE)

/**
 * @brief Submit an entry to the ring.
 * @return 0 on success, -1 if ring is full.
 */
static inline int sigma_ring_submit(sigma_ring_t *ring, sigma_sqe_t *sqe) {
    sigma_u32 next = (ring->sq.tail + 1) & ring->sq.mask;
    if (next == ring->sq.head) return -1; // Full

    ring->sq.entries[ring->sq.tail & ring->sq.mask] = *sqe;
    sigma_smp_wmb();
    ring->sq.tail = next;
    return 0;
}

/**
 * @brief Peek for a completion.
 * @return CQE pointer or NULL if empty.
 */
static inline sigma_cqe_t* sigma_ring_peek(sigma_ring_t *ring) {
    if (ring->cq.head == ring->cq.tail) return (sigma_cqe_t*)0;
    sigma_smp_rmb();
    return &ring->cq.entries[ring->cq.head & ring->cq.mask];
}

#endif // SIGMA_RING_H
