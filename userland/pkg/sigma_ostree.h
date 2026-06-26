// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_ostree.h — content-addressed atomic OS update store (OSTree-inspired)
 *
 * Every OS file is stored as a SHA-256-addressed object. Updates are staged
 * first, then committed atomically via fsync+rename. Rollback is a single
 * symlink swap — three syscalls, no package manager involved.
 *
 * Object store layout:
 *   /sigma/ostree/objects/<aa>/<bb...>/  ← content-addressed files (2+62 hex)
 *   /sigma/ostree/staging/               ← write here first, never fsync yet
 *   /sigma/ostree/refs/heads/<name>      ← commit hash files
 *
 * Boot layout:
 *   /sigma/boot/deployments/<hash>/      ← one dir per deployed commit
 *   /sigma/boot/current -> deployments/<hash>/   ← atomic symlink to active
 */

#include <sigma_kernel_types.h>

typedef struct {
    char  store_root[128];   /* /sigma/ostree/objects/        */
    char  staging_dir[128];  /* /sigma/ostree/staging/        */
    char  current_commit[65];/* SHA-256 hex of active deploy  */
    char  prev_commit[65];   /* SHA-256 hex of previous deploy — for rollback */
    int   staging_dfd;       /* dir fd for batch fsync (O_DIRECTORY) */
} sigma_ostree_t;

/* ── API ──────────────────────────────────────────────────────────────────── */

/* Initialise the object store, creating directories if needed. */
int sigma_ostree_init(sigma_ostree_t* repo,
                      const char*     store_root,
                      const char*     staging_dir);

/*
 * Stage one file into the staging area.
 * Content is written to staging/<sha256> but NOT fsync'd yet.
 * out_sha256 receives the 64-char hex digest + NUL.
 */
int sigma_ostree_stage_object(sigma_ostree_t* repo,
                               const char*    src_path,
                               char           out_sha256[65]);

/*
 * Commit: fsync all objects in staging, then rename() each one into the
 * live object store. This is the atomic cut-over.
 * Crash before commit → old deployment is intact.
 * Crash during commit → partial objects in store, harmless (content-addressed).
 */
int sigma_ostree_commit(sigma_ostree_t* repo,
                         const char*    commit_msg,
                         char           out_commit_hash[65]);

/*
 * Set the active deployment by swapping /sigma/boot/current.
 * Only touches the symlink — no file copies, no package operations.
 * To rollback: call with repo->prev_commit.
 */
int sigma_ostree_set_deployment(sigma_ostree_t* repo,
                                  const char*  commit_hash);

/*
 * Rollback to the previous deployment in one call.
 * Equivalent to: sigma_ostree_set_deployment(repo, repo->prev_commit)
 */
int sigma_ostree_rollback(sigma_ostree_t* repo);

/* Print a summary of all deployments to the audit log. */
void sigma_ostree_list_deployments(const sigma_ostree_t* repo);
