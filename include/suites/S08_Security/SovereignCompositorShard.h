/* S SIGMAOS: SOVEREIGN COMPOSITOR SHARD HEADER */
#ifndef SOVEREIGN_COMPOSITOR_SHARD_H
#define SOVEREIGN_COMPOSITOR_SHARD_H
#include "suites/S01_Genesis/shards/sigma_types.h"

typedef struct { sigma_i32 x, y, w, h; } SigmaRect_t;
typedef enum { WIN_TYPE_TOPLEVEL, WIN_TYPE_POPUP, WIN_TYPE_TOOLTIP, WIN_TYPE_OVERLAY } SigmaWinType_t;

sigma_err_t sigma_compositor_create_window (const char* title, sigma_i32 x, sigma_i32 y, sigma_i32 w, sigma_i32 h, sigma_u32 pid);
void        sigma_compositor_render        (void);
void        sigma_compositor_set_opacity   (sigma_u32 win_id, sigma_f32 alpha);
void        SovereignCompositorShard_Init   (void);
void        SovereignCompositor_Audit       (void);

#endif
