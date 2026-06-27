// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_audio_server.h — Sound server (PipeWire-inspired)
 *
 * The audio server owns the hardware and multiplexes streams from all apps.
 * Apps connect to sigma-audiod via Unix socket, register a stream,
 * then push PCM frames. The server mixes and sends to ALSA/HDA driver.
 *
 * Node graph (PipeWire-inspired):
 *   [App A stream] ─┐
 *   [App B stream] ─┤─→ [Mixer node] → [Sink: HDA device]
 *   [System sounds]─┘
 *                          ↑
 *            [Source: Microphone] → [App capture stream]
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

#define SIGMA_AUDIO_SOCK      "/run/sigma/audiod.sock"
#define SIGMA_AUDIO_MAX_RATE  192000
#define SIGMA_AUDIO_MAX_CHANS 8
#define SIGMA_AUDIO_PERIOD    256   /* frames per callback (low latency)     */

typedef struct {
    sigma_u32 sample_rate;  /* 44100, 48000, 96000, 192000               */
    sigma_u8  channels;     /* 1=mono, 2=stereo, 6=5.1, 8=7.1           */
    sigma_u8  bits;         /* 16 or 32                                  */
    bool      is_float;     /* false=int, true=float32                   */
} sigma_audio_format_t;

typedef enum {
    SIGMA_AUDIO_PLAYBACK = 0,  /* app → server → hardware                */
    SIGMA_AUDIO_CAPTURE  = 1,  /* hardware → server → app                */
} sigma_audio_direction_t;

/* ── Stream API (used by apps) ───────────────────────────────────────────── */
typedef struct sigma_audio_stream sigma_audio_stream_t;

/* Connect to audio server and create a stream */
sigma_audio_stream_t* sigma_audio_stream_open(sigma_audio_direction_t dir,
                                               const sigma_audio_format_t* fmt,
                                               const char* app_name);

/* Write PCM frames to a playback stream (blocks if buffer full) */
int sigma_audio_stream_write(sigma_audio_stream_t* s,
                               const void* frames, sigma_size_t n_frames);

/* Read PCM frames from a capture stream */
int sigma_audio_stream_read(sigma_audio_stream_t* s,
                              void* frames, sigma_size_t n_frames);

int  sigma_audio_stream_start(sigma_audio_stream_t* s);
int  sigma_audio_stream_stop(sigma_audio_stream_t* s);
void sigma_audio_stream_close(sigma_audio_stream_t* s);

/* Set volume 0–100 for this stream */
int sigma_audio_stream_set_volume(sigma_audio_stream_t* s, sigma_u8 vol);

/* ── Global volume control (via sigma.Audio bus interface) ───────────────── */
int  sigma_audio_set_master_volume(sigma_u8 vol_0_to_100);
int  sigma_audio_get_master_volume(sigma_u8* out);
bool sigma_audio_is_muted(void);
int  sigma_audio_set_muted(bool muted);

/* ── PulseAudio compatibility shim ──────────────────────────────────────── */
/* Apps that use PulseAudio API can link against sigma-pulse-compat.so       */
/* which redirects all pa_* calls to sigma_audio_* calls above               */
