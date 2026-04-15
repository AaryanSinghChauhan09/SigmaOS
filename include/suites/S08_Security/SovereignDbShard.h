/* S SIGMAOS: SOVEREIGN DB SHARD HEADER */
#ifndef SOVEREIGN_DB_SHARD_H
#define SOVEREIGN_DB_SHARD_H
#include "sigma_types.h"

sigma_err_t sigma_db_put (const char* key, const char* val);
const char* sigma_db_get (const char* key);
void        SovereignDbShard_Init (void);
void        SovereignDb_Audit     (void);

#endif
