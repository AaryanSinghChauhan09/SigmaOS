// SigmaOS — sigma-media-codec: Minimal Media Codec Registry
// Inspired by: FFmpeg libavcodec, GStreamer plugin registry
// Module: sigma-media-codec
// USP: Plugin-free — codec is a C function pointer, no dlopen/dlsym
// Each codec: one decode fn + one encode fn, registered at boot

#ifndef SIGMA_MEDIA_CODEC_H
#define SIGMA_MEDIA_CODEC_H

#define SIGMA_CODEC_MAX         16
#define SIGMA_CODEC_NAME_LEN    16

typedef int (*codec_decode_fn)(const unsigned char* in, unsigned int in_len,
                                unsigned char* out, unsigned int out_max,
                                unsigned int* out_len);
typedef int (*codec_encode_fn)(const unsigned char* in, unsigned int in_len,
                                unsigned char* out, unsigned int out_max,
                                unsigned int* out_len);

typedef struct SigmaCodec {
    char           name[SIGMA_CODEC_NAME_LEN];
    unsigned int   codec_id;
    codec_decode_fn decode;
    codec_encode_fn encode;
    unsigned long  decode_calls;
    unsigned long  encode_calls;
    unsigned long  total_bytes_decoded;
    unsigned long  total_bytes_encoded;
} SigmaCodec;

typedef struct SigmaCodecRegistry {
    SigmaCodec   codecs[SIGMA_CODEC_MAX];
    unsigned int count;
} SigmaCodecRegistry;

static inline void codec_registry_init(SigmaCodecRegistry* r) { r->count = 0; }

static inline unsigned int codec_register(SigmaCodecRegistry* r, const char* name,
                                            codec_decode_fn dec, codec_encode_fn enc) {
    if (r->count >= SIGMA_CODEC_MAX) return 0xFFFFFFFF;
    SigmaCodec* c = &r->codecs[r->count];
    for (int i = 0; i < SIGMA_CODEC_NAME_LEN - 1 && name[i]; i++) c->name[i] = name[i];
    c->codec_id          = r->count++;
    c->decode            = dec;
    c->encode            = enc;
    c->decode_calls      = 0;
    c->encode_calls      = 0;
    c->total_bytes_decoded = 0;
    c->total_bytes_encoded = 0;
    return c->codec_id;
}

static inline SigmaCodec* codec_find(SigmaCodecRegistry* r, const char* name) {
    for (unsigned int i = 0; i < r->count; i++) {
        const char* n = r->codecs[i].name; const char* s = name;
        while (*n && *s && *n == *s) { n++; s++; }
        if (!*n && !*s) return &r->codecs[i];
    }
    return (void*)0;
}

static inline int codec_decode(SigmaCodec* c,
                                const unsigned char* in, unsigned int in_len,
                                unsigned char* out, unsigned int out_max,
                                unsigned int* out_len) {
    if (!c || !c->decode) return -1;
    int r = c->decode(in, in_len, out, out_max, out_len);
    if (r == 0) { c->decode_calls++; c->total_bytes_decoded += *out_len; }
    return r;
}

static inline int codec_encode(SigmaCodec* c,
                                const unsigned char* in, unsigned int in_len,
                                unsigned char* out, unsigned int out_max,
                                unsigned int* out_len) {
    if (!c || !c->encode) return -1;
    int r = c->encode(in, in_len, out, out_max, out_len);
    if (r == 0) { c->encode_calls++; c->total_bytes_encoded += *out_len; }
    return r;
}

// Built-in: passthrough codec (no-op, for testing)
static inline int codec_passthrough_decode(const unsigned char* in, unsigned int in_len,
                                            unsigned char* out, unsigned int out_max,
                                            unsigned int* out_len) {
    unsigned int n = in_len < out_max ? in_len : out_max;
    for (unsigned int i = 0; i < n; i++) out[i] = in[i];
    *out_len = n;
    return 0;
}

#endif /* SIGMA_MEDIA_CODEC_H */
