/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN ONBOARDING WIZARD (S-ONBOARD)
 * =========================================================================
 * Mission: A first-boot guided setup that configures Persona, Biometrics,
 * Accessibility, Theme, and Network in a beautiful, step-by-step flow.
 * =========================================================================
 */

#ifndef SIGMA_ONBOARD_H
#define SIGMA_ONBOARD_H

#include "./core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Onboarding Primitives --- */
void onboard_init(void);
void onboard_start_wizard(void);
void onboard_complete_step(uint32_t step_id);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_ONBOARD_H */
