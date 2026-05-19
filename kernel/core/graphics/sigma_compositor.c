/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: WAYLAND-EQUIVALENT COMPOSITOR
 * =============================================================================
 * Inspired by: Weston (Wayland reference compositor)
 *              Linux DRM/KMS subsystem
 * =============================================================================
 * Manages window surfaces, z-order, and compositing over the fbdev layer.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

#define MAX_WINDOWS 32

typedef struct {
    sigma_u32 window_id;
    sigma_u32 pid;
    sigma_s32 x, y;
    sigma_u32 width, height;
    sigma_u32 z_index;
    sigma_u8* back_buffer;
    sigma_bool active;
    sigma_bool needs_redraw;
} sigma_window_t;

static sigma_window_t window_list[MAX_WINDOWS];
static sigma_u32 screen_width = 1920;
static sigma_u32 screen_height = 1080;

void compositor_init(void) {
    sigma_memset(window_list, 0, sizeof(window_list));
    sigma_printf("[compositor] Sigma Display Server initialized (%dx%d)\n", screen_width, screen_height);
}

int compositor_create_window(sigma_u32 pid, sigma_u32 width, sigma_u32 height) {
    for (sigma_u32 i = 0; i < MAX_WINDOWS; i++) {
        if (!window_list[i].active) {
            window_list[i].window_id = i + 1;
            window_list[i].pid = pid;
            window_list[i].width = width;
            window_list[i].height = height;
            
            /* Center window by default */
            window_list[i].x = (screen_width - width) / 2;
            window_list[i].y = (screen_height - height) / 2;
            
            /* Find highest Z-index */
            sigma_u32 max_z = 0;
            for (sigma_u32 j = 0; j < MAX_WINDOWS; j++) {
                if (window_list[j].active && window_list[j].z_index > max_z) {
                    max_z = window_list[j].z_index;
                }
            }
            window_list[i].z_index = max_z + 1;
            
            /* In a real kernel, this allocates shared memory via memfd/shm */
            window_list[i].back_buffer = SIGMA_NULL; 
            
            window_list[i].active = SIGMA_TRUE;
            window_list[i].needs_redraw = SIGMA_TRUE;
            
            sigma_printf("[compositor] Created Window %u (PID: %u, %ux%u at %d,%d, Z:%u)\n", 
                         window_list[i].window_id, pid, width, height, window_list[i].x, window_list[i].y, window_list[i].z_index);
            return (int)window_list[i].window_id;
        }
    }
    sigma_printf("[compositor] ERR: Maximum windows reached\n");
    return -1;
}

void compositor_destroy_window(sigma_u32 window_id) {
    for (sigma_u32 i = 0; i < MAX_WINDOWS; i++) {
        if (window_list[i].active && window_list[i].window_id == window_id) {
            window_list[i].active = SIGMA_FALSE;
            sigma_printf("[compositor] Destroyed Window %u\n", window_id);
            /* Trigger global redraw */
            return;
        }
    }
}

void compositor_damage_rect(sigma_u32 window_id, sigma_u32 x, sigma_u32 y, sigma_u32 w, sigma_u32 h) {
    for (sigma_u32 i = 0; i < MAX_WINDOWS; i++) {
        if (window_list[i].active && window_list[i].window_id == window_id) {
            window_list[i].needs_redraw = SIGMA_TRUE;
            sigma_printf("[compositor] Window %u flagged for redraw (Damage: %ux%u)\n", window_id, w, h);
            return;
        }
    }
}

void compositor_render_frame(void) {
    /* 
     * Simulated Compositing Loop:
     * 1. Clear background
     * 2. Sort active windows by Z-index (Painter's Algorithm)
     * 3. Copy back-buffers to the primary framebuffer (fbdev)
     */
    sigma_printf("[compositor] Rendering frame...\n");
    for (sigma_u32 z = 0; z < 100; z++) { /* Simplistic Z-sort */
        for (sigma_u32 i = 0; i < MAX_WINDOWS; i++) {
            if (window_list[i].active && window_list[i].z_index == z) {
                if (window_list[i].needs_redraw) {
                    sigma_printf("[compositor] Blitting Window %u to screen\n", window_list[i].window_id);
                    window_list[i].needs_redraw = SIGMA_FALSE;
                }
            }
        }
    }
    sigma_printf("[compositor] Frame rendered and swapped to hardware\n");
}
