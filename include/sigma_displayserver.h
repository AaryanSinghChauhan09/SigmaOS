/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN DISPLAY SERVER (S-DISPLAY)
 * =========================================================================
 * Mission: Zero-compositor silicon-native display protocol.
 * Competitor parity: Wayland / X11 / macOS Core Display.
 * ZERO-DEPENDENCY: Direct framebuffer orchestration; no X11/Wayland bloat.
 * =========================================================================
 */

#ifndef SIGMA_DISPLAYSERVER_H
#define SIGMA_DISPLAYSERVER_H

#include "./core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Display Output Modes --- */
#define SIGMA_DISPLAY_MODE_VGA      0x00u
#define SIGMA_DISPLAY_MODE_HDMI     0x01u
#define SIGMA_DISPLAY_MODE_DP       0x02u  /* DisplayPort  */
#define SIGMA_DISPLAY_MODE_VIRTUAL  0x03u  /* Headless/VM  */

typedef struct {
    sigma_u32 width;
    sigma_u32 height;
    sigma_u32 depth;         /* bits per pixel: 24 or 32 */
    sigma_u32 refresh_hz;
    sigma_u32 mode;          /* SIGMA_DISPLAY_MODE_*     */
    sigma_addr_t fb_addr;    /* Physical framebuffer base */
} sigma_display_mode_t;

typedef struct {
    sigma_display_mode_t active_mode;
    sigma_u32 display_count;
    sigma_u32 vsync_active;
} sigma_display_state_t;

/* --- Display Primitives --- */
void display_server_init(void);
void display_server_set_mode(const sigma_display_mode_t* mode);
void display_server_vsync_enable(sigma_u32 enable);
void display_server_blit(sigma_u32 x, sigma_u32 y, sigma_u32 w, sigma_u32 h, const void* pixels);
void display_server_flush(void);
const sigma_display_state_t* display_server_get_state(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_DISPLAYSERVER_H */
