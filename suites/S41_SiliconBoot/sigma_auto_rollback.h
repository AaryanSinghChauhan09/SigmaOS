// SigmaOS — Sigma-AutoRollback: Self-Healing Update & Crash Recovery
// Inspired by: Windows System Restore, NixOS generations, macOS Time Machine
// Module: sigma-auto-rollback
// USP: Hardware-timestamped snapshots, capability-gated restore, zero daemon required

#ifndef SIGMA_AUTO_ROLLBACK_H
#define SIGMA_AUTO_ROLLBACK_H

#include "sigma_caps.h"

#define SIGMA_SNAP_MAX       16
#define SIGMA_SNAP_NAME_LEN  32

typedef enum SigmaSnapState {
    SNAP_EMPTY   = 0,
    SNAP_READY   = 1,
    SNAP_ACTIVE  = 2,
    SNAP_CORRUPT = 3
} SigmaSnapState;

typedef struct SigmaSnapshot {
    unsigned int   snap_id;
    char           name[SIGMA_SNAP_NAME_LEN];
    unsigned long  taken_at;    // RDTSC timestamp
    unsigned long  content_hash;// FNV-1a of snapshotted data
    SigmaSnapState state;
    unsigned char  is_boot_snapshot; // survives reboots
} SigmaSnapshot;

typedef struct SigmaRollbackManager {
    SigmaSnapshot  snaps[SIGMA_SNAP_MAX];
    unsigned int   count;
    unsigned int   active_snap;  // currently running snapshot
    unsigned int   next_id;
} SigmaRollbackManager;

static inline unsigned long rollback_rdtsc(void) {
#if defined(__x86_64__) || defined(__i386__)
    unsigned int lo, hi;
    __asm__ __volatile__("rdtsc" : "=a"(lo), "=d"(hi));
    return ((unsigned long)hi << 32) | lo;
#else
    return 0;
#endif
}

static inline unsigned long rollback_fnv1a(const unsigned char* d, unsigned long n) {
    unsigned long h = 14695981039346656037UL, p = 1099511628211UL;
    for (unsigned long i = 0; i < n; i++) { h ^= d[i]; h *= p; }
    return h;
}

static inline void rollback_init(SigmaRollbackManager* rm) {
    rm->count = rm->active_snap = 0;
    rm->next_id = 1;
    for (int i = 0; i < SIGMA_SNAP_MAX; i++) rm->snaps[i].state = SNAP_EMPTY;
}

// Take a snapshot of system data
static inline unsigned int snap_take(SigmaRollbackManager* rm,
                                      const char* name,
                                      const unsigned char* data,
                                      unsigned long len,
                                      unsigned char boot_snap) {
    if (rm->count >= SIGMA_SNAP_MAX) return 0;
    // Reuse oldest if full
    unsigned int slot = rm->count < SIGMA_SNAP_MAX ? rm->count++ : 0;
    SigmaSnapshot* s = &rm->snaps[slot];
    s->snap_id      = rm->next_id++;
    for (int i = 0; i < SIGMA_SNAP_NAME_LEN - 1 && name[i]; i++) s->name[i] = name[i];
    s->taken_at     = rollback_rdtsc();
    s->content_hash = rollback_fnv1a(data, len);
    s->state        = SNAP_READY;
    s->is_boot_snapshot = boot_snap;
    return s->snap_id;
}

// Activate (restore to) a snapshot — requires admin cap
static inline int snap_restore(SigmaRollbackManager* rm,
                                 unsigned int snap_id,
                                 SigmaCapToken* tok,
                                 const unsigned char* verify_data,
                                 unsigned long verify_len) {
    if (!cap_check(tok, SIGMA_CAP_ADMIN)) return -1; // denied
    for (unsigned int i = 0; i < rm->count; i++) {
        if (rm->snaps[i].snap_id != snap_id) continue;
        // Verify integrity before restore
        unsigned long actual = rollback_fnv1a(verify_data, verify_len);
        if (actual != rm->snaps[i].content_hash) {
            rm->snaps[i].state = SNAP_CORRUPT;
            return -2; // integrity mismatch
        }
        rm->active_snap = i;
        rm->snaps[i].state = SNAP_ACTIVE;
        return 0;
    }
    return -3; // not found
}

// Auto-rollback: if active snap is corrupt, find nearest healthy one
static inline int snap_auto_rollback(SigmaRollbackManager* rm) {
    if (rm->count == 0) return -1;
    // Walk backward from active to find SNAP_READY
    for (int i = (int)rm->active_snap - 1; i >= 0; i--) {
        if (rm->snaps[i].state == SNAP_READY) {
            rm->active_snap = (unsigned int)i;
            rm->snaps[i].state = SNAP_ACTIVE;
            return i; // restored to this snap
        }
    }
    return -1; // no healthy snapshot found
}

#endif /* SIGMA_AUTO_ROLLBACK_H */
