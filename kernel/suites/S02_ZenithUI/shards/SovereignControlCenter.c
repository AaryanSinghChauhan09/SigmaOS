// =============================================================================
// SigmaOS — S02_ZenithUI — SovereignControlCenter.c
// Industrial-Grade UI Dashboard & Control Center
// =============================================================================
// Competitor USPs Absorbed:
//   • macOS Control Center — unified toggles for Net, Audio, Focus
//   • Windows 11 Action Center — fluent flyouts and interactive notifications
//   • iOS Dashboard     — widget-based layout with real-time stats
//   • KDE Plasma Widgets— customisable, modular UI widgets
// Aesthetic Goals: 
//   • Gaussian Blur Background (Aero Glass vNext)
//   • Reactive Motion Shaders
//   • Unified Hardware Toggle Hub
// =============================================================================

#include "suites/S01_Genesis/shards/sigma_types.h"


#define MAX_WIDGETS     32
#define DASH_WIDTH      400
#define DASH_HEIGHT     800

// ── Widget Type ───────────────────────────────────────────────────────────────
typedef enum {
    WIDGET_NET_STATUS   = 0,
    WIDGET_AUDIO_MIXER  = 1,
    WIDGET_BATTERY      = 2,
    WIDGET_CPU_STATS    = 3,
    WIDGET_QUICK_TOGGLE = 4
} ZenithWidgetType;

// ── Dashboard Component ──────────────────────────────────────────────────────
typedef struct {
    uint32_t         widget_id;
    ZenithWidgetType type;
    int              x, y, w, h;
    bool             is_active;
    char             label[64];
} ZenithWidget;

static ZenithWidget dashboard[MAX_WIDGETS];
static uint32_t     widget_count = 0;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise the Control Center layout (visionOS style)
void control_center_init(void);

// Add a widget to the dashboard
void control_center_register_widget(ZenithWidget* widget);

// Render the dashboard with Gaussian Blur (Quartz-style)
void control_center_render_flyout(void);

// Receive input event in dashboard context
bool control_center_process_input(int mouse_x, int mouse_y, uint8_t action);

// Update a widget's real-time stat (e.g., CPU load)
void control_center_update_data(uint32_t widget_id, void* raw_data);

// Toggle the entire dashboard visibility (Hot corner or Win+A style)
void control_center_toggle_visibility(bool visible);



