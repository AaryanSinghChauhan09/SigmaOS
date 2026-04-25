// SigmaOS — Sovereign Ring Buffer (Lock-Free SPSC)
// Module: sigma-sys-ipc
// Single responsibility: single-producer / single-consumer message queue
// Used by IPC, DMA notification rings, IRQ event queues

#ifndef SIGMA_RING_BUFFER_H
#define SIGMA_RING_BUFFER_H

#define SIGMA_RING_SIZE 256

typedef struct SigmaRingBuffer {
    unsigned char buf[SIGMA_RING_SIZE];
    volatile unsigned int head;
    volatile unsigned int tail;
} SigmaRingBuffer;

static inline void ring_init(SigmaRingBuffer* rb) {
    rb->head = 0;
    rb->tail = 0;
}

static inline int ring_push(SigmaRingBuffer* rb, unsigned char byte) {
    unsigned int next = (rb->head + 1) % SIGMA_RING_SIZE;
    if (next == rb->tail) return -1; /* full */
    rb->buf[rb->head] = byte;
    rb->head = next;
    return 0;
}

static inline int ring_pop(SigmaRingBuffer* rb, unsigned char* out) {
    if (rb->head == rb->tail) return -1; /* empty */
    *out = rb->buf[rb->tail];
    rb->tail = (rb->tail + 1) % SIGMA_RING_SIZE;
    return 0;
}

static inline int ring_empty(SigmaRingBuffer* rb) {
    return rb->head == rb->tail;
}

#endif /* SIGMA_RING_BUFFER_H */
