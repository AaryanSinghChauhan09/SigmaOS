// =============================================================================
// SigmaOS — S02_ZenithUI — SovereignDisplayServer.c
// Wayland+X11 Hybrid Display Server Shard
// =============================================================================
// Competitor USPs Absorbed:
//   • Wayland (Linux)  — zero-copy GPU composition, no server-side rendering
//   • X11/Xorg         — legacy window protocol for backward compatibility
//   • Quartz Compositor (macOS) — per-window GPU surface, CoreAnimation sync
//   • DWM (Windows)   — Flip model, tearing-free VSync presentation
// Architecture:
//   • Each window owns a DMA-BUF / shared GPU surface (Wayland model)
//   • X11 shim translates legacy Xlib calls into Sovereign protocol
//   • GPU flip chain managed by S04_HAL GpuDriverStack
// =============================================================================

#include <sigma_types.h>


// ── Window Surface ──────────────────────────────────────────────────────────
typedef struct {
    uint32_t  surface_id;
    uint32_t  width, height;
    void*     gpu_dma_buf;      // Zero-copy GPU buffer (Wayland model)
    bool      vsync_enabled;
    bool      is_x11_client;   // True = legacy X11 shim active
} ZenithSurface;

// ── Protocol Opcodes ─────────────────────────────────────────────────────────
typedef enum {
    ZENITH_MSG_CREATE_SURFACE  = 1,
    ZENITH_MSG_ATTACH_BUFFER   = 2,
    ZENITH_MSG_COMMIT          = 3,   // Present surface to compositor
    ZENITH_MSG_DESTROY         = 4,
    ZENITH_MSG_MOVE_RESIZE     = 5,
    ZENITH_MSG_SET_FULLSCREEN  = 6,
} ZenithProtocolMsg;

// ── Public API ───────────────────────────────────────────────────────────────

// Init the Zenith display server, claim /dev/dri/card0 via S04_HAL
void display_server_init(void);

// Create a sovereign GPU-backed surface for a process
ZenithSurface* display_server_create_surface(uint32_t pid,
                                              uint32_t w, uint32_t h);

// Commit (flip) a surface buffer to the screen — VSync-gated
void display_server_commit_surface(ZenithSurface* surface);

// X11 shim: translate an Xlib MapWindow request into a Zenith surface
ZenithSurface* display_server_x11_shim(uint32_t x11_window_id);

// Destroy a surface and reclaim GPU VRAM
void display_server_destroy_surface(ZenithSurface* surface);


