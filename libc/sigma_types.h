/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN TYPE SYSTEM (sigma_types.h)
 * =========================================================================
 * USP Absorbed: musl libc (exact-width types), Fuchsia Zircon (type safety)
 * Principle: ZERO <stdint.h>, ZERO <stddef.h>, ZERO <stdbool.h> dependency.
 *            Pure custom type system for the SigmaOS universe.
 * OOP Principle: Polymorphic type aliasing, compile-time type safety.
 * Languages: C (pure), architecture-aware with inline ASM hooks.
 * =========================================================================
 */

#ifndef SIGMA_TYPES_H
#define SIGMA_TYPES_H

#ifdef __cplusplus
extern "C" {
#endif

/* -----------------------------------------------------------------------
 * ARCHITECTURE DETECTION (No compiler header deps)
 * ----------------------------------------------------------------------- */
#if defined(__x86_64__) || defined(_M_X64)
    #define SIGMA_ARCH_X86_64    1
    #define SIGMA_POINTER_BITS   64
    #define SIGMA_CACHELINE      64
#elif defined(__i386__) || defined(_M_IX86)
    #define SIGMA_ARCH_X86_32    1
    #define SIGMA_POINTER_BITS   32
    #define SIGMA_CACHELINE      64
#elif defined(__aarch64__)
    #define SIGMA_ARCH_ARM64     1
    #define SIGMA_POINTER_BITS   64
    #define SIGMA_CACHELINE      64
#elif defined(__arm__)
    #define SIGMA_ARCH_ARM32     1
    #define SIGMA_POINTER_BITS   32
    #define SIGMA_CACHELINE      32
#elif defined(__riscv) && (__riscv_xlen == 64)
    #define SIGMA_ARCH_RISCV64   1
    #define SIGMA_POINTER_BITS   64
    #define SIGMA_CACHELINE      64
#else
    #define SIGMA_ARCH_UNKNOWN   1
    #define SIGMA_POINTER_BITS   64
    #define SIGMA_CACHELINE      64
#endif

/* -----------------------------------------------------------------------
 * EXACT-WIDTH INTEGER TYPES (Replacing <stdint.h>)
 * ----------------------------------------------------------------------- */
typedef unsigned char       sigma_u8;
typedef signed char         sigma_i8;
typedef unsigned short      sigma_u16;
typedef signed short        sigma_i16;
typedef unsigned int        sigma_u32;
typedef signed int          sigma_i32;

#if defined(SIGMA_ARCH_X86_64) || defined(SIGMA_ARCH_ARM64) || defined(SIGMA_ARCH_RISCV64)
    typedef unsigned long long  sigma_u64;
    typedef signed long long    sigma_i64;
    typedef sigma_u64           sigma_usize;    /* size_t replacement */
    typedef sigma_i64           sigma_isize;    /* ptrdiff_t replacement */
    typedef sigma_u64           sigma_uptr;     /* uintptr_t replacement */
    typedef sigma_i64           sigma_sptr;     /* intptr_t replacement */
    #define SIGMA_USIZE_MAX     (0xFFFFFFFFFFFFFFFFULL)
    #define SIGMA_ISIZE_MAX     (0x7FFFFFFFFFFFFFFFLL)
    #define SIGMA_ISIZE_MIN     (-0x7FFFFFFFFFFFFFFFLL - 1)
#else
    typedef unsigned int        sigma_u64;      /* 32-bit fallback */
    typedef signed int          sigma_i64;
    typedef sigma_u32           sigma_usize;
    typedef sigma_i32           sigma_isize;
    typedef sigma_u32           sigma_uptr;
    typedef sigma_i32           sigma_sptr;
    #define SIGMA_USIZE_MAX     (0xFFFFFFFFU)
    #define SIGMA_ISIZE_MAX     (0x7FFFFFFF)
    #define SIGMA_ISIZE_MIN     (-0x7FFFFFFF - 1)
#endif

/* 128-bit type support (GCC/Clang extension, no stdlib needed) */
#if defined(__SIZEOF_INT128__)
    typedef unsigned __int128   sigma_u128;
    typedef signed __int128     sigma_i128;
    #define SIGMA_HAS_U128      1
#endif

/* -----------------------------------------------------------------------
 * BOOLEAN TYPE (Replacing <stdbool.h>)
 * ----------------------------------------------------------------------- */
typedef sigma_u8    sigma_bool;
#define SIGMA_TRUE  ((sigma_bool)1)
#define SIGMA_FALSE ((sigma_bool)0)

/* -----------------------------------------------------------------------
 * NULL POINTER (Replacing <stddef.h> NULL)
 * ----------------------------------------------------------------------- */
#ifndef SIGMA_NULL
    #ifdef __cplusplus
        #define SIGMA_NULL nullptr
    #else
        #define SIGMA_NULL ((void*)0)
    #endif
#endif

/* -----------------------------------------------------------------------
 * FLOATING POINT TYPES
 * ----------------------------------------------------------------------- */
typedef float       sigma_f32;
typedef double      sigma_f64;

/* -----------------------------------------------------------------------
 * RESULT / STATUS TYPES (OOP-inspired error handling)
 * ----------------------------------------------------------------------- */
typedef sigma_i32 sigma_status;

#define SIGMA_OK                ( 0)   /* Success                  */
#define SIGMA_ERR_GENERIC       (-1)   /* Generic error            */
#define SIGMA_ERR_NOMEM         (-2)   /* Out of memory            */
#define SIGMA_ERR_INVAL         (-3)   /* Invalid argument         */
#define SIGMA_ERR_PERM          (-4)   /* Permission denied        */
#define SIGMA_ERR_IO            (-5)   /* I/O error                */
#define SIGMA_ERR_BOUNDS        (-6)   /* Out of bounds access     */
#define SIGMA_ERR_OVERFLOW      (-7)   /* Integer overflow         */
#define SIGMA_ERR_NOTFOUND      (-8)   /* Resource not found       */
#define SIGMA_ERR_BUSY          (-9)   /* Resource busy            */
#define SIGMA_ERR_TIMEOUT       (-10)  /* Operation timed out      */
#define SIGMA_ERR_FAULT         (-11)  /* Segfault / bad address   */
#define SIGMA_ERR_UNSUPPORTED   (-12)  /* Feature not supported    */

/* -----------------------------------------------------------------------
 * ALIGNMENT MACROS (Replacing __attribute__ wrappers)
 * ----------------------------------------------------------------------- */
#if defined(__GNUC__) || defined(__clang__)
    #define SIGMA_ALIGN(n)          __attribute__((aligned(n)))
    #define SIGMA_PACKED            __attribute__((packed))
    #define SIGMA_NORETURN          __attribute__((noreturn))
    #define SIGMA_INLINE            __attribute__((always_inline)) inline
    #define SIGMA_NOINLINE          __attribute__((noinline))
    #define SIGMA_LIKELY(x)         __builtin_expect(!!(x), 1)
    #define SIGMA_UNLIKELY(x)       __builtin_expect(!!(x), 0)
    #define SIGMA_UNUSED            __attribute__((unused))
    #define SIGMA_PURE              __attribute__((pure))
    #define SIGMA_CONST             __attribute__((const))
    #define SIGMA_BARRIER()         __asm__ volatile("" ::: "memory")
    #define SIGMA_SECTION(s)        __attribute__((section(s)))
    #define SIGMA_PACKED_STRUCT     __attribute__((packed))
    #ifdef __cplusplus
        #define SIGMA_RESTRICT      __restrict
    #else
        #define SIGMA_RESTRICT      restrict
    #endif
#else
    #define SIGMA_ALIGN(n)
    #define SIGMA_PACKED
    #define SIGMA_NORETURN
    #define SIGMA_INLINE            inline
    #define SIGMA_NOINLINE
    #define SIGMA_LIKELY(x)         (x)
    #define SIGMA_UNLIKELY(x)       (x)
    #define SIGMA_UNUSED
    #define SIGMA_PURE
    #define SIGMA_CONST
    #define SIGMA_BARRIER()
    #define SIGMA_SECTION(s)
    #define SIGMA_PACKED_STRUCT
#endif

/* -----------------------------------------------------------------------
 * COMPILE-TIME ASSERTIONS (No <assert.h> needed)
 * ----------------------------------------------------------------------- */
#define SIGMA_STATIC_ASSERT(cond, msg) \
    typedef char sigma_static_assert_##msg[(cond) ? 1 : -1]

/* Validate our type sizes at compile time */
SIGMA_STATIC_ASSERT(sizeof(sigma_u8)  == 1, sigma_u8_must_be_1_byte);
SIGMA_STATIC_ASSERT(sizeof(sigma_u16) == 2, sigma_u16_must_be_2_bytes);
SIGMA_STATIC_ASSERT(sizeof(sigma_u32) == 4, sigma_u32_must_be_4_bytes);

/* -----------------------------------------------------------------------
 * BITFIELD UTILITIES
 * ----------------------------------------------------------------------- */
#define SIGMA_BIT(n)            (1ULL << (n))
#define SIGMA_MASK(n)           (SIGMA_BIT(n) - 1ULL)
#define SIGMA_BITMASK(hi, lo)   ((SIGMA_BIT((hi)-(lo)+1) - 1ULL) << (lo))
#define SIGMA_GETBIT(v, n)      (((v) >> (n)) & 1)
#define SIGMA_SETBIT(v, n)      ((v) | SIGMA_BIT(n))
#define SIGMA_CLRBIT(v, n)      ((v) & ~SIGMA_BIT(n))

/* -----------------------------------------------------------------------
 * CONTAINER/OFFSET MACROS (Replacing <stddef.h> offsetof)
 * ----------------------------------------------------------------------- */
#define SIGMA_OFFSETOF(type, member) \
    ((sigma_usize)((sigma_u8*)&((type*)0)->member - (sigma_u8*)0))

#define SIGMA_CONTAINER_OF(ptr, type, member) \
    ((type*)((sigma_u8*)(ptr) - SIGMA_OFFSETOF(type, member)))

#define SIGMA_ARRAY_SIZE(arr) \
    (sizeof(arr) / sizeof((arr)[0]))

/* -----------------------------------------------------------------------
 * VOLATILE MEMORY BARRIER (For hardware register access)
 * ----------------------------------------------------------------------- */
#define SIGMA_MMIO_READ32(addr)     (*(volatile sigma_u32*)(addr))
#define SIGMA_MMIO_WRITE32(addr, v) (*(volatile sigma_u32*)(addr) = (v))
#define SIGMA_MMIO_READ64(addr)     (*(volatile sigma_u64*)(addr))
#define SIGMA_MMIO_WRITE64(addr, v) (*(volatile sigma_u64*)(addr) = (v))

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_TYPES_H */

