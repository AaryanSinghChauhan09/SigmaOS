/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN MEDIA CODEC ENGINE (v1.0 — PURE C11)
 * =========================================================================
 * Absorbed USPs from: VLC media player (libvlc / libvlccore)
 *   https://github.com/videolan/vlc
 *
 * VLC USPs absorbed:
 *   ✓ Universal codec chain (demuxer → decoder → filter → renderer)
 *   ✓ Hardware-accelerated decode (VAAPI/VDPAU/NVDEC/VideoToolbox stubs)
 *   ✓ Transcoding pipeline (sigma-transcode)
 *   ✓ Network streaming (RTSP, HLS, MPEG-TS, RTP)
 *   ✓ Subtitle rendering (SRT/ASS/WebVTT)
 *   ✓ Audio filter chain (equalizer, normaliser, compressor)
 *   ✓ Plugin/module registration system
 *   ✓ Media information (probe, meta-tags)
 *   ✓ Playback control (play/pause/seek/rate/vol)
 *
 * OBS Studio USPs absorbed:
 *   ✓ Scene compositor (scenes, sources, transitions)
 *   ✓ Video capture pipeline (screen, window, device)
 *   ✓ Stream encoder (x264/NVENC/HEVC) with output targets
 *   ✓ Audio mixer (multi-track, volume, monitoring)
 *   ✓ Plugin/filter registration
 * =========================================================================
 */

#ifndef SOVEREIGN_MEDIA_CODEC_H
#define SOVEREIGN_MEDIA_CODEC_H

#include "suites/S01_Genesis/shards/sigma_types.h"

/* -------------------------------------------------------------------------
 * Codec IDs (mirrors libavcodec / VLC fourcc)
 * ---------------------------------------------------------------------- */
typedef enum {
    SIGMA_CODEC_NONE  = 0,
    /* Video */
    SIGMA_CODEC_H264  = 1,
    SIGMA_CODEC_H265  = 2,
    SIGMA_CODEC_AV1   = 3,
    SIGMA_CODEC_VP9   = 4,
    SIGMA_CODEC_MPEG2 = 5,
    SIGMA_CODEC_MJPEG = 6,
    /* Audio */
    SIGMA_CODEC_AAC   = 20,
    SIGMA_CODEC_MP3   = 21,
    SIGMA_CODEC_OPUS  = 22,
    SIGMA_CODEC_FLAC  = 23,
    SIGMA_CODEC_AC3   = 24,
    SIGMA_CODEC_EAC3  = 25,
    SIGMA_CODEC_VORBIS= 26,
    /* Subtitle */
    SIGMA_CODEC_SRT   = 40,
    SIGMA_CODEC_ASS   = 41,
    SIGMA_CODEC_WEBVTT= 42,
} SigmaCodecID_t;

/* -------------------------------------------------------------------------
 * Hardware decode acceleration backends
 * ---------------------------------------------------------------------- */
typedef enum {
    SIGMA_HW_ACCEL_NONE        = 0,
    SIGMA_HW_ACCEL_VAAPI       = 1,  /* Linux / Intel QSV */
    SIGMA_HW_ACCEL_VDPAU       = 2,  /* NVIDIA (legacy) */
    SIGMA_HW_ACCEL_NVDEC       = 3,  /* NVIDIA CUVID */
    SIGMA_HW_ACCEL_VIDEOTOOLBOX= 4,  /* macOS */
    SIGMA_HW_ACCEL_DXVA2       = 5,  /* Windows */
    SIGMA_HW_ACCEL_VULKAN      = 6,  /* Cross-platform */
} SigmaHWAccel_t;

/* -------------------------------------------------------------------------
 * Stream description
 * ---------------------------------------------------------------------- */
#define SIGMA_MEDIA_URL_MAX   512
#define SIGMA_MEDIA_TITLE_MAX 256

typedef struct {
    SigmaCodecID_t   codec_id;
    sigma_u32        width, height;
    sigma_u32        fps_num, fps_den;
    sigma_u32        bitrate;
    sigma_u32        sample_rate;
    sigma_u32        channels;
    sigma_u64        duration_ms;
    sigma_u32        stream_index;
} SigmaStreamInfo_t;

typedef struct {
    char             url        [SIGMA_MEDIA_URL_MAX];
    char             title      [SIGMA_MEDIA_TITLE_MAX];
    char             artist     [128];
    char             album      [128];
    SigmaStreamInfo_t video;
    SigmaStreamInfo_t audio;
    SigmaStreamInfo_t subtitle;
    sigma_u64        file_size_bytes;
    sigma_bool       is_live;       /* Live stream flag */
} SigmaMediaInfo_t;

/* -------------------------------------------------------------------------
 * Playback state
 * ---------------------------------------------------------------------- */
typedef enum {
    SIGMA_MEDIA_STOPPED  = 0,
    SIGMA_MEDIA_PLAYING  = 1,
    SIGMA_MEDIA_PAUSED   = 2,
    SIGMA_MEDIA_BUFFERING= 3,
    SIGMA_MEDIA_ERROR    = 4,
} SigmaMediaState_t;

