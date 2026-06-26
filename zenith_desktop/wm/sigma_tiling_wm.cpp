/**
 * @file sigma_tiling_wm.cpp
 * @brief Zenith Auto-Tiling Window Manager
 *
 * Competitor Inspiration:
 *  - i3/Sway: Manual tiling with tree-based layout containers
 *  - Hyprland: Dynamic tiling with smooth animations and gaps
 *  - AwesomeWM: Lua-scriptable tiling layouts
 *  - PaperWM: Scrolling workspace with spatial navigation
 *  - Windows 11 Snap Layouts: Quick-tile zones with hover previews
 *  - macOS Stage Manager: Focus groups with side-stage
 *
 * Provides multiple tiling modes (BSP, columns, Fibonacci, floating)
 * with workspace support, window gaps, and animated transitions.
 */

#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_theme.h"

namespace sigma {
namespace wm {

// ─── Layout Modes ────────────────────────────────────────────────────────────
typedef enum : sigma_u32 {
    LAYOUT_BSP          = 0,   // Binary space partition (i3-style manual)
    LAYOUT_COLUMNS      = 1,   // Master + stack columns (dwm-style)
    LAYOUT_FIBONACCI    = 2,   // Spiral layout
    LAYOUT_FLOATING     = 3,   // Traditional floating (macOS-style)
    LAYOUT_MONOCLE      = 4,   // Fullscreen stack (one visible at a time)
    LAYOUT_GRID         = 5,   // Equal grid cells
    LAYOUT_MASTER_STACK = 6,   // Dynamic master+stack (Hyprland/dwm style)
} LayoutMode;

// ─── Direction for Navigation/Split ──────────────────────────────────────────
typedef enum : sigma_u32 {
    DIR_LEFT  = 0,
    DIR_RIGHT = 1,
    DIR_UP    = 2,
    DIR_DOWN  = 3,
} Direction;

// ─── Window Node (BSP tree node) ─────────────────────────────────────────────
struct Rect {
    sigma_i32 x, y;
    sigma_u32 w, h;
};

struct WMWindow {
    sigma_u32 surface_id;       // Link to Wayland surface
    Rect      frame;            // Position + size on screen
    sigma_bool focused;
    sigma_bool fullscreen;
    sigma_bool floating;        // Override tiling for this window
    sigma_u32 workspace_id;
    char      title[128];
    sigma_u32 min_width;        // Size hints
    sigma_u32 min_height;
    sigma_u32 border_color;     // Active vs inactive tracking
    float     opacity;          // For fade animations
};

// ─── BSP Tree Node ──────────────────────────────────────────────────────────
typedef enum : sigma_u32 {
    NODE_LEAF      = 0,
    NODE_SPLIT_H   = 1,   // Horizontal split (children are left/right)
    NODE_SPLIT_V   = 2,   // Vertical split (children are top/bottom)
} NodeType;

struct BSPNode {
    NodeType    type;
    Rect        area;          // Allocated screen area for this node
    sigma_u32   window_idx;    // Valid when type == NODE_LEAF
    sigma_i32   child_a;       // Index into node pool (-1 = none)
    sigma_i32   child_b;
    float       split_ratio;   // 0.0–1.0, default 0.5
};

// ─── Workspace ───────────────────────────────────────────────────────────────
#define MAX_WORKSPACES 10
#define MAX_WINDOWS    256
#define MAX_BSP_NODES  512

struct Workspace {
    sigma_u32  id;
    char       name[32];
    LayoutMode layout;
    sigma_u32  window_ids[MAX_WINDOWS];
    sigma_u32  window_count;
    sigma_i32  root_node;      // Root of BSP tree
    sigma_bool active;
};

// ─── Window Manager State ────────────────────────────────────────────────────
struct WMState {
    WMWindow   windows[MAX_WINDOWS];
    sigma_u32  window_count;
    BSPNode    nodes[MAX_BSP_NODES];
    sigma_u32  node_count;
    Workspace  workspaces[MAX_WORKSPACES];
    sigma_u32  active_workspace;
    sigma_u32  focused_window;
    sigma_u32  gap_outer;       // Outer gaps (Hyprland-style)
    sigma_u32  gap_inner;       // Inner gaps between windows
    Rect       screen;          // Output dimensions
};

static WMState g_wm;

// ─── Init ────────────────────────────────────────────────────────────────────
sigma_status wm_init(sigma_u32 screen_w, sigma_u32 screen_h) {
    g_wm.window_count     = 0;
    g_wm.node_count       = 0;
    g_wm.active_workspace = 0;
    g_wm.focused_window   = 0;
    g_wm.gap_outer        = 8;
    g_wm.gap_inner        = 4;
    g_wm.screen           = {0, 0, screen_w, screen_h};

    // Initialize workspaces (i3-style: 1–10)
    for (sigma_u32 i = 0; i < MAX_WORKSPACES; ++i) {
        g_wm.workspaces[i].id           = i;
        g_wm.workspaces[i].layout       = LAYOUT_BSP;
        g_wm.workspaces[i].window_count = 0;
        g_wm.workspaces[i].root_node    = -1;
        g_wm.workspaces[i].active       = (i == 0) ? SIGMA_TRUE : SIGMA_FALSE;
        // Name: "1", "2", ..., "10"
        if (i < 9) {
            g_wm.workspaces[i].name[0] = '1' + (char)i;
            g_wm.workspaces[i].name[1] = '\0';
        } else {
            g_wm.workspaces[i].name[0] = '1';
            g_wm.workspaces[i].name[1] = '0';
            g_wm.workspaces[i].name[2] = '\0';
        }
    }

    return SIGMA_SUCCESS;
}

// ─── BSP Node Allocation ─────────────────────────────────────────────────────
static sigma_i32 alloc_node() {
    if (g_wm.node_count >= MAX_BSP_NODES) return -1;
    sigma_i32 idx = (sigma_i32)g_wm.node_count++;
    g_wm.nodes[idx].child_a = -1;
    g_wm.nodes[idx].child_b = -1;
    g_wm.nodes[idx].split_ratio = 0.5f;
    return idx;
}

// ─── Apply BSP Layout (recursive) ───────────────────────────────────────────
static void apply_bsp(sigma_i32 node_idx) {
    if (node_idx < 0) return;
    BSPNode* n = &g_wm.nodes[node_idx];

    if (n->type == NODE_LEAF) {
        // Apply area to the window
        if (n->window_idx < g_wm.window_count) {
            WMWindow* w = &g_wm.windows[n->window_idx];
            w->frame.x = n->area.x + (sigma_i32)g_wm.gap_inner;
            w->frame.y = n->area.y + (sigma_i32)g_wm.gap_inner;
            w->frame.w = n->area.w - g_wm.gap_inner * 2;
            w->frame.h = n->area.h - g_wm.gap_inner * 2;
        }
        return;
    }

    // Split the area between children
    if (n->child_a >= 0 && n->child_b >= 0) {
        BSPNode* a = &g_wm.nodes[n->child_a];
        BSPNode* b = &g_wm.nodes[n->child_b];

        if (n->type == NODE_SPLIT_H) {
            sigma_u32 split_x = (sigma_u32)((float)n->area.w * n->split_ratio);
            a->area = {n->area.x, n->area.y, split_x, n->area.h};
            b->area = {n->area.x + (sigma_i32)split_x, n->area.y,
                       n->area.w - split_x, n->area.h};
        } else { // NODE_SPLIT_V
            sigma_u32 split_y = (sigma_u32)((float)n->area.h * n->split_ratio);
            a->area = {n->area.x, n->area.y, n->area.w, split_y};
            b->area = {n->area.x, n->area.y + (sigma_i32)split_y,
                       n->area.w, n->area.h - split_y};
        }

        apply_bsp(n->child_a);
        apply_bsp(n->child_b);
    }
}

// ─── Dynamic Layout: Master/Stack (dwm-style) ──────────────────────────────
static void apply_master_stack(Workspace* ws) {
    if (ws->window_count == 0) return;

    sigma_u32 master_count = 1; // Number of windows in the master area
    float master_ratio = 0.55f; // Master gets 55% of screen width

    sigma_u32 usable_w = g_wm.screen.w - g_wm.gap_outer * 2;
    sigma_u32 usable_h = g_wm.screen.h - g_wm.gap_outer * 2;
    sigma_i32 base_x   = g_wm.gap_outer;
    sigma_i32 base_y   = g_wm.gap_outer;

    for (sigma_u32 i = 0; i < ws->window_count; ++i) {
        WMWindow* w = &g_wm.windows[ws->window_ids[i]];
        if (w->floating || w->fullscreen) continue;

        if (i < master_count) {
            // Master area
            sigma_u32 h = usable_h / master_count;
            w->frame.x = base_x + g_wm.gap_inner;
            w->frame.y = base_y + (i * h) + g_wm.gap_inner;
            w->frame.w = (sigma_u32)((float)usable_w * master_ratio) - g_wm.gap_inner * 2;
            w->frame.h = h - g_wm.gap_inner * 2;
        } else {
            // Stack area
            sigma_u32 stack_count = ws->window_count - master_count;
            sigma_u32 h = usable_h / stack_count;
            sigma_u32 stack_x = base_x + (sigma_u32)((float)usable_w * master_ratio);
            sigma_u32 stack_w = usable_w - (stack_x - base_x);
            
            w->frame.x = stack_x + g_wm.gap_inner;
            w->frame.y = base_y + ((i - master_count) * h) + g_wm.gap_inner;
            w->frame.w = stack_w - g_wm.gap_inner * 2;
            w->frame.h = h - g_wm.gap_inner * 2;
        }
    }
}

// ─── Insert Window into BSP Tree ─────────────────────────────────────────────
static sigma_i32 insert_into_bsp(sigma_i32 root, sigma_u32 win_idx, sigma_bool split_h) {
    if (root < 0) {
        // First window — create a leaf
        sigma_i32 leaf = alloc_node();
        if (leaf < 0) return -1;
        g_wm.nodes[leaf].type = NODE_LEAF;
        g_wm.nodes[leaf].window_idx = win_idx;
        return leaf;
    }

    BSPNode* r = &g_wm.nodes[root];
    if (r->type == NODE_LEAF) {
        // Split the current leaf to make room
        sigma_i32 new_root = alloc_node();
        sigma_i32 new_leaf = alloc_node();
        if (new_root < 0 || new_leaf < 0) return root;

        g_wm.nodes[new_root].type = split_h ? NODE_SPLIT_H : NODE_SPLIT_V;
        g_wm.nodes[new_root].area = r->area;
        g_wm.nodes[new_root].child_a = root;
        g_wm.nodes[new_root].child_b = new_leaf;
        g_wm.nodes[new_root].split_ratio = 0.5f;

        g_wm.nodes[new_leaf].type = NODE_LEAF;
        g_wm.nodes[new_leaf].window_idx = win_idx;

        return new_root;
    }

    // Internal node — insert into the larger child
    BSPNode* a = (r->child_a >= 0) ? &g_wm.nodes[r->child_a] : nullptr;
    BSPNode* b = (r->child_b >= 0) ? &g_wm.nodes[r->child_b] : nullptr;

    sigma_u32 area_a = a ? a->area.w * a->area.h : 0;
    sigma_u32 area_b = b ? b->area.w * b->area.h : 0;

    if (area_a >= area_b) {
        r->child_a = insert_into_bsp(r->child_a, win_idx, !split_h);
    } else {
        r->child_b = insert_into_bsp(r->child_b, win_idx, !split_h);
    }
    return root;
}

// ─── Add Window ──────────────────────────────────────────────────────────────
sigma_status add_window(sigma_u32 surface_id, const char* title) {
    if (g_wm.window_count >= MAX_WINDOWS) return SIGMA_ERROR;

    sigma_u32 idx = g_wm.window_count++;
    WMWindow* w = &g_wm.windows[idx];
    w->surface_id   = surface_id;
    w->focused      = SIGMA_FALSE;
    w->fullscreen   = SIGMA_FALSE;
    w->floating     = SIGMA_FALSE;
    w->workspace_id = g_wm.active_workspace;
    w->min_width    = 200;
    w->min_height   = 150;

    sigma_u32 j = 0;
    if (title) while (title[j] && j < 127) { w->title[j] = title[j]; j++; }
    w->title[j] = '\0';

    // Insert into the active workspace BSP tree
    Workspace* ws = &g_wm.workspaces[g_wm.active_workspace];
    if (ws->window_count < MAX_WINDOWS) {
        ws->window_ids[ws->window_count++] = idx;
    }

    sigma_bool alternate = (ws->window_count % 2 == 0) ? SIGMA_TRUE : SIGMA_FALSE;
    ws->root_node = insert_into_bsp(ws->root_node, idx, alternate);

    // Recompute layout
    if (ws->layout == LAYOUT_BSP) {
        if (ws->root_node >= 0) {
            g_wm.nodes[ws->root_node].area = {
                (sigma_i32)g_wm.gap_outer, (sigma_i32)g_wm.gap_outer,
                g_wm.screen.w - g_wm.gap_outer * 2,
                g_wm.screen.h - g_wm.gap_outer * 2
            };
            apply_bsp(ws->root_node);
        }
    } else if (ws->layout == LAYOUT_MASTER_STACK) {
        apply_master_stack(ws);
    }

    // Unfocus all, focus the new window
    for (sigma_u32 i = 0; i < g_wm.window_count; ++i) {
        g_wm.windows[i].focused = SIGMA_FALSE;
        g_wm.windows[i].border_color = 0x333333FF; // Inactive
    }
    g_wm.focused_window = idx;
    w->focused = SIGMA_TRUE;
    w->border_color = 0x63B3EDFF; // Active Accent (Hyprland style)
    w->opacity = 0.0f; // Start transparent for fade-in animation
    
    // Stub: Signal compositor to animate opacity from 0.0 -> 1.0 over 200ms
    // sys_ipc_send(COMPOSITOR_IPC, ANIMATE_WINDOW_SPAWN, w->surface_id);

    return SIGMA_SUCCESS;
}

// ─── Remove Window ───────────────────────────────────────────────────────────
sigma_status remove_window(sigma_u32 surface_id) {
    for (sigma_u32 i = 0; i < g_wm.window_count; ++i) {
        if (g_wm.windows[i].surface_id == surface_id) {
            // Shift remaining
            for (sigma_u32 j = i; j < g_wm.window_count - 1; ++j) {
                g_wm.windows[j] = g_wm.windows[j + 1];
            }
            g_wm.window_count--;
            // TODO: Rebuild BSP tree for the workspace
            return SIGMA_SUCCESS;
        }
    }
    return SIGMA_ERROR;
}

// ─── Toggle Fullscreen ───────────────────────────────────────────────────────
sigma_status toggle_fullscreen(sigma_u32 surface_id) {
    for (sigma_u32 i = 0; i < g_wm.window_count; ++i) {
        if (g_wm.windows[i].surface_id == surface_id) {
            g_wm.windows[i].fullscreen = g_wm.windows[i].fullscreen ? SIGMA_FALSE : SIGMA_TRUE;
            if (g_wm.windows[i].fullscreen) {
                g_wm.windows[i].frame = {0, 0, g_wm.screen.w, g_wm.screen.h};
            }
            // If unfullscreened, layout will reapply on next retile
            return SIGMA_SUCCESS;
        }
    }
    return SIGMA_ERROR;
}

// ─── Toggle Floating ─────────────────────────────────────────────────────────
sigma_status toggle_floating(sigma_u32 surface_id) {
    for (sigma_u32 i = 0; i < g_wm.window_count; ++i) {
        if (g_wm.windows[i].surface_id == surface_id) {
            g_wm.windows[i].floating = g_wm.windows[i].floating ? SIGMA_FALSE : SIGMA_TRUE;
            return SIGMA_SUCCESS;
        }
    }
    return SIGMA_ERROR;
}

// ─── Switch Workspace ────────────────────────────────────────────────────────
sigma_status switch_workspace(sigma_u32 ws_id) {
    if (ws_id >= MAX_WORKSPACES) return SIGMA_ERROR;
    g_wm.workspaces[g_wm.active_workspace].active = SIGMA_FALSE;
    g_wm.active_workspace = ws_id;
    g_wm.workspaces[ws_id].active = SIGMA_TRUE;

    // Reapply BSP layout for this workspace
    Workspace* ws = &g_wm.workspaces[ws_id];
    if (ws->root_node >= 0) {
        g_wm.nodes[ws->root_node].area = {
            (sigma_i32)g_wm.gap_outer, (sigma_i32)g_wm.gap_outer,
            g_wm.screen.w - g_wm.gap_outer * 2,
            g_wm.screen.h - g_wm.gap_outer * 2
        };
        apply_bsp(ws->root_node);
    }
    return SIGMA_SUCCESS;
}

// ─── Move Window to Workspace ────────────────────────────────────────────────
sigma_status move_to_workspace(sigma_u32 surface_id, sigma_u32 ws_id) {
    if (ws_id >= MAX_WORKSPACES) return SIGMA_ERROR;
    for (sigma_u32 i = 0; i < g_wm.window_count; ++i) {
        if (g_wm.windows[i].surface_id == surface_id) {
            g_wm.windows[i].workspace_id = ws_id;
            return SIGMA_SUCCESS;
        }
    }
    return SIGMA_ERROR;
}

// ─── Focus Direction (i3-style: focus left/right/up/down) ────────────────────
sigma_status focus_direction(Direction dir) {
    if (g_wm.window_count == 0) return SIGMA_ERROR;

    WMWindow* current = &g_wm.windows[g_wm.focused_window];
    sigma_i32 best_idx = -1;
    sigma_u32 best_dist = 0xFFFFFFFF;

    for (sigma_u32 i = 0; i < g_wm.window_count; ++i) {
        if (i == g_wm.focused_window) continue;
        WMWindow* w = &g_wm.windows[i];
        if (w->workspace_id != g_wm.active_workspace) continue;

        sigma_i32 dx = w->frame.x - current->frame.x;
        sigma_i32 dy = w->frame.y - current->frame.y;

        sigma_bool valid = SIGMA_FALSE;
        sigma_u32 dist = 0;

        switch (dir) {
            case DIR_LEFT:  valid = (dx < 0); dist = (sigma_u32)(-dx); break;
            case DIR_RIGHT: valid = (dx > 0); dist = (sigma_u32)(dx);  break;
            case DIR_UP:    valid = (dy < 0); dist = (sigma_u32)(-dy); break;
            case DIR_DOWN:  valid = (dy > 0); dist = (sigma_u32)(dy);  break;
        }

        if (valid && dist < best_dist) {
            best_dist = dist;
            best_idx = (sigma_i32)i;
        }
    }

    if (best_idx >= 0) {
        g_wm.windows[g_wm.focused_window].focused = SIGMA_FALSE;
        g_wm.windows[g_wm.focused_window].border_color = 0x333333FF;
        
        g_wm.focused_window = (sigma_u32)best_idx;
        g_wm.windows[best_idx].focused = SIGMA_TRUE;
        g_wm.windows[best_idx].border_color = 0x63B3EDFF; // Active Accent
    }

    return SIGMA_SUCCESS;
}

// ─── Resize Split Ratio ──────────────────────────────────────────────────────
sigma_status resize_split(Direction dir, float delta) {
    // Walk BSP tree to find the split node adjacent to the focused window
    // and adjust its split_ratio by delta
    (void)dir; (void)delta;
    // Simplified: adjust the root node's split ratio
    Workspace* ws = &g_wm.workspaces[g_wm.active_workspace];
    if (ws->root_node >= 0) {
        BSPNode* root = &g_wm.nodes[ws->root_node];
        if (root->type != NODE_LEAF) {
            root->split_ratio += delta;
            if (root->split_ratio < 0.1f) root->split_ratio = 0.1f;
            if (root->split_ratio > 0.9f) root->split_ratio = 0.9f;
            apply_bsp(ws->root_node);
        }
    }
    return SIGMA_SUCCESS;
}

// ─── Auto-tile active workspace (profile-driven) ─────────────────────────────
sigma_status auto_tile(void) {
    Workspace* ws = &g_wm.workspaces[g_wm.active_workspace];
    if (ws->root_node < 0) return SIGMA_SUCCESS;

    g_wm.nodes[ws->root_node].area = {
        (sigma_i32)g_wm.gap_outer, (sigma_i32)g_wm.gap_outer,
        g_wm.screen.w - g_wm.gap_outer * 2,
        g_wm.screen.h - g_wm.gap_outer * 2
    };

    if (ws->layout == LAYOUT_BSP) {
        apply_bsp(ws->root_node);
    } else if (ws->layout == LAYOUT_MASTER_STACK) {
        apply_master_stack(ws);
    } else if (ws->layout == LAYOUT_GRID) {
        apply_master_stack(ws);
    }
    return SIGMA_SUCCESS;
}

// ─── Set Layout Mode ─────────────────────────────────────────────────────────
sigma_status set_layout(LayoutMode mode) {
    g_wm.workspaces[g_wm.active_workspace].layout = mode;
    // Rebuild layout tree
    return SIGMA_SUCCESS;
}

// ─── Set Gaps ────────────────────────────────────────────────────────────────
sigma_status set_gaps(sigma_u32 inner, sigma_u32 outer) {
    g_wm.gap_inner = inner;
    g_wm.gap_outer = outer;
    // Retile all
    Workspace* ws = &g_wm.workspaces[g_wm.active_workspace];
    if (ws->root_node >= 0) {
        g_wm.nodes[ws->root_node].area = {
            (sigma_i32)outer, (sigma_i32)outer,
            g_wm.screen.w - outer * 2,
            g_wm.screen.h - outer * 2
        };
        apply_bsp(ws->root_node);
    }
    return SIGMA_SUCCESS;
}

} // namespace wm
} // namespace sigma

