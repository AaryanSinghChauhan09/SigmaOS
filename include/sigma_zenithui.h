/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH UI COMPOSITOR (ZCSR-ZENITH)
 * =========================================================================
 * Mission: Ultra-low-latency morphic UI compositor with glassmorphism support.
 * Competitor parity: Windows DWM, macOS WindowServer, Wayland Compositors.
 * ZERO-DEPENDENCY: Direct silicon-native rendering; no OpenGL/Vulkan library.
 * =========================================================================
 */

#ifndef SIGMA_ZENITHUI_H
#define SIGMA_ZENITHUI_H

#include "sigma_types.h"
#include "sigma_displayserver.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- UI Component Types --- */
#define SIGMA_UI_WINDOW      0x00u
#define SIGMA_UI_PANEL       0x01u
#define SIGMA_UI_BUTTON      0x02u
#define SIGMA_UI_WIDGET      0x03u

/* --- Morphic Flags --- */
#define SIGMA_UI_FLAG_GLASS     (1u << 0)  /* Acrylic/Glassmorphism blur    */
#define SIGMA_UI_FLAG_GLOW      (1u << 1)  /* Adaptive neon glow effects    */
#define SIGMA_UI_FLAG_MORPH     (1u << 2)  /* Fluid shape transitions       */
#define SIGMA_UI_FLAG_SHADOW    (1u << 3)  /* Dynamic depth shadows         */

#define SIGMA_UI_ELEMENT_MAX    128u
#define SIGMA_UI_NAME_LEN       32u

typedef struct {
    sigma_u32 id;
    char      name[SIGMA_UI_NAME_LEN];
    sigma_u32 type;            /* SIGMA_UI_* component type        */
    sigma_u32 x, y, w, h;      /* Geometry                         */
    sigma_u32 z_index;         /* Depth layer                      */
    sigma_u32 flags;           /* SIGMA_UI_FLAG_* morphic effects  */
    sigma_u32 opacity;         /* 0-255                            */
} sigma_ui_element_t;

typedef struct {
    sigma_ui_element_t elements[SIGMA_UI_ELEMENT_MAX];
    sigma_u32 count;
    sigma_u32 frame_count;     /* Rendered frames telemetry        */
    sigma_u32 active_glass;    /* Elements with glassmorphism active */
} sigma_zenith_state_t;

/* --- Zenith UI Primitives --- */
void      zenith_init(void);
sigma_u32 zenith_create_element(const char* name, sigma_u32 type, 
                                sigma_u32 x, sigma_u32 y, 
                                sigma_u32 w, sigma_u32 h);
void      zenith_set_flags(sigma_u32 id, sigma_u32 flags);
void      zenith_set_geometry(sigma_u32 id, sigma_u32 x, sigma_u32 y, sigma_u32 w, sigma_u32 h);
void      zenith_render_frame(void);
const sigma_zenith_state_t* zenith_get_state(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_ZENITHUI_H */
