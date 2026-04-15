/* S SIGMAOS: SOVEREIGN DS SHARD HEADER */
#ifndef SOVEREIGN_DS_SHARD_H
#define SOVEREIGN_DS_SHARD_H
#include "suites/S01_Genesis/shards/sigma_types.h"

typedef enum { DS_DTYPE_F32, DS_DTYPE_I32, DS_DTYPE_STRING } SigmaDSDataType_t;

sigma_err_t sigma_ds_allocate (const char* name, SigmaDSDataType_t type, sigma_u32 rows);
void        sigma_ds_compute  (void);
void        SovereignDSShard_Init   (void);
void        SovereignDS_Audit       (void);

#endif
