#include "../../../../include/sigma_hal.h"

static void x86_cpu_halt(void) {
    sigma_printf("[HAL: x86] Executing CPU halt...\n");
}

static void x86_timer_init(void) {
    sigma_printf("[HAL: x86] Initializing PIT/APIC timer...\n");
}

static void x86_interrupt_init(void) {
    sigma_printf("[HAL: x86] Initializing IDT & PIC/APIC controllers...\n");
}

static void x86_mmu_map(sigma_u64 va, sigma_u64 pa, sigma_u64 flags) {
    sigma_printf("[HAL: x86] Mapping VA 0x%llx to PA 0x%llx (flags: 0x%llx)\n", va, pa, flags);
}

static const hal_ops_t x86_hal_ops = {
    x86_cpu_halt,
    x86_timer_init,
    x86_interrupt_init,
    x86_mmu_map
};

const hal_ops_t *hal_ops = &x86_hal_ops;

extern "C" void hal_init(void) {
    sigma_printf("[HAL] hal_init called. Assigning x86 HAL ops.\n");
    hal_ops = &x86_hal_ops;
}
