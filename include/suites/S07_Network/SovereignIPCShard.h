/* S SIGMAOS: SOVEREIGN IPC SHARD HEADER */
#ifndef SOVEREIGN_IPC_SHARD_H
#define SOVEREIGN_IPC_SHARD_H
#include "suites/S01_Genesis/shards/sigma_types.h"
typedef enum { IPC_METHOD_CALL, IPC_SIGNAL, IPC_REPLY, IPC_ERROR } SigmaIPCMsgType_t;
sigma_err_t sigma_ipc_open (const char* name, sigma_u32 owner_pid);
sigma_err_t sigma_ipc_send (const char* ch, sigma_u32 src, sigma_u32 dst,
                              SigmaIPCMsgType_t type, const char* iface,
                              const char* method, const char* payload);
sigma_err_t sigma_ipc_recv (const char* channel);
void        SovereignIPCShard_Init (void);
void        SovereignIPC_Audit      (void);
#endif
