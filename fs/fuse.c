/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: FUSE (Filesystem in Userspace)
 * =============================================================================
 * Inspired by: Linux kernel fs/fuse/dev.c & fs/fuse/dir.c
 *              FreeBSD sys/fs/fuse/fuse_ipc.c
 * =============================================================================
 * Bridges VFS calls to a user-space daemon via /dev/fuse.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

#define FUSE_OP_LOOKUP  1
#define FUSE_OP_READ    15
#define FUSE_OP_WRITE   16
#define FUSE_OP_OPEN    14

#define FUSE_MAX_REQS   32

typedef struct {
    sigma_u32 len;
    sigma_u32 opcode;
    sigma_u64 unique;
    sigma_u64 nodeid;
    sigma_u32 uid;
    sigma_u32 gid;
    sigma_u32 pid;
} __attribute__((packed)) fuse_in_header_t;

typedef struct {
    sigma_u32 len;
    sigma_s32 error;
    sigma_u64 unique;
} __attribute__((packed)) fuse_out_header_t;

typedef struct {
    fuse_in_header_t in_hdr;
    void*  in_args;
    fuse_out_header_t out_hdr;
    void*  out_args;
    sigma_bool active;
    sigma_bool completed;
} sigma_fuse_req_t;

static sigma_fuse_req_t fuse_queue[FUSE_MAX_REQS];
static sigma_u64 next_unique_id = 1;

void fuse_init(void) {
    sigma_memset(fuse_queue, 0, sizeof(fuse_queue));
    sigma_printf("[fuse] Filesystem in Userspace (FUSE) core initialized\n");
}

/* Kernel VFS calls this to send a request to the user-space daemon */
int fuse_send_request(sigma_u32 opcode, sigma_u64 nodeid, void* in_args, sigma_u32 in_len) {
    for (sigma_u32 i = 0; i < FUSE_MAX_REQS; i++) {
        if (!fuse_queue[i].active) {
            sigma_fuse_req_t* req = &fuse_queue[i];
            
            req->in_hdr.len = sizeof(fuse_in_header_t) + in_len;
            req->in_hdr.opcode = opcode;
            req->in_hdr.unique = next_unique_id++;
            req->in_hdr.nodeid = nodeid;
            req->in_hdr.pid = 0; /* Should be current task PID */
            
            req->in_args = in_args;
            req->active = SIGMA_TRUE;
            req->completed = SIGMA_FALSE;
            
            sigma_printf("[fuse] Dispatched OP %u to user-space (Unique: %llu, Node: %llu)\n", 
                         opcode, req->in_hdr.unique, nodeid);
                         
            /* In a real kernel, we would put the current thread to sleep here
               until the user-space daemon writes the response to /dev/fuse. */
            return (int)i;
        }
    }
    sigma_printf("[fuse] ERR: FUSE request queue full\n");
    return -1;
}

/* User-space daemon writes to /dev/fuse, triggering this */
void fuse_process_response(const fuse_out_header_t* out_hdr, void* out_args) {
    for (sigma_u32 i = 0; i < FUSE_MAX_REQS; i++) {
        if (fuse_queue[i].active && fuse_queue[i].in_hdr.unique == out_hdr->unique) {
            sigma_fuse_req_t* req = &fuse_queue[i];
            
            req->out_hdr = *out_hdr;
            req->out_args = out_args;
            req->completed = SIGMA_TRUE;
            
            sigma_printf("[fuse] Received response from user-space (Unique: %llu, Error: %d)\n", 
                         out_hdr->unique, out_hdr->error);
            
            /* In a real kernel, we would wake up the sleeping thread here */
            req->active = SIGMA_FALSE;
            return;
        }
    }
    sigma_printf("[fuse] ERR: Received response for unknown Unique ID %llu\n", out_hdr->unique);
}
