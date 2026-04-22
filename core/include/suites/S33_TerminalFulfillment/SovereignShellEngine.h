/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN SHELL ENGINE HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_SHELL_ENGINE_H
#define SOVEREIGN_SHELL_ENGINE_H

#include "sigma_types.h"

void SovereignShellEngine_Init (void);
void sigma_shell_highlight     (const char* input);
void sigma_shell_suggest       (const char* partial);
void sigma_shell_session       (void);

#endif /* SOVEREIGN_SHELL_ENGINE_H */
