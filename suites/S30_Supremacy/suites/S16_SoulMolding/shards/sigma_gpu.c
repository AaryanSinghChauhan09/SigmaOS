#include "../../../../../include/SovereignLibC.h"
/*
 * =========================================================================
 * S SIGMAOS kernel/suites/S16_SoulMolding/shards/sigma_gpu.c
 * =========================================================================
 */

#include "../../../../../include/drivers/sigma_gpu.h"
#include "../../../../../include/libc/sigma_libc.h"

static sigma_bo_t         s_bos[GPU_MAX_BOS];
static gpu_u32            s_bo_count    = 0;
static gpu_u32            s_next_handle = 1;

static sigma_cmdqueue_t   s_queues[GPU_MAX_CMDQUEUES];
static gpu_u32            s_queue_count = 0;

static sigma_connector_t  s_connectors[GPU_MAX_CONNECTORS];
static gpu_u32            s_conn_count  = 0;

static gpu_u64            s_gpu_clock   = 0;

static const char *pixfmt_str[] = {
    "RGBA8888","BGRA8888","RGB565","ARGB2101010","RGBA16F","YUV420"
};

/* -- Init ------------------------------------------------------------------ */
void sigma_gpu_init(void) {
    sigma_sigma_memset(s_bos,      0, sizeof(s_bos));
    sigma_sigma_memset(s_queues,   0, sizeof(s_queues));
    sigma_sigma_memset(s_connectors,0,sizeof(s_connectors));

    sigma_sigma_printf("S [GPU] Sovereign GPU subsystem initialized\n");
    sigma_sigma_printf("S [GPU] DRM/KMS | GEM/TTM | PRIME/dma-buf | Vulkan-ready\n");

    /* Probe default connectors */
    sigma_connector_probe();
}

/* -- Buffer Objects -------------------------------------------------------- */
gpu_i32 sigma_bo_create(gpu_u32 pid, gpu_u64 size, sigma_pixfmt_t fmt,
                         gpu_u32 w, gpu_u32 h) {
    if (s_bo_count >= GPU_MAX_BOS) return GPU_ERR;

    sigma_bo_t *bo = &s_bos[s_bo_count++];
    sigma_sigma_memset(bo, 0, sizeof(*bo));
    bo->handle    = s_next_handle++;
    bo->size      = size ? size : (gpu_u64)w * h * 4;
    bo->pixfmt    = fmt;
    bo->width     = w;
    bo->height    = h;
    bo->stride    = w * 4; /* assume 4 bytes per pixel                 */
    bo->owner_pid = pid;
    bo->phys_addr = 0x100000000ULL + (gpu_u64)(bo->handle) * 0x100000ULL;

    sigma_sigma_printf("S [GEM] BO create: handle=%u size=%llu %ux%u fmt=%s\n",
                 bo->handle, (unsigned long long)bo->size,
                 w, h, pixfmt_str[fmt]);
    return (gpu_i32)bo->handle;
}

static sigma_bo_t *find_bo(gpu_u32 handle) {
    for (gpu_u32 i = 0; i < s_bo_count; i++)
        if (s_bos[i].handle == handle) return &s_bos[i];
    return GPU_NULL;
}

void sigma_bo_destroy(gpu_u32 handle) {
    for (gpu_u32 i = 0; i < s_bo_count; i++) {
        if (s_bos[i].handle == handle) {
            sigma_sigma_printf("S [GEM] BO destroy: handle=%u\n", handle);
            for (gpu_u32 j = i; j < s_bo_count-1; j++)
                s_bos[j] = s_bos[j+1];
            s_bo_count--;
            return;
        }
    }
}

gpu_i32 sigma_bo_mmap(gpu_u32 handle, gpu_u64 *cpu_addr) {
    sigma_bo_t *bo = find_bo(handle);
    if (!bo) return GPU_ERR;
    /* Simulated: map phys to a kernel virtual address */
    *cpu_addr = 0xFFFF800000000000ULL | bo->phys_addr;
    bo->cpu_map = (void*)(unsigned long)*cpu_addr;
    sigma_sigma_printf("S [GEM] BO mmap: handle=%u vaddr=0x%llx\n",
                 handle, (unsigned long long)*cpu_addr);
    return GPU_OK;
}

gpu_i32 sigma_bo_prime_export(gpu_u32 handle) {
    sigma_bo_t *bo = find_bo(handle);
    if (!bo) return GPU_ERR;
    bo->exported = GPU_TRUE;
    /* Returns simulated dma-buf fd */
    int fd = (int)(100 + handle);
    sigma_sigma_printf("S [PRIME] BO %u exported as dma-buf fd=%d\n", handle, fd);
    return fd;
}

void sigma_bo_list(void) {
    sigma_sigma_printf("\nS GEM BUFFER OBJECTS (%u)\n", s_bo_count);
    sigma_sigma_printf("%-6s %-12s %-10s %-12s %s\n","HDL","SIZE","DIMS","FMT","FLAGS");
    for (gpu_u32 i = 0; i < s_bo_count; i++) {
        sigma_bo_t *b = &s_bos[i];
        sigma_sigma_printf("  %-4u %-12llu %-10s %-12s %s%s\n",
                     b->handle, (unsigned long long)b->size,
                     "see dims", pixfmt_str[b->pixfmt],
                     b->exported ? "[exported] " : "",
                     b->imported ? "[imported]" : "");
    }
}

