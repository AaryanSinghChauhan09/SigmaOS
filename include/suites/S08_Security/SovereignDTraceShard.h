/* S SIGMAOS: SOVEREIGN DTRACE SHARD HEADER */
#ifndef SOVEREIGN_DTRACE_SHARD_H
#define SOVEREIGN_DTRACE_SHARD_H
#include "sigma_types.h"

void sigma_dtrace_probe (const char* target_shard, const char* probe_point);
void sigma_dtrace_trace (const char* filter);
void SovereignDTraceShard_Init (void);
void SovereignDTrace_Audit     (void);

#endif
