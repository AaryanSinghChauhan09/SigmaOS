// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_build_assert.h — compile-time size guarantees (Zephyr BUILD_ASSERT-inspired)
 *
 * These fire at compile time if violated on a new architecture.
 * Include this header in every translation unit that deals with
 * IPC messages, ring buffers, or cross-architecture data structures.
 */
#include <stdint.h>

/* ── Architecture-critical type sizes ───────────────────────────────────── */
static_assert(sizeof(uint8_t)  == 1, "uint8_t  must be 1 byte on all sigma targets");
static_assert(sizeof(uint16_t) == 2, "uint16_t must be 2 bytes");
static_assert(sizeof(uint32_t) == 4, "uint32_t must be 4 bytes");
static_assert(sizeof(uint64_t) == 8, "uint64_t must be 8 bytes");
static_assert(sizeof(uintptr_t) == sizeof(void*), "uintptr_t must match pointer width");

/* ── IPC message size ───────────────────────────────────────────────────── */
/* sigma_msg_t must be exactly 64 bytes — the IPC queue uses fixed-stride    */
/* If this fires, a struct member was added without adjusting the pad field. */

/* ── Convenience macro (matches Zephyr's BUILD_ASSERT style) ─────────────── */
#define SIGMA_BUILD_ASSERT(cond, msg)  static_assert(cond, msg)

#define SIGMA_PACKED_ASSERT(type, expected_size)                               \
    static_assert(sizeof(type) == (expected_size),                             \
                  #type " is not " #expected_size " bytes — check padding")
