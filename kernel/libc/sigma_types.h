/* 
 Σ SIGMAOS ZENITH: SOVEREIGN TYPE DEFINITIONS (v2800.0)
 Mission: Absolute Independence from Standard Toolchain Headers.
*/

#ifndef SIGMA_TYPES_H
#define SIGMA_TYPES_H

// Σ BASIC TYPES
typedef unsigned long long uint64_t;
typedef unsigned int       uint32_t;
typedef unsigned short     uint16_t;
typedef unsigned char      uint8_t;

typedef long long          int64_t;
typedef int               int32_t;
typedef short             int16_t;
typedef signed char       int8_t;

// Σ SYSTEM TYPES
typedef unsigned long long size_t;
typedef long long          ssize_t;
typedef uint64_t           uintptr_t;

// Σ BOOLEAN SHARD
typedef uint8_t bool;
#define true  1
#define false 0

// Σ POINTER SHARD
#define NULL ((void*)0)

#endif
