// SigmaOS — sigma-ui-wayland: Sovereign Display Protocol
// Inspired by: Wayland/Weston, Mir display server
// Module: sigma-ui-wayland
// USP: No libwayland, no DBus, no EGL — native C surface registry
// Compositor manages surfaces as atomic C structs, GPU blit via function pointer

#ifndef SIGMA_UI_WAYLAND_H
#define SIGMA_UI_WAYLAND_H

#define SIGMA_WL_MAX_SURFACES   32
#define SIGMA_WL_MAX_CLIENTS    16
#define SIGMA_WL_NAME_LEN       32

typedef enum SigmaWLSurfaceState {
    WL_SURFACE_FREE      = 0,
    WL_SURFACE_MAPPED    = 1,
    WL_SURFACE_HIDDEN    = 2,
    WL_SURFACE_ANIMATING = 3
} SigmaWLSurfaceState;

typedef struct SigmaWLRect {
    int x, y;
    unsigned int w, h;
} SigmaWLRect;

// GPU blit: user-supplied function renders surface pixels to framebuffer
typedef void (*wl_blit_fn)(unsigned int surface_id, SigmaWLRect* dst,
                             unsigned char* fb, unsigned int fb_stride);

typedef struct SigmaWLSurface {
    unsigned int        surface_id;
    char                title[SIGMA_WL_NAME_LEN];
    SigmaWLRect         geometry;
    unsigned char*      pixel_buf;      // CPU-side pixel buffer
    unsigned int        pixel_stride;   // bytes per row
    SigmaWLSurfaceState state;
    unsigned int        z_order;        // higher = on top
    float               opacity;        // 0.0 – 1.0
    wl_blit_fn          blit;
    unsigned long       frame_count;
} SigmaWLSurface;

typedef struct SigmaWLCompositor {
    SigmaWLSurface surfaces[SIGMA_WL_MAX_SURFACES];
    unsigned int   surface_count;
    unsigned int   screen_w;
    unsigned int   screen_h;
    unsigned char* framebuffer;
    unsigned long  total_frames;
} SigmaWLCompositor;

static inline void wl_compositor_init(SigmaWLCompositor* c,
                                       unsigned int w, unsigned int h,
                                       unsigned char* fb) {
    c->surface_count = 0;
    c->screen_w      = w;
    c->screen_h      = h;
    c->framebuffer   = fb;
    c->total_frames  = 0;
    for (int i = 0; i < SIGMA_WL_MAX_SURFACES; i++)
        c->surfaces[i].state = WL_SURFACE_FREE;
}

static inline unsigned int wl_create_surface(SigmaWLCompositor* c,
                                               const char* title,
                                               int x, int y, unsigned int w, unsigned int h,
                                               unsigned char* pixels,
                                               wl_blit_fn blit) {
    for (unsigned int i = 0; i < SIGMA_WL_MAX_SURFACES; i++) {
        if (c->surfaces[i].state == WL_SURFACE_FREE) {
            SigmaWLSurface* s = &c->surfaces[i];
            s->surface_id     = i;
            s->geometry.x = x; s->geometry.y = y;
            s->geometry.w = w; s->geometry.h = h;
            s->pixel_buf  = pixels;
            s->pixel_stride = w * 4; // RGBA assumed
            s->state      = WL_SURFACE_MAPPED;
            s->z_order    = c->surface_count;
            s->opacity    = 1.0f;
            s->blit       = blit;
            s->frame_count = 0;
            for (int k = 0; k < SIGMA_WL_NAME_LEN - 1 && title[k]; k++) s->title[k] = title[k];
            c->surface_count++;
            return i;
        }
    }
    return 0xFFFFFFFF; // no free slot
}

// Composite all mapped surfaces in z_order
static inline void wl_composite_frame(SigmaWLCompositor* c) {
    // Simple painter's algorithm — z_order ascending
    for (unsigned int z = 0; z <= c->surface_count; z++) {
        for (unsigned int i = 0; i < SIGMA_WL_MAX_SURFACES; i++) {
            SigmaWLSurface* s = &c->surfaces[i];
            if (s->state != WL_SURFACE_MAPPED) continue;
            if (s->z_order != z) continue;
            if (s->blit) s->blit(s->surface_id, &s->geometry,
                                  c->framebuffer, c->screen_w * 4);
            s->frame_count++;
        }
    }
    c->total_frames++;
}

static inline void wl_destroy_surface(SigmaWLCompositor* c, unsigned int sid) {
    if (sid < SIGMA_WL_MAX_SURFACES)
        c->surfaces[sid].state = WL_SURFACE_FREE;
}

static inline void wl_move_surface(SigmaWLCompositor* c, unsigned int sid, int x, int y) {
    if (sid < SIGMA_WL_MAX_SURFACES && c->surfaces[sid].state != WL_SURFACE_FREE) {
        c->surfaces[sid].geometry.x = x;
        c->surfaces[sid].geometry.y = y;
    }
}

#endif /* SIGMA_UI_WAYLAND_H */
