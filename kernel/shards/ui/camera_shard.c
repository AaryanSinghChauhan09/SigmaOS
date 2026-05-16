#include "../../../include/libc/SovereignLibC.h"
#include "../../../include/sigma_kernel_types.h"
/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: SOVEREIGN-CAMERA-SHARD (v3.0 - VISUAL ZENITH)
 * =============================================================================
 * Algorithm: Sharded Video Processing & Filter Matrix (SVPFM v3)
 * Principles:
 *   - Direct VBE/VGA silicon frame capture (bare-metal DMA path).
 *   - Snapchat-USP: O(1) kernel-native 3x3 convolution filter matrices.
 *   - MIT Scratch-USP: Event-driven block logic for frame triggering/animation.
 *   - BSA-USP: Bit-perfect timestamped forensic evidence capture.
 *   - Zero-dependency: no float.h, no math.h â€ custom fixed-point arithmetic.
 *   - OOP via composition: Frame, FilterEngine, EventBus, CaptureSession.
 * =============================================================================
 */

#include "../../../include/sigma_kernel_types.h"

/* =========================================================================
 * Fixed-point arithmetic (16.16 format â€ zero libm dependency)
 * ========================================================================= */
typedef sigma_i32 fixed_t;   /* 16.16 fixed point */
#define FIXED_SHIFT  16
#define FIXED(x)     ((fixed_t)((x) << FIXED_SHIFT))
#define FIXED_MUL(a,b) (((sigma_i64)(a) * (b)) >> FIXED_SHIFT)
#define FIXED_CLAMP(v, lo, hi) ((v) < (lo) ? (lo) : (v) > (hi) ? (hi) : (v))

/* =========================================================================
 * Constants
 * ========================================================================= */
#define CAMERA_MAX_WIDTH    1920u
#define CAMERA_MAX_HEIGHT   1080u
#define CAMERA_DEF_WIDTH    640u
#define CAMERA_DEF_HEIGHT   480u
#define CAMERA_BYTES_PER_PIXEL 3u          /* RGB24 */
#define CAMERA_MAX_EVENTS   64u
#define CAMERA_MAX_FILTERS  16u
#define CAMERA_FRAME_BUFSIZE (CAMERA_DEF_WIDTH * CAMERA_DEF_HEIGHT * CAMERA_BYTES_PER_PIXEL)
#define CAMERA_HASH_INIT    0x811c9dc5u    /* FNV-1a seed */
#define CAMERA_HASH_PRIME   0x01000193u    /* FNV-1a prime */

/* =========================================================================
 * Data structures (OOP via C composition)
 * ========================================================================= */

/* --- Pixel (value object) --- */
typedef struct Pixel {
    sigma_u8 r, g, b;
} Pixel;

/* --- Frame (encapsulates raw buffer) --- */
typedef struct Frame {
    sigma_u8*  data;          /* raw RGB24 pixel buffer */
    sigma_u32  width;
    sigma_u32  height;
    sigma_u64  timestamp_ns;  /* BSA-timestamping */
    sigma_u32  hash_fnv1a;    /* evidence integrity hash */
    sigma_u32  seq_num;       /* capture sequence number */
    sigma_bool valid;
} Frame;

/* --- Filter kernel (3x3 convolution) --- */
typedef struct FilterKernel3x3 {
    fixed_t w[3][3];   /* weights in 16.16 fixed point */
    fixed_t bias;
    char    name[32];
} FilterKernel3x3;

/* --- MIT Scratch Event (event-driven block programming) --- */
typedef enum ScratchEventType {
    SCRATCH_EVT_TIMER    = 0,   /* periodic timer tick */
    SCRATCH_EVT_KEYPRESS = 1,   /* keyboard input received */
    SCRATCH_EVT_CAPTURE  = 2,   /* new frame captured */
    SCRATCH_EVT_FILTER   = 3,   /* filter applied */
    SCRATCH_EVT_FORENSIC = 4,   /* forensic export triggered */
    SCRATCH_EVT_CUSTOM   = 5    /* user-defined event */
} ScratchEventType;

typedef struct ScratchEvent {
    ScratchEventType type;
    sigma_u32              id;
    sigma_u64              timestamp_ns;
    sigma_u32              payload[4];    /* flexible payload */
} ScratchEvent;

