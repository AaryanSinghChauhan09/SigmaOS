/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN SECURITY INTERFACE (v2.0)
 * =========================================================================
 * Mission: Pluggable security paradigms (Jail, Pledge, Unveil, Capsicum).
 * Design: C11 / Zero-Dependency / Registry-Based.
 * =========================================================================
 */

#ifndef SOVEREIGN_SECURITY_H
#define SOVEREIGN_SECURITY_H

#include "suites/S01_Genesis/shards/sigma_types.h"

typedef sigma_err_t (*sigma_sec_init_fn)(void);

typedef struct {
    char name[32];
    sigma_sec_init_fn init;
} sovereign_security_shard_t;

/* Registry API */
void SovereignSecurity_InitRegistry(void);
sigma_err_t SovereignSecurity_Register(const char* name, sigma_sec_init_fn init);
void SovereignSecurity_ActivateMatrix(void);

#endif /* SOVEREIGN_SECURITY_H */
