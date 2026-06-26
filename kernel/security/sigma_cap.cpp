// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * sigma_cap.cpp — seL4-inspired capability space implementation
 */
#include "sigma_cap.h"
#include "sigma_log.h"

void sigma_cspace_init(sigma_cspace_t* cs, sigma_u32 pid) {
    for (int i = 0; i < SIGMA_CSPACE_SLOTS; i++) cs->slots[i].valid = false;
    cs->workload_pid = pid;
    cs->slot_count   = 0;
}

int sigma_cap_mint(sigma_cspace_t* cs, sigma_u64 object_id,
                    sigma_u8 cap_type, sigma_cap_rights_t rights) {
    for (int i = 0; i < SIGMA_CSPACE_SLOTS; i++) {
        if (!cs->slots[i].valid) {
            cs->slots[i] = { object_id, rights, cap_type, true };
            cs->slot_count++;
            sigma_log_info("[sigma-cap] pid=%u minted cap slot=%d obj=%llu type=%u\n",
                           cs->workload_pid, i,
                           (unsigned long long)object_id, cap_type);
            return i;
        }
    }
    sigma_log_err("[sigma-cap] CSpace full for pid=%u\n", cs->workload_pid);
    return -1;
}

int sigma_cap_check(const sigma_cspace_t* cs, sigma_u64 object_id,
                     sigma_u8 cap_type, sigma_cap_rights_t required) {
    for (int i = 0; i < SIGMA_CSPACE_SLOTS; i++) {
        const sigma_cap_t* s = &cs->slots[i];
        if (!s->valid) continue;
        if (s->object_id != object_id || s->cap_type != cap_type) continue;
        if ((s->rights.words[0] & required.words[0]) == required.words[0])
            return 0;  /* sufficient rights */
        sigma_log_warn("[sigma-cap] pid=%u: insufficient rights on obj=%llu"
                       " (have=0x%x need=0x%x)\n",
                       cs->workload_pid, (unsigned long long)object_id,
                       s->rights.words[0], required.words[0]);
        return -1; /* -EPERM */
    }
    sigma_log_warn("[sigma-cap] pid=%u: no cap for obj=%llu type=%u\n",
                   cs->workload_pid, (unsigned long long)object_id, cap_type);
    return -1; /* -EPERM */
}

void sigma_cap_revoke(sigma_cspace_t* cs, int slot) {
    if (slot < 0 || slot >= SIGMA_CSPACE_SLOTS) return;
    if (cs->slots[slot].valid) {
        cs->slots[slot].valid = false;
        cs->slot_count--;
        sigma_log_info("[sigma-cap] pid=%u slot=%d revoked\n",
                       cs->workload_pid, slot);
    }
}

void sigma_cap_revoke_all(sigma_cspace_t* cs, sigma_u64 object_id) {
    for (int i = 0; i < SIGMA_CSPACE_SLOTS; i++) {
        if (cs->slots[i].valid && cs->slots[i].object_id == object_id)
            sigma_cap_revoke(cs, i);
    }
}
