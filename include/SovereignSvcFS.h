/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SERVICE FILESYSTEM HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_SVC_FS_H
#define SOVEREIGN_SVC_FS_H

#include "sigma_types.h"

void        sigma_svcfs_register(const char* name, sigma_err_t (*trigger)(void));
void        sigma_svcfs_ls      (void);
sigma_err_t sigma_svcfs_execute (const char* name);
void        SovereignSvcFS_Init (void);

#endif /* SOVEREIGN_SVC_FS_H */
