#include "../../../include/libc/SovereignLibC.h"
#include "../../../include/sigma_kernel_types.h"
/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: IPC SUBSYSTEM (v1.0 - PURE C11)
 * =============================================================================
 * Mechanisms:
 *   1. Pipes         â€ anonymous unidirectional byte streams
 *   2. Message Queues â€ typed message passing (POSIX mq-style)
 *   3. Shared Memory  â€ mmap-backed shared page regions
 *   4. Futex         â€ fast user-space mutex (kernel arbitration only on contention)
 * Standard: C11, freestanding
 * =============================================================================
 */

#include "../../../include/sigma_kernel_types.h"

/* =========================================================================
 * PIPES
 * ========================================================================= */
#define PIPE_BUF_SIZE  4096u
#define PIPE_MAX       64u

typedef struct SigmaPipe {
    sigma_u8     buf[PIPE_BUF_SIZE];
    sigma_u32    head;
    sigma_u32    tail;
    sigma_u32    count;
    sigma_bool valid;
    sigma_bool write_closed;
    sigma_bool read_closed;
} SigmaPipe;

static SigmaPipe g_pipes[PIPE_MAX];

static sigma_i32 pipe_alloc(void) {
    sigma_u32 i;
    for (i = 0; i < PIPE_MAX; i++) {
        if (!g_pipes[i].valid) {
            SigmaPipe* p = &g_pipes[i];
            p->head = p->tail = p->count = 0;
            p->valid = SIGMA_TRUE;
            p->write_closed = p->read_closed = SIGMA_FALSE;
            return (sigma_i32)i;
        }
    }
    return K_ERR_BUSY;
}

/* Returns two "file descriptors" [read_end, write_end] as pipe index Ã— 2 Â± bit */
sigma_i32 ipc_pipe_create(sigma_i32* read_fd, sigma_i32* write_fd) {
    sigma_i32 idx = pipe_alloc();
    if (idx < 0) return K_ERR_BUSY;
    *read_fd  = idx * 2;
    *write_fd = idx * 2 + 1;
    return K_OK;
}

static SigmaPipe* pipe_from_fd(sigma_i32 fd, sigma_bool* is_write) {
    if (fd < 0) return SIGMA_NULL;
    *is_write = (fd & 1) != 0;
    sigma_u32 idx = (sigma_u32)fd / 2;
    if (idx >= PIPE_MAX || !g_pipes[idx].valid) return SIGMA_NULL;
    return &g_pipes[idx];
}

sigma_i64 ipc_pipe_write(sigma_i32 fd, const void* buf, sigma_usize n) {
    sigma_bool is_write;
    SigmaPipe* p = pipe_from_fd(fd, &is_write);
    if (!p || !is_write || p->write_closed) return K_ERR_INVAL;

    const sigma_u8* src = (const sigma_u8*)buf;
    sigma_usize written = 0;
    while (written < n) {
        if (p->count >= PIPE_BUF_SIZE) break;   /* pipe full â€ block in real impl */
        p->buf[p->tail % PIPE_BUF_SIZE] = src[written++];
        p->tail++;
        p->count++;
    }
    return (sigma_i64)written;
}

sigma_i64 ipc_pipe_read(sigma_i32 fd, void* buf, sigma_usize n) {
    sigma_bool is_write;
    SigmaPipe* p = pipe_from_fd(fd, &is_write);
    if (!p || is_write || p->read_closed) return K_ERR_INVAL;
    if (p->count == 0 && p->write_closed) return 0;  /* EOF */

    sigma_u8* dst = (sigma_u8*)buf;
    sigma_usize rd = 0;
    while (rd < n && p->count > 0) {
        dst[rd++] = p->buf[p->head % PIPE_BUF_SIZE];
        p->head++;
        p->count--;
    }
    return (sigma_i64)rd;
}

sigma_i32 ipc_pipe_close(sigma_i32 fd) {
    sigma_bool is_write;
    SigmaPipe* p = pipe_from_fd(fd, &is_write);
    if (!p) return K_ERR_INVAL;
    if (is_write) p->write_closed = SIGMA_TRUE;
    else          p->read_closed  = SIGMA_TRUE;
    if (p->write_closed && p->read_closed) p->valid = SIGMA_FALSE;
    return K_OK;
}

