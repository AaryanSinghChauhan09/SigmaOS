#include <stdint.h>
#include "sigma_libc.h"

// ---------------------------------------------------------
// SigmaOS Zenith Widget Engine
// USP: Kernel-native dashboard widgets providing instant
// system telemetry without spawning heavy user-space apps.
// ---------------------------------------------------------

#define MAX_WIDGETS 16

typedef enum {
    WIDGET_TYPE_SYS_MONITOR,  // CPU, RAM, Tokens
    WIDGET_TYPE_NETWORK,      // Active Mesh Nodes, Bandwidth
    WIDGET_TYPE_CLOCK,        // Time, Date
    WIDGET_TYPE_AI_STATS      // NPU Usage, Active Tensors
} widget_type_t;

typedef struct {
    uint32_t widget_id;
    widget_type_t type;
    int32_t  x, y;
    uint32_t width, height;
    uint8_t  is_visible;
    uint8_t  refresh_rate_ms; // How often to redraw
    uint64_t last_draw_tick;
} zenith_widget_t;

static zenith_widget_t dashboard_widgets[MAX_WIDGETS];
static uint32_t widget_count = 0;

extern void audit_chain_append(uint32_t pid, uint8_t level, const char* msg);

// Register a new widget to the Zenith Dashboard
void widget_engine_add(widget_type_t type, int32_t x, int32_t y, uint32_t w, uint32_t h) {
    if (widget_count >= MAX_WIDGETS) return;
    
    zenith_widget_t* wid = &dashboard_widgets[widget_count++];
    wid->widget_id = widget_count;
    wid->type = type;
    wid->x = x;
    wid->y = y;
    wid->width = w;
    wid->height = h;
    wid->is_visible = 1;
    wid->refresh_rate_ms = 1000; // Default 1Hz refresh
    wid->last_draw_tick = 0;
}

// Initialise the default widget layout
void widget_engine_init(void) {
    widget_engine_add(WIDGET_TYPE_CLOCK, 20, 20, 200, 100);
    widget_engine_add(WIDGET_TYPE_SYS_MONITOR, 20, 140, 200, 300);
    audit_chain_append(0, 1, "ZENITH_WIDGET_ENGINE_STARTED");
}

// Draw a specific widget (mock implementation)
static void draw_widget(zenith_widget_t* wid) {
    // In reality: 
    // 1. Fetch live system telemetry from kernel structs.
    // 2. Render text/graphs directly to the Zenith framebuffer.
    // 3. Apply glassmorphism backing behind the widget.
}

// Called by the Zenith Compositor render loop
void widget_engine_render(uint64_t current_tick) {
    for (uint32_t i = 0; i < widget_count; i++) {
        zenith_widget_t* wid = &dashboard_widgets[i];
        if (wid->is_visible && (current_tick - wid->last_draw_tick > wid->refresh_rate_ms)) {
            draw_widget(wid);
            wid->last_draw_tick = current_tick;
        }
    }
}
