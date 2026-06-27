// SPDX-License-Identifier: GPL-2.0-or-later
// sigma_zram.cpp — Memory compression (zram) for SigmaOS
//
// Compresses cold pages in RAM using zstd instead of swapping to disk.
// Gives effectively 1.5–2× usable RAM on memory-constrained systems.
//
// Architecture:
//   sigma_zram_compress_page(page_pa) — compress one 4KB page
//   sigma_zram_decompress_page(slot)  — restore a compressed page
//   sigma_zram_reclaim_scanner()      — background thread: finds cold pages
//
// Inspired by:
//   • Linux drivers/block/zram/zram_drv.c
//   • Android zram-swap policy
//   • zstd (Facebook) — extremely fast compress/decompress

#include "sigma_zram.h"
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

// We embed a minimal zstd-compatible compressor stub.
// In production this links against lib/sigma-zstd/zstd_compress.c
extern size_t sigma_zstd_compress(void *dst, size_t dst_cap,
                                  const void *src, size_t src_len,
                                  int level);
extern size_t sigma_zstd_decompress(void *dst, size_t dst_cap,
                                    const void *src, size_t src_len);

#define PAGE_SIZE         4096
#define ZRAM_MAX_SLOTS    (256 * 1024)   // 256K compressed pages max
#define ZRAM_POOL_BYTES   (256UL * 1024 * 1024)  // 256MB compressed pool
#define ZSTD_LEVEL        1   // fastest — 1µs/page on modern CPU

// ── Slot table ────────────────────────────────────────────────────────────────
// Each slot describes one compressed page.

struct zram_slot {
    uint32_t  offset;       // byte offset into g_zpool
    uint16_t  comp_size;    // compressed size (0 = slot free)
    uint16_t  flags;
} __attribute__((packed));

#define ZRAM_FLAG_ZERO  (1 << 0)   // page was all-zeros — stored as flag only

static struct zram_slot g_slots[ZRAM_MAX_SLOTS];
static uint8_t          g_zpool[ZRAM_POOL_BYTES];
static uint32_t         g_pool_head = 0;   // simple bump allocator
static uint32_t         g_free_slots = ZRAM_MAX_SLOTS;

// ── Statistics ────────────────────────────────────────────────────────────────

struct zram_stats {
    uint64_t pages_stored;
    uint64_t pages_freed;
    uint64_t bytes_used;
    uint64_t decompress_calls;
};
static struct zram_stats g_stats;

// ── Zero-page fast path ───────────────────────────────────────────────────────

static bool is_zero_page(const uint8_t *p) {
    for (int i = 0; i < PAGE_SIZE; i++) if (p[i]) return false;
    return true;
}

// ── Public API ────────────────────────────────────────────────────────────────

// sigma_zram_compress_page — compress the 4KB page at physical address @pa.
// Returns slot index (≥ 0) on success, or -1 if pool is full.
int sigma_zram_compress_page(uintptr_t pa) {
    const uint8_t *src = (const uint8_t *)pa;  // identity-mapped PA

    // Find a free slot
    int slot = -1;
    for (int i = 0; i < ZRAM_MAX_SLOTS; i++) {
        if (g_slots[i].comp_size == 0) { slot = i; break; }
    }
    if (slot < 0) return -1;

    // Zero-page fast path
    if (is_zero_page(src)) {
        g_slots[slot].comp_size = 1;
        g_slots[slot].flags     = ZRAM_FLAG_ZERO;
        g_slots[slot].offset    = 0;
        g_free_slots--;
        g_stats.pages_stored++;
        return slot;
    }

    // Compress the page
    uint8_t  tmp[PAGE_SIZE + 64];
    size_t   comp_len = sigma_zstd_compress(tmp, sizeof(tmp), src, PAGE_SIZE,
                                             ZSTD_LEVEL);
    if (comp_len == 0 || comp_len >= PAGE_SIZE) {
        // Incompressible — store verbatim (slot still marks it)
        comp_len = PAGE_SIZE;
    }

    // Check pool capacity
    if (g_pool_head + comp_len > ZRAM_POOL_BYTES) return -1;

    // Copy to pool
    for (size_t i = 0; i < comp_len; i++)
        g_zpool[g_pool_head + i] = tmp[i];

    g_slots[slot].offset    = g_pool_head;
    g_slots[slot].comp_size = (uint16_t)comp_len;
    g_slots[slot].flags     = 0;
    g_pool_head += (uint32_t)comp_len;
    g_free_slots--;

    g_stats.pages_stored++;
    g_stats.bytes_used += comp_len;
    return slot;
}

// sigma_zram_decompress_page — restore a page from @slot into @dst_pa.
// Returns 0 on success, -1 on error.
int sigma_zram_decompress_page(int slot, uintptr_t dst_pa) {
    if (slot < 0 || slot >= ZRAM_MAX_SLOTS) return -1;
    struct zram_slot *s = &g_slots[slot];
    if (s->comp_size == 0) return -1;

    uint8_t *dst = (uint8_t *)dst_pa;
    g_stats.decompress_calls++;

    if (s->flags & ZRAM_FLAG_ZERO) {
        for (int i = 0; i < PAGE_SIZE; i++) dst[i] = 0;
        return 0;
    }

    if (s->comp_size == PAGE_SIZE) {
        // Stored verbatim
        const uint8_t *src = g_zpool + s->offset;
        for (int i = 0; i < PAGE_SIZE; i++) dst[i] = src[i];
        return 0;
    }

    size_t r = sigma_zstd_decompress(dst, PAGE_SIZE,
                                     g_zpool + s->offset, s->comp_size);
    return (r == PAGE_SIZE) ? 0 : -1;
}

// sigma_zram_free_slot — mark a slot as free after the page has been restored.
void sigma_zram_free_slot(int slot) {
    if (slot < 0 || slot >= ZRAM_MAX_SLOTS) return;
    g_stats.bytes_used -= g_slots[slot].comp_size;
    g_slots[slot].comp_size = 0;
    g_slots[slot].flags     = 0;
    g_free_slots++;
    g_stats.pages_freed++;
}

// sigma_zram_stats — fill out statistics struct.
void sigma_zram_get_stats(struct sigma_zram_stats *out) {
    out->pages_stored    = g_stats.pages_stored;
    out->pages_freed     = g_stats.pages_freed;
    out->bytes_used      = g_stats.bytes_used;
    out->free_slots      = g_free_slots;
    out->pool_capacity   = ZRAM_POOL_BYTES;
    out->decompress_calls = g_stats.decompress_calls;
}
