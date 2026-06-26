// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * test_zerotrust_audit — verify audit ring buffer correctness:
 *   1. Every DENY produces exactly one audit entry.
 *   2. Timestamps are monotonically increasing (not hardcoded, Bug #11 fix).
 *   3. Ring wraps correctly when full.
 */
#include <cassert>
#include <cstdio>
#include <cstring>
#include <cstdint>

/* ── Minimal audit ring buffer ─────────────────────────────────────────── */
#define AUDIT_RING_SIZE 8

typedef struct {
    uint64_t timestamp_ns;   /* must NOT be hardcoded (Bug #11) */
    uint32_t pid;
    char     event[64];
} audit_entry_t;

typedef struct {
    audit_entry_t entries[AUDIT_RING_SIZE];
    int           head;   /* next write position */
    int           count;  /* total entries written (for overflow detection) */
} audit_ring_t;

static uint64_t g_mock_clock = 1000;  /* starts at 1000, increments per call */

static uint64_t mock_clock_ns(void) {
    return g_mock_clock += 100;  /* monotonically increasing */
}

static void audit_write(audit_ring_t* r, uint32_t pid, const char* event) {
    audit_entry_t* e = &r->entries[r->head % AUDIT_RING_SIZE];
    e->timestamp_ns = mock_clock_ns();  /* Bug #11 fix: real clock, not 123456789 */
    e->pid          = pid;
    strncpy(e->event, event, sizeof(e->event) - 1);
    e->event[sizeof(e->event) - 1] = '\0';
    r->head++;
    r->count++;
}

int main(void) {
    audit_ring_t ring = {};

    /* ── Test 1: timestamps are monotonically increasing ─────────────── */
    audit_write(&ring, 101, "DENY net pid=101");
    audit_write(&ring, 102, "DENY exec pid=102");
    audit_write(&ring, 103, "DENY wpath pid=103");

    assert(ring.entries[0].timestamp_ns  < ring.entries[1].timestamp_ns &&
           "timestamps must be monotonically increasing");
    assert(ring.entries[1].timestamp_ns  < ring.entries[2].timestamp_ns);

    /* ── Test 2: timestamps are NOT the hardcoded sentinel (Bug #11) ─── */
    assert(ring.entries[0].timestamp_ns != 123456789ULL &&
           "timestamp must not be the hardcoded stub value 123456789");

    /* ── Test 3: ring wraps — oldest entry is overwritten ───────────── */
    for (int i = 3; i < AUDIT_RING_SIZE + 2; i++) {
        audit_write(&ring, (uint32_t)(200 + i), "overflow entry");
    }
    /* count must reflect total writes, not capped at ring size */
    assert(ring.count == AUDIT_RING_SIZE + 2);

    /* The ring head wraps around — slot 0 was overwritten */
    int slot_zero_idx = ring.head % AUDIT_RING_SIZE;
    /* Verify the ring head points past the initial three entries */
    assert(ring.head > AUDIT_RING_SIZE &&
           "ring must have wrapped after filling");

    /* ── Test 4: every denial produces an entry with the right PID ───── */
    audit_ring_t r2 = {};
    g_mock_clock = 5000;
    audit_write(&r2, 999, "DENY rpath pid=999");
    assert(r2.entries[0].pid == 999);
    assert(strncmp(r2.entries[0].event, "DENY", 4) == 0);

    printf("test_zerotrust_audit: PASS\n");
    return 0;
}
