/*
 * =============================================================================
 * Σ SIGMAOS: SOVEREIGN IPC MANAGER (v1.0)
 * =============================================================================
 * Mission: Inter-process communication via message queues, shared memory
 *          segments, and signal delivery between processes/shards.
 * Standard: C11/C++17 — Zero external dependencies.
 * =============================================================================
 */

#ifndef SIGMA_IPC_MANAGER_H
#define SIGMA_IPC_MANAGER_H

#include "../sigma_kernel_types.h"

#define IPC_MAX_QUEUES       128
#define IPC_QUEUE_CAPACITY    64
#define IPC_MSG_PAYLOAD_MAX  256
#define IPC_MAX_SHM_SEGMENTS  64
#define IPC_SHM_NAME_LEN      32

/* Signal types (SigmaOS equivalents) */
typedef enum {
    SIGMA_SIG_TERM  = 0,   /* Graceful termination */
    SIGMA_SIG_KILL  = 1,   /* Immediate termination */
    SIGMA_SIG_STOP  = 2,   /* Suspend process */
    SIGMA_SIG_CONT  = 3,   /* Resume process */
    SIGMA_SIG_USR1  = 4,   /* User-defined 1 */
    SIGMA_SIG_USR2  = 5,   /* User-defined 2 */
    SIGMA_SIG_CHLD  = 6,   /* Child terminated */
    SIGMA_SIG_PIPE  = 7    /* Broken pipe */
} sigma_signal_t;

typedef struct {
    sigma_u32   sender_pid;
    sigma_u32   receiver_pid;
    sigma_u32   msg_type;
    sigma_u64   timestamp;
    sigma_u16   payload_len;
    sigma_u8    payload[IPC_MSG_PAYLOAD_MAX];
} sigma_msg_t;

typedef struct {
    sigma_u32   queue_id;
    sigma_u32   owner_pid;
    sigma_u32   head;
    sigma_u32   tail;
    sigma_u32   count;
    sigma_msg_t messages[IPC_QUEUE_CAPACITY];
} sigma_msg_queue_t;

typedef struct {
    sigma_u32      shm_id;
    char           name[IPC_SHM_NAME_LEN];
    sigma_paddr_t  phys_base;
    sigma_usize    size;
    sigma_u32      ref_count;
    sigma_u32      owner_pid;
    sigma_bool     writable;
} sigma_shm_t;

#ifdef __cplusplus
extern "C" {
#endif

void       ipc_init(void);
sigma_u32  ipc_create_queue(sigma_u32 owner_pid);
int        ipc_destroy_queue(sigma_u32 queue_id);
int        ipc_send(sigma_u32 queue_id, const sigma_msg_t* msg);
int        ipc_receive(sigma_u32 queue_id, sigma_msg_t* out_msg);
sigma_u32  ipc_queue_count(sigma_u32 queue_id);

sigma_u32  shm_create(const char* name, sigma_usize size, sigma_u32 owner_pid);
int        shm_attach(sigma_u32 shm_id, sigma_u32 pid);
int        shm_detach(sigma_u32 shm_id, sigma_u32 pid);
int        shm_destroy(sigma_u32 shm_id);

int        signal_send(sigma_u32 target_pid, sigma_signal_t sig);
void       ipc_print_status(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_IPC_MANAGER_H */
