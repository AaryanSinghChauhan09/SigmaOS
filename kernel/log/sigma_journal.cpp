// SPDX-License-Identifier: GPL-2.0-or-later
// sigma_journal.cpp — Binary structured kernel journal for SigmaOS
//
// A lock-free circular ring buffer in pinned kernel memory.
// Survives OOM (never calls the allocator after init).
// Accessible from userland via /dev/sigma-log (mmap + poll).
//
// Each entry is a fixed-size struct with:
//   • nanosecond monotonic timestamp
//   • CPU ID (which core logged this)
//   • severity level (DEBUG/INFO/WARN/ERROR/PANIC)
//   • subsystem hash (FNV-1a of "kernel.net.wifi" etc.)
//   • message text (256 bytes, NUL-terminated)
//   • structured key-value fields (8 × uint64 pairs)
//
// The ring buffer is seqlock-protected:
//   Writer increments sequence (odd = writing), writes entry, increments again (even).
//   Reader spins if sequence is odd, or retries if sequence changed between reads.
//
// Inspired by:
//   • systemd-journal (structured binary logging)
//   • Linux kernel's printk ring buffer (arch/x86/kernel/traps.c)
//   • DTrace structured records

#include "sigma_journal.h"
#include <stdint.h>
#include <stddef.h>
#include <stdarg.h>
#include <stdatomic.h>
#include <string.h>

// ── Ring buffer constants ─────────────────────────────────────────────────────

#define JOURNAL_CAPACITY   4096    // must be power of 2
#define JOURNAL_MASK       (JOURNAL_CAPACITY - 1)

// ── Entry format ──────────────────────────────────────────────────────────────

#define SIGMA_LOG_FIELD_COUNT  8

typedef struct sigma_log_entry {
    uint32_t  sequence;          // seqlock version (even = complete, odd = writing)
    uint16_t  severity;          // SIGMA_LOG_DEBUG .. SIGMA_LOG_PANIC
    uint16_t  cpu_id;            // originating CPU
    uint64_t  timestamp_ns;      // CLOCK_MONOTONIC nanoseconds
    uint32_t  subsystem_hash;    // FNV-1a("kernel.net.wifi")
    uint32_t  _pad;
    char      message[256];      // NUL-terminated
    uint64_t  field_keys[SIGMA_LOG_FIELD_COUNT];    // hashed key names
    uint64_t  field_values[SIGMA_LOG_FIELD_COUNT];  // associated values
} sigma_log_entry_t;

_Static_assert(sizeof(sigma_log_entry_t) == 320 + 128,
               "sigma_log_entry_t size changed — update /dev/sigma-log ABI");

// ── Ring buffer state ─────────────────────────────────────────────────────────

static sigma_log_entry_t g_ring[JOURNAL_CAPACITY];
static atomic_uint g_write_head;   // next slot to write
static atomic_uint g_read_tail;    // oldest unread slot (consumer side)

// ── Monotonic clock (TSC-based or HPET fallback) ──────────────────────────────

static inline uint64_t sigma_clock_ns(void) {
    uint32_t lo, hi;
    __asm__ volatile("rdtscp" : "=a"(lo), "=d"(hi) :: "ecx");
    return ((uint64_t)hi << 32) | lo;  // raw TSC (calibrated to ns by sigma_tsc)
}

// ── FNV-1a subsystem hash ─────────────────────────────────────────────────────

static uint32_t fnv1a(const char *s) {
    uint32_t h = 2166136261u;
    while (*s) { h ^= (uint8_t)*s++; h *= 16777619u; }
    return h;
}

// ── Minimal snprintf substitute for freestanding kernel ──────────────────────

extern int sigma_ksnprintf(char *buf, size_t sz, const char *fmt, va_list ap);

// ── Write path ────────────────────────────────────────────────────────────────

