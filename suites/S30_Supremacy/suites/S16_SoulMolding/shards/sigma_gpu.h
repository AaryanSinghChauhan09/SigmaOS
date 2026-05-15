/*
 * =========================================================================
 * S SIGMAOS kernel/suites/S16_SoulMolding/shards/sigma_gpu.h
 * =========================================================================
 * Sovereign GPU Subsystem  gap-closes:
 *   Linux  : DRM/KMS, GEM/TTM buffer objects, PRIME (dma-buf), Mesa/Gallium
 *   Windows: WDDM (Display Model), DXGI, DirectX 12, D3D resource heaps
 *   macOS  : Metal, IOSurface, CoreAnimation, CAMetalLayer
 *   Android: Vulkan, HWUI, SurfaceFlinger, Gralloc allocator
 *   RISC-V : VisionFive GPU pipeline
 * =========================================================================
 */

#ifndef SIGMA_GPU_H
#define SIGMA_GPU_H

typedef unsigned long long gpu_u64;
typedef unsigned int       gpu_u32;
typedef unsigned short     gpu_u16;
typedef signed   int       gpu_i32;
typedef unsigned char      gpu_bool;
#define GPU_TRUE  ((gpu_bool)1)
#define GPU_FALSE ((gpu_bool)0)
#define GPU_NULL  ((void*)0)
#define GPU_OK    ((gpu_i32) 0)
#define GPU_ERR   ((gpu_i32)-1)

/* -- Display modes --------------------------------------------------------- */
typedef struct {
    gpu_u32 width;
    gpu_u32 height;
    gpu_u32 refresh_hz;
    gpu_u32 bpp;           /* bits per pixel                            */
} sigma_display_mode_t;

/* -- Pixel formats (DXGI/DRM format parity) ------------------------------- */
typedef enum {
    PIXFMT_RGBA8888 = 0,
    PIXFMT_BGRA8888 = 1,
    PIXFMT_RGB565   = 2,
    PIXFMT_ARGB2101010 = 3,   /* HDR                                   */
    PIXFMT_RGBA16F  = 4,      /* 16-bit float HDR                      */
    PIXFMT_YUV420   = 5       /* video decode output                   */
} sigma_pixfmt_t;

/* -- GEM/PRIME buffer object ----------------------------------------------- */
#define GPU_MAX_BOS 4096
typedef struct {
    gpu_u32       handle;
    gpu_u64       size;
    gpu_u64       phys_addr;
    void         *cpu_map;    /* kernel virtual mapping                 */
    sigma_pixfmt_t pixfmt;
    gpu_u32       width;
    gpu_u32       height;
    gpu_u32       stride;
    gpu_u32       owner_pid;
    gpu_bool      exported;   /* dma-buf PRIME exported                 */
    gpu_bool      imported;
} sigma_bo_t;               /* Buffer Object                            */

/* -- Command queue (Metal MTLCommandQueue / DX12 ID3D12CommandQueue) ------- */
#define GPU_CMDQ_LEN 256
typedef struct {
    gpu_u32      queue_id;
    gpu_u32      head;
    gpu_u32      tail;
    gpu_u64      cmd_buf[GPU_CMDQ_LEN];  /* opaque command words       */
    gpu_u64      submitted;
    gpu_u64      completed;
} sigma_cmdqueue_t;

/* -- Display connector (KMS CRTC/encoder/connector chain) ------------------ */
typedef struct {
    gpu_u32           connector_id;
    char              name[24];     /* e.g. "HDMI-A-1", "eDP-1"        */
    sigma_display_mode_t mode;
    gpu_u32           fb_handle;    /* active framebuffer BO            */
    gpu_bool          connected;
    gpu_bool          enabled;
} sigma_connector_t;

#define GPU_MAX_CONNECTORS 8
#define GPU_MAX_CMDQUEUES  16

/* -- Public API ----------------------------------------------------------- */
void   sigma_gpu_init(void);

/* Buffer objects (GEM alloc equivalent) */
gpu_i32 sigma_bo_create(gpu_u32 pid, gpu_u64 size, sigma_pixfmt_t fmt,
                         gpu_u32 w, gpu_u32 h);
void    sigma_bo_destroy(gpu_u32 handle);
gpu_i32 sigma_bo_mmap(gpu_u32 handle, gpu_u64 *cpu_addr);
gpu_i32 sigma_bo_prime_export(gpu_u32 handle);  /* dma-buf export       */
void    sigma_bo_list(void);

/* Command queues (Metal/DX12 submit model) */
gpu_i32 sigma_cmdq_create(void);
gpu_i32 sigma_cmdq_submit(gpu_u32 qid, gpu_u64 *cmds, gpu_u32 count);
void    sigma_cmdq_wait(gpu_u32 qid);

/* Display / KMS */
gpu_i32 sigma_connector_probe(void);
gpu_i32 sigma_connector_setmode(gpu_u32 conn_id, sigma_display_mode_t *mode);
gpu_i32 sigma_connector_flip(gpu_u32 conn_id, gpu_u32 bo_handle);
void    sigma_connector_list(void);

/* Compositing (SurfaceFlinger/CoreAnimation layer model) */
gpu_i32 sigma_surface_create(gpu_u32 pid, gpu_u32 w, gpu_u32 h, sigma_pixfmt_t fmt);
void    sigma_surface_present(gpu_u32 surface_id);

void   sigma_gpu_stats(void);

#endif /* SIGMA_GPU_H */
