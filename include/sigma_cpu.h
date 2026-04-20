#ifndef SIGMA_CPU_H
#define SIGMA_CPU_H

#include "suites/S01_Genesis/shards/sigma_types.h"

/* =========================================================================
 * SIGMA OS: CENTRAL PROCESSING UNIT ABSTRACTION SHARD (GDT/IDT)
 * ========================================================================= */

// Memory segment structure for GDT (Global Descriptor Table)
struct gdt_entry_struct {
    uint16_t limit_low;
    uint16_t base_low;
    uint8_t  base_middle;
    uint8_t  access;
    uint8_t  granularity;
    uint8_t  base_high;
} __attribute__((packed));

typedef struct gdt_entry_struct gdt_entry_t;

struct gdt_ptr_struct {
    uint16_t limit;
    uintptr_t base;
} __attribute__((packed));

typedef struct gdt_ptr_struct gdt_ptr_t;

void sigma_cpu_init_gdt();
extern void gdt_flush(uintptr_t);

#endif
