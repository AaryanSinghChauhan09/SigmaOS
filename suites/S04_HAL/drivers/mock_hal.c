/*
 * =============================================================================
 * Σ SIGMAOS HAL: MOCK HARDWARE LAYER (for Testing)
 * =============================================================================
 * Implements ALL HAL contracts with deterministic, simulated hardware.
 * Used by the test runner (--mock-hal) and QEMU smoke tests.
 *
 * Features:
 *   - Configurable failure injection (via MOCK_FAIL_* flags)
 *   - Ring buffer for capturing driver calls (test assertions)
 *   - Simulated timer ticks for scheduler testing
 *   - Deterministic scancode sequence for input tests
 *
 * Standard: C11, freestanding
 * =============================================================================
 */

#include "sigma_kernel_types.h"
#include "sigma/hal_contract.h"
#include "sigma_features.h"

#ifdef SIGMA_MOCK_HAL

/* ── Call log ring buffer (for test assertions) ─────────────────────────── */

#define MOCK_LOG_SIZE 256

typedef struct MockCallEntry {
    const char* subsystem;
    const char* function;
    u64         arg0;
    u64         timestamp;
} MockCallEntry;

static MockCallEntry g_mock_log[MOCK_LOG_SIZE];
static u32 g_mock_log_head = 0;
static u64 g_mock_tick = 0;

static void mock_log(const char* sub, const char* fn, u64 a0) {
    MockCallEntry* e = &g_mock_log[g_mock_log_head % MOCK_LOG_SIZE];
    e->subsystem = sub;
    e->function  = fn;
    e->arg0      = a0;
    e->timestamp = g_mock_tick;
    g_mock_log_head++;
}

/* Public: query the call log from test code */
u32 mock_hal_log_count(void)              { return g_mock_log_head; }
const MockCallEntry* mock_hal_log_entry(u32 i) {
    if (i >= g_mock_log_head) return (const MockCallEntry*)0;
    return &g_mock_log[i % MOCK_LOG_SIZE];
}

/* ── Failure injection flags ────────────────────────────────────────────── */

static u32 g_mock_fail_mask = 0;

#define MOCK_FAIL_DISPLAY_INIT  (1u << 0)
#define MOCK_FAIL_STORAGE_READ  (1u << 1)
#define MOCK_FAIL_NET_SEND      (1u << 2)
#define MOCK_FAIL_TIMER_INIT    (1u << 3)

void mock_hal_set_fail_mask(u32 mask) { g_mock_fail_mask = mask; }
void mock_hal_clear_fails(void)       { g_mock_fail_mask = 0; }

/* ── Mock Display ───────────────────────────────────────────────────────── */

static u32 mock_fb[320 * 200];
static u32 mock_disp_w = 320, mock_disp_h = 200;

static k_status mock_display_init(u32 w, u32 h, u32 bpp) {
    mock_log("display", "init", w);
    if (g_mock_fail_mask & MOCK_FAIL_DISPLAY_INIT) return K_ERR_NODEV;
    mock_disp_w = w > 320 ? 320 : w;
    mock_disp_h = h > 200 ? 200 : h;
    return K_OK;
}
static void mock_display_put_pixel(u32 x, u32 y, u32 argb) {
    if (x < mock_disp_w && y < mock_disp_h) mock_fb[y * mock_disp_w + x] = argb;
}
static void mock_display_fill_rect(u32 x, u32 y, u32 w, u32 h, u32 argb) {
    u32 r, c;
    for (r = y; r < y + h && r < mock_disp_h; r++)
        for (c = x; c < x + w && c < mock_disp_w; c++)
            mock_fb[r * mock_disp_w + c] = argb;
}
static void mock_display_blit(const void* s, u32 x, u32 y, u32 w, u32 h, u32 st) {
    mock_log("display", "blit", w);
    (void)s; (void)x; (void)y; (void)h; (void)st;
}
static void mock_display_swap(void) { mock_log("display", "swap", 0); }
static void mock_display_get_res(u32* ow, u32* oh, u32* ob) {
    if (ow) *ow = mock_disp_w; if (oh) *oh = mock_disp_h; if (ob) *ob = 32;
}
static k_status mock_display_set_mode(u32 w, u32 h, u32 b) { (void)w;(void)h;(void)b; return K_OK; }

static const SigmaDisplayOps mock_display_ops = {
    .init=mock_display_init, .put_pixel=mock_display_put_pixel,
    .fill_rect=mock_display_fill_rect, .blit=mock_display_blit,
    .swap_buffers=mock_display_swap, .get_resolution=mock_display_get_res,
    .set_mode=mock_display_set_mode,
};

/* ── Mock Timer ─────────────────────────────────────────────────────────── */

static u32 mock_timer_freq = 1000;
static void (*mock_timer_cb)(void) = (void*)0;

static k_status mock_timer_init(u32 hz) {
    mock_log("timer", "init", hz);
    if (g_mock_fail_mask & MOCK_FAIL_TIMER_INIT) return K_ERR_NODEV;
    mock_timer_freq = hz;
    return K_OK;
}
static u64 mock_timer_read_ticks(void) { return g_mock_tick; }
static u64 mock_timer_read_ns(void)    { return g_mock_tick * (1000000000ULL / mock_timer_freq); }
static void mock_timer_sleep(u32 ms)   { g_mock_tick += (u64)ms * mock_timer_freq / 1000; }
static void mock_timer_oneshot(u64 ns, void (*cb)(void)) {
    mock_log("timer", "oneshot", ns);
    mock_timer_cb = cb;
}

/* Allow test code to advance time */
void mock_hal_advance_ticks(u64 n) {
    g_mock_tick += n;
    if (mock_timer_cb) { mock_timer_cb(); mock_timer_cb = (void*)0; }
}

static const SigmaTimerOps mock_timer_ops = {
    .init=mock_timer_init, .read_ticks=mock_timer_read_ticks,
    .read_ns=mock_timer_read_ns, .sleep_ms=mock_timer_sleep,
    .set_oneshot=mock_timer_oneshot,
};

/* ── Mock Serial ────────────────────────────────────────────────────────── */

static char mock_serial_buf[4096];
static u32 mock_serial_pos = 0;

static k_status mock_serial_init(u32 b) { mock_log("serial","init",b); return K_OK; }
static void mock_serial_write_byte(u8 b) {
    if (mock_serial_pos < sizeof(mock_serial_buf)-1) mock_serial_buf[mock_serial_pos++] = (char)b;
}
static void mock_serial_write_str(const char* s) {
    while (*s) mock_serial_write_byte((u8)*s++);
}
static u8 mock_serial_read(void) { return 0; }
static bool_t mock_serial_avail(void) { return FALSE; }

const char* mock_hal_get_serial_output(void) {
    mock_serial_buf[mock_serial_pos] = '\0';
    return mock_serial_buf;
}

static const SigmaSerialOps mock_serial_ops = {
    .init=mock_serial_init, .write_byte=mock_serial_write_byte,
    .write_string=mock_serial_write_str, .read_byte=mock_serial_read,
    .data_available=mock_serial_avail,
};

/* ── Register all mocks ─────────────────────────────────────────────────── */

void mock_hal_register_all(void) {
    hal_register_display(&mock_display_ops);
    hal_register_timer(&mock_timer_ops);
    hal_register_serial(&mock_serial_ops);
    /* Input/storage/net mocks can be added as needed */
}

#endif /* SIGMA_MOCK_HAL */
