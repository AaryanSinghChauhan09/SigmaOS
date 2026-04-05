/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: IPC SUBSYSTEM (v1.0 - PURE C11)
 * =============================================================================
 * Mechanisms:
 *   1. Pipes         — anonymous unidirectional byte streams
 *   2. Message Queues — typed message passing (POSIX mq-style)
 *   3. Shared Memory  — mmap-backed shared page regions
 *   4. Futex         — fast user-space mutex (kernel arbitration only on contention)
 * Standard: C11, freestanding
 * =============================================================================
 */

#include "../libc/SovereignLibC.h"

/* =========================================================================
 * PIPES
 * ========================================================================= */
#define PIPE_BUF_SIZE  4096u
#define PIPE_MAX       64u

typedef struct SigmaPipe {
    u8     buf[PIPE_BUF_SIZE];
    u32    head;
    u32    tail;
    u32    count;
    bool_t valid;
    bool_t write_closed;
    bool_t read_closed;
} SigmaPipe;

static SigmaPipe g_pipes[PIPE_MAX];

static i32 pipe_alloc(void) {
    u32 i;
    for (i = 0; i < PIPE_MAX; i++) {
        if (!g_pipes[i].valid) {
            SigmaPipe* p = &g_pipes[i];
            p->head = p->tail = p->count = 0;
            p->valid = TRUE;
            p->write_closed = p->read_closed = FALSE;
            return (i32)i;
        }
    }
    return K_ERR_BUSY;
}

/* Returns two "file descriptors" [read_end, write_end] as pipe index × 2 ± bit */
i32 ipc_pipe_create(i32* read_fd, i32* write_fd) {
    i32 idx = pipe_alloc();
    if (idx < 0) return K_ERR_BUSY;
    *read_fd  = idx * 2;
    *write_fd = idx * 2 + 1;
    return K_OK;
}

static SigmaPipe* pipe_from_fd(i32 fd, bool_t* is_write) {
    if (fd < 0) return NULL;
    *is_write = (fd & 1) != 0;
    u32 idx = (u32)fd / 2;
    if (idx >= PIPE_MAX || !g_pipes[idx].valid) return NULL;
    return &g_pipes[idx];
}

i64 ipc_pipe_write(i32 fd, const void* buf, usize n) {
    bool_t is_write;
    SigmaPipe* p = pipe_from_fd(fd, &is_write);
    if (!p || !is_write || p->write_closed) return K_ERR_INVAL;

    const u8* src = (const u8*)buf;
    usize written = 0;
    while (written < n) {
        if (p->count >= PIPE_BUF_SIZE) break;   /* pipe full — block in real impl */
        p->buf[p->tail % PIPE_BUF_SIZE] = src[written++];
        p->tail++;
        p->count++;
    }
    return (i64)written;
}

i64 ipc_pipe_read(i32 fd, void* buf, usize n) {
    bool_t is_write;
    SigmaPipe* p = pipe_from_fd(fd, &is_write);
    if (!p || is_write || p->read_closed) return K_ERR_INVAL;
    if (p->count == 0 && p->write_closed) return 0;  /* EOF */

    u8* dst = (u8*)buf;
    usize rd = 0;
    while (rd < n && p->count > 0) {
        dst[rd++] = p->buf[p->head % PIPE_BUF_SIZE];
        p->head++;
        p->count--;
    }
    return (i64)rd;
}

i32 ipc_pipe_close(i32 fd) {
    bool_t is_write;
    SigmaPipe* p = pipe_from_fd(fd, &is_write);
    if (!p) return K_ERR_INVAL;
    if (is_write) p->write_closed = TRUE;
    else          p->read_closed  = TRUE;
    if (p->write_closed && p->read_closed) p->valid = FALSE;
    return K_OK;
}

/* =========================================================================
 * MESSAGE QUEUES
 * ========================================================================= */
#define MQ_MAX        32u
#define MQ_MSG_MAX    64u
#define MQ_MSG_SIZE   256u

typedef struct SigmaMsg {
    u32  mtype;
    u32  len;
    u8   data[MQ_MSG_SIZE];
} SigmaMsg;

typedef struct SigmaMQ {
    SigmaMsg msgs[MQ_MSG_MAX];
    u32      head;
    u32      count;
    bool_t   valid;
    char     name[32];
} SigmaMQ;

static SigmaMQ g_mqs[MQ_MAX];

i32 ipc_mq_open(const char* name) {
    /* Look for existing */
    u32 i;
    for (i = 0; i < MQ_MAX; i++) {
        if (!g_mqs[i].valid) continue;
        usize j = 0;
        while (g_mqs[i].name[j] && name[j] && g_mqs[i].name[j] == name[j]) j++;
        if (!g_mqs[i].name[j] && !name[j]) return (i32)i;
    }
    /* Create new */
    for (i = 0; i < MQ_MAX; i++) {
        if (!g_mqs[i].valid) {
            g_mqs[i].head  = 0;
            g_mqs[i].count = 0;
            g_mqs[i].valid = TRUE;
            usize j = 0;
            while (j < 31 && name[j]) { g_mqs[i].name[j] = name[j]; j++; }
            g_mqs[i].name[j] = '\0';
            return (i32)i;
        }
    }
    return K_ERR_BUSY;
}

