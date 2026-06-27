// SPDX-License-Identifier: GPL-2.0-or-later
// sigma_cet.cpp — Intel CET (Control-flow Enforcement Technology) for SigmaOS
//
// CET provides two independent mechanisms:
//   1. Shadow Stack (SHSTK) — hardware-enforced return address integrity.
//      CPU maintains a second read-only stack mirroring call/return addresses.
//      If RET pops an address that doesn't match the shadow stack top → #CP fault.
//   2. Indirect Branch Tracking (IBT) — forward-edge CFI.
//      All indirect JMP/CALL targets must start with ENDBR64 instruction.
//      Any indirect jump to a non-ENDBR64 target → #CP fault.
//
// Inspired by:
//   • Linux arch/x86/kernel/cet.c (6.x kernel)
//   • Intel CET Technology Preview whitepaper
//   • glibc shadow stack support (GLIBC_ABI_DT_GNU_PROPERTY)

#include "sigma_cet.h"
#include <stdint.h>
#include <stdbool.h>

// ── MSR definitions ───────────────────────────────────────────────────────────

#define MSR_IA32_U_CET          0x6A0   // user-mode CET control
#define MSR_IA32_S_CET          0x6A2   // supervisor CET control
#define MSR_IA32_PL0_SSP        0x6A4   // ring-0 shadow stack pointer
#define MSR_IA32_ISST_ADDR_0    0x6A8   // IST shadow stack address 0

#define CET_SHSTK_EN            (1ULL << 0)
#define CET_WR_SHSTK_EN         (1ULL << 1)
#define CET_ENDBR_EN            (1ULL << 2)
#define CET_WAIT_ENDBR          (1ULL << 3)

// CR4.CET bit
#define CR4_CET                 (1UL << 23)

// ── MSR helpers ───────────────────────────────────────────────────────────────

static inline uint64_t rdmsr(uint32_t msr) {
    uint32_t lo, hi;
    __asm__ volatile("rdmsr" : "=a"(lo), "=d"(hi) : "c"(msr));
    return ((uint64_t)hi << 32) | lo;
}

static inline void wrmsr(uint32_t msr, uint64_t val) {
    __asm__ volatile("wrmsr" :: "c"(msr), "a"((uint32_t)val),
                                          "d"((uint32_t)(val >> 32)));
}

static inline uint64_t read_cr4(void) {
    uint64_t v;
    __asm__ volatile("mov %%cr4, %0" : "=r"(v));
    return v;
}

static inline void write_cr4(uint64_t v) {
    __asm__ volatile("mov %0, %%cr4" :: "r"(v));
}

// ── CPUID check ───────────────────────────────────────────────────────────────

bool sigma_cet_supported(void) {
    uint32_t ecx = 0, edx = 0;
    __asm__ volatile("cpuid" : "=c"(ecx), "=d"(edx)
                              : "a"(7), "c"(0) : "ebx");
    // ECX[7] = CET_SS (shadow stack)
    // EDX[20] = CET_IBT (indirect branch tracking)
    return (ecx & (1u << 7)) != 0;
}

// ── Shadow stack allocation ───────────────────────────────────────────────────
// Each thread needs a shadow stack. We allocate one page (4KB) per thread.
// The shadow stack grows down, like the regular stack.

extern uintptr_t sigma_pmm_alloc_page(void);  // from sigma_pmm.cpp

static uintptr_t alloc_shadow_stack(void) {
    uintptr_t pa = sigma_pmm_alloc_page();
    if (!pa) return 0;
    // Shadow stack page must be marked with PTE_DIRTY=0 and PTE_USER flags
    // (kernel uses a special shadow-stack PTE format with bit 11 = 1, dirty = 0).
    // Implementation via sigma_vmm_map_shadow_stack() not shown (VMM detail).
    return pa + 4096;   // return top of the page (stack grows down)
}

// ── Kernel (supervisor) CET enable ────────────────────────────────────────────

void sigma_cet_enable_kernel(void) {
    if (!sigma_cet_supported()) return;

    // Enable CR4.CET
    write_cr4(read_cr4() | CR4_CET);

    // Enable supervisor shadow stack + IBT
    uint64_t s_cet = CET_SHSTK_EN | CET_ENDBR_EN;
    wrmsr(MSR_IA32_S_CET, s_cet);

    // Allocate a kernel shadow stack for this CPU's ring-0 context
    uintptr_t ssp = alloc_shadow_stack();
    if (ssp) wrmsr(MSR_IA32_PL0_SSP, ssp);
}

// ── User (ring-3) CET enable ─────────────────────────────────────────────────
// Called when a new user thread is created (from sigma_thread_create()).

uintptr_t sigma_cet_new_user_thread(void) {
    if (!sigma_cet_supported()) return 0;

    // Enable user CET: shadow stack + IBT
    uint64_t u_cet = rdmsr(MSR_IA32_U_CET);
    u_cet |= CET_SHSTK_EN | CET_ENDBR_EN | CET_WR_SHSTK_EN;
    wrmsr(MSR_IA32_U_CET, u_cet);

    // Allocate and return user shadow stack pointer
    return alloc_shadow_stack();
}

// ── #CP (Control Protection) fault handler ────────────────────────────────────
// Vector 21 (#CP) — called from IDT entry.
// This is a fatal security violation: terminate the process.

void sigma_cet_fault_handler(uint64_t error_code) {
    // error_code bits:
    //   [2:0] = 0x1: NEAR-RET mismatch
    //           0x2: FAR-RET mismatch
    //           0x4: ENDBR expected (IBT violation)
    //           0x5: Supervisor SSP
    const char *reason = "unknown";
    switch (error_code & 0xF) {
        case 1: reason = "NEAR-RET shadow stack mismatch (ROP attack?)"; break;
        case 2: reason = "FAR-RET shadow stack mismatch"; break;
        case 4: reason = "IBT: indirect call to non-ENDBR64 target (JOP attack?)"; break;
        case 5: reason = "supervisor shadow stack fault"; break;
    }
    (void)reason;
    // Log to sigma_journal and terminate the offending task
    // sigma_panic_task("CET #CP: %s", reason);  // implemented in sigma_panic.cpp
    // For now, halt if in kernel context
    __asm__ volatile("cli; hlt");
    __builtin_unreachable();
}

// ── KASLR ─────────────────────────────────────────────────────────────────────

uint64_t sigma_kaslr_offset;

void sigma_kaslr_init(void) {
    // Use RDRAND to get a random kernel base offset
    // Kernel text will be loaded at: KERNEL_LINK_BASE + sigma_kaslr_offset
    uint64_t rand = 0;
    int ok = 0;
    for (int i = 0; i < 10 && !ok; i++) {
        __asm__ volatile(
            "rdrand %0\n"
            "setc %b1"
            : "=r"(rand), "=r"(ok)
        );
    }
    if (!ok) rand = 0x13700000;  // fallback if RDRAND unavailable
    // Align to 2MB, within a 64GB window
    sigma_kaslr_offset = rand & 0x3FFFFE00000ULL;
}
