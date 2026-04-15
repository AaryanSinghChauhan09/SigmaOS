/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN DISTRO INTERFACE (v1.0)
 * =========================================================================
 * Mission: Modular absorption of global Linux/BSD distribution USPs.
 * Design: C11 / Zero-Dependency / Registry-Based.
 * =========================================================================
 */

#ifndef SOVEREIGN_DISTRO_H
#define SOVEREIGN_DISTRO_H

#include "sigma_types.h"

#define MAX_DISTROS 128
#define DISTRO_NAME_MAX 32

typedef void (*sigma_distro_absorb_fn)(void);

typedef struct {
    char name[DISTRO_NAME_MAX];
    char pkg_mgr[16];
    char init_system[16];
    char usp_summary[128];
    sigma_distro_absorb_fn absorb;
} sovereign_distro_t;

typedef struct {
    sovereign_distro_t distros[MAX_DISTROS];
    sigma_u32 distro_count;
} sovereign_distro_registry_t;

/* Public API */
void SovereignDistro_InitRegistry(void);
sigma_err_t SovereignDistro_Register(const char* name, const char* pkg, const char* init, const char* usp, sigma_distro_absorb_fn absorb);
void SovereignDistro_Absorb(const char* name);
void SovereignDistro_ListAll(void);

#endif /* SOVEREIGN_DISTRO_H */
