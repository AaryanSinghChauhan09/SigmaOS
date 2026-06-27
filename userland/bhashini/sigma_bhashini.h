// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_bhashini.h — Bhashini AI: 22 Indian languages, fully offline
 *
 * Bhashini is India's national language translation mission (MeitY).
 * sigma_bhashini integrates Bhashini models as OS-level primitives:
 *   - ASR (Automatic Speech Recognition): Whisper-India models
 *   - TTS (Text-to-Speech): 22 Indian languages
 *   - NMT (Neural Machine Translation): any language pair
 *   - OCR: for Devanagari, Tamil, Telugu, Bengali, etc.
 *   - STT → NMT → TTS pipeline: speak Hindi → output Tamil
 *
 * All models run locally via sigma-ai (port 17392).
 * No data leaves the device. Works offline (trains + villages).
 *
 * Supported languages (ISO 639-1 / BCP-47):
 *   hi (Hindi), bn (Bengali), te (Telugu), mr (Marathi), ta (Tamil),
 *   gu (Gujarati), kn (Kannada), pa (Punjabi), ml (Malayalam),
 *   or (Odia), as (Assamese), mai (Maithili), ur (Urdu),
 *   sa (Sanskrit), kok (Konkani), mni (Manipuri), brx (Bodo),
 *   doi (Dogri), ks (Kashmiri), sd (Sindhi), sat (Santali), en (English)
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

#define SIGMA_BHASHINI_MAX_LANGS 22

/* ── Language codes ──────────────────────────────────────────────────────── */
typedef enum {
    SIGMA_LANG_HI  = 1,   /* Hindi    */
    SIGMA_LANG_BN  = 2,   /* Bengali  */
    SIGMA_LANG_TE  = 3,   /* Telugu   */
    SIGMA_LANG_MR  = 4,   /* Marathi  */
    SIGMA_LANG_TA  = 5,   /* Tamil    */
    SIGMA_LANG_GU  = 6,   /* Gujarati */
    SIGMA_LANG_KN  = 7,   /* Kannada  */
    SIGMA_LANG_PA  = 8,   /* Punjabi  */
    SIGMA_LANG_ML  = 9,   /* Malayalam*/
    SIGMA_LANG_OR  = 10,  /* Odia     */
    SIGMA_LANG_AS  = 11,  /* Assamese */
    SIGMA_LANG_UR  = 12,  /* Urdu     */
    SIGMA_LANG_SA  = 13,  /* Sanskrit */
    SIGMA_LANG_EN  = 22,  /* English  */
} sigma_lang_t;

/* ── ASR result ──────────────────────────────────────────────────────────── */
typedef struct {
    char     transcript[4096];
    sigma_lang_t language_detected;
    float    confidence;
    double   duration_s;
} sigma_asr_result_t;

/* ── TTS config ──────────────────────────────────────────────────────────── */
typedef struct {
    sigma_lang_t language;
    char         voice_id[32];   /* "male-1", "female-1" per language      */
    float        speed;          /* 0.5–2.0, 1.0 = normal                 */
    float        pitch;          /* 0.5–2.0, 1.0 = normal                 */
} sigma_tts_config_t;

/* ── Translation pair ────────────────────────────────────────────────────── */
typedef struct {
    sigma_lang_t source_lang;
    sigma_lang_t target_lang;
    char         source_text[4096];
    char         translated_text[4096];
    float        confidence;
} sigma_translation_t;

/* ── API ─────────────────────────────────────────────────────────────────── */

/* Transcribe PCM audio (16kHz mono float32) to text. */
int sigma_bhashini_asr(const float *pcm, size_t n_samples,
                        sigma_lang_t hint_lang,
                        sigma_asr_result_t *out);

/* Synthesise text to PCM audio. */
int sigma_bhashini_tts(const char *text, const sigma_tts_config_t *cfg,
                        float **pcm_out, size_t *n_samples_out);

/* Translate text between any language pair. */
int sigma_bhashini_translate(sigma_translation_t *req);

/* Full pipeline: audio → source text → translated audio. */
int sigma_bhashini_pipeline(const float *input_pcm, size_t input_samples,
                              sigma_lang_t source_lang,
                              sigma_lang_t target_lang,
                              float **output_pcm, size_t *output_samples,
                              char *transcript_out, size_t transcript_max);

/* OCR: image → text in detected Indian script. */
int sigma_bhashini_ocr(const sigma_u8 *image_rgba, sigma_u32 width,
                        sigma_u32 height, sigma_lang_t hint_lang,
                        char *text_out, size_t max_len);

/* Check if a language model is available locally. */
bool sigma_bhashini_model_available(sigma_lang_t lang);

/* Download a language model (requires network, then cached offline). */
int sigma_bhashini_download_model(sigma_lang_t lang);
