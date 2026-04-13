/*
 * =========================================================================
 * Σ SIGMAOS ZENITH: SOVEREIGN DESIGN PATTERNS ENGINE (v1.0)
 * =========================================================================
 * Mission: Classical GoF Design Patterns implemented in pure C11.
 * Patterns: Singleton, Observer, Strategy, Factory, Iterator.
 *
 * Demonstrates that even without C++, the kernel rigorously
 * follows established software engineering design patterns.
 * =========================================================================
 */

#include "../../include/sigma_kernel.h"

/* =======================================================================
 * PATTERN 1: SINGLETON — Global Kernel Logger
 * Only one instance exists; all shards share it.
 * ======================================================================= */

typedef struct {
    sigma_u32 log_level;       /* 0=DEBUG, 1=INFO, 2=WARN, 3=ERROR */
    sigma_u32 entries_written;
    char      last_message[128];
} SigmaLogger_t;

static SigmaLogger_t* s_logger_instance = SIGMA_NULL;
static SigmaLogger_t  s_logger_storage;

SigmaLogger_t* sigma_logger_get(void) {
    if (!s_logger_instance) {
        s_logger_instance = &s_logger_storage;
        s_logger_instance->log_level = 1;  /* INFO */
        s_logger_instance->entries_written = 0;
        sigma_printf("[SINGLETON]: Kernel Logger instance created.\n");
    }
    return s_logger_instance;
}

void sigma_logger_write(SigmaLogger_t* logger, sigma_u32 level,
                        const char* msg) {
    if (level < logger->log_level) return;  /* filtered */
    sigma_strncpy(logger->last_message, msg, 128);
    logger->entries_written++;
}

/* =======================================================================
 * PATTERN 2: OBSERVER — Event Bus
 * Shards subscribe to events; the bus notifies all subscribers.
 * ======================================================================= */

typedef void (*EventHandler_t)(sigma_u32 event_id, void* data);

typedef struct {
    EventHandler_t handler;
    sigma_u32      event_filter;   /* which event_id to listen for */
} SigmaObserver_t;

#define MAX_OBSERVERS 32
static SigmaObserver_t s_observers[MAX_OBSERVERS];
static sigma_u32 s_observer_count = 0;

sigma_err_t sigma_event_subscribe(sigma_u32 event_id, EventHandler_t handler) {
    if (s_observer_count >= MAX_OBSERVERS) return SIGMA_ENOSPC;

    s_observers[s_observer_count].handler      = handler;
    s_observers[s_observer_count].event_filter  = event_id;
    s_observer_count++;
    return SIGMA_OK;
}

void sigma_event_publish(sigma_u32 event_id, void* data) {
    for (sigma_u32 i = 0; i < s_observer_count; i++) {
        if (s_observers[i].event_filter == event_id && s_observers[i].handler) {
            s_observers[i].handler(event_id, data);
        }
    }
}

/* =======================================================================
 * PATTERN 3: STRATEGY — Pluggable Sort Algorithms
 * The caller selects the sorting strategy at runtime.
 * ======================================================================= */

typedef void (*SortStrategy_t)(sigma_f64* arr, sigma_u32 len);

/* Insertion Sort strategy */
static void strategy_insertion_sort(sigma_f64* arr, sigma_u32 len) {
    for (sigma_u32 i = 1; i < len; i++) {
        sigma_f64 key = arr[i];
        sigma_u32 j = i;
        while (j > 0 && arr[j - 1] > key) {
            arr[j] = arr[j - 1];
            j--;
        }
        arr[j] = key;
    }
}

/* Selection Sort strategy */
static void strategy_selection_sort(sigma_f64* arr, sigma_u32 len) {
    for (sigma_u32 i = 0; i < len - 1; i++) {
        sigma_u32 min_idx = i;
        for (sigma_u32 j = i + 1; j < len; j++) {
            if (arr[j] < arr[min_idx]) min_idx = j;
        }
        if (min_idx != i) {
            sigma_f64 tmp = arr[i];
            arr[i] = arr[min_idx];
            arr[min_idx] = tmp;
        }
    }
}

/* Context that holds the chosen strategy */
typedef struct {
    SortStrategy_t strategy;
    const char*    strategy_name;
} SigmaSortContext_t;

void sigma_sort_set_strategy(SigmaSortContext_t* ctx, SortStrategy_t fn,
                             const char* name) {
    ctx->strategy      = fn;
    ctx->strategy_name = name;
}

void sigma_sort_execute(SigmaSortContext_t* ctx, sigma_f64* arr, sigma_u32 len) {
    if (ctx->strategy) {
        sigma_printf("[STRATEGY]: Sorting %u elements with '%s'\n",
                     len, ctx->strategy_name);
        ctx->strategy(arr, len);
    }
}

/* =======================================================================
 * PATTERN 4: FACTORY — Creates typed kernel objects
 * ======================================================================= */

typedef enum {
    KOBJ_THREAD,
    KOBJ_FILE,
    KOBJ_SOCKET,
    KOBJ_TIMER
} KernelObjectType_t;

typedef struct {
    KernelObjectType_t type;
    sigma_u32          id;
    char               label[32];
} KernelObject_t;

static sigma_u32 s_next_obj_id = 1;

KernelObject_t sigma_factory_create(KernelObjectType_t type, const char* label) {
    const char* type_names[] = {"THREAD", "FILE", "SOCKET", "TIMER"};
    KernelObject_t obj;
    obj.type = type;
    obj.id   = s_next_obj_id++;
    sigma_strncpy(obj.label, label, 32);

    sigma_printf("[FACTORY]: Created %s object '%s' (id: %u)\n",
                 type_names[type], label, obj.id);
    return obj;
}

/* =======================================================================
 * PATTERN 5: ITERATOR — Walk a collection without exposing internals
 * ======================================================================= */

typedef struct {
    sigma_f64*  data;
    sigma_u32   length;
    sigma_u32   cursor;
} SigmaIterator_t;

SigmaIterator_t sigma_iter_create(sigma_f64* data, sigma_u32 len) {
    SigmaIterator_t it;
    it.data   = data;
    it.length = len;
    it.cursor = 0;
    return it;
}

int sigma_iter_has_next(SigmaIterator_t* it) {
    return it->cursor < it->length;
}

sigma_f64 sigma_iter_next(SigmaIterator_t* it) {
    return it->data[it->cursor++];
}

void sigma_iter_reset(SigmaIterator_t* it) {
    it->cursor = 0;
}

/* --- Module Factory --- */

void SovereignDesignPatterns_Register(void) {
    sigma_printf("[REGISTRY]: Sovereign Design Patterns Engine (Singleton/Observer/Strategy/Factory/Iterator) active.\n");

    /* Demo: create logger singleton */
    sigma_logger_get();

    /* Demo: register sort strategies */
    SigmaSortContext_t ctx;
    sigma_sort_set_strategy(&ctx, strategy_insertion_sort, "InsertionSort");
    (void)ctx;

    sigma_sort_set_strategy(&ctx, strategy_selection_sort, "SelectionSort");
    (void)ctx;
}
