#include <stdint.h>
#include <stddef.h>

// ---------------------------------------------------------
// SigmaOS Zenith Window Manager
// USP: Tiling and Floating Window Management with
// zero-latency physics-based snapping
// ---------------------------------------------------------

#define MAX_WORKSPACES 9

typedef enum {
    LAYOUT_FLOATING,
    LAYOUT_TILING_BSP,  // Binary Space Partitioning
    LAYOUT_TILING_GRID,
    LAYOUT_MONOCLE      // Full screen focused
} wm_layout_t;

typedef struct {
    uint8_t     workspace_id;
    wm_layout_t layout;
    uint32_t    focused_window_id;
    uint16_t    gap_size; // Gaps between tiled windows
} wm_workspace_t;

static wm_workspace_t workspaces[MAX_WORKSPACES];
static uint8_t active_workspace = 0;

extern void audit_chain_append(uint32_t pid, uint8_t level, const char* msg);

// Initialise window manager with declarative default layouts
void wm_init(void) {
    for(int i=0; i<MAX_WORKSPACES; i++) {
        workspaces[i].workspace_id = i;
        workspaces[i].layout = (i == 0) ? LAYOUT_TILING_BSP : LAYOUT_FLOATING;
        workspaces[i].focused_window_id = 0;
        workspaces[i].gap_size = 8; // 8px gaps by default
    }
    audit_chain_append(0, 1, "ZENITH_WM_INITIALIZED");
}

// Switch workspaces (UX feature)
void wm_switch_workspace(uint8_t new_ws) {
    if (new_ws >= MAX_WORKSPACES) return;
    active_workspace = new_ws;
    
    // In real implementation: tell compositor to hide old windows and show new ones
    // zenith_update_visibility(active_workspace);
    audit_chain_append(0, 1, "WORKSPACE_SWITCHED");
}

// Hot-swap the layout engine for the current workspace
void wm_set_layout(wm_layout_t new_layout) {
    workspaces[active_workspace].layout = new_layout;
    
    // Trigger immediate recalculation of window bounds based on new layout
    // wm_recalculate_bounds(active_workspace);
}
