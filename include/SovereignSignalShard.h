/* Σ SIGMAOS: SOVEREIGN SIGNAL SHARD HEADER */
#ifndef SOVEREIGN_SIGNAL_SHARD_H
#define SOVEREIGN_SIGNAL_SHARD_H
#include "sigma_types.h"
typedef enum { SIGMA_SIGHUP=1, SIGMA_SIGINT=2, SIGMA_SIGKILL=9,
               SIGMA_SIGSEGV=11, SIGMA_SIGTERM=15,
               SIGMA_SIGUSR1=10, SIGMA_SIGUSR2=12 } SigmaSignal_t;
typedef void (*SigmaSignalHandler_t)(sigma_u32 pid, SigmaSignal_t sig);
sigma_err_t sigma_signal_register (SigmaSignal_t sig, const char* name, SigmaSignalHandler_t h);
sigma_err_t sigma_signal_send     (sigma_u32 pid, SigmaSignal_t sig);
void        SovereignSignalShard_Init (void);
void        SovereignSignal_Audit      (void);
#endif
