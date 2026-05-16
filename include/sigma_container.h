/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN CONTAINER RUNTIME (S-CONTAINER)
 * =========================================================================
 * Mission: Native kernel-level container isolation without a daemon layer.
 * Competitor parity: Linux OCI/Docker/Podman, macOS Virtualization.framework,
 *                    Windows Hyper-V Containers.
 * ZERO-DEPENDENCY: Direct namespace + cgroup shard isolation; no containerd.
 * =========================================================================
 */

#ifndef SIGMA_CONTAINER_H
#define SIGMA_CONTAINER_H

#include "./core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Container States --- */
#define SIGMA_CTR_CREATED    0x00u
#define SIGMA_CTR_RUNNING    0x01u
#define SIGMA_CTR_PAUSED     0x02u
#define SIGMA_CTR_STOPPED    0x03u
#define SIGMA_CTR_DEAD       0x04u

/* --- Isolation Flags --- */
#define SIGMA_CTR_ISO_PID    (1u << 0)  /* PID namespace isolation   */
#define SIGMA_CTR_ISO_NET    (1u << 1)  /* Network namespace          */
#define SIGMA_CTR_ISO_MNT    (1u << 2)  /* Mount namespace            */
#define SIGMA_CTR_ISO_USER   (1u << 3)  /* User namespace             */
#define SIGMA_CTR_ISO_IPC    (1u << 4)  /* IPC namespace              */
#define SIGMA_CTR_ISO_ALL    (0x1Fu)    /* Full isolation             */

#define SIGMA_CTR_MAX        32u
#define SIGMA_CTR_NAME_LEN   48u

typedef struct {
    sigma_u32 id;
    char      name[SIGMA_CTR_NAME_LEN];
    sigma_u32 state;           /* SIGMA_CTR_* state                */
    sigma_u32 isolation_flags; /* Bitmask of SIGMA_CTR_ISO_*       */
    sigma_u32 cpu_shares;      /* Relative CPU weight (1-1024)      */
    sigma_u32 mem_limit_mb;    /* 0 = unlimited                     */
    sigma_u32 root_pid;        /* Container init PID                */
} sigma_container_t;

typedef struct {
    sigma_container_t containers[SIGMA_CTR_MAX];
    sigma_u32 count;
    sigma_u32 running_count;
} sigma_container_registry_t;

/* --- Container Primitives --- */
void      container_runtime_init(void);
sigma_u32 container_create(const char* name, sigma_u32 isolation_flags,
                           sigma_u32 cpu_shares, sigma_u32 mem_limit_mb);
void      container_start(sigma_u32 id);
void      container_pause(sigma_u32 id);
void      container_stop(sigma_u32 id);
void      container_destroy(sigma_u32 id);
const sigma_container_t*          container_get(sigma_u32 id);
const sigma_container_registry_t* container_get_registry(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_CONTAINER_H */
