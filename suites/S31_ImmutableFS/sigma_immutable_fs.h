// SigmaOS — Sigma-Immutable: Immutable Root FS + OTA Snapshot
// Inspired by: ChromeOS dm-verity + A/B partitions, NixOS generations
// Module: sigma-sys-immutablefs
// USP: Atomic slot-swap rollback — if new slot fails verification, boot prior
// Zero external dependency — uses capability tokens for slot authorization

#ifndef SIGMA_IMMUTABLE_FS_H
#define SIGMA_IMMUTABLE_FS_H

#include "sigma_caps.h"

#define SIGMA_SLOT_A      0
#define SIGMA_SLOT_B      1
#define SIGMA_SLOT_MAGIC  0x5347534C  // "SGSL"

// Simple 64-bit hash (FNV-1a variant — no external lib)
static inline unsigned long fnv1a_hash(const unsigned char* data,
                                        unsigned long len) {
    unsigned long hash  = 14695981039346656037UL;
    unsigned long prime = 1099511628211UL;
    for (unsigned long i = 0; i < len; i++) {
        hash ^= (unsigned long)data[i];
        hash *= prime;
    }
    return hash;
}

typedef struct SigmaFSSlot {
    unsigned int  magic;
    unsigned char slot_id;         // SLOT_A or SLOT_B
    unsigned long integrity_hash;  // FNV-1a of slot content
    unsigned char verified;        // set after boot-time verification
    unsigned char active;          // currently mounted slot
    unsigned int  boot_count;      // attempts since last success
    unsigned int  max_boot_tries;  // threshold before fallback
} SigmaFSSlot;

typedef struct SigmaImmutableFS {
    SigmaFSSlot slots[2];
    unsigned char current_slot;
} SigmaImmutableFS;

static inline void ifs_init(SigmaImmutableFS* fs,
                              unsigned long hash_a, unsigned long hash_b) {
    fs->slots[SIGMA_SLOT_A].magic            = SIGMA_SLOT_MAGIC;
    fs->slots[SIGMA_SLOT_A].slot_id          = SIGMA_SLOT_A;
    fs->slots[SIGMA_SLOT_A].integrity_hash   = hash_a;
    fs->slots[SIGMA_SLOT_A].verified         = 0;
    fs->slots[SIGMA_SLOT_A].active           = 1;
    fs->slots[SIGMA_SLOT_A].boot_count       = 0;
    fs->slots[SIGMA_SLOT_A].max_boot_tries   = 3;

    fs->slots[SIGMA_SLOT_B].magic            = SIGMA_SLOT_MAGIC;
    fs->slots[SIGMA_SLOT_B].slot_id          = SIGMA_SLOT_B;
    fs->slots[SIGMA_SLOT_B].integrity_hash   = hash_b;
    fs->slots[SIGMA_SLOT_B].verified         = 0;
    fs->slots[SIGMA_SLOT_B].active           = 0;
    fs->slots[SIGMA_SLOT_B].boot_count       = 0;
    fs->slots[SIGMA_SLOT_B].max_boot_tries   = 3;

    fs->current_slot = SIGMA_SLOT_A;
}

// Verify active slot integrity
static inline int ifs_verify(SigmaImmutableFS* fs,
                               const unsigned char* data, unsigned long len) {
    unsigned long computed = fnv1a_hash(data, len);
    SigmaFSSlot* slot = &fs->slots[fs->current_slot];
    slot->boot_count++;
    if (computed == slot->integrity_hash) {
        slot->verified   = 1;
        slot->boot_count = 0;
        return 1; // OK
    }
    return 0; // integrity violation
}

// Atomically swap to other slot (OTA update path)
static inline void ifs_swap_slot(SigmaImmutableFS* fs, SigmaCapToken* cap) {
    if (!cap_check(cap, SIGMA_CAP_ADMIN)) return; // only admin caps can swap
    fs->slots[fs->current_slot].active = 0;
    fs->current_slot = (fs->current_slot == SIGMA_SLOT_A) ? SIGMA_SLOT_B : SIGMA_SLOT_A;
    fs->slots[fs->current_slot].active = 1;
    fs->slots[fs->current_slot].boot_count = 0;
}

// Auto-rollback if too many failed boots
static inline void ifs_maybe_rollback(SigmaImmutableFS* fs) {
    SigmaFSSlot* slot = &fs->slots[fs->current_slot];
    if (slot->boot_count >= slot->max_boot_tries && !slot->verified) {
        slot->active     = 0;
        fs->current_slot = (fs->current_slot == SIGMA_SLOT_A) ? SIGMA_SLOT_B : SIGMA_SLOT_A;
        fs->slots[fs->current_slot].active = 1;
    }
}

#endif /* SIGMA_IMMUTABLE_FS_H */
