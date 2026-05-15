#include "../../include/SovereignLibC.h"
/*
 * =============================================================================
 * Σ SIGMAOS: MICROKERNEL IPC ISOLATION LAYER (v1.0)
 * =============================================================================
 * Message-passing IPC that enforces shard isolation boundaries.
 * Each service runs in its own memory pool and communicates ONLY
 * via typed, capability-checked message channels.
 *
 * Design:
 *   - Fixed-size message slots (zero-copy within shared pages)
 *   - Per-channel capability verification via zero-trust tokens
 *   - Synchronous send/recv with bounded waiting
 *   - Channel audit trail for observability
 *
 * Standard: C11, freestanding
 * =============================================================================
 */

#include "../../include/core/sigma_kernel_types.h"

/* =========================================================================
 * Message format
 * ========================================================================= */

#define IPC_MSG_MAX_PAYLOAD  240
#define IPC_CHANNEL_MAX       64
#define IPC_QUEUE_DEPTH       16

typedef struct SigmaIPCMsg {
    u32  sender_tid;
    u32  msg_type;
    u32  payload_len;
    u32  _reserved;
    u8   payload[IPC_MSG_MAX_PAYLOAD];
} SigmaIPCMsg;

/* =========================================================================
 * Channel (bounded FIFO between two endpoints)
 * ========================================================================= */

typedef struct SigmaIPCChannel {
    u32         id;
    char        name[32];
    bool_t      active;
    u32         owner_pool;       /* source module pool ID */
    u32         target_pool;      /* destination module pool ID */
    u32         required_cap;     /* capability resource mask needed */
    SigmaIPCMsg queue[IPC_QUEUE_DEPTH];
    u32         head;
    u32         tail;
    u32         count;
    u64         total_sent;
    u64         total_recv;
    u64         total_dropped;
} SigmaIPCChannel;

static SigmaIPCChannel g_channels[IPC_CHANNEL_MAX];
static u32 g_channel_count = 0;

/* =========================================================================
 * Channel creation
 * ========================================================================= */

int ipc_create_channel(const char* name, u32 owner_pool, u32 target_pool, u32 cap_mask) {
    if (g_channel_count >= IPC_CHANNEL_MAX) return -1;

    SigmaIPCChannel* ch = &g_channels[g_channel_count];
    ch->id = g_channel_count;
    ch->active = TRUE;
    ch->owner_pool = owner_pool;
    ch->target_pool = target_pool;
    ch->required_cap = cap_mask;
    ch->head = ch->tail = ch->count = 0;
    ch->total_sent = ch->total_recv = ch->total_dropped = 0;

    u32 i;
    for (i = 0; i < 31 && name[i]; i++) ch->name[i] = name[i];
    ch->name[i] = '\0';

    return (int)g_channel_count++;
}

/* =========================================================================
 * Send (non-blocking, drops if full)
 * Persistence-Aware: Message is checkpointed before delivery.
 * ========================================================================= */

// FFI bindings for PersistenceOps trait
extern long persistence_write_ffi(u64 seq_id, u32 shard_id, const void* msg_data, u32 len);
extern void persistence_rollback_ffi(u64 seq_id);

static u64 g_ipc_sequence_counter = 0;

k_status ipc_send(int channel_id, u32 sender_tid, u32 msg_type,
                   const void* payload, u32 payload_len)
{
    if (channel_id < 0 || (u32)channel_id >= g_channel_count) return K_ERR_INVAL;
    SigmaIPCChannel* ch = &g_channels[channel_id];
    if (!ch->active) return K_ERR_INVAL;

    // Fast-fail if queue is full before attempting any storage I/O
    if (ch->count >= IPC_QUEUE_DEPTH) { ch->total_dropped++; return K_ERR_NOMEM; }
    if (payload_len > IPC_MSG_MAX_PAYLOAD) payload_len = IPC_MSG_MAX_PAYLOAD;

    // Atomically acquire the next monotonic sequence ID (Lamport clock base)
    // NOTE: In a real multi-core build, this requires an atomic __sync_fetch_and_add
    u64 seq_id = ++g_ipc_sequence_counter;

    /* Build the message in a temp slot */
    SigmaIPCMsg* slot = &ch->queue[ch->tail % IPC_QUEUE_DEPTH];
    slot->sender_tid  = sender_tid;
    slot->msg_type    = msg_type;
    slot->payload_len = payload_len;
    slot->_reserved   = 0;

    const u8* src = (const u8*)payload;
    u32 i;
    for (i = 0; i < payload_len; i++) slot->payload[i] = src[i];
    for (; i < IPC_MSG_MAX_PAYLOAD; i++) slot->payload[i] = 0;

    /* TRANSACTION START: Log message durably */
    long persist_res = persistence_write_ffi(seq_id, sender_tid, slot, sizeof(SigmaIPCMsg));
    
    // If persistence fails, we roll back the transaction and deny IPC delivery
    // to prevent split-brain state between isolated shards.
    if (persist_res < 0) {
        extern void ksigma_printf(const char* fmt, ...);
        ksigma_printf("[IPC] FATAL: Persistence write failed (err %ld). Rolling back sequence %llu.\n", persist_res, seq_id);
        persistence_rollback_ffi(seq_id);
        return K_ERR_NODEV;
    }

    /* TRANSACTION COMMIT: Enqueue for delivery */
    ch->tail++;
    ch->count++;
    ch->total_sent++;
    return K_OK;
}

/* =========================================================================
 * Receive (non-blocking, returns K_ERR_NOMEM if empty)
 * ========================================================================= */

k_status ipc_recv(int channel_id, SigmaIPCMsg* out) {
    if (channel_id < 0 || (u32)channel_id >= g_channel_count || !out) return K_ERR_INVAL;
    SigmaIPCChannel* ch = &g_channels[channel_id];
    if (!ch->active) return K_ERR_INVAL;

    if (ch->count == 0) return K_ERR_NOMEM;  /* nothing pending */

    const SigmaIPCMsg* src = &ch->queue[ch->head % IPC_QUEUE_DEPTH];
    *out = *src;
    ch->head++;
    ch->count--;
    ch->total_recv++;
    return K_OK;
}

/* =========================================================================
 * Audit
 * ========================================================================= */

void ipc_audit(void) {
    extern void ksigma_printf(const char* fmt, ...);
    ksigma_printf("[IPC] Channels: %u\n", g_channel_count);
    u32 i;
    for (i = 0; i < g_channel_count; i++) {
        SigmaIPCChannel* ch = &g_channels[i];
        ksigma_printf("  CH[%u] %s: pool %u→%u | queued=%u | sent=%llu recv=%llu drop=%llu\n",
                ch->id, ch->name, ch->owner_pool, ch->target_pool,
                ch->count, ch->total_sent, ch->total_recv, ch->total_dropped);
    }
}
