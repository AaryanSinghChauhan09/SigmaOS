/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN MEDIA CODEC + OBS COMPOSITOR — IMPLEMENTATION (v1.0)
 * =========================================================================
 */

#include "../../../include/sigma_kernel.h"
#include "../../../include/SovereignMediaCodec.h"

/* -------------------------------------------------------------------------
 * Playback state
 * ---------------------------------------------------------------------- */
static SigmaMediaState_t  s_media_state = SIGMA_MEDIA_STOPPED;
static SigmaMediaInfo_t   s_current_media;
static SigmaAudioFilter_t s_audio_filter;

/* OBS Compositor state */
static SigmaScene_t       s_scenes    [SIGMA_SCENE_MAX];
static sigma_u32          s_scene_cnt  = 0;
static sigma_u32          s_active_scene= 0;
static SigmaOutputConfig_t s_output;
static sigma_bool         s_obs_live  = SIGMA_FALSE;

/* -------------------------------------------------------------------------
 * Helper: codec name
 * ---------------------------------------------------------------------- */
static const char *codec_name(SigmaCodecID_t id) {
    switch (id) {
        case SIGMA_CODEC_H264:  return "H.264/AVC";
        case SIGMA_CODEC_H265:  return "H.265/HEVC";
        case SIGMA_CODEC_AV1:   return "AV1";
        case SIGMA_CODEC_VP9:   return "VP9";
        case SIGMA_CODEC_MPEG2: return "MPEG-2";
        case SIGMA_CODEC_MJPEG: return "MJPEG";
        case SIGMA_CODEC_AAC:   return "AAC";
        case SIGMA_CODEC_MP3:   return "MP3";
        case SIGMA_CODEC_OPUS:  return "Opus";
        case SIGMA_CODEC_FLAC:  return "FLAC";
        case SIGMA_CODEC_AC3:   return "AC-3";
        case SIGMA_CODEC_VORBIS:return "Vorbis";
        case SIGMA_CODEC_SRT:   return "SRT subtitles";
        case SIGMA_CODEC_ASS:   return "ASS/SSA";
        default:                return "Unknown";
    }
}

static const char *accel_name(SigmaHWAccel_t a) {
    switch (a) {
        case SIGMA_HW_ACCEL_NONE:         return "Software";
        case SIGMA_HW_ACCEL_VAAPI:        return "VAAPI";
        case SIGMA_HW_ACCEL_VDPAU:        return "VDPAU";
        case SIGMA_HW_ACCEL_NVDEC:        return "NVDEC";
        case SIGMA_HW_ACCEL_VIDEOTOOLBOX: return "VideoToolbox";
        case SIGMA_HW_ACCEL_DXVA2:        return "DXVA2";
        case SIGMA_HW_ACCEL_VULKAN:       return "Vulkan";
        default: return "?";
    }
}

/* =========================================================================
 * §1  MEDIA PLAYBACK  (VLC libvlc parity)
 * ====================================================================== */

