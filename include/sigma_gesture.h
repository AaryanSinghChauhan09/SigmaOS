/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN GESTURAL UX (S-GESTURE)
 * =========================================================================
 * Mission: Zero-latency, camera-based hand tracking and gesture 
 * recognition for completely touchless system navigation.
 * =========================================================================
 */

#ifndef SIGMA_GESTURE_H
#define SIGMA_GESTURE_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    GESTURE_SWIPE_LEFT,
    GESTURE_SWIPE_RIGHT,
    GESTURE_PINCH_ZOOM,
    GESTURE_PALM_STOP
} sigma_gesture_type_t;

/* --- Gesture UX Primitives --- */
void gesture_init(void);
void gesture_process_camera_frame(const void* frame_data);
void gesture_dispatch_event(sigma_gesture_type_t gesture);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_GESTURE_H */
