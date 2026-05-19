/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: VIRTIO SUBSYSTEM (SPLIT VIRTQUEUE)
 * =============================================================================
 * Inspired by: Linux kernel drivers/virtio/virtio_ring.c
 *              OASIS Virtual I/O Device (VIRTIO) Specification
 * =============================================================================
 * Provides ring buffer queues for high-performance IO paravirtualization.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

#define VIRTQ_MAX_Q_SIZE 256

/* Virtqueue descriptors: 16 bytes.
 * These can chain together via "next". */
typedef struct {
    sigma_u64 addr;    /* guest-physical address */
    sigma_u32 len;     /* length */
    sigma_u16 flags;   /* VRING_DESC_F_NEXT, VRING_DESC_F_WRITE */
    sigma_u16 next;    /* next field if flags & NEXT */
} __attribute__((packed)) vring_desc_t;

/* Available ring: guest writes, host reads */
typedef struct {
    sigma_u16 flags;
    sigma_u16 idx;
    sigma_u16 ring[VIRTQ_MAX_Q_SIZE];
} __attribute__((packed)) vring_avail_t;

/* Used ring: host writes, guest reads */
typedef struct {
    sigma_u32 id;      /* Index of start of used descriptor chain */
    sigma_u32 len;     /* Total length of the descriptor chain */
} __attribute__((packed)) vring_used_elem_t;

typedef struct {
    sigma_u16 flags;
    sigma_u16 idx;
    vring_used_elem_t ring[VIRTQ_MAX_Q_SIZE];
} __attribute__((packed)) vring_used_t;

typedef struct {
    sigma_u16      queue_size;
    
    vring_desc_t*  desc;
    vring_avail_t* avail;
    vring_used_t*  used;
    
    sigma_u16      last_used_idx;
    sigma_u16      num_free;
    sigma_u16      free_head;
} sigma_virtqueue_t;

/* Dummy allocations for simulation */
static sigma_u8 virtqueue_mem[32768];

void virtio_ring_init(sigma_virtqueue_t* vq, sigma_u16 size) {
    if (size > VIRTQ_MAX_Q_SIZE) size = VIRTQ_MAX_Q_SIZE;
    
    vq->queue_size = size;
    
    /* Layout: Desc -> Avail -> Padding -> Used */
    sigma_u8* base = virtqueue_mem;
    vq->desc  = (vring_desc_t*)base;
    base += size * sizeof(vring_desc_t);
    
    vq->avail = (vring_avail_t*)base;
    base += sizeof(vring_avail_t); // Simplified padding
    
    vq->used  = (vring_used_t*)base;
    
    vq->num_free      = size;
    vq->free_head     = 0;
    vq->last_used_idx = 0;
    
    /* Link free list */
    for (sigma_u16 i = 0; i < size - 1; i++) {
        vq->desc[i].next = i + 1;
    }
    
    sigma_printf("[virtio] Virtqueue initialized (size=%u)\n", size);
}

int virtqueue_add_buf(sigma_virtqueue_t* vq, void* buf, sigma_u32 len, sigma_bool is_write) {
    if (vq->num_free == 0) return -1;
    
    sigma_u16 head = vq->free_head;
    sigma_u16 next = vq->desc[head].next;
    
    vq->desc[head].addr  = (sigma_u64)buf; /* Physical translation needed in real kernel */
    vq->desc[head].len   = len;
    vq->desc[head].flags = is_write ? 2 : 0; /* 2 = VRING_DESC_F_WRITE */
    vq->desc[head].next  = 0;
    
    vq->free_head = next;
    vq->num_free--;
    
    /* Put in available ring */
    sigma_u16 avail_idx = vq->avail->idx % vq->queue_size;
    vq->avail->ring[avail_idx] = head;
    
    /* Memory barrier goes here */
    vq->avail->idx++;
    
    sigma_printf("[virtio] Added buffer (addr=0x%llx, len=%u) at head %u\n", 
                 (sigma_u64)buf, len, head);
    return head;
}

void virtqueue_kick(sigma_virtqueue_t* vq) {
    /* Write to PCI notify register (simulated) */
    sigma_printf("[virtio] Kick! (Avail idx: %u)\n", vq->avail->idx);
}

void* virtqueue_get_buf(sigma_virtqueue_t* vq, sigma_u32* len) {
    if (vq->last_used_idx == vq->used->idx) {
        return SIGMA_NULL; /* Nothing new used */
    }
    
    /* Memory barrier goes here */
    
    sigma_u16 used_idx = vq->last_used_idx % vq->queue_size;
    sigma_u32 head = vq->used->ring[used_idx].id;
    *len = vq->used->ring[used_idx].len;
    
    vq->last_used_idx++;
    
    /* Reclaim descriptor */
    vq->desc[head].next = vq->free_head;
    vq->free_head = (sigma_u16)head;
    vq->num_free++;
    
    sigma_printf("[virtio] Retrieved buffer from host (len=%u)\n", *len);
    return (void*)vq->desc[head].addr; /* Reverse physical translation needed */
}
