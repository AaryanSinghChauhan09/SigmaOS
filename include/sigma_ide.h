/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN NATIVE IDE (S-IDE)
 * =========================================================================
 * Mission: OS-native, lightweight development environment for kernel shards.
 * =========================================================================
 */

#ifndef SIGMA_IDE_H
#define SIGMA_IDE_H

#include "../include/core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    char current_file[64];
    uint32_t cursor_line;
    uint32_t cursor_col;
    bool is_dirty;
} sigma_ide_state_t;

/* --- IDE Primitives --- */
void ide_init(void);
void ide_open_shard(uint32_t shard_id);
void ide_compile_active_shard(void);
void ide_render_ui(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_IDE_H */
