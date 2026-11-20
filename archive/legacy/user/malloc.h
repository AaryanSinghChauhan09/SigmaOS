/* SigmaOS user heap — first-fit allocator with coalescing on SYS_BRK(47).
 * Block: [hdr{cap,free,next}][payload...]. All blocks on one list.
 * Static-linked per program = per-process heap (private address space). */
#ifndef SIGMA_MALLOC_H
#define SIGMA_MALLOC_H
#include "lib.h"

typedef struct mhdr { u32 cap; u32 is_free; struct mhdr* next; } mhdr_t;
static mhdr_t* g_heap = 0;

static u32 sys_brk2(u32 addr) {
    u32 r;
    __asm__ volatile("movl $47, %%eax\n\tmovl %1, %%ebx\n\tint $0x80\n\tmovl %%eax, %0"
                     : "=m"(r) : "m"(addr) : "eax", "ebx", "memory");
    return r;
}
static mhdr_t* heap_grow(u32 need) {
    u32 cur = sys_brk2(0);
    if (!cur || cur == 0xFFFFFFFFu) return 0;
    u32 total = (need + sizeof(mhdr_t) + 15u) & ~15u;
    mhdr_t* nb = (mhdr_t*)(u32)cur;
    if (sys_brk2(cur + total) == 0xFFFFFFFFu) return 0;
    nb->cap = total - sizeof(mhdr_t); nb->is_free = 1; nb->next = 0;
    if (!g_heap) { g_heap = nb; return nb; }
    mhdr_t* p = g_heap;
    while (p->next) p = p->next;
    p->next = nb;
    return nb;
}
static int heap_init(void) {
    if (g_heap) return 1;
    u32 b = sys_brk2(0);
    if (!b || b == 0xFFFFFFFFu) return 0;
    mhdr_t* h = (mhdr_t*)(u32)b;
    h->cap = 0; h->is_free = 0; h->next = 0;   /* sentinel at brk start */
    g_heap = h;
    return 1;
}
void* malloc(u32 n) {
    if (!n) return 0;
    n = (n + 15u) & ~15u;
    if (!heap_init()) return 0;
    mhdr_t* p = g_heap;
    while (p) {
        if (p->is_free && p->cap >= n) {
            if (p->cap >= n + sizeof(mhdr_t) + 16) {
                /* split: carve tail into a new free block */
                mhdr_t* nb = (mhdr_t*)((u32)(u32)p + sizeof(mhdr_t) + n);
                nb->cap = p->cap - n - sizeof(mhdr_t);
                nb->is_free = 1; nb->next = p->next;
                p->next = nb;
                p->cap = n;
            }
            p->is_free = 0;
            return (void*)((u32)(u32)p + sizeof(mhdr_t));
        }
        p = p->next;
    }
    mhdr_t* g = heap_grow(n);
    if (!g) return 0;
    if (g->cap >= n + sizeof(mhdr_t) + 16) {
        mhdr_t* nb = (mhdr_t*)((u32)(u32)g + sizeof(mhdr_t) + n);
        nb->cap = g->cap - n - sizeof(mhdr_t);
        nb->is_free = 1; nb->next = g->next;
        g->next = nb;
        g->cap = n;
    }
    g->is_free = 0;
    return (void*)((u32)(u32)g + sizeof(mhdr_t));
}
void free(void* ptr) {
    if (!ptr) return;
    mhdr_t* p = (mhdr_t*)((u32)(u32)ptr - sizeof(mhdr_t));
    if (!p->is_free) p->is_free = 1;
    /* coalesce forward */
    while (p->next && p->next->is_free) {
        p->cap += sizeof(mhdr_t) + p->next->cap;
        p->next = p->next->next;
    }
    /* coalesce backward */
    mhdr_t* q = g_heap;
    while (q && q->next != p) q = q->next;
    if (q && q->is_free) {
        q->cap += sizeof(mhdr_t) + p->cap;
        q->next = p->next;
    }
}
#endif