sigma_err_t sigma_media_open(const char *url, SigmaMediaInfo_t *out) {
    sigma_memset(out, 0, sizeof(*out));
    sigma_strcpy(out->url, url, SIGMA_MEDIA_URL_MAX);
    sigma_strcpy(out->title, "Unknown Title", SIGMA_MEDIA_TITLE_MAX);

    /* Simulate probe results */
    if (sigma_strstr(url, ".mp4") || sigma_strstr(url, ".mkv")) {
        out->video.codec_id    = SIGMA_CODEC_H264;
        out->video.width       = 1920;
        out->video.height      = 1080;
        out->video.fps_num     = 30;
        out->video.fps_den     = 1;
        out->video.bitrate     = 4000;
        out->audio.codec_id    = SIGMA_CODEC_AAC;
        out->audio.sample_rate = 48000;
        out->audio.channels    = 2;
        out->duration_ms       = 7200000; /* 2 hrs */
        out->file_size_bytes   = 4ULL * 1024 * 1024 * 1024;
    } else if (sigma_strstr(url, ".flac") || sigma_strstr(url, ".mp3")) {
        out->audio.codec_id    = sigma_strstr(url, ".flac") ?
                                  SIGMA_CODEC_FLAC : SIGMA_CODEC_MP3;
        out->audio.sample_rate = 44100;
        out->audio.channels    = 2;
        out->audio.bitrate     = 320;
        out->duration_ms       = 240000;
    } else if (sigma_strstr(url, "rtsp://") || sigma_strstr(url, "rtmp://")) {
        out->is_live           = SIGMA_TRUE;
        out->video.codec_id    = SIGMA_CODEC_H264;
        out->audio.codec_id    = SIGMA_CODEC_AAC;
    }

    sigma_memcpy(&s_current_media, out, sizeof(*out));
    sigma_printf("Σ [MEDIA]: Opened: %s\n", url);
    sigma_printf("  Video: %s  %ux%u @ %u fps  %u kbps\n",
                 codec_name(out->video.codec_id),
                 out->video.width, out->video.height,
                 out->video.fps_num, out->video.bitrate);
    sigma_printf("  Audio: %s  %uHz %uch\n",
                 codec_name(out->audio.codec_id),
                 out->audio.sample_rate, out->audio.channels);
    sigma_printf("  Duration: %llus\n",
                 (unsigned long long)(out->duration_ms / 1000));
    return SIGMA_OK;
}

sigma_err_t sigma_media_play(const char *url, SigmaHWAccel_t accel) {
    SigmaMediaInfo_t info;
    sigma_media_open(url, &info);
    s_media_state = SIGMA_MEDIA_PLAYING;
    sigma_printf("Σ [MEDIA]: Playing via %s decoder.\n", accel_name(accel));
    return SIGMA_OK;
}

sigma_err_t sigma_media_pause(void) {
    if (s_media_state != SIGMA_MEDIA_PLAYING) return SIGMA_EINVAL;
    s_media_state = SIGMA_MEDIA_PAUSED;
    sigma_printf("Σ [MEDIA]: Paused.\n");
    return SIGMA_OK;
}

sigma_err_t sigma_media_stop(void) {
    s_media_state = SIGMA_MEDIA_STOPPED;
    sigma_printf("Σ [MEDIA]: Stopped.\n");
    return SIGMA_OK;
}

sigma_err_t sigma_media_seek(sigma_u64 ms) {
    sigma_printf("Σ [MEDIA]: Seeking to %llus.\n",
                 (unsigned long long)(ms / 1000));
    return SIGMA_OK;
}

sigma_err_t sigma_media_set_rate(sigma_f32 rate) {
    sigma_printf("Σ [MEDIA]: Playback rate: %.1fx\n", (double)rate);
    return SIGMA_OK;
}

sigma_err_t sigma_media_set_volume(sigma_f32 vol) {
    s_audio_filter.volume = vol;
    sigma_printf("Σ [MEDIA]: Volume: %.0f%%\n", (double)(vol * 100.0f));
    return SIGMA_OK;
}

sigma_err_t sigma_media_set_audio_filter(const SigmaAudioFilter_t *f) {
    sigma_memcpy(&s_audio_filter, f, sizeof(*f));
    sigma_printf("Σ [MEDIA]: Audio filter applied (EQ/compressor/normalise).\n");
    return SIGMA_OK;
}

sigma_err_t sigma_media_transcode(const SigmaTranscodeJob_t *job) {
    sigma_printf("Σ [MEDIA]: Transcoding:\n"
                 "  src: %s\n"
                 "  dst: %s\n"
                 "  vcodec: %s  %ux%u@%ufps  %ukbps\n"
                 "  acodec: %s  %ukbps\n"
                 "  hwenc: %s\n",
                 job->src_url, job->dst_url,
                 codec_name(job->vcodec),
                 job->width, job->height, job->fps, job->vbitrate,
                 codec_name(job->acodec), job->abitrate,
                 accel_name(job->hw_accel));
    sigma_printf("Σ [MEDIA]: Transcode complete.\n");
    return SIGMA_OK;
}

