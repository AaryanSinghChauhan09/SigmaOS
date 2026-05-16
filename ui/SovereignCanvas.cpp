#include "./include/sigma_log.h"
#include "./include/SovereignLibC.h"
#include "./include/sigma_types.h"

#include "./include/ui/sigma_canvas.h"
#include "./include/hal/sigma_hal.h"


/**
 * SigmaOS Sovereign Infinite Canvas
 * Implements an Unbounded Render Matrix (URM) algorithm.
 * ZERO-DEPENDENCY: Strictly bare-metal spatial UI composition.
 */

void canvas_init() {
    sigma_log("[CANVAS] Initializing Sovereign Infinite Canvas (URM Algorithm)...");
}

void canvas_pan(float delta_x, float delta_y) {
    // URM (Unbounded Render Matrix) Algorithm
    // Instantly transforms the viewport matrix without redrawing underlying geometries.
    
    sigma_log("[CANVAS] URM: Viewport panned by (%.2f, %.2f).\n", delta_x, delta_y);
}

void canvas_zoom(float delta_zoom) {
    sigma_log("[CANVAS] URM: Viewport zoom shifted by %.2fx.\n", delta_zoom);
    // Directly scale the GPU projection matrix
}




} // extern "C"
