/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN FOCUS MODE (S-FOCUS)
 * =========================================================================
 * Mission: Hardware-level distraction blocking. Temporarily suspends
 * network traffic and IPC for all shards except the active user task.
 * =========================================================================
 */

#ifndef SIGMA_FOCUS_H
#define SIGMA_FOCUS_H

#include "../include/core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Focus Mode Primitives --- */
void focus_init(void);
void focus_engage(uint32_t active_app_id, uint32_t duration_minutes);
void focus_disengage(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_FOCUS_H */
