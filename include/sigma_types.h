/*
 * SPDX-License-Identifier: MIT
 * SPDX-FileCopyrightText: 2026 SigmaSovereign Singh SigmaSovereign
 *
 * sigmaos/include/sigma_types.h
 * ------------------------------------------------------------------
 * Fundamental type aliases and constants used throughout the kernel.
 *
 * Design:
 *   - All public identifiers use the 'sigma_' prefix to avoid
 *     collisions with the host libc during hosted builds.
 *   - Signed/unsigned sizes precisely named (u8/i8, u16/i16, …).
 *   - SIGMA_NULL avoids implicit conversions from integer 0.
 * ------------------------------------------------------------------
 */

#ifndef SIGMAOS_SIGMA_TYPES_H
#define SIGMAOS_SIGMA_TYPES_H

/* Architecture-natural word sizes (pointer-compatible). */
typedef unsigned long long sigma_uptr;   /* holds a pointer as integer    */
typedef signed   long long sigma_iptr;   /* signed pointer-size integer   */
typedef unsigned long long sigma_size_t; /* size of an object in bytes    */
typedef signed   long long sigma_ssize_t;/* signed size (for error codes) */

/* Boolean */
typedef unsigned char      sigma_bool;
#define SIGMA_TRUE  ((sigma_bool)1)
#define SIGMA_FALSE ((sigma_bool)0)

/* Error code type — negative values indicate errors. */
typedef signed int         sigma_err_t;

/* Sized integer types — matches C99 <stdint.h> semantics. */
typedef unsigned char      sigma_u8;
typedef signed   char      sigma_i8;
typedef unsigned short     sigma_u16;
typedef signed   short     sigma_i16;
typedef unsigned int       sigma_u32;
typedef signed   int       sigma_i32;
typedef unsigned long long sigma_u64;
typedef signed   long long sigma_i64;

/* Industrial Standard Aliases (Optional Host Compatibility) */
#ifndef SIGMA_EXCLUDE_STD_ALIASES
#ifndef _STDINT_H
#ifndef _STDINT_H_
typedef sigma_u8           uint8_t;
typedef sigma_i8           int8_t;
typedef sigma_u16          uint16_t;
typedef sigma_i16          int16_t;
typedef sigma_u32          uint32_t;
typedef sigma_i32          int32_t;
typedef sigma_u64          uint64_t;
typedef sigma_i64          int64_t;
#endif
#endif

/* Standard Boolean Aliases */
#ifndef _STDBOOL_H
typedef sigma_bool         bool;
#define true               SIGMA_TRUE
#define false              SIGMA_FALSE
#endif

/* Standard Size Aliases */
#ifndef _SIZE_T_DEFINED
typedef sigma_size_t       size_t;      /* industrial standard alias     */
#endif

#endif /* SIGMA_EXCLUDE_STD_ALIASES */

/* Null pointer. */
#define SIGMA_NULL ((void*)0)

/* General status codes. */
#define SIGMA_ERROR ((sigma_err_t)-1)

/* Compile-time array size helper. */
#define SIGMA_ARRAY_SIZE(arr) (sizeof(arr) / sizeof((arr)[0]))

/* Mark a value unused to silence -Wunused-parameter. */
#define SIGMA_UNUSED(x) ((void)(x))

/* Branch-prediction hints. */
#define SIGMA_LIKELY(x)   __builtin_expect(!!(x), 1)
#define SIGMA_UNLIKELY(x) __builtin_expect(!!(x), 0)

/* Alignment helper. */
#define SIGMA_ALIGN(n) __attribute__((aligned(n)))

/* Pack a struct to remove padding. */
#define SIGMA_PACKED __attribute__((packed))

/* Mark a function as not returning (e.g. halt, panic). */
#define SIGMA_NORETURN __attribute__((noreturn))

/* Floating Point Primitives */
typedef float  sigma_f32;
typedef double sigma_f64;

/* Physical / virtual address typedefs — for clarity in mm/. */
typedef sigma_u64 phys_addr_t;
typedef sigma_u64 virt_addr_t;

/* Process identifier */
typedef sigma_i32 pid_t;

/* Common error codes (negative, like Linux <errno.h>). */
#define SIGMA_OK      ((sigma_err_t)  0)
#define SIGMA_EPERM   ((sigma_err_t) -1)  /* Operation not permitted */
#define SIGMA_ENOENT  ((sigma_err_t) -2)  /* No such file or directory */
#define SIGMA_ESRCH   ((sigma_err_t) -3)  /* No such process */
#define SIGMA_EINTR   ((sigma_err_t) -4)  /* Interrupted system call */
#define SIGMA_EIO     ((sigma_err_t) -5)  /* I/O error */
#define SIGMA_ENOMEM  ((sigma_err_t) -12) /* Out of memory */
#define SIGMA_EACCES  ((sigma_err_t) -13) /* Permission denied */
#define SIGMA_EBUSY   ((sigma_err_t) -16) /* Resource busy */
#define SIGMA_EINVAL  ((sigma_err_t) -22) /* Invalid argument */
#define SIGMA_ENOSPC  ((sigma_err_t) -28) /* No space left on device */
#define SIGMA_ENOSYS  ((sigma_err_t) -38) /* Function not implemented */

/** Returns true if @err indicates success (== SIGMA_OK). */
static inline sigma_bool sigma_ok(sigma_err_t err) {
    return err == SIGMA_OK;
}

/** Returns true if @err indicates failure (< 0). */
static inline sigma_bool sigma_err(sigma_err_t err) {
    return err < 0;
}

/* Sovereign stdarg parity (Compiler Builtins) */
typedef __builtin_va_list  sigma_va_list;
#define sigma_va_start(ap, last) __builtin_va_start(ap, last)
#define sigma_va_arg(ap, type)   __builtin_va_arg(ap, type)
#define sigma_va_end(ap)         __builtin_va_end(ap)

#endif /* SIGMAOS_SIGMA_TYPES_H */
