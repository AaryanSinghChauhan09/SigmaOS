/**
 * =========================================================================
 * Σ SIGMAOS: ZENITHD STRUCTURED LOGGING — IMPLEMENTATION (Phase 16)
 * =========================================================================
 * Lock-free ring buffer logger with severity filtering, CPU-ID stamps,
 * and correlation IDs for causal tracing.
 *
 * Design:
 *   - Ring buffer of ZENITH_LOG_RING_SIZE entries (power of 2)
 *   - Write index advanced via atomic CAS (no spinlock in log path)
 *   - Serial output mirrors high-severity entries (WARN+) in real-time
 *   - Ring never blocks: oldest entries are silently overwritten
 *   - Flush-to-disk writes the entire ring to zenithd.log when VFS ready
 *
 * Inspired by:
 *   - Linux kernel's printk ring buffer (log_buf)
 *   - systemd's journal binary format
 *   - NixOS's structured trace logs
 * =========================================================================
 */

#include "../../include/sigma_zenithd_log.h"
#include "../../include/sigma_kernel_types.h"

/* We need string helpers but NOT the full sigma_libc.h to avoid circular deps.
 * The inline versions in sigma_kernel_types.h are sufficient. */

/* ---- Ring Buffer State ---- */

static zenith_log_entry_t g_ring[ZENITH_LOG_RING_SIZE];
static volatile sigma_u32 g_write_idx = 0;
static sigma_u32          g_min_severity = ZEN_INFO;  /* Default: filter TRACE/DEBUG */
static sigma_u64          g_total_emitted = 0;
static sigma_u64          g_total_dropped = 0;
static sigma_u64          g_ring_wraps = 0;
static sigma_bool         g_log_initialized = SIGMA_FALSE;


/* ---- Severity Name Table ---- */

static const char* severity_names[] = {
    "TRACE", "DEBUG", "INFO ", "WARN ", "ERROR", "CRIT ", "PANIC"
};


/* ---- Internal Helpers ---- */

/**
 * Copy src into dst, bounded to max-1 chars, always NUL-terminated.
 */
static void log_strcpy_bounded(char* dst, const char* src, sigma_size_t max) {
    sigma_size_t i = 0;
    if (!src) { dst[0] = '\0'; return; }
    while (i < max - 1 && src[i] != '\0') {
        dst[i] = src[i];
        i++;
    }
    dst[i] = '\0';
}

/**
 * Atomic compare-and-swap for the write index.
 * Returns true if the swap succeeded.
 */
static inline sigma_bool atomic_cas_u32(volatile sigma_u32* ptr,
                                         sigma_u32 expected,
                                         sigma_u32 desired) {
#if defined(__GNUC__) || defined(__clang__)
    return __sync_bool_compare_and_swap(ptr, expected, desired);
#else
    /* Fallback: non-atomic (only safe on single-CPU) */
    if (*ptr == expected) { *ptr = desired; return SIGMA_TRUE; }
    return SIGMA_FALSE;
#endif
}


/* ---- Core API Implementation ---- */

void zenith_log_init(void) {
    /* Zero the ring buffer */
    sigma_u8* ring_bytes = (sigma_u8*)g_ring;
    for (sigma_size_t i = 0; i < sizeof(g_ring); i++) {
        ring_bytes[i] = 0;
    }
    g_write_idx     = 0;
    g_total_emitted = 0;
    g_total_dropped = 0;
    g_ring_wraps    = 0;
    g_min_severity  = ZEN_INFO;
    g_log_initialized = SIGMA_TRUE;
}

