/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN SYSTEM AUTOMATION (S-AUTO)
 * =========================================================================
 * Mission: Event-driven shard automation and autonomous task hub.
 * =========================================================================
 */

#ifndef SIGMA_AUTO_H
#define SIGMA_AUTO_H

#include "./core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    uint32_t event_id;
    uint32_t target_shard_id;
    uint32_t action_mask;
    bool is_periodic;
} sigma_automation_rule_t;

/* --- Automation Primitives --- */
void auto_init(void);
void auto_register_rule(uint32_t event_id, uint32_t shard_id, uint32_t action);
void auto_trigger_event(uint32_t event_id);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_AUTO_H */
