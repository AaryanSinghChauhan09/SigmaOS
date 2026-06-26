// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * sigma_avc.cpp — O(1) Access Vector Cache (SELinux avc.c-inspired)
 */
#include "sigma_avc.h"
#include "sigma_log.h"

void sigma_avc_init(sigma_avc_t* avc) {
    for (int i = 0; i < SIGMA_AVC_SLOTS; i++) avc->entries[i].valid = false;
    avc->cache_hits = avc->cache_misses = avc->policy_reloads = 0;
    sigma_log_info("[sigma-avc] AVC initialised (%d slots)\n", SIGMA_AVC_SLOTS);
}

int sigma_avc_check(sigma_avc_t* avc,
                     sigma_trust_label_t src,
                     sigma_trust_label_t dst,
                     sigma_u16 op) {
    sigma_u32 slot = sigma_avc_hash(src, dst, op);
    sigma_avc_entry_t* e = &avc->entries[slot];

    if (e->valid && e->src_label == src && e->dst_label == dst && e->operation == op) {
        avc->cache_hits++;
        e->hits++;
        return e->decision;  /* O(1) cache hit */
    }

    /* Cache miss — evaluate policy */
    avc->cache_misses++;
    int decision = (sigma_mac_check_iflow(src, dst) == 0) ? 1 : 0;

    e->src_label = src; e->dst_label = dst; e->operation = op;
    e->decision = decision; e->valid = true; e->hits = 1;

    return decision;
}

void sigma_avc_flush(sigma_avc_t* avc) {
    for (int i = 0; i < SIGMA_AVC_SLOTS; i++) avc->entries[i].valid = false;
    avc->policy_reloads++;
    sigma_log_info("[sigma-avc] cache flushed (policy_reloads=%llu)\n",
                   (unsigned long long)avc->policy_reloads);
}

void sigma_avc_stats(const sigma_avc_t* avc) {
    sigma_u64 total = avc->cache_hits + avc->cache_misses;
    sigma_u64 ratio = total ? (avc->cache_hits * 100) / total : 0;
    sigma_log_info("[sigma-avc] hits=%llu misses=%llu ratio=%llu%% reloads=%llu\n",
                   (unsigned long long)avc->cache_hits,
                   (unsigned long long)avc->cache_misses,
                   (unsigned long long)ratio,
                   (unsigned long long)avc->policy_reloads);
}
