/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN CONTEXTUAL MENUS (S-CONTEXTMENU)
 * =========================================================================
 * Mission: AI-driven contextual menus that dynamically prune irrelevant
 * options and predict the user's intended action.
 * =========================================================================
 */

#ifndef SIGMA_CONTEXTMENU_H
#define SIGMA_CONTEXTMENU_H

#include "./core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Context Menu Primitives --- */
void contextmenu_init(void);
void contextmenu_invoke(uint32_t target_id, uint32_t x, uint32_t y);
void contextmenu_dismiss(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_CONTEXTMENU_H */
