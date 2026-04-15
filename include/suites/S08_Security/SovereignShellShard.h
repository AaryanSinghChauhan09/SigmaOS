/* S SIGMAOS: SOVEREIGN SHELL SHARD HEADER */
#ifndef SOVEREIGN_SHELL_SHARD_H
#define SOVEREIGN_SHELL_SHARD_H
#include "suites/S01_Genesis/shards/sigma_types.h"

void        sigma_shell_process (const char* input);
const char* sigma_shell_suggest (const char* partial);
void        SovereignShellShard_Init   (void);
void        SovereignShell_Audit       (void);

#endif
