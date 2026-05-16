/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN PARENTAL CONTROLS (S-PARENTAL)
 * =========================================================================
 * Mission: Hardware-backed, cryptographically enforced usage restrictions
 * for child accounts â€” app filtering, time limits, and content gating.
 * =========================================================================
 */

#ifndef SIGMA_PARENTAL_H
#define SIGMA_PARENTAL_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Parental Controls Primitives --- */
void parental_init(void);
void parental_create_child_profile(const char* name);
void parental_set_app_whitelist(const char* profile_name, const uint32_t* app_ids, uint32_t count);
void parental_set_time_window(const char* profile_name, uint32_t start_hour, uint32_t end_hour);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_PARENTAL_H */
