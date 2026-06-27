// SPDX-License-Identifier: GPL-2.0-or-later
// sigma_drm.cpp — DRM/KMS kernel interface for SigmaOS GPU subsystem
//
// Provides:
//   • Mode setting (resolution, refresh rate, connector detection)
//   • GEM (Graphics Execution Manager) — GPU memory allocation/tracking
//   • dma-buf — cross-subsystem buffer sharing (GPU↔camera↔codec)
//   • Framebuffer — fallback software rendering path
//
// Inspired by:
//   • Linux drivers/gpu/drm/drm_drv.c, drm_gem.c, drm_modes.c
//   • Mesa Gallium3D st/dri interface
//   • FreeBSD sys/dev/drm2/

#include "sigma_drm.h"
#include <stdint.h>
#include <stddef.h>
#include <string.h>
#include <stdbool.h>

// ── Constants ─────────────────────────────────────────────────────────────────

#define MAX_CONNECTORS  8
#define MAX_MODES       64
#define MAX_GEM_OBJECTS 4096
#define GEM_PAGE_SIZE   4096

// ── Display mode ──────────────────────────────────────────────────────────────

struct drm_mode {
    uint32_t hdisplay;    // horizontal pixels
    uint32_t vdisplay;    // vertical pixels
    uint32_t vrefresh;    // refresh rate in Hz
    uint32_t clock;       // pixel clock in kHz
    uint32_t hsync_start, hsync_end, htotal;
    uint32_t vsync_start, vsync_end, vtotal;
    uint32_t flags;
    char     name[32];
};

// ── Connector ─────────────────────────────────────────────────────────────────

typedef enum { CONN_HDMI, CONN_DP, CONN_VGA, CONN_LVDS } connector_type_t;
typedef enum { STATUS_CONNECTED, STATUS_DISCONNECTED, STATUS_UNKNOWN } conn_status_t;

struct drm_connector {
    uint32_t         id;
    connector_type_t type;
    conn_status_t    status;
    struct drm_mode  modes[MAX_MODES];
    uint32_t         num_modes;
    struct drm_mode  preferred;
    uint32_t         active_fb;   // currently displayed framebuffer GEM handle
};

static struct drm_connector g_connectors[MAX_CONNECTORS];
static uint32_t             g_num_connectors = 0;

// ── GEM object ────────────────────────────────────────────────────────────────

struct gem_object {
    uint32_t  handle;
    uintptr_t gpu_pa;       // GPU physical address (iommu-mapped)
    uintptr_t cpu_va;       // CPU virtual address for mmap
    size_t    size;
    uint32_t  refcount;
    bool      in_use;
    uint32_t  domain;       // SIGMA_GEM_DOMAIN_CPU / GPU / VRAM
};

static struct gem_object g_gems[MAX_GEM_OBJECTS];

// ── Framebuffer ───────────────────────────────────────────────────────────────

struct sigma_framebuffer {
    uint32_t  handle;       // GEM handle
    uint32_t  width;
    uint32_t  height;
    uint32_t  stride;       // bytes per row
    uint32_t  format;       // DRM_FORMAT_XRGB8888 etc.
};

// ── GEM allocator ─────────────────────────────────────────────────────────────

extern uintptr_t sigma_pmm_alloc_contiguous(size_t bytes);  // from sigma_pmm.cpp

int sigma_drm_gem_create(size_t size, uint32_t *out_handle) {
    if (!out_handle || size == 0) return -1;
    // Round up to page boundary
    size = (size + GEM_PAGE_SIZE - 1) & ~((size_t)(GEM_PAGE_SIZE - 1));

    for (uint32_t i = 0; i < MAX_GEM_OBJECTS; i++) {
        if (!g_gems[i].in_use) {
            uintptr_t pa = sigma_pmm_alloc_contiguous(size);
            if (!pa) return -1;
            g_gems[i].handle   = i + 1;
            g_gems[i].gpu_pa   = pa;
            g_gems[i].cpu_va   = pa;  // identity mapped
            g_gems[i].size     = size;
            g_gems[i].refcount = 1;
            g_gems[i].in_use   = true;
            *out_handle = g_gems[i].handle;
            return 0;
        }
    }
    return -1;
}

int sigma_drm_gem_close(uint32_t handle) {
    if (!handle || handle > MAX_GEM_OBJECTS) return -1;
    struct gem_object *g = &g_gems[handle - 1];
    if (!g->in_use) return -1;
    if (--g->refcount == 0) {
        // sigma_pmm_free_contiguous(g->gpu_pa, g->size);
        memset(g, 0, sizeof(*g));
    }
    return 0;
}

uintptr_t sigma_drm_gem_cpu_va(uint32_t handle) {
    if (!handle || handle > MAX_GEM_OBJECTS) return 0;
    return g_gems[handle - 1].cpu_va;
}

// ── Mode setting ──────────────────────────────────────────────────────────────

int sigma_drm_set_mode(uint32_t connector_id, const struct drm_mode *mode,
                       uint32_t fb_handle) {
    if (connector_id >= g_num_connectors) return -1;
    struct drm_connector *c = &g_connectors[connector_id];
    c->preferred      = *mode;
    c->active_fb      = fb_handle;
    c->status         = STATUS_CONNECTED;

    // Program CRTC hardware (platform-specific — called via sigma_drm_hw_ops)
    extern int sigma_drm_hw_set_mode(uint32_t, const struct drm_mode *, uint32_t);
    return sigma_drm_hw_set_mode(connector_id, mode, fb_handle);
}

int sigma_drm_get_modes(uint32_t connector_id, struct drm_mode *out, uint32_t max) {
    if (connector_id >= g_num_connectors) return -1;
    struct drm_connector *c = &g_connectors[connector_id];
    uint32_t n = c->num_modes < max ? c->num_modes : max;
    for (uint32_t i = 0; i < n; i++) out[i] = c->modes[i];
    return (int)n;
}

// ── dma-buf export/import ─────────────────────────────────────────────────────
// dma-buf allows GPU buffers to be shared with video decoders, cameras etc.
// without copying — zero-copy pipeline.

int sigma_drm_dmabuf_export(uint32_t gem_handle, int *out_fd) {
    // In a full implementation this creates an anon file descriptor backed by
    // the GEM object's physical pages.  Here we return the handle as a pseudo-fd.
    if (!gem_handle || gem_handle > MAX_GEM_OBJECTS) return -1;
    *out_fd = (int)gem_handle;   // pseudo — real impl uses anon_inode
    return 0;
}

int sigma_drm_dmabuf_import(int fd, uint32_t *out_gem_handle) {
    if (fd <= 0 || (uint32_t)fd > MAX_GEM_OBJECTS) return -1;
    struct gem_object *g = &g_gems[(uint32_t)fd - 1];
    if (!g->in_use) return -1;
    g->refcount++;
    *out_gem_handle = g->handle;
    return 0;
}

// ── Init ──────────────────────────────────────────────────────────────────────

void sigma_drm_init(void) {
    memset(g_connectors, 0, sizeof(g_connectors));
    memset(g_gems, 0, sizeof(g_gems));
    // Platform-specific connector discovery
    extern void sigma_drm_hw_init(struct drm_connector *, uint32_t *count);
    sigma_drm_hw_init(g_connectors, &g_num_connectors);
}
