// SPDX-License-Identifier: GPL-2.0-or-later
// sigma_wasm_syscall.cpp — SigmaOS → WebAssembly syscall bridge
//
// Maps SigmaOS system calls to browser Web APIs.
// Built with: cmake -DSIGMA_TARGET=wasm -DWASI_SDK=/opt/wasi-sdk ..
//
// Syscall mapping strategy:
//   open/read/write/close → OPFS (Origin Private File System)
//   socket/send/recv      → WebSocket proxy (sigma-net-proxy server)
//   mmap(framebuffer)     → WebGPU canvas texture
//   gettime               → performance.now()
//   getrandom             → crypto.getRandomValues()
//
// Inspired by: Emscripten POSIX layer, WASI SDK, Cosmopolitan libc WASM

#include <stdint.h>
#include <stddef.h>
#include <string.h>

// ── WASM imports (provided by browser host page) ──────────────────────────

// These functions are imported from the JavaScript host.
// Declared with __attribute__((import_module("sigma_env"), import_name("...")))
// for use with the WASI SDK.

extern void   sigma_host_draw_frame  (const uint8_t *fb, uint32_t w, uint32_t h);
extern void   sigma_host_play_audio  (const float *buf, uint32_t frames, uint32_t rate);
extern int    sigma_host_net_send    (const uint8_t *buf, uint32_t len);
extern int    sigma_host_net_recv    (uint8_t *buf, uint32_t cap);
extern double sigma_host_get_time_ms (void);
extern void   sigma_host_random_bytes(uint8_t *buf, uint32_t len);
extern void   sigma_host_log         (const char *msg, uint32_t len);
extern int    sigma_host_fs_open     (const char *path, uint32_t flags);
extern int    sigma_host_fs_read     (int handle, uint8_t *buf, uint32_t len);
extern int    sigma_host_fs_write    (int handle, const uint8_t *buf, uint32_t len);
extern int    sigma_host_fs_close    (int handle);
extern int    sigma_host_fs_stat     (const char *path, uint32_t *out_size);

// ── SigmaOS WASM syscall numbers ──────────────────────────────────────────

#define SIGMA_WASM_SYS_EXIT        1
#define SIGMA_WASM_SYS_READ        3
#define SIGMA_WASM_SYS_WRITE       4
#define SIGMA_WASM_SYS_OPEN        5
#define SIGMA_WASM_SYS_CLOSE       6
#define SIGMA_WASM_SYS_GETTIME     13
#define SIGMA_WASM_SYS_GETRANDOM   318
#define SIGMA_WASM_SYS_SOCKET      281
#define SIGMA_WASM_SYS_SENDTO      290
#define SIGMA_WASM_SYS_RECVFROM    292

// ── Framebuffer → WebGPU canvas ───────────────────────────────────────────

static uint8_t wasm_framebuffer[1280 * 720 * 4];  // XRGB8888
static uint32_t wasm_fb_width  = 1280;
static uint32_t wasm_fb_height = 720;

void sigma_wasm_flush_display(void) {
    sigma_host_draw_frame(wasm_framebuffer, wasm_fb_width, wasm_fb_height);
}

uint8_t *sigma_wasm_get_framebuffer(uint32_t *w, uint32_t *h) {
    *w = wasm_fb_width;
    *h = wasm_fb_height;
    return wasm_framebuffer;
}

// ── OPFS filesystem bridge ────────────────────────────────────────────────

int sigma_wasm_open(const char *path, uint32_t flags) {
    return sigma_host_fs_open(path, flags);
}

ssize_t sigma_wasm_read(int fd, void *buf, size_t len) {
    return sigma_host_fs_read(fd, (uint8_t *)buf, (uint32_t)len);
}

ssize_t sigma_wasm_write(int fd, const void *buf, size_t len) {
    return sigma_host_fs_write(fd, (const uint8_t *)buf, (uint32_t)len);
}

int sigma_wasm_close(int fd) {
    return sigma_host_fs_close(fd);
}

// ── WebSocket network bridge ──────────────────────────────────────────────
// All TCP connections are proxied through the sigma-net-proxy WebSocket relay.

int sigma_wasm_socket_send(const void *buf, size_t len) {
    return sigma_host_net_send((const uint8_t *)buf, (uint32_t)len);
}

int sigma_wasm_socket_recv(void *buf, size_t cap) {
    return sigma_host_net_recv((uint8_t *)buf, (uint32_t)cap);
}

// ── Time ──────────────────────────────────────────────────────────────────

uint64_t sigma_wasm_clock_ns(void) {
    return (uint64_t)(sigma_host_get_time_ms() * 1e6);
}

// ── Random ────────────────────────────────────────────────────────────────

void sigma_wasm_getrandom(void *buf, size_t len) {
    sigma_host_random_bytes((uint8_t *)buf, (uint32_t)len);
}

// ── Log ───────────────────────────────────────────────────────────────────

void sigma_wasm_log(const char *msg) {
    sigma_host_log(msg, (uint32_t)strlen(msg));
}

// ── WASM entry point (called by JS after WebAssembly.instantiate) ─────────

__attribute__((export_name("sigma_wasm_main")))
void sigma_wasm_main(void) {
    // Override platform hooks before calling kernel main
    extern void sigma_platform_init_wasm(void);
    sigma_platform_init_wasm();

    // Boot the SigmaOS kernel (sigma_boot.cpp)
    extern int sigma_kernel_main(void);
    sigma_kernel_main();
}

// ── Input events (called from JS event listeners) ─────────────────────────

__attribute__((export_name("sigma_input_key")))
void sigma_input_key(uint32_t keycode, uint32_t pressed) {
    extern void sigma_hid_key_event(uint32_t keycode, uint32_t pressed);
    sigma_hid_key_event(keycode, pressed);
}

__attribute__((export_name("sigma_input_mouse")))
void sigma_input_mouse(int32_t x, int32_t y, uint32_t buttons) {
    extern void sigma_hid_mouse_event(int32_t x, int32_t y, uint32_t buttons);
    sigma_hid_mouse_event(x, y, buttons);
}