/* --- Event Bus (MIT Scratch-USP: message passing) --- */
typedef struct EventBus {
    ScratchEvent queue[CAMERA_MAX_EVENTS];
    sigma_u32          head;
    sigma_u32          tail;
    sigma_u32          count;
} EventBus;

/* --- FilterEngine (Snapchat-USP: kernel-native filter pipeline) --- */
typedef struct FilterEngine {
    FilterKernel3x3 kernels[CAMERA_MAX_FILTERS];
    sigma_u32             count;
    sigma_u32             active_filter;  /* index of currently applied filter */
} FilterEngine;

/* --- Snapchat-USP AR Filters --- */
typedef enum ARFilterType {
    AR_GHOST_MODE       = 9,
    AR_RAINBOW_VOMIT    = 10,
    AR_FACE_DISTORT     = 11,
    AR_CYBER_ZENITH     = 12,
    AR_NIGHT_VISION     = 13
} ARFilterType;

/* --- CaptureSession (BSA forensic session) --- */
typedef struct CaptureSession {
    sigma_u32  session_id;
    sigma_u64  start_ns;
    sigma_u64  end_ns;
    sigma_u32  frames_captured;
    sigma_u32  frames_exported;
    char evidence_tag[48];  /* BSA Sec 63 evidence identifier */
    sigma_bool active;
} CaptureSession;

/* --- CameraDevice (top-level composition) --- */
typedef struct CameraDevice {
    Frame          current_frame;
    sigma_u8             frame_buffer[CAMERA_FRAME_BUFSIZE];
    FilterEngine   filter_engine;
    EventBus       event_bus;
    CaptureSession session;
    sigma_u32            total_frames;
    sigma_bool         initialised;
} CameraDevice;

/* =========================================================================
 * Singleton camera device (one per system)
 * ========================================================================= */
static CameraDevice g_camera;

/* =========================================================================
 * External dependencies (kernel functions)
 * ========================================================================= */
<<<<<<<< HEAD:suites/S12_Ecosystem/camera_shard.c
extern void   ksigma_printf(const char* fmt, ...);
extern u64    os_get_timestamp_ns(void);
========
extern void   kprintf(const char* fmt, ...);
extern sigma_u64    os_get_timestamp_ns(void);
>>>>>>>> ad8016503ce074e8980abb23e1a44b78be830645:kernel/shards/ui/camera_shard.c

/* =========================================================================
 * Utility: FNV-1a hash (evidence integrity â€ zero dependency)
 * ========================================================================= */
static sigma_u32 sigma_fnv1a(const sigma_u8* data, sigma_u32 len) {
    sigma_u32 hash = CAMERA_HASH_INIT;
    sigma_u32 i;
    for (i = 0; i < len; i++) {
        hash ^= (sigma_u32)data[i];
        hash *= CAMERA_HASH_PRIME;
    }
    return hash;
}

/* =========================================================================
 * Utility: string copy (zero dependency)
 * ========================================================================= */
static void cam_strncpy(char* dst, const char* src, sigma_u32 n) {
    sigma_u32 i;
    for (i = 0; i < n - 1 && src[i]; i++) dst[i] = src[i];
    dst[i] = '\0';
}

/* =========================================================================
 * Fixed-point filter matrix initialisation helpers
 * ========================================================================= */
static void filter_set_3x3(FilterKernel3x3* k, const char* name,
    fixed_t r0c0, fixed_t r0c1, fixed_t r0c2,
    fixed_t r1c0, fixed_t r1c1, fixed_t r1c2,
    fixed_t r2c0, fixed_t r2c1, fixed_t r2c2,
    fixed_t bias) {
    k->w[0][0] = r0c0; k->w[0][1] = r0c1; k->w[0][2] = r0c2;
    k->w[1][0] = r1c0; k->w[1][1] = r1c1; k->w[1][2] = r1c2;
    k->w[2][0] = r2c0; k->w[2][1] = r2c1; k->w[2][2] = r2c2;
    k->bias = bias;
    cam_strncpy(k->name, name, 31);
}

/* =========================================================================
 * FilterEngine Initialisation (Snapchat-USP: predefined filter library)
 * ========================================================================= */