/* -------------------------------------------------------------------------
 * Audio filter chain node (EQ, normalise, compress)
 * ---------------------------------------------------------------------- */
#define SIGMA_AUDIO_EQ_BANDS 10

typedef struct {
    sigma_f32  eq_gain_db  [SIGMA_AUDIO_EQ_BANDS]; /* 10-band parametric EQ */
    sigma_f32  volume;            /* 0.0 – 2.0 (1.0 = unity) */
    sigma_f32  compressor_thresh; /* dBFS */
    sigma_f32  compressor_ratio;
    sigma_bool normalise;
    sigma_bool muted;
} SigmaAudioFilter_t;

/* -------------------------------------------------------------------------
 * Transcode job
 * ---------------------------------------------------------------------- */
typedef struct {
    char          src_url [SIGMA_MEDIA_URL_MAX];
    char          dst_url [SIGMA_MEDIA_URL_MAX];
    SigmaCodecID_t vcodec, acodec;
    sigma_u32     vbitrate, abitrate;
    sigma_u32     width, height;
    sigma_u32     fps;
    SigmaHWAccel_t hw_accel;
} SigmaTranscodeJob_t;

/* -------------------------------------------------------------------------
 * OBS Studio Scene Compositor
 * ---------------------------------------------------------------------- */
#define SIGMA_SCENE_MAX       16
#define SIGMA_SOURCE_MAX      32
#define SIGMA_SCENE_NAME_MAX  64

typedef enum {
    SIGMA_SOURCE_DISPLAY   = 0,
    SIGMA_SOURCE_WINDOW    = 1,
    SIGMA_SOURCE_CAMERA    = 2,
    SIGMA_SOURCE_MEDIA     = 3,
    SIGMA_SOURCE_TEXT      = 4,
    SIGMA_SOURCE_IMAGE     = 5,
    SIGMA_SOURCE_BROWSER   = 6,
} SigmaSourceType_t;

typedef struct {
    char             name [SIGMA_SCENE_NAME_MAX];
    SigmaSourceType_t type;
    sigma_i32        x, y, w, h;
    sigma_f32        opacity;     /* 0.0–1.0 */
    sigma_bool       visible;
    sigma_bool       locked;
} SigmaSource_t;

typedef struct {
    char         name[SIGMA_SCENE_NAME_MAX];
    SigmaSource_t sources[SIGMA_SOURCE_MAX];
    sigma_u32    source_count;
    sigma_bool   active;
} SigmaScene_t;

/* -------------------------------------------------------------------------
 * Stream/Record output
 * ---------------------------------------------------------------------- */
typedef enum {
    SIGMA_OUTPUT_STREAM = 0,   /* RTMP/SRT to server */
    SIGMA_OUTPUT_RECORD = 1,   /* Local file */
    SIGMA_OUTPUT_BOTH   = 2,
} SigmaOutputMode_t;

typedef struct {
    char             server_url [SIGMA_MEDIA_URL_MAX];
    char             stream_key [128];
    char             record_path[SIGMA_MEDIA_URL_MAX];
    SigmaOutputMode_t mode;
    SigmaCodecID_t   venc;
    SigmaHWAccel_t   hw_enc;
    sigma_u32        vbitrate;    /* kbps */
    sigma_u32        fps;
    sigma_bool       active;
} SigmaOutputConfig_t;

/* -------------------------------------------------------------------------
 * Public API — Media Playback (VLC parity)
 * ---------------------------------------------------------------------- */
sigma_err_t  sigma_media_open       (const char *url, SigmaMediaInfo_t *out);
sigma_err_t  sigma_media_play       (const char *url, SigmaHWAccel_t accel);
sigma_err_t  sigma_media_pause      (void);
sigma_err_t  sigma_media_stop       (void);
sigma_err_t  sigma_media_seek       (sigma_u64 ms);
sigma_err_t  sigma_media_set_rate   (sigma_f32 rate);         /* 0.5–4.0x  */
sigma_err_t  sigma_media_set_volume (sigma_f32 vol);          /* 0.0–2.0   */
sigma_err_t  sigma_media_set_audio_filter(const SigmaAudioFilter_t *f);
sigma_err_t  sigma_media_transcode  (const SigmaTranscodeJob_t *job);
void         sigma_media_info       (const char *url);        /* probe      */
SigmaMediaState_t sigma_media_state (void);

/* Public API — OBS Compositor */
sigma_err_t  sigma_obs_scene_create (const char *name);
sigma_err_t  sigma_obs_scene_switch (const char *name);
sigma_err_t  sigma_obs_source_add   (const char *scene, const char *name,
                                      SigmaSourceType_t type,
                                      sigma_i32 x, sigma_i32 y,
                                      sigma_i32 w, sigma_i32 h);
sigma_err_t  sigma_obs_source_remove(const char *scene, const char *name);
sigma_err_t  sigma_obs_output_start (const SigmaOutputConfig_t *cfg);
sigma_err_t  sigma_obs_output_stop  (void);
void         sigma_obs_scene_list   (void);
void         sigma_obs_stats        (void);

void SovereignMediaCodec_Init(void);

#endif /* SOVEREIGN_MEDIA_CODEC_H */
