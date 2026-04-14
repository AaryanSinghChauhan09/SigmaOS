/* Σ SIGMAOS: SOVEREIGN AUTOCLEAN SHARD HEADER */
#ifndef SOVEREIGN_AUTOCLEAN_SHARD_H
#define SOVEREIGN_AUTOCLEAN_SHARD_H
#include "sigma_types.h"
typedef enum { DEBRIS_CACHE, DEBRIS_TEMP, DEBRIS_LOG_OLD, DEBRIS_CORE_DUMP,
               DEBRIS_PKG_CACHE, DEBRIS_THUMBNAIL, DEBRIS_ORPHAN_LIB,
               DEBRIS_CRASH_REPORT } SigmaDebrisType_t;
sigma_err_t sigma_autoclean_register (SigmaDebrisType_t type, const char* path,
                                       sigma_u64 size_kb, sigma_u32 age_days);
void        sigma_autoclean_scan     (void);
void        sigma_autoclean_run      (sigma_bool dry_run);
void        SovereignAutoCleanShard_Init (void);
void        SovereignAutoClean_Audit      (void);
#endif
