/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: KERNEL RING BUFFER LOGGER (dmesg)
 * =============================================================================
 * Inspired by: Linux kernel printk / kernel/printk/printk.c
 *              FreeBSD msgbuf (sys/msgbuf.h)
 *              syslog(3) severity levels (RFC 5424)
 * =============================================================================
 * Circular ring buffer capturing all kernel log messages for dmesg retrieval.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

#define KLOG_BUF_SIZE     16384
#define KLOG_MAX_MSG_LEN  256
#define KLOG_MAX_ENTRIES  128

/* Severity levels (syslog-compatible) */
#define KLOG_EMERG   0  /* System is unusable */
#define KLOG_ALERT   1  /* Immediate action required */
#define KLOG_CRIT    2  /* Critical conditions */
#define KLOG_ERR     3  /* Error conditions */
#define KLOG_WARN    4  /* Warning conditions */
#define KLOG_NOTICE  5  /* Normal but significant */
#define KLOG_INFO    6  /* Informational */
#define KLOG_DEBUG   7  /* Debug-level messages */

typedef struct {
    sigma_u32 sequence;
    sigma_u32 timestamp_ms;
    sigma_u8  level;
    char      facility[16];
    char      message[KLOG_MAX_MSG_LEN];
} klog_entry_t;

typedef struct {
    klog_entry_t entries[KLOG_MAX_ENTRIES];
    sigma_u32    head;
    sigma_u32    tail;
    sigma_u32    count;
    sigma_u32    seq_counter;
    sigma_u32    dropped;
    sigma_u8     min_level;  /* Only log messages >= this level */
} klog_ring_t;

static klog_ring_t klog;

static const char* klog_level_str(sigma_u8 level) {
    switch (level) {
        case KLOG_EMERG:  return "EMERG";
        case KLOG_ALERT:  return "ALERT";
        case KLOG_CRIT:   return "CRIT";
        case KLOG_ERR:    return "ERR";
        case KLOG_WARN:   return "WARN";
        case KLOG_NOTICE: return "NOTICE";
        case KLOG_INFO:   return "INFO";
        case KLOG_DEBUG:  return "DEBUG";
        default:          return "???";
    }
}

void klog_init(void) {
    sigma_memset(&klog, 0, sizeof(klog));
    klog.min_level = KLOG_DEBUG; /* Log everything by default */
    sigma_printf("[klog] Kernel ring buffer initialized (%u entry slots)\n", KLOG_MAX_ENTRIES);
}

void klog_set_level(sigma_u8 level) {
    klog.min_level = level;
    sigma_printf("[klog] Minimum log level set to %s (%u)\n", klog_level_str(level), level);
}

void klog_write(sigma_u8 level, const char* facility, const char* msg) {
    if (level > klog.min_level) return; /* Filter by severity */

    klog_entry_t* e = &klog.entries[klog.tail];
    e->sequence     = klog.seq_counter++;
    e->timestamp_ms = klog.seq_counter * 10; /* Simulated monotonic clock */
    e->level        = level;

    /* Copy facility name */
    sigma_u32 i = 0;
    while (i < 15 && facility[i]) { e->facility[i] = facility[i]; i++; }
    e->facility[i] = '\0';

    /* Copy message */
    i = 0;
    while (i < KLOG_MAX_MSG_LEN - 1 && msg[i]) { e->message[i] = msg[i]; i++; }
    e->message[i] = '\0';

    klog.tail = (klog.tail + 1) % KLOG_MAX_ENTRIES;
    if (klog.count < KLOG_MAX_ENTRIES) {
        klog.count++;
    } else {
        klog.head = (klog.head + 1) % KLOG_MAX_ENTRIES;
        klog.dropped++;
    }
}

void klog_dmesg(sigma_u32 max_lines) {
    sigma_printf("\n--- Σ KERNEL LOG (dmesg) ---\n");
    if (klog.count == 0) {
        sigma_printf("  (empty)\n");
        return;
    }

    sigma_u32 start = klog.head;
    sigma_u32 lines = (max_lines > 0 && max_lines < klog.count) ? max_lines : klog.count;
    sigma_u32 skip  = klog.count - lines;

    for (sigma_u32 i = 0; i < lines; i++) {
        sigma_u32 idx = (start + skip + i) % KLOG_MAX_ENTRIES;
        klog_entry_t* e = &klog.entries[idx];
        sigma_printf("[%5u.%03u] <%s> %s: %s\n",
            e->timestamp_ms / 1000, e->timestamp_ms % 1000,
            klog_level_str(e->level), e->facility, e->message);
    }
    if (klog.dropped > 0) {
        sigma_printf("  (%u messages dropped due to buffer overflow)\n", klog.dropped);
    }
    sigma_printf("--- END ---\n");
}

void klog_clear(void) {
    klog.head = 0;
    klog.tail = 0;
    klog.count = 0;
    klog.dropped = 0;
    sigma_printf("[klog] Ring buffer cleared\n");
}

void klog_stats(void) {
    sigma_printf("\n--- Σ KLOG STATS ---\n");
    sigma_printf("| Entries    : %u / %u\n", klog.count, KLOG_MAX_ENTRIES);
    sigma_printf("| Seq Counter: %u\n", klog.seq_counter);
    sigma_printf("| Dropped    : %u\n", klog.dropped);
    sigma_printf("| Min Level  : %s\n", klog_level_str(klog.min_level));
    sigma_printf("--------------------\n");
}