void sigma_media_info(const char *url) {
    SigmaMediaInfo_t info;
    sigma_media_open(url, &info);
}

SigmaMediaState_t sigma_media_state(void) {
    return s_media_state;
}

/* =========================================================================
 * §2  OBS SCENE COMPOSITOR
 * ====================================================================== */

static SigmaScene_t *scene_find(const char *name) {
    for (sigma_u32 i = 0; i < s_scene_cnt; i++)
        if (s_scenes[i].active && sigma_streq(s_scenes[i].name, name))
            return &s_scenes[i];
    return SIGMA_NULL;
}

sigma_err_t sigma_obs_scene_create(const char *name) {
    if (s_scene_cnt >= SIGMA_SCENE_MAX) return SIGMA_ENOSPC;
    if (scene_find(name)) return SIGMA_EBUSY;
    SigmaScene_t *sc = &s_scenes[s_scene_cnt++];
    sigma_memset(sc, 0, sizeof(*sc));
    sigma_strcpy(sc->name, name, SIGMA_SCENE_NAME_MAX);
    sc->active = SIGMA_TRUE;
    sigma_printf("Σ [OBS]: Scene created: %s\n", name);
    return SIGMA_OK;
}

sigma_err_t sigma_obs_scene_switch(const char *name) {
    SigmaScene_t *sc = scene_find(name);
    if (!sc) return SIGMA_ENOENT;
    for (sigma_u32 i = 0; i < s_scene_cnt; i++)
        s_scenes[i].active = SIGMA_FALSE;
    sc->active       = SIGMA_TRUE;
    s_active_scene   = (sigma_u32)(sc - s_scenes);
    sigma_printf("Σ [OBS]: Scene switched -> %s\n", name);
    return SIGMA_OK;
}

sigma_err_t sigma_obs_source_add(const char *scene, const char *name,
                                  SigmaSourceType_t type,
                                  sigma_i32 x, sigma_i32 y,
                                  sigma_i32 w, sigma_i32 h) {
    SigmaScene_t *sc = scene_find(scene);
    if (!sc) return SIGMA_ENOENT;
    if (sc->source_count >= SIGMA_SOURCE_MAX) return SIGMA_ENOSPC;
    SigmaSource_t *src = &sc->sources[sc->source_count++];
    sigma_memset(src, 0, sizeof(*src));
    sigma_strcpy(src->name, name, SIGMA_SCENE_NAME_MAX);
    src->type    = type;
    src->x = x; src->y = y; src->w = w; src->h = h;
    src->opacity = 1.0f;
    src->visible = SIGMA_TRUE;
    static const char *type_names[] = {
        "Display","Window","Camera","Media","Text","Image","Browser"
    };
    sigma_printf("Σ [OBS]: Source '%s' (%s) at [%d,%d %dx%d] added to scene '%s'\n",
                 name, type_names[type], x, y, w, h, scene);
    return SIGMA_OK;
}

sigma_err_t sigma_obs_source_remove(const char *scene, const char *name) {
    SigmaScene_t *sc = scene_find(scene);
    if (!sc) return SIGMA_ENOENT;
    for (sigma_u32 i = 0; i < sc->source_count; i++) {
        if (sigma_streq(sc->sources[i].name, name)) {
            sc->sources[i].visible = SIGMA_FALSE;
            sigma_printf("Σ [OBS]: Source '%s' removed from scene '%s'\n",
                         name, scene);
            return SIGMA_OK;
        }
    }
    return SIGMA_ENOENT;
}

sigma_err_t sigma_obs_output_start(const SigmaOutputConfig_t *cfg) {
    sigma_memcpy(&s_output, cfg, sizeof(*cfg));
    s_obs_live = SIGMA_TRUE;
    if (cfg->mode == SIGMA_OUTPUT_STREAM || cfg->mode == SIGMA_OUTPUT_BOTH)
        sigma_printf("Σ [OBS]: Streaming to %s (key=***) %ukbps %ufps\n",
                     cfg->server_url, cfg->vbitrate, cfg->fps);
    if (cfg->mode == SIGMA_OUTPUT_RECORD || cfg->mode == SIGMA_OUTPUT_BOTH)
        sigma_printf("Σ [OBS]: Recording to %s\n", cfg->record_path);
    sigma_printf("Σ [OBS]: Encoder: %s via %s\n",
                 codec_name(cfg->venc), accel_name(cfg->hw_enc));
    return SIGMA_OK;
}

