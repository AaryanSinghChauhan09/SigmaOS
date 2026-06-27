/* SPDX-License-Identifier: GPL-2.0-or-later */
/**
 * drm.h — SigmaOS Graphics / DRM shard interface
 *
 * DRM-style: kernel shard manages modesetting, userspace shard renders.
 * The DRM shard communicates with GPU driver shards via sigma-bus IPC.
 * Supports dumb buffers (CPU rendering) and GEM (GPU rendering).
 *
 * Inspired by: Linux DRM/KMS (drm_drv.c, drm_gem.c), Wayland DRM protocol
 */

#pragma once
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

/* ── Pixel formats (DRM_FORMAT_* compatible) ─────────────────────────────── */

#define SIGMA_FMT_XRGB8888  0x34325258
#define SIGMA_FMT_ARGB8888  0x34325241
#define SIGMA_FMT_RGB565    0x36314752
#define SIGMA_FMT_XBGR2101010 0x30335258

/* ── Dumb buffer (CPU-accessible framebuffer) ───────────────────────────── */

typedef struct sigma_dumb_buf {
    uint32_t handle;
    uint32_t width;
    uint32_t height;
    uint32_t pitch;     /* bytes per row */
    uint32_t bpp;       /* bits per pixel */
    uint64_t mmap_off;  /* offset for mmap(2) */
    size_t   size;      /* total size in bytes */
} sigma_dumb_buf_t;

/* ── Framebuffer ─────────────────────────────────────────────────────────── */

typedef struct sigma_framebuffer {
    uint32_t id;
    uint32_t buf_handle;
    uint32_t width;
    uint32_t height;
    uint32_t pitch;
    uint32_t format;    /* SIGMA_FMT_* */
    uint32_t flags;
} sigma_framebuffer_t;

/* ── Display mode ────────────────────────────────────────────────────────── */

typedef struct sigma_display_mode {
    uint32_t hdisplay;
    uint32_t hsync_start;
    uint32_t hsync_end;
    uint32_t htotal;
    uint32_t vdisplay;
    uint32_t vsync_start;
    uint32_t vsync_end;
    uint32_t vtotal;
    uint32_t vrefresh;   /* Hz */
    uint32_t clock;      /* pixel clock in kHz */
    uint32_t flags;
#define MODE_FLAG_INTERLACE (1u << 0)
#define MODE_FLAG_DBLSCAN   (1u << 1)
    char     name[32];
} sigma_display_mode_t;

/* ── CRTC (display controller) ───────────────────────────────────────────── */

typedef struct sigma_crtc {
    uint32_t              id;
    uint32_t              fb_id;
    uint32_t              x, y;       /* display origin */
    bool                  active;
    sigma_display_mode_t  mode;
} sigma_crtc_t;

/* ── Connector (HDMI, DP, VGA, eDP, LVDS) ───────────────────────────────── */

typedef enum sigma_connector_type {
    CONN_HDMI = 0, CONN_DP, CONN_VGA, CONN_LVDS, CONN_EDP,
    CONN_DSI, CONN_VIRTUAL,
} sigma_connector_type_t;

typedef enum sigma_connector_status {
    CONN_STATUS_CONNECTED    = 0,
    CONN_STATUS_DISCONNECTED = 1,
    CONN_STATUS_UNKNOWN      = 2,
} sigma_connector_status_t;

typedef struct sigma_connector {
    uint32_t                  id;
    sigma_connector_type_t    type;
    sigma_connector_status_t  status;
    uint32_t                  crtc_id;    /* currently attached CRTC */
    sigma_display_mode_t      modes[64];
    uint32_t                  mode_count;
    sigma_display_mode_t      preferred_mode;
    /* EDID: raw 128-byte block */
    uint8_t                   edid[256];
    bool                      edid_valid;
} sigma_connector_t;

/* ── GEM buffer object ────────────────────────────────────────────────────── */

typedef struct sigma_gem_obj {
    uint32_t  handle;
    size_t    size;
    uint64_t  gpu_pa;      /* GPU physical address (iommu-mapped) */
    void     *cpu_ptr;     /* CPU virtual address (NULL until mmap'd) */
    uint32_t  domain;      /* SIGMA_GEM_DOMAIN_CPU / GPU / VRAM */
    uint32_t  refcount;
} sigma_gem_obj_t;

#define SIGMA_GEM_DOMAIN_CPU  (1u << 0)
#define SIGMA_GEM_DOMAIN_GPU  (1u << 1)
#define SIGMA_GEM_DOMAIN_VRAM (1u << 2)

/* ── DRM/GEM API ─────────────────────────────────────────────────────────── */

void sigma_drm_init(void);

/* Dumb buffers */
int  sigma_drm_dumb_create (uint32_t w, uint32_t h, uint32_t bpp,
                             sigma_dumb_buf_t *out);
void *sigma_drm_dumb_mmap  (uint32_t handle);
int  sigma_drm_dumb_destroy(uint32_t handle);

/* Framebuffers */
int  sigma_drm_fb_create   (uint32_t buf_handle, uint32_t w, uint32_t h,
                             uint32_t pitch, uint32_t format,
                             sigma_framebuffer_t *out);
int  sigma_drm_fb_destroy  (uint32_t fb_id);

/* Mode setting */
int  sigma_drm_set_crtc    (uint32_t crtc_id, uint32_t fb_id,
                             uint32_t x, uint32_t y,
                             const uint32_t *connectors, uint32_t n,
                             const sigma_display_mode_t *mode);
int  sigma_drm_page_flip   (uint32_t crtc_id, uint32_t fb_id);

/* Connector / mode enumeration */
int  sigma_drm_get_connectors(sigma_connector_t *out, uint32_t max, uint32_t *count);
int  sigma_drm_get_modes    (uint32_t connector_id, sigma_display_mode_t *out,
                              uint32_t max, uint32_t *count);

/* GEM objects */
int  sigma_gem_create  (size_t size, sigma_gem_obj_t *out);
int  sigma_gem_close   (uint32_t handle);
int  sigma_gem_mmap    (uint32_t handle, void **out_ptr);
int  sigma_gem_export  (uint32_t handle, int *out_dmabuf_fd);   /* dma-buf */
int  sigma_gem_import  (int dmabuf_fd, uint32_t *out_handle);

/* Page-flip event listener (called on vsync) */
typedef void (*sigma_drm_vblank_cb)(uint32_t crtc_id, void *data);
int  sigma_drm_vblank_listen(uint32_t crtc_id, sigma_drm_vblank_cb cb, void *data);
