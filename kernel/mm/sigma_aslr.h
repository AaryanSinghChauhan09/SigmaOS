// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_aslr.h — Address Space Layout Randomization (HardenedBSD-inspired)
 *
 * 42-bit entropy on x86_64; separate per-region randomisation; W^X enforcement.
 * Expose via sysctl: security.aslr.enabled / security.aslr.entropy_bits
 */
#include <sigma_kernel_types.h>

typedef struct {
    uintptr_t stack_base;
    uintptr_t heap_base;
    uintptr_t mmap_base;
    uintptr_t vdso_base;
    sigma_u8  entropy_bits;   /* 42 on x86_64, 16 on 32-bit */
} sigma_aslr_layout_t;

/* Called on every exec() — fills layout with CSPRNG-derived addresses */
int  sigma_aslr_generate_layout(sigma_aslr_layout_t* layout);

/* W^X check — returns -EPERM if PROT_WRITE|PROT_EXEC are both set */
int  sigma_mm_check_wx(sigma_u32 prot_flags);

void sigma_aslr_init(void);
