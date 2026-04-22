/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN SILICON STORE HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_SILICON_STORE_H
#define SOVEREIGN_SILICON_STORE_H

#include "sigma_types.h"

sigma_err_t sigma_store_set       (const char* key, const char* value);
const char* sigma_store_get       (const char* key);
void        SovereignSiliconStore_Init (void);
void        SovereignSiliconStore_Audit(void);

#endif /* SOVEREIGN_SILICON_STORE_H */
