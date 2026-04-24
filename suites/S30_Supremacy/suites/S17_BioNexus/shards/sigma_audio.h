/*
 * =========================================================================
 * S SIGMAOS kernel/suites/S17_BioNexus/shards/sigma_audio.h
 * =========================================================================
 * Sovereign Audio Subsystem — gap-closes:
 *   Linux  : ALSA (Advanced Linux Sound Architecture), PipeWire, PulseAudio
 *   Windows: WASAPI, WDM audio, DirectSound, ASIO
 *   macOS  : CoreAudio, AudioHAL, AudioUnit, AVAudioEngine
 *   Android: AAudio, Oboe, AudioFlinger, HAL audio
 *   RTOS   : PDM/SAI/I2S drivers, low-latency direct buffer
 * =========================================================================
 */

#ifndef SIGMA_AUDIO_H
#define SIGMA_AUDIO_H

typedef unsigned long long au_u64;
typedef unsigned int       au_u32;
typedef signed   int       au_i32;
typedef unsigned short     au_u16;
typedef unsigned char      au_u8;
typedef unsigned char      au_bool;
#define AU_TRUE  ((au_bool)1)
#define AU_FALSE ((au_bool)0)
#define AU_OK    ((au_i32) 0)
#define AU_ERR   ((au_i32)-1)

/* -- Sample formats -------------------------------------------------------- */
typedef enum {
    AU_FMT_S16LE  = 0,   /* 16-bit signed little-endian (CD quality)   */
    AU_FMT_S24LE  = 1,   /* 24-bit signed                              */
    AU_FMT_S32LE  = 2,   /* 32-bit signed                              */
    AU_FMT_F32LE  = 3,   /* 32-bit float (WASAPI exclusive mode)       */
    AU_FMT_F64LE  = 4    /* 64-bit float (studio mastering)            */
} sigma_au_fmt_t;

/* -- Stream direction ------------------------------------------------------ */
typedef enum {
    AU_PLAYBACK = 0,
    AU_CAPTURE  = 1,
    AU_DUPLEX   = 2
} sigma_au_dir_t;

/* -- Audio stream parameters ----------------------------------------------- */
typedef struct {
    sigma_au_fmt_t  fmt;
    sigma_au_dir_t  dir;
    au_u32          sample_rate;     /* 44100, 48000, 96000, 192000 Hz  */
    au_u32          channels;        /* 1=mono, 2=stereo, 8=7.1 surround*/
    au_u32          period_frames;   /* ALSA period_size equivalent     */
    au_u32          buffer_frames;   /* total ring buffer               */
    au_u32          owner_pid;
} sigma_au_params_t;

/* -- PCM stream (ALSA snd_pcm_t / WASAPI IAudioClient) ------------------- */
#define AU_MAX_STREAMS   32
#define AU_BUF_FRAMES  4096

typedef struct {
    au_u32          stream_id;
    sigma_au_params_t params;
    au_i32          state;      /* 0=idle 1=prepared 2=running 3=paused */

    /* Ring buffer */
    au_u8           buf[AU_BUF_FRAMES * 8]; /* max 8 bytes/sample/ch   */
    au_u32          buf_head;
    au_u32          buf_tail;
    au_u32          avail_frames;

    /* Telemetry */
    au_u64          frames_written;
    au_u64          frames_read;
    au_u64          xruns;      /* underruns (playback) / overruns (cap)*/
} sigma_au_stream_t;

/* -- Mixer control (ALSA amixer / CoreAudio volume) ----------------------- */
typedef struct {
    char    name[32];
    au_u32  volume_pct;   /* 0–100                                     */
    au_bool muted;
} sigma_au_control_t;

#define AU_MAX_CONTROLS 16

/* -- Public API ----------------------------------------------------------- */
void   sigma_audio_init(void);

/* Stream lifecycle */
au_i32 sigma_au_open(sigma_au_params_t *params);
au_i32 sigma_au_prepare(au_u32 sid);
au_i32 sigma_au_start(au_u32 sid);
au_i32 sigma_au_pause(au_u32 sid);
au_i32 sigma_au_stop(au_u32 sid);
void   sigma_au_close(au_u32 sid);

/* Data transfer */
au_i32 sigma_au_write(au_u32 sid, const void *buf, au_u32 frames);
au_i32 sigma_au_read(au_u32 sid, void *buf, au_u32 frames);
au_u32 sigma_au_avail(au_u32 sid);   /* frames available for read/write */

/* Mixer */
au_i32 sigma_au_set_volume(const char *ctrl, au_u32 pct);
au_u32 sigma_au_get_volume(const char *ctrl);
void   sigma_au_set_mute(const char *ctrl, au_bool muted);

/* Routing (PipeWire/AudioFlinger graph) */
void   sigma_au_route(au_u32 src_sid, au_u32 dst_sid);  /* software mix */

void   sigma_audio_stats(void);

#endif /* SIGMA_AUDIO_H */