/* -- Command Queues -------------------------------------------------------- */
gpu_i32 sigma_cmdq_create(void) {
    if (s_queue_count >= GPU_MAX_CMDQUEUES) return GPU_ERR;
    sigma_cmdqueue_t *q = &s_queues[s_queue_count];
    sigma_sigma_memset(q, 0, sizeof(*q));
    q->queue_id = s_queue_count++;
    sigma_sigma_printf("S [GPU] Command queue created: id=%u\n", q->queue_id);
    return (gpu_i32)q->queue_id;
}

gpu_i32 sigma_cmdq_submit(gpu_u32 qid, gpu_u64 *cmds, gpu_u32 count) {
    if (qid >= s_queue_count) return GPU_ERR;
    sigma_cmdqueue_t *q = &s_queues[qid];
    for (gpu_u32 i = 0; i < count && q->tail < GPU_CMDQ_LEN; i++)
        q->cmd_buf[q->tail++ % GPU_CMDQ_LEN] = cmds[i];
    q->submitted += count;
    sigma_sigma_printf("S [GPU] Queue %u: %u cmds submitted (total=%llu)\n",
                 qid, count, (unsigned long long)q->submitted);
    return GPU_OK;
}

void sigma_cmdq_wait(gpu_u32 qid) {
    if (qid >= s_queue_count) return;
    sigma_cmdqueue_t *q = &s_queues[qid];
    q->completed = q->submitted;
    q->head = q->tail;
    s_gpu_clock += 1000;
    sigma_sigma_printf("S [GPU] Queue %u: fence signaled (completed=%llu)\n",
                 qid, (unsigned long long)q->completed);
}

/* -- Display / KMS --------------------------------------------------------- */
gpu_i32 sigma_connector_probe(void) {
    /* Simulate probing 2 connectors: eDP-1 (laptop) + HDMI-A-1 */
    if (s_conn_count == 0) {
        sigma_connector_t *edp = &s_connectors[s_conn_count++];
        sigma_strncpy(edp->name, "eDP-1", 23);
        edp->connector_id = 1;
        edp->connected    = GPU_TRUE;
        edp->enabled      = GPU_TRUE;
        edp->mode = (sigma_display_mode_t){ 2560, 1600, 120, 32 };

        sigma_connector_t *hdmi = &s_connectors[s_conn_count++];
        sigma_strncpy(hdmi->name, "HDMI-A-1", 23);
        hdmi->connector_id = 2;
        hdmi->connected    = GPU_FALSE;
        hdmi->enabled      = GPU_FALSE;
        hdmi->mode = (sigma_display_mode_t){ 3840, 2160, 60, 32 };
    }
    sigma_sigma_printf("S [KMS] Probed %u connectors\n", s_conn_count);
    return (gpu_i32)s_conn_count;
}

gpu_i32 sigma_connector_setmode(gpu_u32 conn_id, sigma_display_mode_t *mode) {
    for (gpu_u32 i = 0; i < s_conn_count; i++) {
        if (s_connectors[i].connector_id == conn_id) {
            s_connectors[i].mode    = *mode;
            s_connectors[i].enabled = GPU_TRUE;
            sigma_sigma_printf("S [KMS] %s: mode set to %ux%u@%uHz %ubpp\n",
                         s_connectors[i].name,
                         mode->width, mode->height,
                         mode->refresh_hz, mode->bpp);
            return GPU_OK;
        }
    }
    return GPU_ERR;
}

gpu_i32 sigma_connector_flip(gpu_u32 conn_id, gpu_u32 bo_handle) {
    for (gpu_u32 i = 0; i < s_conn_count; i++) {
        if (s_connectors[i].connector_id == conn_id) {
            s_connectors[i].fb_handle = bo_handle;
            s_gpu_clock += 16666; /* ~60Hz vsync period (ns)           */
            sigma_sigma_printf("S [KMS] %s: page-flip to BO %u (vblank)\n",
                         s_connectors[i].name, bo_handle);
            return GPU_OK;
        }
    }
    return GPU_ERR;
}

void sigma_connector_list(void) {
    sigma_sigma_printf("\nS DISPLAY CONNECTORS (%u)\n", s_conn_count);
    for (gpu_u32 i = 0; i < s_conn_count; i++) {
        sigma_connector_t *c = &s_connectors[i];
        sigma_sigma_printf("  %s [%s] %ux%u@%u fb=%u\n",
                     c->name, c->connected ? "connected":"disconnected",
                     c->mode.width, c->mode.height, c->mode.refresh_hz,
                     c->fb_handle);
    }
}

/* -- Compositing ----------------------------------------------------------- */
gpu_i32 sigma_surface_create(gpu_u32 pid, gpu_u32 w, gpu_u32 h, sigma_pixfmt_t fmt) {
    gpu_i32 h_bo = sigma_bo_create(pid, 0, fmt, w, h);
    if (h_bo < 0) return GPU_ERR;
    sigma_sigma_printf("S [COMP] Surface created: pid=%u %ux%u -> BO %d\n", pid, w, h, h_bo);
    return h_bo;
}

void sigma_surface_present(gpu_u32 surface_id) {
    sigma_sigma_printf("S [COMP] Present surface BO %u -> compositor layer\n", surface_id);
    if (s_conn_count > 0)
        sigma_connector_flip(s_connectors[0].connector_id, surface_id);
}

/* -- Stats ----------------------------------------------------------------- */
void sigma_gpu_stats(void) {
    sigma_sigma_printf("\nS GPU STATS\n");
    sigma_sigma_printf("  BOs: %u   Queues: %u   Clock: %lluns\n",
                 s_bo_count, s_queue_count, (unsigned long long)s_gpu_clock);
    sigma_connector_list();
    sigma_bo_list();
}
