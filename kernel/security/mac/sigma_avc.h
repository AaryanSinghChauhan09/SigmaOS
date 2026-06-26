// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_avc.h — Access Vector Cache for O(1) MAC decisions (SELinux avc.c-inspired)
 *
 * Caches (src_label, dst_label, operation) → decision in a 512-slot hash table.
 * Cache hit: nanosecond latency (array lookup + comparison).
 * Cache miss: evaluates the trust label policy, stores the result.
 * Cache flush: called on policy reload (like SELinux avc_ss_reset).
 */
#include <sigma_kernel_types.h>

typedef sigma_u32 sigma_trust_label_t;  /* numeric label ID, not a string */

#define SIGMA_AVC_SLOTS     512   /* power of 2 — bitmask indexing */
#define SIGMA_AVC_MAXNODES  410   /* matching SELinux's load factor  */

/* AVC operation bits */
#define SIGMA_AVC_OP_READ   0x01
#define SIGMA_AVC_OP_WRITE  0x02
#define SIGMA_AVC_OP_EXEC   0x04
#define SIGMA_AVC_OP_IPC    0x08
#define SIGMA_AVC_OP_NET    0x10

typedef struct {
    sigma_trust_label_t src_label;
    sigma_trust_label_t dst_label;
    sigma_u16           operation;
    int                 decision;   /* 1 = allow, 0 = deny */
    sigma_u32           hits;
    bool                valid;
} sigma_avc_entry_t;

typedef struct {
    sigma_avc_entry_t entries[SIGMA_AVC_SLOTS];
    sigma_u64         cache_hits;
    sigma_u64         cache_misses;
    sigma_u64         policy_reloads;
} sigma_avc_t;

static inline sigma_u32 sigma_avc_hash(sigma_trust_label_t src,
                                        sigma_trust_label_t dst,
                                        sigma_u16           op) {
    return ((sigma_u32)src ^ ((sigma_u32)dst << 2) ^ (sigma_u32)op)
           & (SIGMA_AVC_SLOTS - 1);
}

void sigma_avc_init(sigma_avc_t* avc);
int  sigma_avc_check(sigma_avc_t* avc,
                      sigma_trust_label_t src,
                      sigma_trust_label_t dst,
                      sigma_u16 operation);
void sigma_avc_flush(sigma_avc_t* avc);
void sigma_avc_stats(const sigma_avc_t* avc);

/* Called by the MAC policy engine on a cache miss */
extern int sigma_mac_check_iflow(sigma_trust_label_t src,
                                   sigma_trust_label_t dst);
