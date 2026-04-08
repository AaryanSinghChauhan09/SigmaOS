#ifndef SOVEREIGN_MEMORY_MANAGER_H
#define SOVEREIGN_MEMORY_MANAGER_H

#include "../../SovereignLibC.h"
#include "../../SigmaOOP.h"

/*
 * Σ SIGMAOS: SOVEREIGN VMM & PMM SUB-SYSTEMS
 * Implements Advanced Memory Management Overhaul as dictated by Zenith Specifications.
 *
 * Provides x86_64 Recursive Paging Tables & Buddy Allocator structures natively to replace bump allocators.
 */

// Example of a basic Page Table Entry structure for x86_64
typedef struct {
    sigma_u64 present    : 1;
    sigma_u64 writable   : 1;
    sigma_u64 user       : 1;
    sigma_u64 write_thru : 1;
    sigma_u64 cache_dis  : 1;
    sigma_u64 accessed   : 1;
    sigma_u64 dirty      : 1;
    sigma_u64 huge_page  : 1;
    sigma_u64 global     : 1;
    sigma_u64 available  : 3;
    sigma_u64 address    : 40;
    sigma_u64 reserved   : 11;
    sigma_u64 no_execute : 1;
} __attribute__((packed)) pt_entry_t;

// Buddy Allocator Block for PMM (Physical Memory Manager)
typedef struct buddy_block {
    struct buddy_block* next;
    struct buddy_block* prev;
    sigma_u32 size_order;
    sigma_u32 is_free;
} buddy_block_t;

// UBSan / KASan Minimal Hooks
void __ksan_check_bounds(void* ptr, sigma_u64 size);
void __ksan_report_violation(void* ptr);

#endif // SOVEREIGN_MEMORY_MANAGER_H