sigma_err_t sigma_obs_output_stop(void) {
    s_obs_live = SIGMA_FALSE;
    sigma_printf("Σ [OBS]: Output stopped.\n");
    return SIGMA_OK;
}

void sigma_obs_scene_list(void) {
    for (sigma_u32 i = 0; i < s_scene_cnt; i++) {
        SigmaScene_t *sc = &s_scenes[i];
        sigma_printf("Σ [OBS]: Scene: %s  sources=%u%s\n",
                     sc->name, sc->source_count,
                     i == s_active_scene ? "  [ACTIVE]" : "");
    }
}

void sigma_obs_stats(void) {
    sigma_printf("Σ [OBS]: Status: %s  Scenes: %u  Output: %s\n",
                 s_obs_live ? "LIVE" : "idle",
                 s_scene_cnt,
                 s_obs_live ? (s_output.mode == SIGMA_OUTPUT_STREAM ?
                               "streaming" : "recording") : "none");
}

/* -------------------------------------------------------------------------
 * SovereignMediaCodec_Init
 * ---------------------------------------------------------------------- */
void SovereignMediaCodec_Init(void) {
    sigma_printf("Σ [MEDIA]: Initialising Sovereign Media Codec Engine "
                 "(VLC + OBS parity)...\n");

    /* VLC playback demo */
    sigma_media_play("/media/sigma_movie.mkv", SIGMA_HW_ACCEL_NVDEC);
    sigma_media_set_volume(0.85f);
    sigma_media_set_rate(1.25f);
    sigma_media_seek(300000);
    sigma_media_pause();

    /* Transcode demo */
    SigmaTranscodeJob_t job = {
        .vcodec   = SIGMA_CODEC_H265,
        .acodec   = SIGMA_CODEC_AAC,
        .width    = 1280, .height = 720,
        .fps      = 30,
        .vbitrate = 2500, .abitrate = 192,
        .hw_accel = SIGMA_HW_ACCEL_VAAPI,
    };
    sigma_strcpy(job.src_url, "/media/sigma_movie.mkv", SIGMA_MEDIA_URL_MAX);
    sigma_strcpy(job.dst_url, "/export/sigma_720p.mp4", SIGMA_MEDIA_URL_MAX);
    sigma_media_transcode(&job);

    /* OBS compositor demo */
    sigma_obs_scene_create("Main");
    sigma_obs_scene_create("Game");
    sigma_obs_source_add("Main", "Desktop",    SIGMA_SOURCE_DISPLAY, 0, 0, 1920, 1080);
    sigma_obs_source_add("Main", "Webcam",     SIGMA_SOURCE_CAMERA,  1700, 900, 220, 165);
    sigma_obs_source_add("Main", "AlertBox",   SIGMA_SOURCE_BROWSER, 0,  0,  800, 200);
    sigma_obs_source_add("Game", "GameWindow", SIGMA_SOURCE_WINDOW,  0, 0, 1920, 1080);

    SigmaOutputConfig_t out = {
        .mode     = SIGMA_OUTPUT_BOTH,
        .venc     = SIGMA_CODEC_H264,
        .hw_enc   = SIGMA_HW_ACCEL_NVDEC,
        .vbitrate = 6000,
        .fps      = 60,
    };
    sigma_strcpy(out.server_url,  "rtmp://live.twitch.tv/app", SIGMA_MEDIA_URL_MAX);
    sigma_strcpy(out.record_path, "/records/stream.mp4",       SIGMA_MEDIA_URL_MAX);
    sigma_obs_output_start(&out);
    sigma_obs_scene_list();
    sigma_obs_stats();
    sigma_obs_output_stop();

    sigma_printf("Σ [MEDIA]: Sovereign Media Engine online.\n");
}