i32 ipc_mq_send(i32 mqd, u32 mtype, const void* data, u32 len) {
    if (mqd < 0 || (u32)mqd >= MQ_MAX || !g_mqs[mqd].valid) return K_ERR_INVAL;
    SigmaMQ* mq = &g_mqs[mqd];
    if (mq->count >= MQ_MSG_MAX) return K_ERR_BUSY;

    u32 idx = (mq->head + mq->count) % MQ_MSG_MAX;
    SigmaMsg* m = &mq->msgs[idx];
    m->mtype = mtype;
    m->len   = (len > MQ_MSG_SIZE) ? MQ_MSG_SIZE : len;

    const u8* src = (const u8*)data;
    u32 ci;
    for (ci = 0; ci < m->len; ci++) m->data[ci] = src[ci];
    mq->count++;
    return K_OK;
}

i64 ipc_mq_recv(i32 mqd, u32* mtype_out, void* buf, u32 buflen) {
    if (mqd < 0 || (u32)mqd >= MQ_MAX || !g_mqs[mqd].valid) return K_ERR_INVAL;
    SigmaMQ* mq = &g_mqs[mqd];
    if (mq->count == 0) return 0;

    SigmaMsg* m = &mq->msgs[mq->head];
    if (mtype_out) *mtype_out = m->mtype;
    u32 n = (m->len < buflen) ? m->len : buflen;
    u8* dst = (u8*)buf;
    u32 ci;
    for (ci = 0; ci < n; ci++) dst[ci] = m->data[ci];
    mq->head = (mq->head + 1) % MQ_MSG_MAX;
    mq->count--;
    return (i64)n;
}

/* =========================================================================
 * SHARED MEMORY
 * ========================================================================= */
#define SHM_MAX     16u
#define SHM_MAX_SZ  (1024u * 1024u)   /* 1 MB per segment */

typedef struct SigmaSHM {
    paddr_t paddr;
    usize   size;
    u32     key;
    u32     refs;
    bool_t  valid;
} SigmaSHM;

static SigmaSHM g_shm[SHM_MAX];
extern paddr_t  pmm_alloc_page(void);

i32 ipc_shm_get(u32 key, usize size) {
    /* Find existing */
    u32 i;
    for (i = 0; i < SHM_MAX; i++) {
        if (g_shm[i].valid && g_shm[i].key == key) return (i32)i;
    }
    /* Create new */
    for (i = 0; i < SHM_MAX; i++) {
        if (!g_shm[i].valid) {
            if (size > SHM_MAX_SZ) size = SHM_MAX_SZ;
            u64 npages = (size + PAGE_SIZE - 1) / PAGE_SIZE;
            paddr_t pa = pmm_alloc_page();   /* alloc first page */
            u64 pi;
            for (pi = 1; pi < npages; pi++) pmm_alloc_page();  /* subsequent pages */
            g_shm[i].paddr = pa;
            g_shm[i].size  = size;
            g_shm[i].key   = key;
            g_shm[i].refs  = 0;
            g_shm[i].valid = TRUE;
            return (i32)i;
        }
    }
    return K_ERR_BUSY;
}

void* ipc_shm_attach(i32 shmid) {
    if (shmid < 0 || (u32)shmid >= SHM_MAX || !g_shm[shmid].valid) return NULL;
    g_shm[shmid].refs++;
    return (void*)(usize)g_shm[shmid].paddr;
}

i32 ipc_shm_detach(i32 shmid) {
    if (shmid < 0 || (u32)shmid >= SHM_MAX || !g_shm[shmid].valid) return K_ERR_INVAL;
    if (g_shm[shmid].refs > 0) g_shm[shmid].refs--;
    return K_OK;
}

/* =========================================================================
 * FUTEX (Fast User-Space Mutex — kernel side)
 * ========================================================================= */
#define FUTEX_MAX  128u

typedef struct SigmaFutex {
    volatile u32* uaddr;    /* user-space address of lock word */
    u64           waiter_tid;
    bool_t        valid;
} SigmaFutex;

static SigmaFutex g_futexes[FUTEX_MAX];

/* FUTEX_WAIT: block if *uaddr == val */
i32 ipc_futex_wait(volatile u32* uaddr, u32 val) {
    if (*uaddr != val) return K_ERR_BUSY;  /* value changed — spurious wake */
    u32 i;
    for (i = 0; i < FUTEX_MAX; i++) {
        if (!g_futexes[i].valid) {
            g_futexes[i].uaddr = uaddr;
            g_futexes[i].valid = TRUE;
            /* In a real kernel: block current task here */
            extern void sched_yield(void);
            sched_yield();
            return K_OK;
        }
    }
    return K_ERR_BUSY;
}

/* FUTEX_WAKE: wake up to n waiters */
i32 ipc_futex_wake(volatile u32* uaddr, u32 n) {
    u32 i, woken = 0;
    for (i = 0; i < FUTEX_MAX && woken < n; i++) {
        if (g_futexes[i].valid && g_futexes[i].uaddr == uaddr) {
            g_futexes[i].valid = FALSE;
            woken++;
        }
    }
    return (i32)woken;
}

/* =========================================================================
 * IPC Init
 * ========================================================================= */
void ipc_init(void) {
    u32 i;
    for (i = 0; i < PIPE_MAX; i++)  g_pipes[i].valid   = FALSE;
    for (i = 0; i < MQ_MAX; i++)    g_mqs[i].valid     = FALSE;
    for (i = 0; i < SHM_MAX; i++)   g_shm[i].valid     = FALSE;
    for (i = 0; i < FUTEX_MAX; i++) g_futexes[i].valid = FALSE;

    extern void kprintf(const char* fmt, ...);
    kprintf("[IPC]: Pipes(%u) | MQueues(%u) | SHM(%u) | Futexes(%u) online.\n",
            PIPE_MAX, MQ_MAX, SHM_MAX, FUTEX_MAX);
}
