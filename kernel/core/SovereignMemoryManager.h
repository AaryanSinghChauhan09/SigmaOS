#ifndef SOVEREIGN_MEMORY_MANAGER_H
#define SOVEREIGN_MEMORY_MANAGER_H

#include <stdint.h>

/*
 * Σ SIGMAOS: SOVEREIGN VMM & PMM SUB-SYSTEMS
 * Implements Advanced Memory Management Overhaul as dictated by Zenith Specifications.
 *
 * Provides x86_64 Recursive Paging Tables & Buddy Allocator structures natively to replace bump allocators.
 */

// Example of a basic Page Table Entry structure for x86_64
typedef struct {
    uint64_t present    : 1;
    uint64_t writable   : 1;
    uint64_t user       : 1;
    uint64_t write_thru : 1;
    uint64_t cache_dis  : 1;
    uint64_t accessed   : 1;
    uint64_t dirty      : 1;
    uint64_t huge_page  : 1;
    uint64_t global     : 1;
    uint64_t available  : 3;
    uint64_t address    : 40;
    uint64_t reserved   : 11;
    uint64_t no_execute : 1;
} __attribute__((packed)) pt_entry_t;

// Buddy Allocator Block for PMM (Physical Memory Manager)
typedef struct buddy_block {
    struct buddy_block* next;
    struct buddy_block* prev;
    uint32_t size_order;
    uint32_t is_free;
} buddy_block_t;

// UBSan / KASan Minimal Hooks
void __ksan_check_bounds(void* ptr, uint64_t size);
void __ksan_report_violation(void* ptr);

#endif // SOVEREIGN_MEMORY_MANAGER_H
