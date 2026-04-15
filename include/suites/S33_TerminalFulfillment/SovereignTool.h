/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN TOOL INTERFACE (v1.0)
 * =========================================================================
 * Mission: Modular absorption of global developer/ops tools into SigmaOS.
 * Design: C11 / Zero-Dependency / Registry-Based.
 * =========================================================================
 */

#ifndef SOVEREIGN_TOOL_H
#define SOVEREIGN_TOOL_H

#include "suites/S01_Genesis/shards/sigma_types.h"

#define MAX_TOOLS 128
#define TOOL_NAME_MAX 32

typedef void (*sigma_tool_absorb_fn)(void);

typedef struct {
    char name[TOOL_NAME_MAX];
    char replacement[64];
    sigma_tool_absorb_fn absorb;
} sovereign_tool_t;

typedef struct {
    sovereign_tool_t tools[MAX_TOOLS];
    sigma_u32 tool_count;
} sovereign_tool_registry_t;

/* Public API */
void SovereignTool_InitRegistry(void);
sigma_err_t SovereignTool_Register(const char* name, const char* replacement, sigma_tool_absorb_fn absorb);
void SovereignTool_Absorb(const char* name);
void SovereignTool_ListAll(void);

#endif /* SOVEREIGN_TOOL_H */
