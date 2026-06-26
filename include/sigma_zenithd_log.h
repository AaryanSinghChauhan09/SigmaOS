/*
 * =========================================================================
 * Σ SIGMAOS: ZENITHD STRUCTURED LOGGING — PUBLIC API (Phase 16)
 * =========================================================================
 * Mission: Deterministic, machine-parseable, lock-free kernel logging.
 *
 * Features:
 *   - Ring buffer (no heap allocation in log path)
 *   - Severity-gated output (runtime adjustable)
 *   - Sovereign error code taxonomy (ZEN-DRV-xxx, ZEN-NET-xxx, etc.)
 *   - CPU-ID stamped entries for SMP debugging
 *   - Correlation IDs for tracing causal chains across subsystems
 *   - Atomic CAS-based write index (no spinlock in hot path)
 *
 * Format of zenithd.log entries (text serialization):
 *   [TSC:0000000012345678] [CPU:0] [WARN ] [ZEN-NET-0501] [net_stack] NIC init failed
 * =========================================================================
 */

#ifndef SIGMA_ZENITHD_LOG_H
#define SIGMA_ZENITHD_LOG_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* ---- Severity Levels ---- */
#define ZEN_TRACE   0u
#define ZEN_DEBUG   1u
#define ZEN_INFO    2u
#define ZEN_WARN    3u
#define ZEN_ERROR   4u
#define ZEN_CRIT    5u
#define ZEN_PANIC   6u

/* ---- Ring Buffer Configuration ---- */
#define ZENITH_LOG_RING_SIZE    4096u   /* Must be power of 2 */
#define ZENITH_LOG_MSG_LEN      112u
#define ZENITH_LOG_COMP_LEN      16u

/* ---- Log Entry ---- */
typedef struct {
    sigma_u64 timestamp_tsc;                   /* CPU timestamp counter */
    sigma_u32 severity;                        /* ZEN_TRACE .. ZEN_PANIC */
    sigma_u32 error_code;                      /* Sovereign error code (0 = none) */
    sigma_u32 cpu_id;                          /* Which CPU logged this */
    sigma_u32 correlation_id;                  /* Causal chain ID (0 = standalone) */
    char      component[ZENITH_LOG_COMP_LEN];  /* e.g. "net_stack", "vfs", "sched" */
    char      message[ZENITH_LOG_MSG_LEN];     /* Human-readable message */
} zenith_log_entry_t;

/* ---- Statistics ---- */
typedef struct {
    sigma_u64 total_emitted;        /* Total entries ever written */
    sigma_u64 total_dropped;        /* Entries dropped due to severity filter */
    sigma_u64 ring_wraps;           /* How many times the ring has wrapped */
    sigma_u32 current_severity;     /* Current filter threshold */
    sigma_u32 ring_write_idx;       /* Current write position */
} zenith_log_stats_t;


/* ---- Core API ---- */

/** Initialize the logging subsystem. Call once at boot. */
void zenith_log_init(void);

/**
 * Emit a structured log entry.
 *
 * @param severity    One of ZEN_TRACE..ZEN_PANIC
 * @param error_code  Sovereign error code (e.g., ZEN_DRV_GPU_INIT_FAILED), or 0
 * @param component   Short subsystem name (e.g., "net_stack")
 * @param message     Human-readable message
 * @param cid         Correlation ID (0 for standalone entries)
 */
void zenith_log_emit(sigma_u32 severity, sigma_u32 error_code,
                     const char* component, const char* message,
                     sigma_u32 cid);

/**
 * Legacy-compatible wrapper (matches the extern in recovery.c).
 * Maps to zenith_log_emit(ZEN_ERROR, code, comp, desc, cid).
 */
void zenith_log_structured(sigma_u32 code, const char* comp,
                           const char* desc, sigma_u32 cid);

/** Set the minimum severity level. Entries below this are dropped. */
void zenith_log_set_severity(sigma_u32 min_severity);

/** Get current logging statistics. */
zenith_log_stats_t zenith_log_get_stats(void);

/** Dump the entire ring buffer to serial output (for debugging). */
void zenith_log_dump(void);

/** Dump ring buffer to a file path (when VFS is available). */
void zenith_log_flush_to_disk(const char* path);

/** Get a read-only pointer to a specific ring entry (by index mod size). */
const zenith_log_entry_t* zenith_log_entry_at(sigma_u32 index);


/* ---- Convenience Macros ---- */

#define ZENITH_LOG(sev, code, comp, msg, cid) \
    zenith_log_emit((sev), (code), (comp), (msg), (cid))

#define ZENITH_TRACE(comp, msg) \
    zenith_log_emit(ZEN_TRACE, 0, (comp), (msg), 0)

#define ZENITH_DEBUG(comp, msg) \
    zenith_log_emit(ZEN_DEBUG, 0, (comp), (msg), 0)

#define ZENITH_INFO(comp, msg) \
    zenith_log_emit(ZEN_INFO, 0, (comp), (msg), 0)

#define ZENITH_WARN(comp, msg) \
    zenith_log_emit(ZEN_WARN, 0, (comp), (msg), 0)

#define ZENITH_ERROR(code, comp, msg) \
    zenith_log_emit(ZEN_ERROR, (code), (comp), (msg), 0)

#define ZENITH_CRIT(code, comp, msg) \
    zenith_log_emit(ZEN_CRIT, (code), (comp), (msg), 0)

#define ZENITH_PANIC(code, comp, msg) \
    zenith_log_emit(ZEN_PANIC, (code), (comp), (msg), 0)

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_ZENITHD_LOG_H */
