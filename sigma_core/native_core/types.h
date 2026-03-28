/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

// SigmaOS Native Type Definitions
// ==========================================
// Replaces standard <stdint.h> and <stddef.h>
// 100% Zero-Dependency C/C++ Header

#ifndef SIGMA_TYPES_H
#define SIGMA_TYPES_H

// Core primitive types
typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;
typedef unsigned long long u64;

typedef signed char        i8;
typedef signed short       i16;
typedef signed int         i32;
typedef signed long long   i64;

typedef float              f32;
typedef double             f64;

#define NULL ((void*)0)

// Boolean type
typedef u8 b8;
#define true 1
#define false 0

// Size definitions
typedef u64 size_t;
typedef i64 ssize_t;

// Architecture detection
#if defined(__x86_64__) || defined(_M_X64)
    #define SIGMA_ARCH_64 1
#elif defined(__i386) || defined(_M_IX86)
    #define SIGMA_ARCH_32 1
#endif

#endif // SIGMA_TYPES_H

