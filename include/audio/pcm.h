/* SPDX-License-Identifier: GPL-2.0-or-later */
/**
 * pcm.h — SigmaOS audio PCM subsystem
 *
 * sigma-audio-server: user-space audio mixer shard.
 * Drivers register as sub-shards (HDA, USB Audio, HDMI audio).
 * Clients (media player, browser, sigma-voice) open PCM streams.
 * The mixer combines all streams before writing to hardware DMA buffer.
 *
 * Real-time guarantee: period callback fires every 5ms at 48kHz.
 * This requires SCHED_RT_FIFO (sigma_rtsched.h) for the mixer thread.
 *
 * Inspired by: PipeWire (graph model), ALSA PCM (snd_pcm_*), PulseAudio
 */

#pragma once
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

/* ── PCM stream format ───────────────────────────────────────────────────── */

typedef enum sigma_sample_fmt {
    SIGMA_PCM_S16_LE  = 0,   /* 16-bit signed little-endian */
    SIGMA_PCM_S24_3LE = 1,   /* 24-bit packed */
    SIGMA_PCM_S32_LE  = 2,
    SIGMA_PCM_F32_LE  = 3,   /* 32-bit float (preferred internally) */
} sigma_sample_fmt_t;

typedef struct sigma_pcm_format {
    uint32_t           sample_rate;    /* 44100, 48000, 96000, 192000 */
    uint32_t           channels;       /* 1, 2, 4, 6 (5.1), 8 (7.1) */
    sigma_sample_fmt_t format;
    uint32_t           period_frames;  /* frames per period callback */
    uint32_t           buffer_periods; /* ring buffer size in periods */
} sigma_pcm_format_t;

/* ── PCM stream ───────────────────────────────────────────────────────────── */

typedef enum sigma_pcm_state {
    PCM_STATE_OPEN       = 0,
    PCM_STATE_PREPARED   = 1,
    PCM_STATE_RUNNING    = 2,
    PCM_STATE_XRUN       = 3,   /* underrun (playback) or overrun (capture) */
    PCM_STATE_SUSPENDED  = 4,
    PCM_STATE_PAUSED     = 5,
} sigma_pcm_state_t;

typedef struct sigma_pcm_stream {
    uint32_t             id;
    uint32_t             client_shard;
    sigma_pcm_format_t   fmt;
    sigma_pcm_state_t    state;
    bool                 is_capture;   /* true=record, false=playback */
    float                volume;       /* 0.0–1.0 */
    float                pan;          /* -1.0 (left) to +1.0 (right) */
    /* Ring buffer (shared memory between client and server) */
    float               *ring_buf;     /* F32 interleaved frames */
    uint32_t             ring_frames;  /* total ring buffer size in frames */
    uint32_t             write_pos;    /* server updates (playback) */
    uint32_t             read_pos;     /* server updates (capture) */
    /* Statistics */
    uint64_t             xrun_count;
    uint64_t             frames_processed;
} sigma_pcm_stream_t;

/* ── Audio device (hardware) ─────────────────────────────────────────────── */

typedef struct sigma_audio_device {
    uint32_t    shard_id;
    char        name[64];
    char        card_name[32];
    uint32_t    min_rate;
    uint32_t    max_rate;
    uint32_t    max_channels;
    bool        has_capture;
    bool        has_playback;
    bool        hw_volume;     /* hardware volume control available */
    bool        hw_mute;
} sigma_audio_device_t;

/* ── Period callback type ────────────────────────────────────────────────── */

typedef void (*sigma_pcm_period_cb)(uint32_t stream_id, uint32_t avail_frames,
                                     void *userdata);

/* ── PCM API ─────────────────────────────────────────────────────────────── */

/* Device enumeration */
int  sigma_audio_list_devices (sigma_audio_device_t *out, uint32_t max, uint32_t *count);
int  sigma_audio_default_device(bool capture, sigma_audio_device_t *out);

/* Stream lifecycle */
int  sigma_pcm_open    (uint32_t device_shard, const sigma_pcm_format_t *fmt,
                         bool capture, sigma_pcm_stream_t *out);
int  sigma_pcm_prepare (uint32_t stream_id);
int  sigma_pcm_start   (uint32_t stream_id);
int  sigma_pcm_stop    (uint32_t stream_id);
int  sigma_pcm_pause   (uint32_t stream_id, bool pause);
int  sigma_pcm_close   (uint32_t stream_id);
int  sigma_pcm_recover (uint32_t stream_id);  /* recover from xrun */

/* Data transfer (F32 interleaved) */
ssize_t sigma_pcm_write(uint32_t stream_id, const float *buf, uint32_t frames);
ssize_t sigma_pcm_read (uint32_t stream_id, float *buf, uint32_t frames);

/* Non-blocking: returns available frames */
uint32_t sigma_pcm_avail_write(uint32_t stream_id);
uint32_t sigma_pcm_avail_read (uint32_t stream_id);

/* Period callback (called from RT audio thread) */
int  sigma_pcm_set_period_cb (uint32_t stream_id, sigma_pcm_period_cb cb,
                                void *userdata);

/* Volume / mixing */
int  sigma_pcm_set_volume(uint32_t stream_id, float volume);
int  sigma_pcm_set_pan   (uint32_t stream_id, float pan);

/* Mixer graph (PipeWire-style linking) */
int  sigma_pcm_link  (uint32_t src_id, uint32_t dst_id);
int  sigma_pcm_unlink(uint32_t src_id, uint32_t dst_id);

/* Hardware volume (if device supports it) */
int  sigma_hw_set_volume(uint32_t device_shard, uint32_t channel, float vol);
int  sigma_hw_set_mute  (uint32_t device_shard, bool mute);
