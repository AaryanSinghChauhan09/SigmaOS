// =============================================================================
// SigmaOS — S02_ZenithUI — SovereignDesktopShell.c
// Industrial-Grade Unified desktop Environment
// =============================================================================
// Competitor USPs Absorbed:
//   • Windows Explorer — Integrated File Browsing + Taskbar
//   • macOS Finder      — Column-view navigation and Spotlight integration
//   • GNOME Shell       — Clean, extension-based modularity
//   • visionOS Dashboard — Floating volumetric window management
// Architecture:
//   • Native C11 rendering via SovereignGraphicsBridge
//   • Asynchronous thumbnailer (Zero-latency folder browsing)
//   • Integrated Spotlight (S02) and Control Center (S02) hooks
// =============================================================================

#include <sigma_types.h>


#define MAX_WINDOWS         128
#define TASKBAR_HEIGHT      48

typedef struct {
    uint32_t window_id;
    char     title[128];
    int      x, y, w, h;
    uint32_t z_order;
    bool     is_focused;
    bool     is_minimized;
} ZenithWindow;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise the desktop shell (Mount taskbar, desktop icons, etc)
void shell_init(void);

// Create a new high-fidelity window for a .sab app bundle
ZenithWindow* shell_create_window(const char* title, int w, int h);

// Handle global desktop events (Mouse click, Hotkeys)
void shell_process_events(void);

// Render the entire desktop (Taskbar + Windows + Desktop icons)
void shell_render_frame(void);

// Pin an app to the Sovereign Dock (macOS parity)
void shell_dock_pin(const char* app_id);

// Switch between Virtual Workspaces (S13 Sentiment predictions)
void shell_switch_workspace(uint8_t workspace_id);

// Broadcast shell state to S12 Continuity for "Handoff" across screens
void shell_sync_state_to_mesh(void);


