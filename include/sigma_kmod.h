/* SPDX-License-Identifier: MIT */
/*
 * =========================================================================
 * Σ SIGMAOS: KERNEL MODULE & DKMS INTERFACE (S-KMOD)
 * =========================================================================
 * Kernel module loading, symbol export, DKMS auto-compilation, and
 * Linux (insmod/modprobe) & BSD (kldload/kldstat) parity interface.
 * =========================================================================
 */

#ifndef SIGMA_KMOD_H
#define SIGMA_KMOD_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Kernel Module Taint Flags --- */
#define SIGMA_TAINT_PROPRIETARY_MODULE  (1U << 0)
#define SIGMA_TAINT_FORCED_MODULE       (1U << 1)
#define SIGMA_TAINT_UNSAFE_SMP          (1U << 2)
#define SIGMA_TAINT_CRAP                (1U << 3)
#define SIGMA_TAINT_OOT_MODULE          (1U << 4)
#define SIGMA_TAINT_UNSIGNED_MODULE     (1U << 5)

/* --- Module State --- */
typedef enum {
    SIGMA_MODULE_STATE_LIVE = 0,
    SIGMA_MODULE_STATE_COMING = 1,
    SIGMA_MODULE_STATE_GOING = 2,
    SIGMA_MODULE_STATE_UNFORMED = 3,
} sigma_module_state_t;

/* --- Kernel Export Symbol Table Entry --- */
struct sigma_kernel_symbol {
    sigma_uintptr_t value;
    const char     *name;
    const char     *namespace_name;
};

/* --- Kernel Module Structure --- */
struct sigma_module {
    sigma_module_state_t state;
    char                 name[64];
    sigma_u32            refcnt;
    sigma_uintptr_t      core_layout_base;
    sigma_size_t         core_layout_size;
    sigma_uintptr_t      init_layout_base;
    sigma_size_t         init_layout_size;
    sigma_u32            taint_flags;
    int (*init)(void);
    void (*exit)(void);
};

/* --- Helper Macros for Kernel Drivers --- */
#define SIGMA_EXPORT_SYMBOL(sym) \
    static const struct sigma_kernel_symbol __ksym_##sym \
    __attribute__((section("___ksymtab+" #sym), used)) = { (sigma_uintptr_t)&sym, #sym, "" }

#define SIGMA_MODULE_INIT(fn) \
    int (*__sigma_init_fn)(void) __attribute__((section(".initcall"))) = fn

#define SIGMA_MODULE_EXIT(fn) \
    void (*__sigma_exit_fn)(void) __attribute__((section(".exitcall"))) = fn

#define SIGMA_MODULE_LICENSE(lic) \
    static const char __module_license[] __attribute__((section(".modinfo"), used)) = "license=" lic

#define SIGMA_MODULE_AUTHOR(author) \
    static const char __module_author[] __attribute__((section(".modinfo"), used)) = "author=" author

#define SIGMA_MODULE_DESCRIPTION(desc) \
    static const char __module_desc[] __attribute__((section(".modinfo"), used)) = "description=" desc

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_KMOD_H */
