// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_assert.h — Unikraft UK_ASSERT-style debug assertions
 *
 * SIGMA_DEBUG_BUILD:  full check with file + line + message → sigma_panic()
 * Release build:      compiles to __builtin_assume() — zero instructions
 *
 * Usage:
 *   SIGMA_ASSERT(zt != NULL);
 *   SIGMA_ASSERT_MSG(count < MAX, "overflow: count=%d max=%d", count, MAX);
 *   SIGMA_STATIC_ASSERT(sizeof(sigma_msg_t) == 64, "IPC message must be 64 bytes");
 */
#include "sigma_log.h"

#ifdef SIGMA_DEBUG_BUILD

#define SIGMA_ASSERT(cond)                                                    \
    do {                                                                      \
        if (__builtin_expect(!(cond), 0)) {                                   \
            sigma_log_err("[ASSERT FAIL] %s:%d: %s\n",                        \
                          __FILE__, __LINE__, #cond);                         \
            sigma_panic("Assertion failed: " #cond);                          \
        }                                                                     \
    } while (0)

#define SIGMA_ASSERT_MSG(cond, msg, ...)                                      \
    do {                                                                      \
        if (__builtin_expect(!(cond), 0)) {                                   \
            sigma_log_err("[ASSERT] %s:%d: " msg "\n",                        \
                          __FILE__, __LINE__, ##__VA_ARGS__);                 \
            sigma_panic("Assertion failed: " #cond);                          \
        }                                                                     \
    } while (0)

#else   /* release build — zero overhead */

#define SIGMA_ASSERT(cond)               __builtin_assume(cond)
#define SIGMA_ASSERT_MSG(cond, msg, ...) __builtin_assume(cond)

#endif  /* SIGMA_DEBUG_BUILD */

/* Compile-time assertion — always active (same as Zephyr BUILD_ASSERT) */
#define SIGMA_STATIC_ASSERT(cond, msg)   static_assert(cond, msg)
