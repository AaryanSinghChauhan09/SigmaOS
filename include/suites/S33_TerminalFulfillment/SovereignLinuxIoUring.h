/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN LINUX IO_URING (v1.0 — C11)
 * =========================================================================
 * Absorbed USPs from: torvalds/linux (io_uring)
 *   https://github.com/torvalds/linux/tree/master/io_uring
 *
 * Features implemented:
 *   ✓ Submission Queue (SQ) Rings
 *   ✓ Completion Queue (CQ) Rings
 *   ✓ Asynchronous I/O operations (NOP, READ, WRITE)
 *   ✓ Zero-syscall event submission via kernel polling mode
 * =========================================================================
 */

#ifndef SOVEREIGN_LINUX_IO_URING_H
#define SOVEREIGN_LINUX_IO_URING_H

#include "suites/S01_Genesis/shards/sigma_types.h"

typedef struct {
    sigma_u8 opcode;
    sigma_u8 flags;
    sigma_u16 ioprio;
    sigma_i32 fd;
    sigma_u64 off;
    sigma_u64 addr;
    sigma_u32 len;
    sigma_u64 user_data;
} SigmaSQE_t;

typedef struct {
    sigma_u64 user_data;
    sigma_i32 res;
    sigma_u32 flags;
} SigmaCQE_t;

typedef struct {
    SigmaSQE_t sqes[64];
    sigma_u32 sq_head;
    sigma_u32 sq_tail;
    
    SigmaCQE_t cqes[64];
    sigma_u32 cq_head;
    sigma_u32 cq_tail;
} SigmaIoURing_t;

sigma_err_t sigma_io_uring_setup(sigma_u32 entries, SigmaIoURing_t *ring);
SigmaSQE_t* sigma_io_uring_get_sqe(SigmaIoURing_t *ring);
sigma_err_t sigma_io_uring_submit(SigmaIoURing_t *ring);
sigma_err_t sigma_io_uring_wait_cqe(SigmaIoURing_t *ring, SigmaCQE_t **cqe_ptr);

void SovereignLinuxIoUring_Init(void);

#endif /* SOVEREIGN_LINUX_IO_URING_H */
