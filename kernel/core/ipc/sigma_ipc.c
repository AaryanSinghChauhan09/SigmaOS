/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: INTER-PROCESS COMMUNICATION (IPC) SUBSYSTEM
 * =============================================================================
 * Inspired by: Linux kernel ipc/ (msgqueue, shm, semaphore)
 *              POSIX IPC (IEEE Std 1003.1)
 *              GNU Hurd mach_msg() message-passing primitives
 * =============================================================================
 * Provides: Message Queues, Shared Memory Segments, Counting Semaphores
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

/* ---- Message Queue ---- */
#define SIGMA_MSG_MAX       64
#define SIGMA_MSG_PAYLOAD   256

typedef struct {
    sigma_u32 sender_pid;
    sigma_u32 receiver_pid;
    sigma_u32 msg_type;
    sigma_u32 payload_len;
    sigma_u8  payload[SIGMA_MSG_PAYLOAD];
} sigma_msg_t;

typedef struct {
    sigma_msg_t messages[SIGMA_MSG_MAX];
    sigma_u32   head;
    sigma_u32   tail;
    sigma_u32   count;
    sigma_u32   max_msgs;
} sigma_msgqueue_t;

void sigma_msgqueue_init(sigma_msgqueue_t* q) {
    sigma_memset(q, 0, sizeof(*q));
    q->max_msgs = SIGMA_MSG_MAX;
    sigma_printf("[ipc/msgqueue] Message queue initialized (capacity: %u)\n", q->max_msgs);
}

int sigma_msgqueue_send(sigma_msgqueue_t* q, sigma_u32 sender, sigma_u32 receiver,
                        sigma_u32 type, const void* data, sigma_u32 len) {
    if (q->count >= q->max_msgs) {
        sigma_printf("[ipc/msgqueue] ERR: Queue full (%u/%u)\n", q->count, q->max_msgs);
        return -1;
    }
    if (len > SIGMA_MSG_PAYLOAD) len = SIGMA_MSG_PAYLOAD;

    sigma_msg_t* msg = &q->messages[q->tail];
    msg->sender_pid   = sender;
    msg->receiver_pid = receiver;
    msg->msg_type     = type;
    msg->payload_len  = len;
    sigma_memcpy(msg->payload, data, len);

    q->tail = (q->tail + 1) % SIGMA_MSG_MAX;
    q->count++;
    sigma_printf("[ipc/msgqueue] Message enqueued: PID %u -> PID %u (type=%u, %u bytes)\n",
                 sender, receiver, type, len);
    return 0;
}

int sigma_msgqueue_recv(sigma_msgqueue_t* q, sigma_u32 receiver, sigma_msg_t* out) {
    if (q->count == 0) return -1;

    /* Linear scan for matching receiver (simplified; production uses wait queues) */
    for (sigma_u32 i = 0; i < q->count; i++) {
        sigma_u32 idx = (q->head + i) % SIGMA_MSG_MAX;
        if (q->messages[idx].receiver_pid == receiver) {
            sigma_memcpy(out, &q->messages[idx], sizeof(sigma_msg_t));
            /* Compact (shift remaining messages forward) */
            for (sigma_u32 j = idx; j != q->tail; j = (j + 1) % SIGMA_MSG_MAX) {
                sigma_u32 next = (j + 1) % SIGMA_MSG_MAX;
                sigma_memcpy(&q->messages[j], &q->messages[next], sizeof(sigma_msg_t));
            }
            q->tail = (q->tail == 0) ? SIGMA_MSG_MAX - 1 : q->tail - 1;
            q->count--;
            sigma_printf("[ipc/msgqueue] Message delivered to PID %u (type=%u)\n",
                         receiver, out->msg_type);
            return 0;
        }
    }
    return -1; /* No message for this receiver */
}

/* ---- Shared Memory ---- */
#define SIGMA_SHM_MAX_REGIONS  16
#define SIGMA_SHM_PAGE_SIZE    4096

