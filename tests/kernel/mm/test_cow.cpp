// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * test_cow — copy-on-write page semantics
 *
 * After a fork()-like copy:
 *   - Parent and child initially share the same physical page (refcount == 2).
 *   - Writing in the child triggers a page fault → a new physical page is
 *     allocated for the child; parent's page is unchanged.
 *   - After the write, refcount on the original page drops back to 1.
 */
#include <cassert>
#include <cstdio>
#include <cstring>
#include <cstdint>

/* ── Minimal PMM / CoW simulation ─────────────────────────────────────── */
#define PAGE_SIZE   4096
#define MAX_FRAMES  16

typedef struct {
    uint8_t  data[PAGE_SIZE];
    int      refcount;
    bool     dirty;
} phys_frame_t;

static phys_frame_t frames[MAX_FRAMES];
static int          frame_alloc_cursor = 0;

static int alloc_frame(void) {
    assert(frame_alloc_cursor < MAX_FRAMES && "out of physical frames");
    int idx = frame_alloc_cursor++;
    frames[idx].refcount = 1;
    frames[idx].dirty    = false;
    memset(frames[idx].data, 0, PAGE_SIZE);
    return idx;
}

/* Simulate a virtual → physical mapping per process */
typedef struct { int phys_frame; } vma_t;

/* CoW write: if refcount > 1, allocate a new frame and copy */
static int cow_write(vma_t* vma, const uint8_t* data, size_t len) {
    phys_frame_t* f = &frames[vma->phys_frame];
    if (f->refcount > 1) {
        /* Page fault path: copy the frame */
        int new_frame = alloc_frame();
        memcpy(frames[new_frame].data, f->data, PAGE_SIZE);
        f->refcount--;          /* release share from original */
        vma->phys_frame = new_frame;
        f = &frames[new_frame];
    }
    memcpy(f->data, data, len < PAGE_SIZE ? len : PAGE_SIZE);
    f->dirty = true;
    return 0;
}

int main(void) {
    /* ── Test 1: allocate one frame, "fork" by sharing it ───────────── */
    int pf = alloc_frame();
    const char* msg = "parent data";
    memcpy(frames[pf].data, msg, strlen(msg) + 1);

    vma_t parent = { pf };
    vma_t child  = { pf };          /* initially share the same frame   */
    frames[pf].refcount = 2;

    assert(frames[pf].refcount == 2 && "shared frame must have refcount 2");
    assert(parent.phys_frame == child.phys_frame && "must share same frame");

    /* ── Test 2: child writes — CoW allocates a new frame ───────────── */
    const char* child_msg = "child write";
    cow_write(&child, (const uint8_t*)child_msg, strlen(child_msg) + 1);

    assert(child.phys_frame != parent.phys_frame &&
           "child must get a new frame after CoW write");
    assert(frames[parent.phys_frame].refcount == 1 &&
           "original frame refcount must drop to 1");

    /* ── Test 3: parent data is unchanged ───────────────────────────── */
    assert(strcmp((char*)frames[parent.phys_frame].data, "parent data") == 0 &&
           "parent data must be unchanged after child CoW write");

    /* ── Test 4: child data reflects the write ──────────────────────── */
    assert(strcmp((char*)frames[child.phys_frame].data, "child write") == 0 &&
           "child must see its own write");

    printf("test_cow: PASS\n");
    return 0;
}
