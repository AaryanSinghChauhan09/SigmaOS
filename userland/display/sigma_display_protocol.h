// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_display_protocol.h — Isolated display server protocol (Haiku app_server-inspired)
 *
 * Applications send drawing commands to sigma-display-server via a Unix socket.
 * The display server renders on behalf of the app — apps never touch the framebuffer.
 * A compromised browser cannot read other windows or sniff keystrokes.
 *
 * Haiku analogy:
 *   BApplication ↔ ServerApp (via port/IPC)
 *   BView drawing calls → AS_* commands serialized to pipe
 *   Painter in app_server executes them with AGG
 *
 * SigmaOS:
 *   App → sigma_ds_fill_rect() → serialized cmd on cmd_fd
 *   sigma-display-server reads, composites, blits to real framebuffer
 *   Input events flow back on event_fd (key/mouse/touch)
 */
#include <sigma_kernel_types.h>
#include <stdint.h>

/* Drawing command opcodes */
typedef enum {
    SIGMA_DS_FILL_RECT      = 0x01,
    SIGMA_DS_DRAW_STRING    = 0x02,
    SIGMA_DS_BLIT_BITMAP    = 0x03,
    SIGMA_DS_DRAW_BEZIER    = 0x04,
    SIGMA_DS_SET_CLIP_RECT  = 0x05,
    SIGMA_DS_PUSH_STATE     = 0x06,
    SIGMA_DS_POP_STATE      = 0x07,
    SIGMA_DS_CREATE_WINDOW  = 0x10,
    SIGMA_DS_DESTROY_WINDOW = 0x11,
    SIGMA_DS_RESIZE_WINDOW  = 0x12,
    SIGMA_DS_MOVE_WINDOW    = 0x13,
    SIGMA_DS_SHOW_WINDOW    = 0x14,
    SIGMA_DS_HIDE_WINDOW    = 0x15,
    SIGMA_DS_SHOW_CURSOR    = 0x20,
    SIGMA_DS_HIDE_CURSOR    = 0x21,
    SIGMA_DS_COMPOSITE      = 0x30,
    SIGMA_DS_FLUSH          = 0x31,   /* present to screen */
} sigma_ds_opcode_t;

/* Wire format: fixed header + variable payload */
typedef struct __attribute__((packed)) {
    sigma_u32 opcode;     /* sigma_ds_opcode_t                  */
    sigma_u32 len;        /* bytes of payload following         */
    sigma_u32 window_id;
    sigma_u32 seq;        /* sequence number for ordering       */
} sigma_ds_cmd_hdr_t;

/* FILL_RECT payload */
typedef struct __attribute__((packed)) {
    sigma_i32 x, y, w, h;
    sigma_u32 rgba;
} sigma_ds_fill_rect_t;

/* BLIT_BITMAP payload (followed by pixel data) */
typedef struct __attribute__((packed)) {
    sigma_i32 dst_x, dst_y;
    sigma_i32 src_w, src_h;
    sigma_u32 format;    /* 0 = RGBA8888                        */
    /* pixel data follows: src_w * src_h * 4 bytes              */
} sigma_ds_blit_bitmap_t;

/* CREATE_WINDOW payload */
typedef struct __attribute__((packed)) {
    sigma_i32 x, y, w, h;
    sigma_u32 flags;      /* SIGMA_WIN_RESIZABLE | SIGMA_WIN_BORDERLESS | ... */
    char      title[128];
} sigma_ds_create_window_t;

#define SIGMA_WIN_RESIZABLE  (1u << 0)
#define SIGMA_WIN_BORDERLESS (1u << 1)
#define SIGMA_WIN_ALWAYS_TOP (1u << 2)
#define SIGMA_WIN_DIALOG     (1u << 3)

/* Input event (sent by display server to app on event_fd) */
typedef struct __attribute__((packed)) {
    sigma_u32 type;       /* SIGMA_EVT_KEY_DOWN | SIGMA_EVT_MOUSE_MOVE | ... */
    sigma_u32 window_id;
    sigma_u64 timestamp_ns;
    union {
        struct { sigma_u32 keycode; sigma_u32 modifiers; char utf8[8]; } key;
        struct { sigma_i32 x, y; sigma_u32 buttons; } mouse;
        struct { sigma_i32 x, y; float pressure; } touch;
    };
} sigma_ds_event_t;

#define SIGMA_EVT_KEY_DOWN    0x01
#define SIGMA_EVT_KEY_UP      0x02
#define SIGMA_EVT_MOUSE_MOVE  0x10
#define SIGMA_EVT_MOUSE_DOWN  0x11
#define SIGMA_EVT_MOUSE_UP    0x12
#define SIGMA_EVT_TOUCH       0x20
#define SIGMA_EVT_CLOSE       0x30

/* Application-side channel (connected to sigma-display-server) */
typedef struct {
    int      cmd_fd;        /* write drawing commands here                */
    int      event_fd;      /* read input events from here                */
    sigma_u32 window_id;
    void*    shared_fb;     /* optional shared framebuffer for zero-copy  */
    sigma_size_t shared_fb_size;
} sigma_ds_channel_t;

/* ── Application-side API ─────────────────────────────────────────────────── */

int  sigma_ds_connect   (sigma_ds_channel_t* ch, const char* app_name);
void sigma_ds_disconnect(sigma_ds_channel_t* ch);

/* Create/destroy a window */
sigma_u32 sigma_ds_create_window (sigma_ds_channel_t* ch, int x, int y, int w, int h,
                                    sigma_u32 flags, const char* title);
void      sigma_ds_destroy_window(sigma_ds_channel_t* ch, sigma_u32 window_id);

/* Drawing primitives */
void sigma_ds_fill_rect  (sigma_ds_channel_t* ch, int x, int y, int w, int h, sigma_u32 rgba);
void sigma_ds_draw_string(sigma_ds_channel_t* ch, int x, int y, const char* text,
                           sigma_u32 color, float size);
void sigma_ds_blit_bitmap(sigma_ds_channel_t* ch, int x, int y,
                           const sigma_u8* pixels, int px_w, int px_h);
void sigma_ds_flush      (sigma_ds_channel_t* ch);

/* Poll for input events (non-blocking) */
int  sigma_ds_poll_event (sigma_ds_channel_t* ch, sigma_ds_event_t* out);
