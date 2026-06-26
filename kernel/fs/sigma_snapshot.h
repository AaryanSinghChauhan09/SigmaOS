// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_snapshot.h — Btrfs-style copy-on-write filesystem snapshots
 *
 * Snapshots are O(1) to create (just record current shard-matrix state).
 * Restore is a single symlink swap (via sigma_ostree_set_deployment).
 * Snapshots are stored in /sigma/ostree/snapshots/<timestamp>/
 *
 * CLI:
 *   sigma snapshot create [--name "before-update"]
 *   sigma snapshot list
 *   sigma snapshot restore <id>
 *   sigma snapshot delete <id>
 */
#include <sigma_kernel_types.h>
#include <time.h>

#define SIGMA_SNAPSHOT_ID_LEN    32
#define SIGMA_SNAPSHOT_NAME_LEN  128
#define SIGMA_SNAPSHOT_MAX       64   /* max snapshots retained */

typedef struct {
    char     id[SIGMA_SNAPSHOT_ID_LEN];     /* SHA-256 truncated to 32 chars */
    char     name[SIGMA_SNAPSHOT_NAME_LEN]; /* human-readable label           */
    char     commit_hash[65];               /* OSTree commit this points to   */
    time_t   created_at;
    sigma_u64 size_bytes;                   /* estimated incremental size     */
    bool     is_automatic;                  /* true = created before update   */
} sigma_snapshot_t;

typedef struct {
    sigma_snapshot_t entries[SIGMA_SNAPSHOT_MAX];
    int              count;
    char             snapshots_dir[256];    /* /sigma/ostree/snapshots/       */
} sigma_snapshot_registry_t;

/* ── API ──────────────────────────────────────────────────────────────────── */

int  sigma_snapshot_init(sigma_snapshot_registry_t* reg, const char* dir);

/* Create a snapshot of the current deployment. id_out receives the new ID. */
int  sigma_snapshot_create(sigma_snapshot_registry_t* reg,
                            const char* name,
                            bool        automatic,
                            char        id_out[SIGMA_SNAPSHOT_ID_LEN]);

/* Restore a snapshot — swaps /sigma/boot/current symlink, then signals reboot. */
int  sigma_snapshot_restore(sigma_snapshot_registry_t* reg, const char* id);

/* Delete a snapshot and free its disk space */
int  sigma_snapshot_delete(sigma_snapshot_registry_t* reg, const char* id);

/* List all snapshots (sorted newest first) */
void sigma_snapshot_list(const sigma_snapshot_registry_t* reg,
                          void (*cb)(const sigma_snapshot_t*, void*),
                          void* userdata);

/* Auto-prune: keep only the N most recent snapshots */
int  sigma_snapshot_prune(sigma_snapshot_registry_t* reg, int keep_count);
