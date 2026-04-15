/* S SIGMAOS: SOVEREIGN CONTAINER SHARD HEADER */
#ifndef SOVEREIGN_CONTAINER_SHARD_H
#define SOVEREIGN_CONTAINER_SHARD_H
#include "sigma_types.h"
typedef enum { NS_PID=0x01,NS_MOUNT=0x02,NS_NET=0x04,NS_UTS=0x08,NS_IPC=0x10,NS_USER=0x20 } SigmaNamespaceFlags_t;
typedef enum { CONTAINER_CREATED,CONTAINER_RUNNING,CONTAINER_PAUSED,CONTAINER_EXITED } SigmaContainerState_t;
sigma_err_t sigma_container_run  (const char* image, const char* hostname,
                                   SigmaNamespaceFlags_t ns, sigma_u32 mem_mb,
                                   sigma_u32 cpu_pct, sigma_bool privileged);
sigma_err_t sigma_container_pause(const char* id);
sigma_err_t sigma_container_stop (const char* id);
void        sigma_container_exec  (const char* id, const char* cmd);
void        SovereignContainerShard_Init (void);
void        SovereignContainer_Audit      (void);
#endif
