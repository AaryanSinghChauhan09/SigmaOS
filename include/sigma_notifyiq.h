/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN NOTIFICATION INTELLIGENCE (S-NOTIFYIQ)
 * =========================================================================
 * Mission: Replace dumb notification flooding with an AI-curated priority
 * system that batches, summarizes, and silences based on context.
 * =========================================================================
 */

#ifndef SIGMA_NOTIFYIQ_H
#define SIGMA_NOTIFYIQ_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    NOTIFY_PRIORITY_CRITICAL,
    NOTIFY_PRIORITY_HIGH,
    NOTIFY_PRIORITY_NORMAL,
    NOTIFY_PRIORITY_LOW,
    NOTIFY_PRIORITY_BATCHED
} sigma_notify_priority_t;

/* --- NotifyIQ Primitives --- */
void notifyiq_init(void);
void notifyiq_push(const char* source, const char* message, sigma_notify_priority_t priority);
void notifyiq_deliver_batch(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_NOTIFYIQ_H */