/* =========================================================================
 * MESSAGE QUEUES
 * ========================================================================= */
#define MQ_MAX        32u
#define MQ_MSG_MAX    64u
#define MQ_MSG_SIZE   256u

typedef struct SigmaMsg {
    sigma_u32  mtype;
    sigma_u32  len;
    sigma_u8   data[MQ_MSG_SIZE];
} SigmaMsg;

typedef struct SigmaMQ {
    SigmaMsg msgs[MQ_MSG_MAX];
    sigma_u32      head;
    sigma_u32      count;
    sigma_bool   valid;
    char     name[32];
} SigmaMQ;

static SigmaMQ g_mqs[MQ_MAX];

sigma_i32 ipc_mq_open(const char* name) {
    /* Look for existing */
    sigma_u32 i;
    for (i = 0; i < MQ_MAX; i++) {
        if (!g_mqs[i].valid) continue;
        sigma_usize j = 0;
        while (g_mqs[i].name[j] && name[j] && g_mqs[i].name[j] == name[j]) j++;
        if (!g_mqs[i].name[j] && !name[j]) return (sigma_i32)i;
    }
    /* Create new */
    for (i = 0; i < MQ_MAX; i++) {
        if (!g_mqs[i].valid) {
            g_mqs[i].head  = 0;
            g_mqs[i].count = 0;
            g_mqs[i].valid = SIGMA_TRUE;
            sigma_usize j = 0;
            while (j < 31 && name[j]) { g_mqs[i].name[j] = name[j]; j++; }
            g_mqs[i].name[j] = '\0';
            return (sigma_i32)i;
        }
    }
    return K_ERR_BUSY;
}

sigma_i32 ipc_mq_send(sigma_i32 mqd, sigma_u32 mtype, const void* data, sigma_u32 len) {
    if (mqd < 0 || (sigma_u32)mqd >= MQ_MAX || !g_mqs[mqd].valid) return K_ERR_INVAL;
    SigmaMQ* mq = &g_mqs[mqd];
    if (mq->count >= MQ_MSG_MAX) return K_ERR_BUSY;

    sigma_u32 idx = (mq->head + mq->count) % MQ_MSG_MAX;
    SigmaMsg* m = &mq->msgs[idx];
    m->mtype = mtype;
    m->len   = (len > MQ_MSG_SIZE) ? MQ_MSG_SIZE : len;

    const sigma_u8* src = (const sigma_u8*)data;
    sigma_u32 ci;
    for (ci = 0; ci < m->len; ci++) m->data[ci] = src[ci];
    mq->count++;
    return K_OK;
}

sigma_i64 ipc_mq_recv(sigma_i32 mqd, sigma_u32* mtype_out, void* buf, sigma_u32 buflen) {
    if (mqd < 0 || (sigma_u32)mqd >= MQ_MAX || !g_mqs[mqd].valid) return K_ERR_INVAL;
    SigmaMQ* mq = &g_mqs[mqd];
    if (mq->count == 0) return 0;

    SigmaMsg* m = &mq->msgs[mq->head];
    if (mtype_out) *mtype_out = m->mtype;
    sigma_u32 n = (m->len < buflen) ? m->len : buflen;
    sigma_u8* dst = (sigma_u8*)buf;
    sigma_u32 ci;
    for (ci = 0; ci < n; ci++) dst[ci] = m->data[ci];
    mq->head = (mq->head + 1) % MQ_MSG_MAX;
    mq->count--;
    return (sigma_i64)n;
}

/* =========================================================================
 * SHARED MEMORY
 * ========================================================================= */
#define SHM_MAX     16u
#define SHM_MAX_SZ  (1024u * 1024u)   /* 1 MB per segment */

typedef struct SigmaSHM {
    sigma_paddr_t paddr;
    sigma_usize   size;
    sigma_u32     key;
    sigma_u32     refs;
    sigma_bool  valid;
} SigmaSHM;

static SigmaSHM g_shm[SHM_MAX];
extern sigma_paddr_t  pmm_alloc_page(void);

