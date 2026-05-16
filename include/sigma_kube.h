/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN KERNEL-NATIVE ORCHESTRATOR (S-KUBE)
 * =========================================================================
 * Mission: High-level shard orchestration and service mesh for KNSI containers.
 * Competitor parity: Kubernetes (K8s), Docker Swarm, Nomad.
 * ZERO-DEPENDENCY: No etcd, no kubelet; direct lattice state management.
 * =========================================================================
 */

#ifndef SIGMA_KUBE_H
#define SIGMA_KUBE_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Orchestration Patterns --- */
#define SIGMA_KUBE_PATTERN_SINGLETON  0x00u  /* Exactly one shard active     */
#define SIGMA_KUBE_PATTERN_REPLICA    0x01u  /* N identical shards (HA)      */
#define SIGMA_KUBE_PATTERN_DAEMON     0x02u  /* One shard per silicon core   */
#define SIGMA_KUBE_PATTERN_JOB        0x03u  /* Run to completion            */

#define SIGMA_KUBE_DEPLOYMENT_MAX     16u
#define SIGMA_KUBE_NAME_LEN           48u

typedef struct {
    sigma_u32 deployment_id;
    char      name[SIGMA_KUBE_NAME_LEN];
    sigma_u32 pattern;         /* SIGMA_KUBE_PATTERN_*             */
    sigma_u32 replicas;        /* Desired count                    */
    sigma_u32 current_count;   /* Actual count                     */
    sigma_u32 container_id;    /* Template container ID            */
} sigma_kube_deployment_t;

typedef struct {
    sigma_kube_deployment_t deployments[SIGMA_KUBE_DEPLOYMENT_MAX];
    sigma_u32 active_deployments;
    sigma_u32 total_pods;      /* Total active shards managed      */
    sigma_u32 self_heals;      /* Telemetry: automatic recoveries  */
} sigma_kube_state_t;

/* --- Kube Primitives --- */
void      kube_init(void);
sigma_u32 kube_create_deployment(const char* name, sigma_u32 container_id,
                                 sigma_u32 pattern, sigma_u32 replicas);
void      kube_scale_deployment(sigma_u32 deployment_id, sigma_u32 replicas);
void      kube_delete_deployment(sigma_u32 deployment_id);
void      kube_reconcile_lattice(void);
const sigma_kube_state_t* kube_get_state(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_KUBE_H */
