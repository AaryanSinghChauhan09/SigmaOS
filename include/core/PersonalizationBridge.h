#ifndef PERSONALIZATION_BRIDGE_H
#define PERSONALIZATION_BRIDGE_H

#include "../../include/core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Personalization Bridge
 * Links kernel industrial profiles to the Zenith UI theme engine.
 */

void personalization_sync_ui(const char* profile_name);

#ifdef __cplusplus
}
#endif

#endif // PERSONALIZATION_BRIDGE_H
