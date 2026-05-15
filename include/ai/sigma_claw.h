#ifndef SIGMA_CLAW_H
#define SIGMA_CLAW_H

#include "include/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    CLAW_SANDBOX_STRICT,
    CLAW_SANDBOX_NON_MAIN,
    CLAW_SANDBOX_OPEN
} sigma_claw_sandbox_mode_t;

typedef struct {
    char agent_id[32];
    sigma_claw_sandbox_mode_t sandbox_mode;
    bool voice_wake_enabled;
    bool live_canvas_enabled;
} sigma_claw_config_t;

/* --- Sovereign Claw Gateway Primitives --- */
/* The Lobster Way 🦞 */
void claw_gateway_init(void);
void claw_route_message(const char* channel, const char* message);
void claw_render_canvas(void);
void claw_execute_tool(const char* tool_name, const char* payload);
void claw_sandbox_policy(sigma_claw_sandbox_mode_t mode);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_CLAW_H */
