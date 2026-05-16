/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN QUICK ACTIONS BAR (S-QUICKACT)
 * =========================================================================
 * Mission: A single-keystroke universal command palette (like Spotlight/
 * Krunner) deeply integrated with NeuralSearch, DeepLink, and TaskAutomator.
 * =========================================================================
 */

#ifndef SIGMA_QUICKACT_H
#define SIGMA_QUICKACT_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Quick Actions Primitives --- */
void quickact_init(void);
void quickact_invoke(void);
void quickact_process_input(const char* user_input);
void quickact_dismiss(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_QUICKACT_H */