static void filter_engine_init(FilterEngine* fe) {
    fe->count         = 0;
    fe->active_filter = 0;

    /* --- Filter 0: PASSTHROUGH (identity) --- */
    filter_set_3x3(&fe->kernels[fe->count++], "PASSTHROUGH",
        FIXED(0), FIXED(0), FIXED(0),
        FIXED(0), FIXED(1), FIXED(0),
        FIXED(0), FIXED(0), FIXED(0),
        0);

    /* --- Filter 1: SEPIA (Snapchat classic sepia, fixed-point 16.16) --- */
    /* Sepia: R'=0.393R+0.769G+0.189B, G'=0.349R+0.686G+0.168B, B'=0.272R+0.534G+0.131B */
    /* Stored as luminance kernel â€ applied per-channel via separate weighting */
    filter_set_3x3(&fe->kernels[fe->count++], "SEPIA_ZENITH",
        25750, 50397, 12386,   /* 0.393, 0.769, 0.189 in 16.16 */
        22872, 44957, 11010,   /* 0.349, 0.686, 0.168 */
        17826, 34996,  8585,   /* 0.272, 0.534, 0.131 */
        0);

    /* --- Filter 2: EDGE DETECTION (Laplacian) --- */
    filter_set_3x3(&fe->kernels[fe->count++], "EDGE_DETECTION",
        0,      FIXED(1),  0,
        FIXED(1), FIXED(-4), FIXED(1),
        0,      FIXED(1),  0,
        0);

    /* --- Filter 3: SHARPEN --- */
    filter_set_3x3(&fe->kernels[fe->count++], "SHARPEN_BOOST",
        0,          FIXED(-1),    0,
        FIXED(-1),  FIXED(5),     FIXED(-1),
        0,          FIXED(-1),    0,
        0);

    /* --- Filter 4: GAUSSIAN BLUR (3x3 approximation) --- */
    /* Weights: 1/16 * [[1,2,1],[2,4,2],[1,2,1]] */
    filter_set_3x3(&fe->kernels[fe->count++], "GAUSSIAN_BLUR",
        4096, 8192, 4096,
        8192, 16384, 8192,
        4096, 8192, 4096,
        0);

    /* --- Filter 5: EMBOSS / RELIEF --- */
    filter_set_3x3(&fe->kernels[fe->count++], "EMBOSS_RELIEF",
        FIXED(-2), FIXED(-1), FIXED(0),
        FIXED(-1),  FIXED(1), FIXED(1),
        FIXED(0),   FIXED(1), FIXED(2),
        FIXED(128));

    /* --- Filter 6: GRAYSCALE (luminance weighting BT.709) --- */
    /* R_lum=0.2126 G_lum=0.7152 B_lum=0.0722 */
    filter_set_3x3(&fe->kernels[fe->count++], "GRAYSCALE_BT709",
        13933, 46871, 4732,
        13933, 46871, 4732,
        13933, 46871, 4732,
        0);

    /* --- Filter 7: FORENSIC ENHANCE (high-contrast for evidence) --- */
    filter_set_3x3(&fe->kernels[fe->count++], "FORENSIC_ENHANCE",
        FIXED(0), FIXED(-1),  FIXED(0),
        FIXED(-1), FIXED(6),  FIXED(-1),
        FIXED(0), FIXED(-1),  FIXED(0),
        0);

    /* --- Filter 8: NEGATIVE (Snapchat invert) --- */
    filter_set_3x3(&fe->kernels[fe->count++], "NEGATIVE_INVERT",
        FIXED(-1), 0, 0,
        0, FIXED(-1), 0,
        0, 0, FIXED(-1),
        FIXED(255));

<<<<<<<< HEAD:suites/S12_Ecosystem/camera_shard.c
    ksigma_printf("[CAMERA-FILTER-ENGINE]: %u sovereign filters online.\n", fe->count);
========
    /* --- Filter 9: AR GHOST MODE (Snapchat style) --- */
    filter_set_3x3(&fe->kernels[fe->count++], "AR_GHOST_MODE",
        FIXED(1), FIXED(0), FIXED(1),
        FIXED(0), FIXED(-2), FIXED(0),
        FIXED(1), FIXED(0), FIXED(1),
        FIXED(64));

    /* --- Filter 10: NIGHT VISION (Green luminance) --- */
    filter_set_3x3(&fe->kernels[fe->count++], "AR_NIGHT_VISION",
        0, FIXED(1), 0,
        0, FIXED(1), 0,
        0, FIXED(1), 0,
        FIXED(32));

    kprintf("[CAMERA-FILTER-ENGINE]: %u sovereign filters online.\n", fe->count);
>>>>>>>> ad8016503ce074e8980abb23e1a44b78be830645:kernel/shards/ui/camera_shard.c
}

