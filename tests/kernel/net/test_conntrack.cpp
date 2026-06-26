// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * test_conntrack — verify the connection-tracking counter:
 *   - increments on new connection
 *   - decrements on close (Bug #19 fix verification)
 *   - rejects entries when CONNTRACK_MAX is reached
 */
#include <cassert>
#include <cstdio>
#include <cstring>

/* ── Minimal conntrack implementation for host-mode testing ─────────────── */
#define CONNTRACK_MAX 8

typedef struct {
    uint32_t src_ip;
    uint16_t src_port;
    uint32_t dst_ip;
    uint16_t dst_port;
    int      active;
} ct_entry_t;

static ct_entry_t ct_table[CONNTRACK_MAX];
static int        ct_count = 0;  /* MUST be decremented on close */

static int ct_add(uint32_t sip, uint16_t sp, uint32_t dip, uint16_t dp) {
    if (ct_count >= CONNTRACK_MAX) return -1;  /* table full */
    for (int i = 0; i < CONNTRACK_MAX; i++) {
        if (!ct_table[i].active) {
            ct_table[i] = { sip, sp, dip, dp, 1 };
            ct_count++;
            return i;
        }
    }
    return -1;
}

static void ct_remove(int idx) {
    if (idx < 0 || idx >= CONNTRACK_MAX) return;
    if (!ct_table[idx].active)           return;
    memset(&ct_table[idx], 0, sizeof(ct_table[idx]));
    ct_count--;  /* Bug #19 fix: this MUST happen */
}

int main(void) {
    /* ── Test 1: add 3 connections, counter == 3 ─────────────────────── */
    int i0 = ct_add(0x0A000001, 1234, 0x08080808, 443);
    int i1 = ct_add(0x0A000001, 1235, 0x08080808, 443);
    int i2 = ct_add(0x0A000001, 1236, 0x08080808, 80);
    assert(i0 >= 0 && i1 >= 0 && i2 >= 0);
    assert(ct_count == 3 && "counter must be 3 after 3 additions");

    /* ── Test 2: remove one — counter decrements (Bug #19 regression) ── */
    ct_remove(i1);
    assert(ct_count == 2 && "counter must decrement on removal");

    /* ── Test 3: fill to max, then reject ────────────────────────────── */
    while (ct_count < CONNTRACK_MAX) {
        ct_add(0x0A000002, (uint16_t)ct_count, 0x01020304, 8080);
    }
    assert(ct_count == CONNTRACK_MAX && "counter must equal CONNTRACK_MAX");

    int overflow = ct_add(0xFF000001, 9999, 0xDEADBEEF, 1337);
    assert(overflow == -1 && "must reject when table is full");
    assert(ct_count == CONNTRACK_MAX && "counter must not exceed max");

    /* ── Test 4: drain all — counter returns to 0 ────────────────────── */
    for (int i = 0; i < CONNTRACK_MAX; i++) ct_remove(i);
    assert(ct_count == 0 && "counter must be 0 after draining all entries");

    printf("test_conntrack: PASS\n");
    return 0;
}
