/*
 * Σ SHARD: SOVEREIGN-DS — Data Structures v2.0
 * Doctrine: Pure C11. No stdlib. All UDF.
 * Provides: Stack, Queue, Hash-Map primitives.
 */
#include "../sigma_kernel_types.h"

/* --- Generic Stack (fixed capacity, byte elements) --- */
#define SIGMA_STACK_CAP 256
typedef struct { u8 data[SIGMA_STACK_CAP]; u32 top; } SigmaStack;

static inline void   stack_init(SigmaStack* s)            { s->top = 0; }
static inline bool_t stack_push(SigmaStack* s, u8 v)      { if (s->top >= SIGMA_STACK_CAP) return FALSE; s->data[s->top++] = v; return TRUE; }
static inline bool_t stack_pop (SigmaStack* s, u8* out)   { if (s->top == 0) return FALSE; *out = s->data[--s->top]; return TRUE; }
static inline bool_t stack_peek(SigmaStack* s, u8* out)   { if (s->top == 0) return FALSE; *out = s->data[s->top-1]; return TRUE; }
static inline bool_t stack_empty(const SigmaStack* s)     { return s->top == 0 ? TRUE : FALSE; }

/* --- Generic Queue (circular, fixed capacity) --- */
#define SIGMA_QUEUE_CAP 256
typedef struct { u8 data[SIGMA_QUEUE_CAP]; u32 head; u32 tail; u32 count; } SigmaQueue;

static inline void   queue_init(SigmaQueue* q)           { q->head = q->tail = q->count = 0; }
static inline bool_t queue_push(SigmaQueue* q, u8 v)     { if (q->count >= SIGMA_QUEUE_CAP) return FALSE; q->data[q->tail] = v; q->tail = (q->tail+1) % SIGMA_QUEUE_CAP; q->count++; return TRUE; }
static inline bool_t queue_pop (SigmaQueue* q, u8* out)  { if (q->count == 0) return FALSE; *out = q->data[q->head]; q->head = (q->head+1) % SIGMA_QUEUE_CAP; q->count--; return TRUE; }
static inline bool_t queue_empty(const SigmaQueue* q)    { return q->count == 0 ? TRUE : FALSE; }

/* --- Open-address Hash Map (u32 key → u64 value, FNV probe) --- */
#define SIGMA_HM_CAP 64
typedef struct { u32 key; u64 val; bool_t used; } SigmaHMEntry;
typedef struct { SigmaHMEntry entries[SIGMA_HM_CAP]; } SigmaHashMap;

static inline void hm_init(SigmaHashMap* m) {
    for (u32 i = 0; i < SIGMA_HM_CAP; i++) { m->entries[i].used = FALSE; m->entries[i].key = 0; m->entries[i].val = 0; }
}
static inline u32 hm_slot(u32 key) {
    /* FNV-1a single word — no stdlib */
    u32 h = 0x811c9dc5u ^ key;
    h *= 0x01000193u;
    return h % SIGMA_HM_CAP;
}
static inline bool_t hm_set(SigmaHashMap* m, u32 key, u64 val) {
    u32 slot = hm_slot(key);
    for (u32 i = 0; i < SIGMA_HM_CAP; i++) {
        u32 s = (slot + i) % SIGMA_HM_CAP;
        if (!m->entries[s].used || m->entries[s].key == key) {
            m->entries[s].key = key; m->entries[s].val = val; m->entries[s].used = TRUE;
            return TRUE;
        }
    }
    return FALSE; /* full */
}
static inline bool_t hm_get(SigmaHashMap* m, u32 key, u64* out) {
    u32 slot = hm_slot(key);
    for (u32 i = 0; i < SIGMA_HM_CAP; i++) {
        u32 s = (slot + i) % SIGMA_HM_CAP;
        if (!m->entries[s].used) return FALSE;
        if (m->entries[s].key == key) { *out = m->entries[s].val; return TRUE; }
    }
    return FALSE;
}