typedef struct {
    sigma_u32 shm_id;
    sigma_u32 owner_pid;
    sigma_u32 size;
    sigma_u32 attach_count;
    sigma_u8  data[SIGMA_SHM_PAGE_SIZE];
    sigma_bool active;
} sigma_shm_region_t;

static sigma_shm_region_t shm_table[SIGMA_SHM_MAX_REGIONS];
static sigma_u32 shm_next_id = 1;

int sigma_shm_create(sigma_u32 owner_pid, sigma_u32 size) {
    if (size > SIGMA_SHM_PAGE_SIZE) size = SIGMA_SHM_PAGE_SIZE;
    for (sigma_u32 i = 0; i < SIGMA_SHM_MAX_REGIONS; i++) {
        if (!shm_table[i].active) {
            shm_table[i].shm_id       = shm_next_id++;
            shm_table[i].owner_pid    = owner_pid;
            shm_table[i].size         = size;
            shm_table[i].attach_count = 0;
            shm_table[i].active       = SIGMA_TRUE;
            sigma_memset(shm_table[i].data, 0, size);
            sigma_printf("[ipc/shm] Region %u created by PID %u (%u bytes)\n",
                         shm_table[i].shm_id, owner_pid, size);
            return (int)shm_table[i].shm_id;
        }
    }
    sigma_printf("[ipc/shm] ERR: No free shared memory slots\n");
    return -1;
}

sigma_u8* sigma_shm_attach(sigma_u32 shm_id) {
    for (sigma_u32 i = 0; i < SIGMA_SHM_MAX_REGIONS; i++) {
        if (shm_table[i].active && shm_table[i].shm_id == shm_id) {
            shm_table[i].attach_count++;
            sigma_printf("[ipc/shm] Region %u attached (count: %u)\n",
                         shm_id, shm_table[i].attach_count);
            return shm_table[i].data;
        }
    }
    return SIGMA_NULL;
}

void sigma_shm_detach(sigma_u32 shm_id) {
    for (sigma_u32 i = 0; i < SIGMA_SHM_MAX_REGIONS; i++) {
        if (shm_table[i].active && shm_table[i].shm_id == shm_id) {
            if (shm_table[i].attach_count > 0) shm_table[i].attach_count--;
            if (shm_table[i].attach_count == 0) {
                shm_table[i].active = SIGMA_FALSE;
                sigma_printf("[ipc/shm] Region %u released (zero attachments)\n", shm_id);
            }
        }
    }
}

/* ---- Counting Semaphore ---- */
typedef struct {
    sigma_u32 value;
    sigma_u32 max_value;
    sigma_u32 waiters;
} sigma_semaphore_t;

void sigma_sem_init(sigma_semaphore_t* sem, sigma_u32 initial, sigma_u32 max_val) {
    sem->value     = initial;
    sem->max_value = max_val;
    sem->waiters   = 0;
    sigma_printf("[ipc/sem] Semaphore initialized (value=%u, max=%u)\n", initial, max_val);
}

int sigma_sem_wait(sigma_semaphore_t* sem) {
    if (sem->value == 0) {
        sem->waiters++;
        sigma_printf("[ipc/sem] Thread blocked (waiters: %u)\n", sem->waiters);
        return -1; /* Would block — in real kernel, context-switch here */
    }
    sem->value--;
    sigma_printf("[ipc/sem] Acquired (remaining: %u)\n", sem->value);
    return 0;
}

int sigma_sem_post(sigma_semaphore_t* sem) {
    if (sem->value >= sem->max_value) return -1;
    sem->value++;
    if (sem->waiters > 0) {
        sem->waiters--;
        sigma_printf("[ipc/sem] Woke blocked thread (waiters: %u)\n", sem->waiters);
    }
    sigma_printf("[ipc/sem] Released (value: %u)\n", sem->value);
    return 0;
}
