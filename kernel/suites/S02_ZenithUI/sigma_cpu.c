#include "../../../include/sigma_cpu.h"

/* =========================================================================
 * SIGMA OS: GLOBAL DESCRIPTOR TABLE (GDT) KERNEL SHARD
 * Defines memory protection rings for Kernel Mode vs User Mode.
 * ========================================================================= */

// We define 5 standard tables: Null, Kernel Code, Kernel Data, User Code, User Data.
gdt_entry_t gdt_entries[5];
gdt_ptr_t   gdt_ptr;

static void gdt_set_gate(int32_t num, uint32_t base, uint32_t limit, uint8_t access, uint8_t gran) {
    gdt_entries[num].base_low    = (base & 0xFFFF);
    gdt_entries[num].base_middle = (base >> 16) & 0xFF;
    gdt_entries[num].base_high   = (base >> 24) & 0xFF;

    gdt_entries[num].limit_low   = (limit & 0xFFFF);
    gdt_entries[num].granularity = (limit >> 16) & 0x0F;

    gdt_entries[num].granularity |= gran & 0xF0;
    gdt_entries[num].access      = access;
}

void sigma_cpu_init_gdt() {
    gdt_ptr.limit = (sizeof(gdt_entry_t) * 5) - 1;
    gdt_ptr.base  = (uintptr_t)&gdt_entries;

    // 0: Null segment
    gdt_set_gate(0, 0, 0, 0, 0);
    // 1: Kernel Code segment (Ring 0)
    gdt_set_gate(1, 0, 0xFFFFFFFF, 0x9A, 0xCF);
    // 2: Kernel Data segment (Ring 0)
    gdt_set_gate(2, 0, 0xFFFFFFFF, 0x92, 0xCF);
    // 3: User Code segment (Ring 3)
    gdt_set_gate(3, 0, 0xFFFFFFFF, 0xFA, 0xCF);
    // 4: User Data segment (Ring 3)
    gdt_set_gate(4, 0, 0xFFFFFFFF, 0xF2, 0xCF);

    // Call ASM subroutine to flush the GDT into CPU registers
    gdt_flush((uintptr_t)&gdt_ptr);
}
