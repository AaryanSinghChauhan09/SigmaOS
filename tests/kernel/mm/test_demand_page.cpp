// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * test_demand_page — lazy page allocation
 *
 * A virtual mapping is created but no physical frame is assigned yet.
 * The first access triggers a "page fault" → frame is allocated.
 * Subsequent accesses use the same frame (no re-allocation).
 */
#include <cassert>
#include <cstdio>
#include <cstring>
#include <cstdint>

#define PAGE_SIZE  4096
#define MAX_FRAMES 16

typedef struct {
    uint8_t data[PAGE_SIZE];
    bool    allocated;
} frame_t;

static frame_t frame_pool[MAX_FRAMES];
static int     next_frame = 0;

typedef struct {
    bool  present;    /* false = not yet faulted in */
    int   frame_idx;  /* valid only when present == true */
} pte_t;             /* page table entry */

/* Simulate a page fault handler: allocate on first access */
static void handle_page_fault(pte_t* pte) {
    assert(!pte->present && "fault only on non-present pages");
    assert(next_frame < MAX_FRAMES && "out of frames");
    frame_pool[next_frame].allocated = true;
    memset(frame_pool[next_frame].data, 0, PAGE_SIZE);
    pte->frame_idx = next_frame++;
    pte->present   = true;
}

int main(void) {
    pte_t page = { false, -1 };

    /* ── Test 1: page starts not-present ────────────────────────────── */
    assert(!page.present && "page must start non-present (lazy allocation)");

    /* ── Test 2: first access — triggers fault, frame allocated ─────── */
    handle_page_fault(&page);
    assert(page.present   && "page must be present after fault");
    assert(page.frame_idx == 0 && "first fault must allocate frame 0");

    /* ── Test 3: write to the frame and verify data persists ────────── */
    const char* msg = "demand paged";
    memcpy(frame_pool[page.frame_idx].data, msg, strlen(msg) + 1);
    assert(strcmp((char*)frame_pool[page.frame_idx].data, msg) == 0);

    /* ── Test 4: second access does NOT fault again ──────────────────── */
    int before = next_frame;
    if (!page.present) handle_page_fault(&page);  /* should NOT trigger */
    assert(next_frame == before && "no new frame on second access");
    assert(page.frame_idx == 0  && "frame index unchanged on re-access");

    /* ── Test 5: two distinct virtual pages get distinct frames ──────── */
    pte_t page2 = { false, -1 };
    handle_page_fault(&page2);
    assert(page2.frame_idx != page.frame_idx &&
           "two distinct virtual pages must map to distinct frames");

    printf("test_demand_page: PASS\n");
    return 0;
}