void sigma_journal_log(uint16_t severity, const char *subsystem,
                       const char *fmt, ...) {
    uint32_t head = atomic_fetch_add(&g_write_head, 1u);
    uint32_t slot = head & JOURNAL_MASK;
    sigma_log_entry_t *e = &g_ring[slot];

    // Mark slot as being written (odd sequence)
    uint32_t seq = e->sequence + 1;
    atomic_store((atomic_uint *)&e->sequence, seq | 1u);
    atomic_thread_fence(memory_order_seq_cst);

    e->timestamp_ns    = sigma_clock_ns();
    e->severity        = severity;
    e->cpu_id          = (uint16_t)0;  // sigma_smp_current_cpu() if SMP ready
    e->subsystem_hash  = fnv1a(subsystem);
    e->_pad            = 0;

    va_list ap;
    va_start(ap, fmt);
    sigma_ksnprintf(e->message, sizeof(e->message), fmt, ap);
    va_end(ap);

    for (int i = 0; i < SIGMA_LOG_FIELD_COUNT; i++) {
        e->field_keys[i]   = 0;
        e->field_values[i] = 0;
    }

    // Mark slot as complete (even sequence)
    atomic_thread_fence(memory_order_seq_cst);
    atomic_store((atomic_uint *)&e->sequence, seq + 1);
}

void sigma_journal_log_fields(uint16_t severity, const char *subsystem,
                               const char *msg,
                               uint32_t nfields,
                               const char **keys, const uint64_t *values) {
    uint32_t head = atomic_fetch_add(&g_write_head, 1u);
    uint32_t slot = head & JOURNAL_MASK;
    sigma_log_entry_t *e = &g_ring[slot];

    uint32_t seq = e->sequence + 1;
    atomic_store((atomic_uint *)&e->sequence, seq | 1u);
    atomic_thread_fence(memory_order_seq_cst);

    e->timestamp_ns   = sigma_clock_ns();
    e->severity       = severity;
    e->cpu_id         = 0;
    e->subsystem_hash = fnv1a(subsystem);

    size_t msglen = strlen(msg);
    if (msglen >= sizeof(e->message)) msglen = sizeof(e->message) - 1;
    memcpy(e->message, msg, msglen);
    e->message[msglen] = '\0';

    uint32_t n = nfields < SIGMA_LOG_FIELD_COUNT ? nfields : SIGMA_LOG_FIELD_COUNT;
    for (uint32_t i = 0; i < n; i++) {
        e->field_keys[i]   = fnv1a(keys[i]);
        e->field_values[i] = values[i];
    }
    for (uint32_t i = n; i < SIGMA_LOG_FIELD_COUNT; i++) {
        e->field_keys[i] = e->field_values[i] = 0;
    }

    atomic_thread_fence(memory_order_seq_cst);
    atomic_store((atomic_uint *)&e->sequence, seq + 1);
}

// ── Read path (userland via /dev/sigma-log mmap) ──────────────────────────────
// Returns number of entries copied into @out.  Returns 0 if no new entries.

uint32_t sigma_journal_read(sigma_log_entry_t *out, uint32_t max,
                             uint32_t *cursor) {
    uint32_t head = atomic_load(&g_write_head);
    if (*cursor >= head) return 0;

    uint32_t count = 0;
    while (*cursor < head && count < max) {
        uint32_t slot = *cursor & JOURNAL_MASK;
        sigma_log_entry_t *e = &g_ring[slot];

        uint32_t seq1, seq2;
        do {
            seq1 = atomic_load((atomic_uint *)&e->sequence);
            if (seq1 & 1u) { /* still writing */ __asm__ volatile("pause"); continue; }
            memcpy(&out[count], e, sizeof(sigma_log_entry_t));
            seq2 = atomic_load((atomic_uint *)&e->sequence);
        } while (seq1 != seq2);

        (*cursor)++;
        count++;
    }
    return count;
}

// ── Ring buffer pointer for /dev/sigma-log ────────────────────────────────────

const sigma_log_entry_t *sigma_journal_ring(void) { return g_ring; }
uint32_t sigma_journal_capacity(void)              { return JOURNAL_CAPACITY; }
uint32_t sigma_journal_head(void) {
    return atomic_load(&g_write_head);
}