sigma_i32 ipc_shm_get(sigma_u32 key, sigma_usize size) {
    /* Find existing */
    sigma_u32 i;
    for (i = 0; i < SHM_MAX; i++) {
        if (g_shm[i].valid && g_shm[i].key == key) return (sigma_i32)i;
    }
    /* Create new */
    for (i = 0; i < SHM_MAX; i++) {
        if (!g_shm[i].valid) {
            if (size > SHM_MAX_SZ) size = SHM_MAX_SZ;
            sigma_u64 npages = (size + PAGE_SIZE - 1) / PAGE_SIZE;
            sigma_paddr_t pa = pmm_alloc_page();   /* alloc first page */
            sigma_u64 pi;
            for (pi = 1; pi < npages; pi++) pmm_alloc_page();  /* subsequent pages */
            g_shm[i].paddr = pa;
            g_shm[i].size  = size;
            g_shm[i].key   = key;
            g_shm[i].refs  = 0;
            g_shm[i].valid = SIGMA_TRUE;
            return (sigma_i32)i;
        }
    }
    return K_ERR_BUSY;
}

void* ipc_shm_attach(sigma_i32 shmid) {
    if (shmid < 0 || (sigma_u32)shmid >= SHM_MAX || !g_shm[shmid].valid) return SIGMA_NULL;
    g_shm[shmid].refs++;
    return (void*)(sigma_usize)g_shm[shmid].paddr;
}

sigma_i32 ipc_shm_detach(sigma_i32 shmid) {
    if (shmid < 0 || (sigma_u32)shmid >= SHM_MAX || !g_shm[shmid].valid) return K_ERR_INVAL;
    if (g_shm[shmid].refs > 0) g_shm[shmid].refs--;
    return K_OK;
}

/* =========================================================================
 * FUTEX (Fast User-Space Mutex â€ kernel side)
 * ========================================================================= */
#define FUTEX_MAX  128u

typedef struct SigmaFutex {
    volatile sigma_u32* uaddr;    /* user-space address of lock word */
    sigma_u64           waiter_tid;
    sigma_bool        valid;
} SigmaFutex;

static SigmaFutex g_futexes[FUTEX_MAX];

/* FUTEX_WAIT: block if *uaddr == val */
sigma_i32 ipc_futex_wait(volatile sigma_u32* uaddr, sigma_u32 val) {
    if (*uaddr != val) return K_ERR_BUSY;  /* value changed â€ spurious wake */
    sigma_u32 i;
    for (i = 0; i < FUTEX_MAX; i++) {
        if (!g_futexes[i].valid) {
            g_futexes[i].uaddr = uaddr;
            g_futexes[i].valid = SIGMA_TRUE;
            /* In a real kernel: block current task here */
            extern void sched_yield(void);
            sched_yield();
            return K_OK;
        }
    }
    return K_ERR_BUSY;
}

/* FUTEX_WAKE: wake up to n waiters */
sigma_i32 ipc_futex_wake(volatile sigma_u32* uaddr, sigma_u32 n) {
    sigma_u32 i, woken = 0;
    for (i = 0; i < FUTEX_MAX && woken < n; i++) {
        if (g_futexes[i].valid && g_futexes[i].uaddr == uaddr) {
            g_futexes[i].valid = SIGMA_FALSE;
            woken++;
        }
    }
    return (sigma_i32)woken;
}

/* =========================================================================
 * IPC Init
 * ========================================================================= */
void ipc_init(void) {
    sigma_u32 i;
    for (i = 0; i < PIPE_MAX; i++)  g_pipes[i].valid   = SIGMA_FALSE;
    for (i = 0; i < MQ_MAX; i++)    g_mqs[i].valid     = SIGMA_FALSE;
    for (i = 0; i < SHM_MAX; i++)   g_shm[i].valid     = SIGMA_FALSE;
    for (i = 0; i < FUTEX_MAX; i++) g_futexes[i].valid = SIGMA_FALSE;

    extern void ksigma_printf(const char* fmt, ...);
    ksigma_printf("[IPC]: Pipes(%u) | MQueues(%u) | SHM(%u) | Futexes(%u) online.\n",
            PIPE_MAX, MQ_MAX, SHM_MAX, FUTEX_MAX);
}