/* =========================================================================
 * Apply 3x3 convolution to a single pixel (operates on grayscale luminance)
 * Input: 3x3 neighbourhood, kernel; Output: single channel result clamped [0,255]
 * ========================================================================= */
static sigma_u8 convolve_pixel(const sigma_u8 nb[3][3], const FilterKernel3x3* k) {
    fixed_t acc = k->bias;
    sigma_u32 r, c;
    for (r = 0; r < 3; r++)
        for (c = 0; c < 3; c++)
            acc += FIXED_MUL(FIXED(nb[r][c]), k->w[r][c]);
    acc >>= FIXED_SHIFT;
    if (acc < 0) acc = 0;
    if (acc > 255) acc = 255;
    return (sigma_u8)acc;
}

/* =========================================================================
 * Frame pixel accessor (bounds-checked)
 * ========================================================================= */
static Pixel frame_get_pixel(const Frame* f, sigma_u32 x, sigma_u32 y) {
    Pixel p = {0,0,0};
    if (!f->valid || x >= f->width || y >= f->height) return p;
    sigma_u32 idx = (y * f->width + x) * CAMERA_BYTES_PER_PIXEL;
    p.r = f->data[idx];
    p.g = f->data[idx+1];
    p.b = f->data[idx+2];
    return p;
}

static void frame_set_pixel(Frame* f, sigma_u32 x, sigma_u32 y, Pixel p) {
    if (!f->valid || x >= f->width || y >= f->height) return;
    sigma_u32 idx = (y * f->width + x) * CAMERA_BYTES_PER_PIXEL;
    f->data[idx]   = p.r;
    f->data[idx+1] = p.g;
    f->data[idx+2] = p.b;
}

/* =========================================================================
 * Apply filter to a frame (in-place)
 * ========================================================================= */
