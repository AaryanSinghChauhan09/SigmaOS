/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN VISUAL SCRIPTING (S-VISSCRIPT)
 * =========================================================================
 * Mission: A node-based visual scripting interface built directly into 
 * the OS to democratize automation and shell scripting for non-programmers.
 * =========================================================================
 */

#ifndef SIGMA_VISSCRIPT_H
#define SIGMA_VISSCRIPT_H

#include "core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    uint32_t node_id;
    char operation[32];
    uint32_t next_node_id;
} sigma_visscript_node_t;

/* --- Visual Scripting Primitives --- */
void visscript_init(void);
void visscript_execute_graph(const sigma_visscript_node_t* start_node);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_VISSCRIPT_H */
