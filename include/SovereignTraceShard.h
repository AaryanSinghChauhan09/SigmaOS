/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN TRACE SHARD HEADER
 * =========================================================================
 */
#ifndef SOVEREIGN_TRACE_SHARD_H
#define SOVEREIGN_TRACE_SHARD_H

#include "sigma_types.h"

void sigma_trace_attach (sigma_u32 pid);
void sigma_trace_record (sigma_u32 pid, sigma_u32 nr, const char* name,
                          sigma_u64 a0, sigma_u64 a1, sigma_u64 a2,
                          sigma_i64 retval, sigma_u64 elapsed_ns);
void sigma_trace_detach (void);
void SovereignTraceShard_Init (void);
void SovereignTrace_Audit     (void);

#endif /* SOVEREIGN_TRACE_SHARD_H */
