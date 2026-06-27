/* SPDX-License-Identifier: GPL-2.0-or-later */
/**
 * namespace.h — SigmaOS container / shard namespace isolation
 *
 * Each container gets isolated views of:
 *   VFS   — private mount namespace (pivot_root to container rootfs)
 *   Net   — private network stack (veth pair connecting to host)
 *   PID   — PID 1 inside the container
 *   IPC   — private sigma-bus channels
 *   User  — UID/GID mapping (user namespaces)
 *
 * Inspired by: Linux namespaces (clone(2)), OCI Runtime Spec, Genode, gVisor
 */

#pragma once
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

#ifdef __cplusplus
namespace sigmaos { namespace containers {
#endif

/* ── Namespace configuration ────────────────────────────────────────────── */

typedef struct sigma_ns_config {
    bool        isolate_vfs;        /* private mount namespace */
    bool        isolate_net;        /* private network stack */
    bool        isolate_pid;        /* PID 1 in container */
    bool        isolate_ipc;        /* private sigma-bus channels */
    bool        isolate_user;       /* UID/GID mapping */
    bool        isolate_uts;        /* private hostname/domainname */
    char        rootfs[256];        /* container root directory */
    char        hostname[64];
    uint64_t    mem_limit;          /* bytes, 0 = no limit */
    uint64_t    cpu_shares;         /* relative CPU weight (1024 = default) */
    uint64_t    cpu_quota_us;       /* CPU time per period (0 = no limit) */
    uint64_t    cpu_period_us;      /* cgroup v2 period */
    uint32_t    uid_map_host;       /* host UID mapped to container UID 0 */
    uint32_t    gid_map_host;
    /* OCI-compatible capability bounding set */
    uint64_t    cap_bounding;
    uint64_t    cap_effective;
    /* Seccomp filter program (serialised BPF) */
    const uint8_t *seccomp_prog;
    size_t         seccomp_len;
} sigma_ns_config_t;

/* ── Container state ────────────────────────────────────────────────────── */

typedef enum sigma_container_state {
    CONTAINER_CREATING  = 0,
    CONTAINER_RUNNING   = 1,
    CONTAINER_PAUSED    = 2,
    CONTAINER_STOPPED   = 3,
    CONTAINER_DELETED   = 4,
} sigma_container_state_t;

typedef struct sigma_container {
    uint32_t                id;
    char                    oci_id[64];   /* OCI container ID (SHA-256 prefix) */
    sigma_container_state_t state;
    uint32_t                init_shard;
    uint32_t                init_pid;
    sigma_ns_config_t       config;
    uint64_t                created_at;
    uint64_t                started_at;
    int                     exit_code;
} sigma_container_t;

/* ── Container lifecycle API ────────────────────────────────────────────── */

int  sigma_container_create  (const sigma_ns_config_t *cfg,
                               sigma_container_t *out);
int  sigma_container_start   (uint32_t container_id);
int  sigma_container_stop    (uint32_t container_id, int timeout_s);
int  sigma_container_kill    (uint32_t container_id, int signal);
int  sigma_container_pause   (uint32_t container_id);
int  sigma_container_resume  (uint32_t container_id);
int  sigma_container_delete  (uint32_t container_id, bool force);
int  sigma_container_exec    (uint32_t container_id,
                               const char *binary, char *const argv[],
                               uint32_t *out_pid);
int  sigma_container_status  (uint32_t container_id,
                               sigma_container_t *out);
int  sigma_container_list    (sigma_container_t *out, size_t max,
                               size_t *count);
/* Attach stdin/stdout/stderr of calling process to container init */
int  sigma_container_attach  (uint32_t container_id);

/* OCI bundle format support */
int  sigma_container_from_oci(const char *bundle_dir,
                               sigma_container_t *out);

/* Checkpoint / Restore (CRIU-style) */
int  sigma_container_checkpoint(uint32_t container_id,
                                 const char *checkpoint_dir);
int  sigma_container_restore    (const char *checkpoint_dir,
                                  sigma_container_t *out);

#ifdef __cplusplus
} /* containers */ } /* sigmaos */
#endif
