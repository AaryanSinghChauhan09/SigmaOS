/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN SECURITY VAULT HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_SECURITY_VAULT_H
#define SOVEREIGN_SECURITY_VAULT_H

#include "suites/S01_Genesis/shards/sigma_types.h"
#include "suites/S03_Orchestrator/shards/SigmaOOP.h"

typedef struct {
    SigmaObject_t core;
} SovereignSecurityVault_t;

SovereignSecurityVault_t SovereignSecurityVault_Create(void);
sigma_err_t sigma_pledge(sigma_u32 capabilities);
sigma_err_t sigma_unveil(const char* path);
void SovereignSecurityVault_Audit(SovereignSecurityVault_t* self);
void SovereignSecurityVault_Init(void);

#endif /* SOVEREIGN_SECURITY_VAULT_H */
