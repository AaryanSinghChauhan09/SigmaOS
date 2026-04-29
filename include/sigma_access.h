/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ACCESSIBILITY CORE (S-ACCESS)
 * =========================================================================
 * Mission: Universal usability features built into the kernel, providing
 * zero-overhead screen reading, high-contrast, and motor-assist layers.
 * =========================================================================
 */

#ifndef SIGMA_ACCESS_H
#define SIGMA_ACCESS_H

#include <sigma_types.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    ACCESS_MODE_SCREEN_READER,
    ACCESS_MODE_HIGH_CONTRAST,
    ACCESS_MODE_MOTOR_ASSIST
} sigma_access_mode_t;

/* --- Accessibility Primitives --- */
void access_init(void);
void access_enable_mode(sigma_access_mode_t mode);
void access_announce_ui_element(const char* element_desc);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_ACCESS_H */
