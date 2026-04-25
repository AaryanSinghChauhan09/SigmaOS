// SigmaOS — sigma-fw-update: Firmware Update Framework
// Inspired by: fwupd, UEFI Capsule Updates, ChromeOS OTA
// Module: sigma-auto-update
// USP: Atomic double-buffer swap — power-cut safe, FNV-1a verified
// No DBus, no NetworkManager — uses sigma_caps for authorization

#ifndef SIGMA_FW_UPDATE_H
#define SIGMA_FW_UPDATE_H

#include "sigma_caps.h"
#include "sigma_immutable_fs.h"

#define SIGMA_FW_MAX_COMPONENTS 16
#define SIGMA_FW_NAME_LEN       32
#define SIGMA_FW_VER_LEN        16

typedef enum SigmaFWUpdateState {
    FW_IDLE         = 0,
    FW_DOWNLOADING  = 1,
    FW_VERIFYING    = 2,
    FW_STAGING      = 3,
    FW_COMMITTED    = 4,
    FW_FAILED       = 5
} SigmaFWUpdateState;

typedef struct SigmaFWComponent {
    char               name[SIGMA_FW_NAME_LEN];
    char               current_ver[SIGMA_FW_VER_LEN];
    char               pending_ver[SIGMA_FW_VER_LEN];
    unsigned long      current_hash;
    unsigned long      pending_hash;
    SigmaFWUpdateState state;
    unsigned int       retry_count;
} SigmaFWComponent;

typedef struct SigmaFWUpdater {
    SigmaFWComponent   components[SIGMA_FW_MAX_COMPONENTS];
    unsigned int       count;
    SigmaImmutableFS*  ifs;           // A/B slot backend
} SigmaFWUpdater;

static inline void fwup_init(SigmaFWUpdater* u, SigmaImmutableFS* ifs) {
    u->count = 0;
    u->ifs   = ifs;
}

static inline int fwup_register(SigmaFWUpdater* u, const char* name,
                                  const char* ver, unsigned long hash) {
    if (u->count >= SIGMA_FW_MAX_COMPONENTS) return -1;
    SigmaFWComponent* c = &u->components[u->count++];
    for (int i = 0; i < SIGMA_FW_NAME_LEN - 1 && name[i]; i++) c->name[i] = name[i];
    for (int i = 0; i < SIGMA_FW_VER_LEN  - 1 && ver[i];  i++) c->current_ver[i] = ver[i];
    c->current_hash = hash;
    c->pending_hash = 0;
    c->state        = FW_IDLE;
    c->retry_count  = 0;
    return (int)(u->count - 1);
}

// Stage a firmware update (download complete, pre-verified image ready)
static inline int fwup_stage(SigmaFWUpdater* u, unsigned int comp_id,
                               const char* new_ver, unsigned long new_hash,
                               const unsigned char* image, unsigned long len,
                               SigmaCapToken* tok) {
    if (!cap_check(tok, SIGMA_CAP_ADMIN)) return -1;
    if (comp_id >= u->count) return -2;

    SigmaFWComponent* c = &u->components[comp_id];
    // Verify image integrity
    unsigned long actual = fnv1a_hash(image, len);
    if (actual != new_hash) { c->state = FW_FAILED; return -3; }

    for (int i = 0; i < SIGMA_FW_VER_LEN - 1 && new_ver[i]; i++) c->pending_ver[i] = new_ver[i];
    c->pending_hash = new_hash;
    c->state        = FW_STAGING;
    return 0;
}

// Commit: atomically swap to new firmware (A/B slot)
static inline int fwup_commit(SigmaFWUpdater* u, unsigned int comp_id,
                                SigmaCapToken* tok) {
    if (!cap_check(tok, SIGMA_CAP_ADMIN)) return -1;
    if (comp_id >= u->count) return -2;
    SigmaFWComponent* c = &u->components[comp_id];
    if (c->state != FW_STAGING) return -3;

    // Swap A/B slot via ImmutableFS
    if (u->ifs) ifs_swap_slot(u->ifs, tok);

    // Promote pending to current
    for (int i = 0; i < SIGMA_FW_VER_LEN; i++) {
        c->current_ver[i] = c->pending_ver[i];
        c->pending_ver[i] = 0;
    }
    c->current_hash = c->pending_hash;
    c->pending_hash = 0;
    c->state        = FW_COMMITTED;
    c->retry_count  = 0;
    return 0;
}

static inline void fwup_rollback(SigmaFWUpdater* u, unsigned int comp_id,
                                   SigmaCapToken* tok) {
    if (comp_id >= u->count) return;
    if (u->ifs) ifs_maybe_rollback(u->ifs);
    u->components[comp_id].state = FW_IDLE;
}

#endif /* SIGMA_FW_UPDATE_H */
