/* =========================================================================
 * S SIGMAOS: SOVEREIGN CONFIG SHARD HEADER
 * ========================================================================= */
#ifndef SOVEREIGN_CONFIG_SHARD_H
#define SOVEREIGN_CONFIG_SHARD_H
#include "sigma_types.h"
typedef enum { CFG_STRING, CFG_INT, CFG_BOOL, CFG_FLOAT_X100 } SigmaCfgType_t;
sigma_err_t  sigma_cfg_set       (const char* key, const char* val,
                                   SigmaCfgType_t type, sigma_bool lock);
const char*  sigma_cfg_get       (const char* key);
void         sigma_cfg_commit    (const char* tag);
void         sigma_cfg_rollback  (void);
void         SovereignConfigShard_Init (void);
void         SovereignConfig_Audit      (void);
#endif
