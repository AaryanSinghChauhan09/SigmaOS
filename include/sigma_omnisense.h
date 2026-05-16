/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN OMNI-SENSE HUB (S-OMNISENSE)
 * =========================================================================
 * Mission: A unified hardware sensor matrix that collects ambient data
 * to automatically adapt the system's UI and power profiles in real-time.
 * =========================================================================
 */

#ifndef SIGMA_OMNISENSE_H
#define SIGMA_OMNISENSE_H

#include "./core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    uint32_t ambient_light_lux;
    int32_t ambient_temp_celsius;
    bool user_presence_detected;
} sigma_omnisense_data_t;

/* --- Omni-Sense Primitives --- */
void omnisense_init(void);
void omnisense_poll_sensors(void);
void omnisense_adapt_system(const sigma_omnisense_data_t* data);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_OMNISENSE_H */
