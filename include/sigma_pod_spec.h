/*
 * SigmaOS — shared native pod orchestration spec (CLI <-> kernel orchestrator)
 */
#ifndef SIGMA_POD_SPEC_H
#define SIGMA_POD_SPEC_H

#include "sigma_kernel_types.h"

#define SIGMA_MSG_SPAWN_CONTAINER         1u
#define SIGMA_MSG_STOP_CONTAINER          2u
#define SIGMA_MSG_LIST_CONTAINERS         3u
#define SIGMA_MSG_SPAWN_NATIVE_CONTAINER  4u

#define SIGMA_NS_MNT  (1u << 0)
#define SIGMA_NS_PID  (1u << 1)
#define SIGMA_NS_NET  (1u << 2)
#define SIGMA_NS_UTS  (1u << 3)
#define SIGMA_NS_IPC  (1u << 4)

typedef struct SigmaPodNativeSpec {
    sigma_u64 package_path;
    sigma_u32 namespace_flags;
    sigma_u32 cgroup_cpu_millis;
    sigma_u32 cgroup_mem_mb;
    sigma_u32 io_weight;
} SigmaPodNativeSpec;

#endif /* SIGMA_POD_SPEC_H */
