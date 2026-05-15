// SigmaOS — Sigma-AIO: Asynchronous I/O Submission Ring
// Inspired by: Linux io_uring — but lock-free, no syscall overhead
// Module: sigma-sys-aio
// USP over io_uring: No kernel/user boundary crossing — direct hardware ring
// Zero-copy: completion events written directly to caller's ring buffer

#ifndef SIGMA_AIO_H
#define SIGMA_AIO_H

#include "../../include/sigma_ring_buffer.h"

#define SIGMA_AIO_RING_SIZE  128
#define SIGMA_AIO_OP_READ    0x01
#define SIGMA_AIO_OP_WRITE   0x02
#define SIGMA_AIO_OP_FSYNC   0x03
#define SIGMA_AIO_OP_POLL    0x04

typedef struct SigmaAIOReq {
    unsigned char  op;          // READ / WRITE / FSYNC / POLL
    unsigned int   fd;          // file descriptor token
    unsigned char* buf;         // user buffer pointer
    unsigned int   len;         // transfer length
    unsigned long  offset;      // file offset
    unsigned int   user_data;   // caller-defined correlation ID
} SigmaAIOReq;

typedef struct SigmaAIOCompletion {
    unsigned int   user_data;
    int            result;      // bytes transferred or error code
} SigmaAIOCompletion;

typedef struct SigmaAIOSQRing {
    SigmaAIOReq    reqs[SIGMA_AIO_RING_SIZE];
    volatile unsigned int head;
    volatile unsigned int tail;
} SigmaAIOSQRing;

typedef struct SigmaAIOCQRing {
    SigmaAIOCompletion comps[SIGMA_AIO_RING_SIZE];
    volatile unsigned int head;
    volatile unsigned int tail;
} SigmaAIOCQRing;

typedef struct SigmaAIOContext {
    SigmaAIOSQRing sq; // submission queue
    SigmaAIOCQRing cq; // completion queue
} SigmaAIOContext;

static inline void aio_init(SigmaAIOContext* ctx) {
    ctx->sq.head = ctx->sq.tail = 0;
    ctx->cq.head = ctx->cq.tail = 0;
}

// Submit an async I/O request (lock-free SPSC push)
static inline int aio_submit(SigmaAIOContext* ctx, SigmaAIOReq* req) {
    unsigned int next = (ctx->sq.tail + 1) % SIGMA_AIO_RING_SIZE;
    if (next == ctx->sq.head) return -1; // ring full
    ctx->sq.reqs[ctx->sq.tail] = *req;
    ctx->sq.tail = next;
    return 0;
}

// Process one pending request and write completion (simulated hardware path)
static inline int aio_process_one(SigmaAIOContext* ctx) {
    if (ctx->sq.head == ctx->sq.tail) return 0; // empty
    SigmaAIOReq* req = &ctx->sq.reqs[ctx->sq.head];
    ctx->sq.head = (ctx->sq.head + 1) % SIGMA_AIO_RING_SIZE;

    // Simulate hardware DMA completion
    SigmaAIOCompletion comp;
    comp.user_data = req->user_data;
    comp.result    = (int)req->len; // success: bytes transferred

    unsigned int next = (ctx->cq.tail + 1) % SIGMA_AIO_RING_SIZE;
    if (next != ctx->cq.head) {
        ctx->cq.comps[ctx->cq.tail] = comp;
        ctx->cq.tail = next;
    }
    return 1;
}

// Harvest one completion event
static inline int aio_harvest(SigmaAIOContext* ctx, SigmaAIOCompletion* out) {
    if (ctx->cq.head == ctx->cq.tail) return 0;
    *out = ctx->cq.comps[ctx->cq.head];
    ctx->cq.head = (ctx->cq.head + 1) % SIGMA_AIO_RING_SIZE;
    return 1;
}

#endif /* SIGMA_AIO_H */
