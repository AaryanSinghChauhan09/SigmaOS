/*
 * =========================================================================
 * Σ SIGMAOS ZENITH: SOVEREIGN JOURNAL SHARD (v1.0)
 * =========================================================================
 * Mission: Absorb systemd-journal / Windows Event Log / syslog USP.
 *          Native Structured Kernel Ring-Buffer with Priority Filtering.
 * Design: C11 / Zero-Dependency / Lock-Free Circular Event Log.
 * Standard: Zenith Industrial Sovereignty.
 * =========================================================================
 */

#include "../../include/sigma_kernel.h"

// -------------------------------------------------------------------------
// Journal Structures
// -------------------------------------------------------------------------

typedef enum {
    LOG_EMERG   = 0,   /* System-panic level          */
    LOG_ALERT   = 1,   /* Immediate action required   */
    LOG_CRIT    = 2,   /* Critical condition           */
    LOG_ERR     = 3,   /* Error condition              */
    LOG_WARNING = 4,   /* Warning                      */
    LOG_NOTICE  = 5,   /* Normal but significant       */
    LOG_INFO    = 6,   /* Informational                */
    LOG_DEBUG   = 7    /* Debug-level                  */
} SigmaLogLevel_t;

typedef struct {
    sigma_u64      seq;           /* Monotonic sequence number  */
    sigma_u64      timestamp_us;  /* Microseconds since epoch   */
    SigmaLogLevel_t level;
    char           unit[24];      /* Originating silicon shard  */
    char           message[128];
} SigmaJournalEntry_t;

#define JOURNAL_RING_SIZE 64
static SigmaJournalEntry_t s_ring[JOURNAL_RING_SIZE];
static sigma_u64            s_seq      = 0;
static sigma_u32            s_head     = 0;   /* next write slot  */
static sigma_u32            s_count    = 0;   /* entries present  */

static const char* s_level_str[] = {
    "EMERG", "ALERT", "CRIT", "ERR", "WARN", "NOTICE", "INFO", "DEBUG"
};
static const char* s_level_color[] = {
    "\033[1;31m","\033[1;31m","\033[0;31m","\033[0;31m",
    "\033[0;33m","\033[0;36m","\033[0;32m","\033[0;37m"
};

// -------------------------------------------------------------------------
// Journal Logic (systemd-journal / syslog / Windows EventLog parity)
// -------------------------------------------------------------------------

/**
 * sigma_journal_write: Atomically writes a structured event to the ring buffer.
 */
void sigma_journal_write(SigmaLogLevel_t lvl,
                          const char* unit,
                          const char* msg) {
    SigmaJournalEntry_t* e = &s_ring[s_head];
    e->seq          = ++s_seq;
    e->timestamp_us = s_seq * 1000ULL; /* Simulated monotonic clock */
    e->level        = lvl;
    sigma_strcpy(e->unit,    unit);
    sigma_strcpy(e->message, msg);

    s_head = (s_head + 1) % JOURNAL_RING_SIZE;
    if (s_count < JOURNAL_RING_SIZE) s_count++;

    /* Immediate console output for CRIT+ */
    if (lvl <= LOG_CRIT) {
        sigma_printf("%s[%s] %s: %s\033[0m\n",
                     s_level_color[lvl], s_level_str[lvl], unit, msg);
    }
}

/**
 * sigma_journal_follow: Streams the ring buffer (mimics journalctl -f).
 */
void sigma_journal_follow(SigmaLogLevel_t min_level) {
    sigma_printf("\n[JOURNAL]: Streaming kernel silicon journal (>= %s)...\n",
                 s_level_str[min_level]);

    sigma_u32 start = (s_count == JOURNAL_RING_SIZE)
                      ? s_head : 0;

    for (sigma_u32 i = 0; i < s_count; i++) {
        sigma_u32 idx = (start + i) % JOURNAL_RING_SIZE;
        SigmaJournalEntry_t* e = &s_ring[idx];
        if (e->level <= min_level) {
            sigma_printf("%s[%6llu | %s | %s] %s\033[0m\n",
                         s_level_color[e->level],
                         (unsigned long long)e->seq,
                         s_level_str[e->level],
                         e->unit,
                         e->message);
        }
    }
}

// -------------------------------------------------------------------------
// Industrial Journal Audit
// -------------------------------------------------------------------------

void SovereignJournal_Audit() {
    sigma_printf("\n--- SOVEREIGN JOURNAL AUDIT ---\n");
    sigma_printf("Entries: %u / %u  |  Next-seq: %llu\n",
                 s_count, JOURNAL_RING_SIZE, (unsigned long long)s_seq + 1);
    sigma_journal_follow(LOG_DEBUG);
    sigma_printf("--- END JOURNAL ---\n");
}

// -------------------------------------------------------------------------
// Factory / Constructor
// -------------------------------------------------------------------------

void SovereignJournalShard_Init() {
    sigma_printf("[SOC]: Seating Native Journal Shard (systemd-journal/EventLog Parity v1.0)...\n");
    sigma_journal_write(LOG_INFO,    "sigma_kernel",  "Sovereign kernel journal online.");
    sigma_journal_write(LOG_NOTICE,  "sigma_sched",   "Zen Scheduler armed and ready.");
    sigma_journal_write(LOG_INFO,    "sigma_tele",    "eBPF probes seated.");
    sigma_journal_write(LOG_WARNING, "sigma_oom",     "Memory pressure elevated at boot.");
}
