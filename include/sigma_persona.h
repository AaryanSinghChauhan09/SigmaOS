/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN PERSONA ENGINE (S-PERSONA)
 * =========================================================================
 * Mission: Deeply integrated system personalization, automation, and 
 * user experience adaptation based on real-time telemetry and user habits.
 * =========================================================================
 */

#ifndef SIGMA_PERSONA_H
#define SIGMA_PERSONA_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    PERSONA_MODE_DEVELOPER,
    PERSONA_MODE_GAMER,
    PERSONA_MODE_CREATOR,
    PERSONA_MODE_ENTERPRISE
} sigma_persona_mode_t;

/* --- Persona Primitives --- */
void persona_init(void);
void persona_set_mode(sigma_persona_mode_t mode);
void persona_automate_workflow(uint32_t trigger_id);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_PERSONA_H */
