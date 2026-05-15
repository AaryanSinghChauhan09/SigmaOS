// SigmaOS Sovereign Touchscreen & Input Stack
// Absorbs evdev (Linux), HID (Windows), IOKit Multitouch (macOS)
// Modular C11 — feeds directly into ZenithUI compositor and gesture recognizer

#include "../../../../../include/core/sigma_types.h"


#define SIGMA_MAX_TOUCH_POINTS 10

typedef struct {
    uint32_t  touch_id;
    float     x, y;           // Normalized 0.0–1.0 screen coordinates
    float     pressure;        // 0.0–1.0 force level
    bool      is_active;
} SigmaTouchPoint;

typedef enum {
    SIGMA_GESTURE_TAP            = 0,
    SIGMA_GESTURE_LONG_PRESS     = 1,
    SIGMA_GESTURE_SWIPE          = 2,
    SIGMA_GESTURE_PINCH_ZOOM     = 3,
    SIGMA_GESTURE_ROTATE         = 4,
    SIGMA_GESTURE_THREE_FINGER   = 5,
} SigmaGestureType;

static SigmaTouchPoint touch_state[SIGMA_MAX_TOUCH_POINTS];

// Initialize HID multitouch device via USB/I2C bus
void input_init_touchscreen(void);

// Poll hardware for raw touch events from the I2C interrupt
void input_poll_touch_events(void);

// Classify raw touch points into a semantic gesture type
SigmaGestureType input_classify_gesture(SigmaTouchPoint* points, uint8_t count);

// Dispatch gesture to the ZenithUI window manager compositor
void input_dispatch_gesture(SigmaGestureType gesture, SigmaTouchPoint* origin);



