/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN KERNEL USP INTERFACE (v1.0)
 * =========================================================================
 * Mission: Modular implementation of global Kernel USPs (eBPF, XDP, etc.)
 * Design: C11 / Zero-Dependency / Registry-Based.
 * =========================================================================
 */

#ifndef SOVEREIGN_USP_H
#define SOVEREIGN_USP_H

#include "sigma_types.h"

#define MAX_USPS 64
#define USP_NAME_MAX 32

typedef void (*sigma_usp_show_fn)(void);

typedef struct {
    char name[USP_NAME_MAX];
    char description[128];
    sigma_usp_show_fn show;
} sovereign_usp_t;

typedef struct {
    sovereign_usp_t usps[MAX_USPS];
    sigma_u32 usp_count;
} sovereign_usp_registry_t;

/* Public API */
void SovereignUSP_InitRegistry(void);
sigma_err_t SovereignUSP_Register(const char* name, const char* desc, sigma_usp_show_fn show);
void SovereignUSP_Show(const char* name);
void SovereignUSP_ListAll(void);

#endif /* SOVEREIGN_USP_H */
