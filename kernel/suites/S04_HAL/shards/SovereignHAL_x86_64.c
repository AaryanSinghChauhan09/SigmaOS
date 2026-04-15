/*
 * =========================================================================
 * S SIGMAOS: S04_HAL — SovereignHAL_x86_64.c
 * =========================================================================
 * Implementation of Idea 131 (Apex Infinity): Hardware Abstraction Layer.
 * Direct silicon interaction for x86_64 architecture.
 * =========================================================================
 */

#include "suites/S01_Genesis/shards/sigma_base.h"
#include "suites/S01_Genesis/shards/sigma_types.h"

/* Port I/O primitives */
static inline void outb(uint16_t port, uint8_t val) {
    __asm__ volatile ( "outb %b0, %w1" : : "a"(val), "Nd"(port) : "memory");
}

static inline uint8_t inb(uint16_t port) {
    uint8_t ret;
    __asm__ volatile ( "inb %w1, %b0" : "=a"(ret) : "Nd"(port) : "memory");
    return ret;
}

/* CPUID Abstraction */
void hal_get_cpuid(uint32_t leaf, uint32_t* eax, uint32_t* ebx, uint32_t* ecx, uint32_t* edx) {
    __asm__ volatile (
        "cpuid"
        : "=a"(*eax), "=b"(*ebx), "=c"(*ecx), "=d"(*edx)
        : "a"(leaf)
    );
}

/* MSR Interaction */
uint64_t hal_rdmsr(uint32_t msr) {
    uint32_t low, high;
    __asm__ volatile ("rdmsr" : "=a"(low), "=d"(high) : "c"(msr));
    return ((uint64_t)high << 32) | low;
}

void hal_wrmsr(uint32_t msr, uint64_t val) {
    uint32_t low = (uint32_t)val;
    uint32_t high = (uint32_t)(val >> 32);
    __asm__ volatile ("wrmsr" : : "a"(low), "d"(high), "c"(msr));
}

void hal_init(void) {
    uint32_t eax, ebx, ecx, edx;
    hal_get_cpuid(0, &eax, &ebx, &ecx, &edx);
    sigma_printf("S [S04]: x86_64 HAL Materialized. CPU: %4.4s%4.4s%4.4s\n", 
                 (char*)&ebx, (char*)&edx, (char*)&ecx);
}
