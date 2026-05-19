/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: BLOCK I/O SCHEDULER & DEVICE LAYER
 * =============================================================================
 * Inspired by: Linux kernel block/blk-core.c
 *              FreeBSD sys/kern/subr_disk.c
 * =============================================================================
 * Manages block storage devices, sector addressing, and an I/O request queue.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

#define MAX_BLOCK_DEVICES 16
#define MAX_IO_REQUESTS   64

#define REQ_OP_READ  0
#define REQ_OP_WRITE 1
#define REQ_OP_FLUSH 2

typedef struct {
    sigma_u32  dev_id;
    sigma_u32  operation;
    sigma_u64  sector;
    sigma_u32  count;
    void*      buffer;
    sigma_bool active;
    sigma_bool completed;
    int        status;
} sigma_bio_request_t;

typedef int (*blk_submit_fn)(sigma_bio_request_t* req);

typedef struct {
    char          name[32];
    sigma_u64     capacity_sectors;
    sigma_u32     sector_size;
    blk_submit_fn submit_bio;
    sigma_bool    active;
} sigma_block_device_t;

static sigma_block_device_t bdev_table[MAX_BLOCK_DEVICES];
static sigma_bio_request_t  io_queue[MAX_IO_REQUESTS];
static sigma_u32            io_head = 0;
static sigma_u32            io_tail = 0;

void block_io_init(void) {
    sigma_memset(bdev_table, 0, sizeof(bdev_table));
    sigma_memset(io_queue, 0, sizeof(io_queue));
    sigma_printf("[block] Block I/O layer initialized\n");
}

int block_device_register(const char* name, sigma_u64 sectors, sigma_u32 sec_size, blk_submit_fn submit) {
    for (sigma_u32 i = 0; i < MAX_BLOCK_DEVICES; i++) {
        if (!bdev_table[i].active) {
            sigma_u32 j = 0;
            while (j < 31 && name[j]) { bdev_table[i].name[j] = name[j]; j++; }
            bdev_table[i].name[j] = '\0';
            
            bdev_table[i].capacity_sectors = sectors;
            bdev_table[i].sector_size      = sec_size;
            bdev_table[i].submit_bio       = submit;
            bdev_table[i].active           = SIGMA_TRUE;
            
            sigma_printf("[block] Registered device '%s' (%llu MB, %u byte sectors)\n", 
                         bdev_table[i].name, 
                         (sectors * sec_size) / (1024 * 1024), 
                         sec_size);
            return (int)i;
        }
    }
    sigma_printf("[block] ERR: Max block devices reached\n");
    return -1;
}

int block_submit_io(sigma_u32 dev_id, sigma_u32 op, sigma_u64 sector, sigma_u32 count, void* buffer) {
    if (dev_id >= MAX_BLOCK_DEVICES || !bdev_table[dev_id].active) return -1;
    
    if (sector + count > bdev_table[dev_id].capacity_sectors) {
        sigma_printf("[block] ERR: I/O out of bounds on '%s' (sector %llu, count %u)\n", 
                     bdev_table[dev_id].name, sector, count);
        return -1;
    }
    
    /* Enqueue request (simplified NOOP scheduler logic) */
    sigma_u32 next_tail = (io_tail + 1) % MAX_IO_REQUESTS;
    if (next_tail == io_head) {
        sigma_printf("[block] ERR: I/O queue full\n");
        return -1;
    }
    
    sigma_bio_request_t* req = &io_queue[io_tail];
    req->dev_id    = dev_id;
    req->operation = op;
    req->sector    = sector;
    req->count     = count;
    req->buffer    = buffer;
    req->active    = SIGMA_TRUE;
    req->completed = SIGMA_FALSE;
    req->status    = 0;
    
    io_tail = next_tail;
    
    sigma_printf("[block] Enqueued %s req for '%s': sector %llu, count %u\n", 
                 op == REQ_OP_READ ? "READ" : "WRITE",
                 bdev_table[dev_id].name, sector, count);
                 
    /* Dispatch immediately if driver supports synchronous submit */
    if (bdev_table[dev_id].submit_bio) {
        req->status = bdev_table[dev_id].submit_bio(req);
        req->completed = SIGMA_TRUE;
        req->active = SIGMA_FALSE;
        
        /* Pop head if completed */
        if (io_head == io_tail - 1 || (io_tail == 0 && io_head == MAX_IO_REQUESTS - 1)) {
            io_head = (io_head + 1) % MAX_IO_REQUESTS;
        }
    }
    
    return req->status;
}
