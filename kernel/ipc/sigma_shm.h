// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_shm.h — Shared Memory & Named Pipes (IPC layer)
 *
 * Inspired by POSIX shm_open, Plan 9 pipes, and Fuchsia VMO objects.
 *
 * Three primitives:
 *   1. sigma_shm  — named, capability-gated shared memory regions
 *   2. sigma_pipe — bidirectional named pipe with backpressure
 *   3. sigma_mq   — POSIX-like message queue with priority
 *
 * All objects are reference-counted and tied to capability tokens
 * (sigma_cap.cpp).  A process needs SIGMA_CAP_IPC_SHM to create
 * regions larger than 64 KiB.
 *
 * Regions are mounted under /sigma/shm/<name> and can be mmap'd
 * directly.  They survive process death until the last reference drops.
 */

#include <sigma_kernel_types.h>

/* ── Flags ───────────────────────────────────────────────────────────────── */
#define SIGMA_SHM_CREATE   (1 << 0)   /* create if not exists                */
#define SIGMA_SHM_EXCL     (1 << 1)   /* fail if already exists              */
#define SIGMA_SHM_RDONLY   (1 << 2)   /* map read-only                       */
#define SIGMA_SHM_HUGETLB  (1 << 3)   /* back with 2 MiB huge pages          */
#define SIGMA_SHM_SEALED   (1 << 4)   /* F_SEAL_WRITE — immutable after seal */

/* ── Shared memory ───────────────────────────────────────────────────────── */
typedef struct sigma_shm sigma_shm_t;

sigma_shm_t* sigma_shm_open(const char* name, int flags, sigma_size_t size);
void*        sigma_shm_map(sigma_shm_t* shm, sigma_size_t offset,
                            sigma_size_t len, int flags);
int          sigma_shm_seal(sigma_shm_t* shm);
int          sigma_shm_resize(sigma_shm_t* shm, sigma_size_t new_size);
void         sigma_shm_close(sigma_shm_t* shm);
int          sigma_shm_unlink(const char* name);

/* ── Named pipe ──────────────────────────────────────────────────────────── */
typedef struct sigma_pipe sigma_pipe_t;

#define SIGMA_PIPE_DEFAULT_BUF (64 * 1024)   /* 64 KiB ring buffer          */

sigma_pipe_t* sigma_pipe_open(const char* name, int flags);
int           sigma_pipe_write(sigma_pipe_t* p, const void* buf,
                                sigma_size_t len);
int           sigma_pipe_read(sigma_pipe_t*  p, void* buf,
                               sigma_size_t   len, sigma_u32 timeout_ms);
void          sigma_pipe_close(sigma_pipe_t* p);

/* ── Message queue ───────────────────────────────────────────────────────── */
typedef struct sigma_mq sigma_mq_t;

typedef struct {
    sigma_u32 priority;       /* higher = delivered first                    */
    sigma_u32 len;
    sigma_u8  data[4096];
} sigma_mq_msg_t;

sigma_mq_t* sigma_mq_open(const char* name, int flags, sigma_u32 max_msgs);
int         sigma_mq_send(sigma_mq_t* mq, const sigma_mq_msg_t* msg);
int         sigma_mq_recv(sigma_mq_t* mq, sigma_mq_msg_t* out,
                           sigma_u32 timeout_ms);
int         sigma_mq_unlink(const char* name);
void        sigma_mq_close(sigma_mq_t* mq);