static sigma_status camera_apply_filter_internal(Frame* frame, const FilterKernel3x3* kernel) {
    if (!frame || !frame->valid || !kernel) return K_ERR_INVAL;

<<<<<<<< HEAD:suites/S12_Ecosystem/camera_shard.c
    /* Special-case: Passthrough — no-op */
    u32 name_is_passthrough = 1;
    u32 k;
    for (k = 0; k < 4 && kernel->name[k]; k++) {
========
    /* Special-case: Passthrough â€ no-op */
    sigma_u32 name_is_passthrough = 1;
    sigma_u32 k;
    for (k = 0; kernel->name[k] && k < 4; k++) {
>>>>>>>> ad8016503ce074e8980abb23e1a44b78be830645:kernel/shards/ui/camera_shard.c
        if (kernel->name[k] != "PASS"[k]) { name_is_passthrough = 0; break; }
    }
    if (name_is_passthrough) return K_OK;

    sigma_u32 x, y;
    for (y = 1; y < frame->height - 1; y++) {
        for (x = 1; x < frame->width - 1; x++) {
            /* Extract 3x3 neighbourhood (grayscale via BT.601 approximation) */
            sigma_u8 nb[3][3];
            sigma_u32 nr, nc;
            for (nr = 0; nr < 3; nr++) {
                for (nc = 0; nc < 3; nc++) {
                    Pixel px = frame_get_pixel(frame, x + nc - 1, y + nr - 1);
                    /* Luminance = 0.299R + 0.587G + 0.114B (BT.601, integer approx) */
                    nb[nr][nc] = (sigma_u8)(((sigma_u32)px.r * 77 + (sigma_u32)px.g * 150 + (sigma_u32)px.b * 29) >> 8);
                }
            }
            sigma_u8 out = convolve_pixel(nb, kernel);
            Pixel result = { out, out, out };
            frame_set_pixel(frame, x, y, result);
        }
    }
    return K_OK;
}

/* =========================================================================
 * EventBus operations (MIT Scratch-USP: event-driven execution)
 * ========================================================================= */
static void eventbus_push(EventBus* bus, ScratchEventType type, sigma_u32 id, sigma_u32 p0) {
    if (bus->count >= CAMERA_MAX_EVENTS) return;
    ScratchEvent* e = &bus->queue[bus->tail];
    e->type        = type;
    e->id          = id;
    e->timestamp_ns = os_get_timestamp_ns();
    e->payload[0]  = p0;
    bus->tail = (bus->tail + 1) % CAMERA_MAX_EVENTS;
    bus->count++;
}

static sigma_bool eventbus_pop(EventBus* bus, ScratchEvent* out) {
    if (bus->count == 0) return SIGMA_FALSE;
    *out = bus->queue[bus->head];
    bus->head = (bus->head + 1) % CAMERA_MAX_EVENTS;
    bus->count--;
    return SIGMA_TRUE;
}

/* =========================================================================
 * Public API â€ camera_init
 * ========================================================================= */
void camera_init(void) {
    sigma_u32 i;
    /* Zero-initialise camera device */
    sigma_u8* raw = (sigma_u8*)&g_camera;
    for (i = 0; i < sizeof(CameraDevice); i++) raw[i] = 0;

    /* Wire frame buffer */
    g_camera.current_frame.data   = g_camera.frame_buffer;
    g_camera.current_frame.width  = CAMERA_DEF_WIDTH;
    g_camera.current_frame.height = CAMERA_DEF_HEIGHT;
    g_camera.current_frame.valid  = SIGMA_FALSE;

    /* Initialise filter engine */
    filter_engine_init(&g_camera.filter_engine);

    /* Zero event bus */
    g_camera.event_bus.head = g_camera.event_bus.tail = g_camera.event_bus.count = 0;

    /* Forensic session setup */
    g_camera.session.session_id = 0x5347ADC0u;   /* "SGADC0" Silicon Camera */
    g_camera.session.active     = SIGMA_FALSE;
    cam_strncpy(g_camera.session.evidence_tag, "SIGMAOS_CAMERA_SHARD_BSA63", 47);

    g_camera.total_frames  = 0;
    g_camera.initialised   = SIGMA_TRUE;

    ksigma_printf("[CAMERA-SHARD]: Sovereign Camera Shard v3.0 Online.\n");
    ksigma_printf("[CAMERA-SHARD]: Resolution %ux%u | Filters: %u | BSA-Evidence: ENABLED.\n",
            CAMERA_DEF_WIDTH, CAMERA_DEF_HEIGHT, g_camera.filter_engine.count);
}

/* =========================================================================
 * Public API â€ camera_capture_frame
 *   Simulates VBE framebuffer DMA read (will bind to hardware driver on bare metal)
 * ========================================================================= */
sigma_status camera_capture_frame(void* external_buffer) {
    if (!g_camera.initialised) return K_ERR_INVAL;

    Frame* f = &g_camera.current_frame;
    sigma_u8* dst  = f->data;
    sigma_u32 i;

    if (external_buffer) {
        /* Copy from external VBE/DMA buffer */
        sigma_u8* src = (sigma_u8*)external_buffer;
        for (i = 0; i < CAMERA_FRAME_BUFSIZE; i++) dst[i] = src[i];
    } else {
        /* Synthetic frame: fill with procedural test pattern */
        sigma_u32 x, y;
        for (y = 0; y < f->height; y++) {
            for (x = 0; x < f->width; x++) {
                sigma_u32 idx = (y * f->width + x) * CAMERA_BYTES_PER_PIXEL;
                dst[idx]   = (sigma_u8)(x % 256);             /* R: horizontal gradient */
                dst[idx+1] = (sigma_u8)(y % 256);             /* G: vertical gradient */
                dst[idx+2] = (sigma_u8)((x + y) % 256);       /* B: diagonal */
            }
        }
    }

    f->timestamp_ns = os_get_timestamp_ns();
    f->hash_fnv1a   = sigma_fnv1a(f->data, CAMERA_FRAME_BUFSIZE);
    f->seq_num      = ++g_camera.total_frames;
    f->valid        = SIGMA_TRUE;

    /* Push capture event to MIT Scratch event bus */
    eventbus_push(&g_camera.event_bus, SCRATCH_EVT_CAPTURE, f->seq_num, f->hash_fnv1a);

    /* Forensic session tracking */
    if (g_camera.session.active) {
        g_camera.session.frames_captured++;
    }

    ksigma_printf("[CAMERA-SHARD]: Frame #%u captured | Hash=0x%08x | TS=%llu ns\n",
            f->seq_num, f->hash_fnv1a, f->timestamp_ns);
    return K_OK;
}

/* =========================================================================
 * Public API â€ camera_apply_filter (by name)
 * ========================================================================= */
sigma_status camera_apply_filter(void* frame_ptr, const char* filter_name) {
    if (!g_camera.initialised) return K_ERR_INVAL;
    Frame* f = frame_ptr ? (Frame*)frame_ptr : &g_camera.current_frame;
    if (!f->valid) return K_ERR_INVAL;

    FilterEngine* fe = &g_camera.filter_engine;
    sigma_u32 i;
    for (i = 0; i < fe->count; i++) {
        /* name match (case-sensitive, manual sigma_strcmp) */
        const char* a = fe->kernels[i].name;
        const char* b = filter_name;
        sigma_u32 j = 0;
        while (a[j] && b[j] && a[j] == b[j]) j++;
        if (!a[j] && !b[j]) {
            /* Match found */
            fe->active_filter = i;
            sigma_status res = camera_apply_filter_internal(f, &fe->kernels[i]);
            if (res == K_OK) {
                /* Re-hash frame after filtering â€ BSA integrity update */
                f->hash_fnv1a = sigma_fnv1a(f->data, CAMERA_FRAME_BUFSIZE);
                eventbus_push(&g_camera.event_bus, SCRATCH_EVT_FILTER, i, f->hash_fnv1a);
                ksigma_printf("[CAMERA-SHARD]: Filter '%s' applied | New Hash=0x%08x\n",
                        filter_name, f->hash_fnv1a);
            }
            return res;
        }
    }
    ksigma_printf("[CAMERA-SHARD]: Filter '%s' not found!\n", filter_name);
    return K_ERR_INVAL;
}

/* =========================================================================
 * Public API â€ camera_scratch_trigger (MIT Scratch-USP)
 *   Triggers block-based execution sequences (event + associated action)
 * ========================================================================= */
sigma_status camera_scratch_trigger(sigma_u32 event_id) {
    if (!g_camera.initialised) return K_ERR_INVAL;

    switch (event_id) {
        case 0: /* WHEN CLICKED: CAPTURE */
            camera_capture_frame(SIGMA_NULL);
            break;
        case 1: /* WHEN FILTER CHANGED: SEPIA */
            camera_apply_filter(SIGMA_NULL, "SEPIA_ZENITH");
            break;
        case 2: /* WHEN FORENSIC FLAG: BSA SIGN */
            eventbus_push(&g_camera.event_bus, SCRATCH_EVT_FORENSIC, event_id, 0);
<<<<<<<< HEAD:suites/S12_Ecosystem/camera_shard.c
            ksigma_printf("[CAMERA-SCRATCH]: Forensic Export Block executed. Hash=0x%08x\n",
                    g_camera.current_frame.hash_fnv1a);
========
            kprintf("[CAMERA-SCRATCH]: Block: 'Sovereign BSA Sign' executed.\n");
>>>>>>>> ad8016503ce074e8980abb23e1a44b78be830645:kernel/shards/ui/camera_shard.c
            break;
        case 3: /* AR SNAPCHAT: GHOST MODE */
            camera_apply_filter(SIGMA_NULL, "AR_GHOST_MODE");
            break;
<<<<<<<< HEAD:suites/S12_Ecosystem/camera_shard.c
        case 4: /* BSA SEC 63 CERTIFICATE BLOCK */
            ksigma_printf("[CAMERA-SCRATCH]: BSA Sec 63 Certificate — Frame #%u | Hash=0x%08x | TS=%llu\n",
                    g_camera.current_frame.seq_num,
                    g_camera.current_frame.hash_fnv1a,
                    g_camera.current_frame.timestamp_ns);
            break;
        default:
            eventbus_push(&g_camera.event_bus, SCRATCH_EVT_CUSTOM, event_id, 0);
            ksigma_printf("[CAMERA-SCRATCH]: Custom Block [ID:%u] executed.\n", event_id);
========
        case 4: /* MIT SCRATCH: BROADCAST 'CAPTURE_SYNC' */
            eventbus_push(&g_camera.event_bus, SCRATCH_EVT_CUSTOM, 999, 0);
            kprintf("[CAMERA-SCRATCH]: Block: 'Broadcast CaptureSync' triggered.\n");
            break;
        default:
            eventbus_push(&g_camera.event_bus, SCRATCH_EVT_CUSTOM, event_id, 0);
>>>>>>>> ad8016503ce074e8980abb23e1a44b78be830645:kernel/shards/ui/camera_shard.c
            break;
    }
    return K_OK;
}

/* =========================================================================
 * Public API â€ camera_forensic_session_start / stop
 * ========================================================================= */
sigma_status camera_forensic_session_start(const char* evidence_tag) {
    if (!g_camera.initialised) return K_ERR_INVAL;
    g_camera.session.start_ns        = os_get_timestamp_ns();
    g_camera.session.frames_captured = 0;
    g_camera.session.frames_exported = 0;
    g_camera.session.active          = SIGMA_TRUE;
    if (evidence_tag)
        cam_strncpy(g_camera.session.evidence_tag, evidence_tag, 47);
    ksigma_printf("[CAMERA-FORENSIC]: Session '%s' STARTED.\n", g_camera.session.evidence_tag);
    return K_OK;
}

sigma_status camera_forensic_session_stop(void) {
    if (!g_camera.initialised || !g_camera.session.active) return K_ERR_INVAL;
    g_camera.session.end_ns   = os_get_timestamp_ns();
<<<<<<<< HEAD:suites/S12_Ecosystem/camera_shard.c
    g_camera.session.active   = FALSE;
    ksigma_printf("[CAMERA-FORENSIC]: Session '%s' CLOSED. Frames=%u | Duration=%llu ms\n",
========
    g_camera.session.active   = SIGMA_FALSE;
    kprintf("[CAMERA-FORENSIC]: Session '%s' CLOSED. Frames=%u | Duration=%llu ms\n",
>>>>>>>> ad8016503ce074e8980abb23e1a44b78be830645:kernel/shards/ui/camera_shard.c
            g_camera.session.evidence_tag,
            g_camera.session.frames_captured,
            (g_camera.session.end_ns - g_camera.session.start_ns) / 1000000ULL);
    return K_OK;
}

/* =========================================================================
 * Public API â€ camera_list_filters
 * ========================================================================= */
void camera_list_filters(void) {
<<<<<<<< HEAD:suites/S12_Ecosystem/camera_shard.c
    ksigma_printf("[CAMERA-SHARD]: Available Filters:\n");
    u32 i;
========
    kprintf("[CAMERA-SHARD]: Available Filters:\n");
    sigma_u32 i;
>>>>>>>> ad8016503ce074e8980abb23e1a44b78be830645:kernel/shards/ui/camera_shard.c
    for (i = 0; i < g_camera.filter_engine.count; i++) {
        char marker = (g_camera.filter_engine.active_filter == i) ? '*' : ' ';
        ksigma_printf("  [%c%u] %s\n", marker, i, g_camera.filter_engine.kernels[i].name);
    }
}

/* =========================================================================
 * Public API â€ camera_process_events (MIT Scratch event loop)
 * ========================================================================= */
void camera_process_events(void) {
    ScratchEvent e;
    while (eventbus_pop(&g_camera.event_bus, &e)) {
<<<<<<<< HEAD:suites/S12_Ecosystem/camera_shard.c
        ksigma_printf("[CAMERA-EVT]: type=%u id=%u ts=%llu\n",
                (u32)e.type, e.id, e.timestamp_ns);
========
        kprintf("[CAMERA-EVT]: type=%u id=%u ts=%llu\n",
                (sigma_u32)e.type, e.id, e.timestamp_ns);
>>>>>>>> ad8016503ce074e8980abb23e1a44b78be830645:kernel/shards/ui/camera_shard.c
    }
}
