/* S SIGMAOS: SOVEREIGN WINDOW SHARD HEADER */
#ifndef SOVEREIGN_WINDOW_SHARD_H
#define SOVEREIGN_WINDOW_SHARD_H
#include "suites/S01_Genesis/shards/sigma_types.h"

typedef enum { LAYOUT_FLOAT, LAYOUT_TILE, LAYOUT_SNAP, LAYOUT_STAGE } SigmaLayout_t;
typedef enum { SNAP_LEFT, SNAP_RIGHT, SNAP_TOP, SNAP_BOTTOM, SNAP_QUAD_TL, SNAP_QUAD_TR, SNAP_QUAD_BL, SNAP_QUAD_BR } SigmaSnapPos_t;

void sigma_wm_set_layout      (SigmaLayout_t layout);
void sigma_wm_snap_window     (sigma_u32 win_id, SigmaSnapPos_t pos);
void sigma_wm_cycle_workspace (sigma_u32 ws_id);
void SovereignWindowShard_Init (void);
void SovereignWindow_Audit     (void);

#endif
