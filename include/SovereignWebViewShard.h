/* Σ SIGMAOS: SOVEREIGN WEBVIEW SHARD HEADER */
#ifndef SOVEREIGN_WEBVIEW_SHARD_H
#define SOVEREIGN_WEBVIEW_SHARD_H
#include "sigma_types.h"

sigma_err_t sigma_web_load (const char* url);
void        sigma_web_render_frame (sigma_u32 view_id);
void        SovereignWebViewShard_Init (void);
void        SovereignWebView_Audit      (void);

#endif
