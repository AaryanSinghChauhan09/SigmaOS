// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_cap.h — seL4-inspired capability-based security model
 *
 * Every resource (endpoint, memory region, device) is accessed only through
 * an unforgeable capability token. A workload that doesn't hold a capability
 * cannot reference the object at all — not even to attempt access.
 * Revocation is O(1): invalidate the slot, no URI scanning needed.
 */
#include <sigma_kernel_types.h>

/* Capability rights bits */
#define SIGMA_CAP_READ         (1u << 0)
#define SIGMA_CAP_WRITE        (1u << 1)
#define SIGMA_CAP_GRANT        (1u << 2)  /* can delegate to another workload */
#define SIGMA_CAP_GRANT_REPLY  (1u << 3)  /* one-shot reply cap               */

/* Capability object types */
#define SIGMA_CAP_TYPE_ENDPOINT  1
#define SIGMA_CAP_TYPE_MEMORY    2
#define SIGMA_CAP_TYPE_DEVICE    3
#define SIGMA_CAP_TYPE_IPC       4
#define SIGMA_CAP_TYPE_FS        5

typedef struct {
    sigma_u32 words[1];  /* bitmask of SIGMA_CAP_* rights */
} sigma_cap_rights_t;

/* A capability slot — kernel-managed, unforgeable from userland */
typedef struct {
    sigma_u64         object_id;
    sigma_cap_rights_t rights;
    sigma_u8          cap_type;
    bool              valid;
} sigma_cap_t;

/* Capability space (CSpace) — per-workload */
#define SIGMA_CSPACE_SLOTS 256

typedef struct {
    sigma_cap_t slots[SIGMA_CSPACE_SLOTS];
    sigma_u32   workload_pid;
    int         slot_count;
} sigma_cspace_t;

/* ── API ──────────────────────────────────────────────────────────────────── */

void sigma_cspace_init(sigma_cspace_t* cs, sigma_u32 pid);

/* Mint a new capability — only the kernel may call this */
int sigma_cap_mint(sigma_cspace_t* cs, sigma_u64 object_id,
                    sigma_u8 cap_type, sigma_cap_rights_t rights);

/* Check a capability — O(1) slot lookup */
int sigma_cap_check(const sigma_cspace_t* cs, sigma_u64 object_id,
                     sigma_u8 cap_type, sigma_cap_rights_t required);

/* Revoke a capability slot — O(1) */
void sigma_cap_revoke(sigma_cspace_t* cs, int slot);

/* Revoke all capabilities for an object (on resource destruction) */
void sigma_cap_revoke_all(sigma_cspace_t* cs, sigma_u64 object_id);
