/* S SIGMAOS: SOVEREIGN RECALL SHARD HEADER */
#ifndef SOVEREIGN_RECALL_SHARD_H
#define SOVEREIGN_RECALL_SHARD_H
#include "sigma_types.h"

typedef enum { EVENT_WM_FOCUS, EVENT_CLI_CMD, EVENT_FS_ACCESS, EVENT_APP_ACTION } SigmaRecallType_t;

void sigma_recall_record (SigmaRecallType_t type, const char* summary);
void sigma_recall_query  (const char* keyword);
void SovereignRecallShard_Init (void);
void SovereignRecall_Audit      (void);

#endif