void zenith_log_emit(sigma_u32 severity, sigma_u32 error_code,
                     const char* component, const char* message,
                     sigma_u32 cid) {
    /* Severity gate */
    if (severity < g_min_severity) {
        g_total_dropped++;
        return;
    }

    /* Claim a slot in the ring via atomic CAS */
    sigma_u32 idx;
    sigma_u32 next;
    do {
        idx  = g_write_idx;
        next = (idx + 1) & (ZENITH_LOG_RING_SIZE - 1);
    } while (!atomic_cas_u32(&g_write_idx, idx, next));

    /* Track wraps */
    if (next == 0) g_ring_wraps++;

    /* Fill the entry */
    zenith_log_entry_t* entry = &g_ring[idx];
    entry->timestamp_tsc  = cpu_rdtsc();
    entry->severity       = severity;
    entry->error_code     = error_code;
    entry->cpu_id         = cpu_get_id();
    entry->correlation_id = cid;
    log_strcpy_bounded(entry->component, component, ZENITH_LOG_COMP_LEN);
    log_strcpy_bounded(entry->message, message, ZENITH_LOG_MSG_LEN);

    g_total_emitted++;

    /* Mirror high-severity entries to serial output immediately */
    if (severity >= ZEN_WARN) {
        const char* sev_name = (severity <= ZEN_PANIC)
                               ? severity_names[severity] : "?????";

        /* Format: [TSC:xxxx] [CPU:n] [LEVEL] [ZEN-CODE] [comp] message */
        serial_puts("[TSC:");
        /* Quick hex output of TSC (upper 32 bits only for brevity) */
        char hex[9];
        sigma_u32 tsc_hi = (sigma_u32)(entry->timestamp_tsc >> 32);
        for (int i = 7; i >= 0; i--) {
            hex[i] = "0123456789abcdef"[tsc_hi & 0xF];
            tsc_hi >>= 4;
        }
        hex[8] = '\0';
        serial_puts(hex);
        serial_puts("] [CPU:");
        serial_putc('0' + (char)(entry->cpu_id % 10));
        serial_puts("] [");
        serial_puts(sev_name);
        serial_puts("] ");

        if (error_code != 0) {
            serial_puts("[0x");
            char code_hex[9];
            sigma_u32 ec = error_code;
            for (int i = 7; i >= 0; i--) {
                code_hex[i] = "0123456789abcdef"[ec & 0xF];
                ec >>= 4;
            }
            code_hex[8] = '\0';
            serial_puts(code_hex);
            serial_puts("] ");
        }

        serial_puts("[");
        serial_puts(entry->component);
        serial_puts("] ");
        serial_puts(entry->message);
        serial_puts("\n");
    }
}

void zenith_log_structured(sigma_u32 code, const char* comp,
                           const char* desc, sigma_u32 cid) {
    /* Legacy wrapper — default to ERROR severity */
    sigma_u32 severity = ZEN_ERROR;

    /* Infer severity from code ranges */
    if (code >= 0xB000 && code <= 0xBFFF) severity = ZEN_PANIC;  /* KRN panics */
    else if (code >= 0xA000 && code <= 0xAFFF) severity = ZEN_CRIT;   /* MEM leaks */
    else if (code >= 0xD000 && code <= 0xFFFF) severity = ZEN_ERROR;  /* Driver/Net/FS */

    zenith_log_emit(severity, code, comp, desc, cid);
}

void zenith_log_set_severity(sigma_u32 min_severity) {
    if (min_severity <= ZEN_PANIC) {
        g_min_severity = min_severity;
    }
}

zenith_log_stats_t zenith_log_get_stats(void) {
    zenith_log_stats_t stats;
    stats.total_emitted    = g_total_emitted;
    stats.total_dropped    = g_total_dropped;
    stats.ring_wraps       = g_ring_wraps;
    stats.current_severity = g_min_severity;
    stats.ring_write_idx   = g_write_idx;
    return stats;
}

const zenith_log_entry_t* zenith_log_entry_at(sigma_u32 index) {
    return &g_ring[index & (ZENITH_LOG_RING_SIZE - 1)];
}

void zenith_log_dump(void) {
    serial_puts("\n=== ZENITHD LOG DUMP ===\n");

    sigma_u32 count = g_total_emitted < ZENITH_LOG_RING_SIZE
                      ? (sigma_u32)g_total_emitted
                      : ZENITH_LOG_RING_SIZE;

    sigma_u32 start = (g_write_idx - count) & (ZENITH_LOG_RING_SIZE - 1);

    for (sigma_u32 i = 0; i < count; i++) {
        sigma_u32 idx = (start + i) & (ZENITH_LOG_RING_SIZE - 1);
        const zenith_log_entry_t* e = &g_ring[idx];

        const char* sev_name = (e->severity <= ZEN_PANIC)
                               ? severity_names[e->severity] : "?????";

        serial_puts("[");
        serial_puts(sev_name);
        serial_puts("] [");
        serial_puts(e->component);
        serial_puts("] ");
        serial_puts(e->message);
        serial_puts("\n");
    }

    serial_puts("=== END DUMP ===\n");
}

void zenith_log_flush_to_disk(const char* path) {
    /* TODO: When VFS is available, write ring contents to path */
    (void)path;
    serial_puts("[zenithd] flush_to_disk: VFS not yet available\n");
}
