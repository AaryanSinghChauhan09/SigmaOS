/*
 * =========================================================================
 * S SIGMAOS ZENITH: SOVEREIGN LOGGER ENGINE (v2.0)
 * =========================================================================
 * Mission: High-performance, low-latency kernel observability.
 * Principles: Buffered Journaling, Log-Level Isolation, Atomic Flushing.
 *
 * Implements a ring-buffered logging system for the Sovereign Kernel.
 * =========================================================================
 */

#include "suites/S01_Genesis/shards/sigma_kernel.h"

#define LOG_BUFFER_SIZE 2048

typedef enum {
    SIGMA_LOG_DEBUG,
    SIGMA_LOG_INFO,
    SIGMA_LOG_WARN,
    SIGMA_LOG_ERROR,
    SIGMA_LOG_FATAL
} SigmaLogLevel_t;

typedef struct {
    char            buffer[LOG_BUFFER_SIZE];
    sigma_u32       head;
    sigma_u32       tail;
    SigmaLogLevel_t current_filter;
} SigmaJournal_t;

static SigmaJournal_t s_journal;

/**
 * sigma_log_init: Initializes the ring-buffered journal.
 */
void sigma_log_init(void) {
    s_journal.head = 0;
    s_journal.tail = 0;
    s_journal.current_filter = SIGMA_LOG_DEBUG;
}

/**
 * sigma_log_write: Buffered write into the journal.
 */
void sigma_log_write(SigmaLogLevel_t level, const char* msg) {
    if (level < s_journal.current_filter) return;

    const char* prefix = "[INFO]";
    if (level == SIGMA_LOG_WARN)  prefix = "[WARN]";
    if (level == SIGMA_LOG_ERROR) prefix = "[ERR ]";
    if (level == SIGMA_LOG_DEBUG) prefix = "[DEBG]";

    char full_msg[128];
    sigma_snprintf(full_msg, 128, "%s %s\n", prefix, msg);

    for (int i = 0; full_msg[i] != '\0'; i++) {
        s_journal.buffer[s_journal.head % LOG_BUFFER_SIZE] = full_msg[i];
        s_journal.head++;
        
        /* If head catches tail, overflow logic: move tail forward */
        if (s_journal.head - s_journal.tail > LOG_BUFFER_SIZE) {
            s_journal.tail++;
        }
    }
}

/**
 * sigma_log_flush: Flushes the journal to the console/serial port.
 */
void sigma_log_flush(void) {
    while (s_journal.tail < s_journal.head) {
        sigma_putchar(s_journal.buffer[s_journal.tail % LOG_BUFFER_SIZE]);
        s_journal.tail++;
    }
}

/* --- Module Factory --- */

sigma_err_t sigma_logger_start(void) {
    sigma_log_init();
    sigma_log_write(SIGMA_LOG_INFO, "Sovereign Logger Engine v2.0 Seated.");
    sigma_log_write(SIGMA_LOG_DEBUG, "Ring buffer allocated (2KB).");
    return SIGMA_OK;
}

void SovereignLogger_Register(void) {
    sigma_logger_start();
}



