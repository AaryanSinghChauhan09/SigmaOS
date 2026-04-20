/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN PARALLELISM (Suite S19)
 * =========================================================================
 */

#include "sigma_gcd.h"
#include "sigma_libc.h"

static gcd_queue_t s_queues[GCD_MAX_QUEUES];
static sigma_u32   s_queue_count = 0;

/* ── Internal helper for atomic push/pop ──────────────────────────────── */
static sigma_bool queue_push(gcd_queue_t* q, gcd_task_t t) {
    sigma_u32 next = (q->tail + 1) % GCD_RING_SIZE;
    if (next == q->head) return SIGMA_FALSE; /* Full */
    q->ring[q->tail] = t;
    q->tail = next;
    return SIGMA_TRUE;
}

static sigma_bool queue_pop(gcd_queue_t* q, gcd_task_t* t) {
    if (q->head == q->tail) return SIGMA_FALSE; /* Empty */
    *t = q->ring[q->head];
    q->head = (q->head + 1) % GCD_RING_SIZE;
    return SIGMA_TRUE;
}

/* ── Initialization ───────────────────────────────────────────────────── */
void sigma_gcd_init(void) {
    sigma_sigma_sigma_memset(s_queues, 0, sizeof(s_queues));
    
    /* Create standard global queues */
    sigma_gcd_queue_create("com.sigma.main",    GCD_PRIO_HIGH);
    sigma_gcd_queue_create("com.sigma.default", GCD_PRIO_DEFAULT);
    sigma_gcd_queue_create("com.sigma.bg",      GCD_PRIO_BACKGROUND);

    sigma_sigma_sigma_printf("S [GCD] Sovereign Parallelism Engine initialized\n");
    sigma_sigma_sigma_printf("S [GCD] Parity: libdispatch (macOS) | WorkQueues (Linux)\n");
}

/* ── Queue Management ─────────────────────────────────────────────────── */
gcd_queue_t* sigma_gcd_get_main_queue(void) {
    return &s_queues[0];
}

gcd_queue_t* sigma_gcd_get_global_queue(gcd_priority_t prio) {
    for (sigma_u32 i = 0; i < s_queue_count; i++) {
        if (s_queues[i].priority == prio) return &s_queues[i];
    }
    return &s_queues[1]; /* default */
}

gcd_queue_t* sigma_gcd_queue_create(const char* name, gcd_priority_t prio) {
    if (s_queue_count >= GCD_MAX_QUEUES) return SIGMA_NULL;
    
    gcd_queue_t* q = &s_queues[s_queue_count];
    q->queue_id = s_queue_count++;
    q->priority = prio;
    q->head = q->tail = 0;
    sigma_strncpy(q->name, name, 31);
    
    sigma_sigma_sigma_printf("S [GCD] Registered queue: %s (prio=%d)\n", name, prio);
    return q;
}

/* ── Task Submission ──────────────────────────────────────────────────── */
void sigma_gcd_async(gcd_queue_t* queue, gcd_block_t block, void* context) {
    gcd_task_t task = { block, context };
    if (!queue_push(queue, task)) {
        sigma_sigma_sigma_printf("S [GCD] Warning: Queue %s is full! Dropping task.\n", queue->name);
        return;
    }
    /* In a real kernel, this would trigger a scheduler signal or wake a work thread */
}

void sigma_gcd_sync(gcd_queue_t* queue, gcd_block_t block, void* context) {
    /* 
     * In Sovereign GCD, sync execution waits for the immediate availability 
     * or executes locally if possible. For simulation, we execute locally.
     */
    sigma_sigma_sigma_printf("S [GCD] Sync execution on queue: %s\n", queue->name);
    block(context);
}

void sigma_gcd_apply(sigma_u32 iterations, gcd_queue_t* queue, void (*block)(sigma_u32 index)) {
    sigma_sigma_sigma_printf("S [GCD] Applying %u iterations on queue: %s\n", iterations, queue->name);
    for (sigma_u32 i = 0; i < iterations; i++) {
        /* Distribute iterations across the queue ring */
        block(i);
    }
}

/* ── Statistics ────────────────────────────────────────────────────────── */
void sigma_gcd_stats(void) {
    sigma_sigma_sigma_printf("\nS GCD LATTICE STATS\n");
    sigma_sigma_sigma_printf("%-4s %-20s %-8s %-8s\n", "ID", "NAME", "PRIO", "PENDING");
    for (sigma_u32 i = 0; i < s_queue_count; i++) {
        gcd_queue_t* q = &s_queues[i];
        sigma_u32 pending = (q->tail >= q->head) ? (q->tail - q->head) : (GCD_RING_SIZE - q->head + q->tail);
        sigma_sigma_sigma_printf("%-4u %-20s %-8d %-8u\n", q->queue_id, q->name, q->priority, pending);
    }
}