extern "C" {
    sigma_status sigma_wm_init(sigma_u32 w, sigma_u32 h)            { return sigma::wm::wm_init(w, h); }
    sigma_status sigma_wm_add(sigma_u32 sid, const char* t)          { return sigma::wm::add_window(sid, t); }
    sigma_status sigma_wm_remove(sigma_u32 sid)                      { return sigma::wm::remove_window(sid); }
    sigma_status sigma_wm_fullscreen(sigma_u32 sid)                  { return sigma::wm::toggle_fullscreen(sid); }
    sigma_status sigma_wm_floating(sigma_u32 sid)                    { return sigma::wm::toggle_floating(sid); }
    sigma_status sigma_wm_switch_ws(sigma_u32 ws)                    { return sigma::wm::switch_workspace(ws); }
    sigma_status sigma_wm_move_ws(sigma_u32 sid, sigma_u32 ws)       { return sigma::wm::move_to_workspace(sid, ws); }
    sigma_status sigma_wm_focus(sigma_u32 dir)                       { return sigma::wm::focus_direction((sigma::wm::Direction)dir); }
    sigma_status sigma_wm_resize(sigma_u32 dir, float d)             { return sigma::wm::resize_split((sigma::wm::Direction)dir, d); }
    sigma_status sigma_wm_layout(sigma_u32 m)                        { return sigma::wm::set_layout((sigma::wm::LayoutMode)m); }
    sigma_status sigma_wm_gaps(sigma_u32 inner, sigma_u32 outer)     { return sigma::wm::set_gaps(inner, outer); }
    sigma_status sigma_wm_auto_tile(void)                            { return sigma::wm::auto_tile(); }
}
