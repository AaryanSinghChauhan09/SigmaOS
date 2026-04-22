/* S SIGMAOS: SOVEREIGN PACKAGE SHARD HEADER */
#ifndef SOVEREIGN_PACKAGE_SHARD_H
#define SOVEREIGN_PACKAGE_SHARD_H
#include "sigma_types.h"

sigma_err_t sigma_pkg_install (const char* name, const char* ver);
void        sigma_pkg_update_all(void);
void        SovereignPackageShard_Init (void);
void        SovereignPackage_Audit      (void);

#endif
