/*
 * =============================================================================
 * Σ SIGMAOS: HAL CONTRACT HEADER (Micro-Interface Layer v1.0)
 * =============================================================================
 * Purpose:
 *   This is a "Contract Header" — it defines the expected behavior of every
 *   hardware abstraction module WITHOUT providing any implementation.
 *
 *   The kernel core MUST interact with hardware ONLY through these interfaces.
 *   Concrete implementations live in suites/S04_HAL*/drivers/ and are selected
 *   at compile-time via feature flags (see sigma_features.h).
 *
 * Design:
 *   - Each subsystem exposes a struct of function pointers (vtable pattern).
 *   - At boot, the appropriate driver registers itself by filling the vtable.
 *   - The kernel calls hal_*() wrappers which dispatch through the vtable.
 *   - Swapping a driver (e.g., VGA → Framebuffer) requires ZERO core changes.
 *
 * Standard: C11, freestanding
 * =============================================================================
 */

#ifndef SIGMA_HAL_CONTRACT_H
#define SIGMA_HAL_CONTRACT_H

#include "../../../../include/sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* =========================================================================
 * §1  DISPLAY SUBSYSTEM CONTRACT
 * ========================================================================= */

typedef struct SigmaDisplayOps {
    /* Initialize the display device. Returns K_OK on success. */
    k_status (*init)(u32 width, u32 height, u32 bpp);

    /* Write a single pixel at (x, y) with the given ARGB color. */
    void     (*put_pixel)(u32 x, u32 y, u32 argb);

    /* Fill a rectangle with a solid color. */
    void     (*fill_rect)(u32 x, u32 y, u32 w, u32 h, u32 argb);

    /* Copy a framebuffer region (blit). */
    void     (*blit)(const void* src, u32 x, u32 y, u32 w, u32 h, u32 stride);

    /* Swap the back-buffer to the screen (vsync-aware). */
    void     (*swap_buffers)(void);

    /* Query current resolution. */
    void     (*get_resolution)(u32* out_w, u32* out_h, u32* out_bpp);

    /* Set display mode / resolution. */
    k_status (*set_mode)(u32 width, u32 height, u32 bpp);
} SigmaDisplayOps;

/* =========================================================================
 * §2  KEYBOARD / INPUT SUBSYSTEM CONTRACT
 * ========================================================================= */

typedef struct SigmaInputOps {
    /* Initialize the input subsystem (PS/2, USB-HID, etc.). */
    k_status (*init)(void);

    /* Poll for a scancode. Returns 0 if no key is pending. */
    u16      (*poll_scancode)(void);

    /* Get a decoded ASCII character (blocking). */
    char     (*get_char)(void);

    /* Check if a key is currently held down. */
    bool_t   (*is_key_down)(u16 scancode);

    /* Register an IRQ callback for asynchronous key events. */
    void     (*register_handler)(void (*callback)(u16 scancode, bool_t pressed));
} SigmaInputOps;

/* =========================================================================
 * §3  STORAGE SUBSYSTEM CONTRACT
 * ========================================================================= */

typedef struct SigmaStorageOps {
    /* Initialize the storage controller (ATA/AHCI/NVMe). */
    k_status (*init)(void);

    /* Read `count` sectors starting at `lba` into `buf`. */
    k_status (*read_sectors)(u64 lba, u32 count, void* buf);

    /* Write `count` sectors starting at `lba` from `buf`. */
    k_status (*write_sectors)(u64 lba, u32 count, const void* buf);

    /* Flush write caches to persistent media. */
    k_status (*flush)(void);

    /* Query total sector count and sector size. */
    void     (*get_geometry)(u64* out_sectors, u32* out_sector_size);
} SigmaStorageOps;

/* =========================================================================
 * §4  NETWORK SUBSYSTEM CONTRACT
 * ========================================================================= */

typedef struct SigmaNetOps {
    /* Initialize the NIC. */
    k_status (*init)(void);

    /* Send a raw ethernet frame. */
    k_status (*send_frame)(const void* buf, u32 len);

    /* Receive a raw ethernet frame (non-blocking). Returns bytes received. */
    u32      (*recv_frame)(void* buf, u32 max_len);

    /* Get the hardware MAC address. */
    void     (*get_mac)(u8 out_mac[6]);

    /* Set promiscuous mode on/off. */
    void     (*set_promiscuous)(bool_t enable);

    /* Register an IRQ callback for incoming frames. */
    void     (*register_rx_handler)(void (*callback)(const void* buf, u32 len));
} SigmaNetOps;

/* =========================================================================
 * §5  TIMER / CLOCK CONTRACT
 * ========================================================================= */

typedef struct SigmaTimerOps {
    /* Initialize the system timer (PIT/HPET/LAPIC). */
    k_status (*init)(u32 frequency_hz);

    /* Read the current tick count since boot. */
    u64      (*read_ticks)(void);

    /* Read wall-clock time in nanoseconds since boot. */
    u64      (*read_ns)(void);

    /* Sleep for the given number of milliseconds (busy-wait or yield). */
    void     (*sleep_ms)(u32 ms);

    /* Register a one-shot timer callback. */
    void     (*set_oneshot)(u64 ns_from_now, void (*callback)(void));
} SigmaTimerOps;

/* =========================================================================
 * §6  SERIAL / DEBUG PORT CONTRACT
 * ========================================================================= */

typedef struct SigmaSerialOps {
    /* Initialize a serial port (COM1, etc.). */
    k_status (*init)(u32 baud_rate);

    /* Write a single byte. */
    void     (*write_byte)(u8 byte);

    /* Write a null-terminated string. */
    void     (*write_string)(const char* str);

    /* Read a single byte (blocking). */
    u8       (*read_byte)(void);

    /* Check if data is available to read. */
    bool_t   (*data_available)(void);
} SigmaSerialOps;

/* =========================================================================
 * §7  GLOBAL HAL REGISTRY
 *
 * At boot, each driver calls hal_register_*() to install its vtable.
 * The kernel accesses hardware ONLY via hal_get_*().
 * ========================================================================= */

void                    hal_register_display(const SigmaDisplayOps* ops);
void                    hal_register_input  (const SigmaInputOps*   ops);
void                    hal_register_storage(const SigmaStorageOps* ops);
void                    hal_register_net    (const SigmaNetOps*     ops);
void                    hal_register_timer  (const SigmaTimerOps*   ops);
void                    hal_register_serial (const SigmaSerialOps*  ops);

const SigmaDisplayOps*  hal_get_display(void);
const SigmaInputOps*    hal_get_input  (void);
const SigmaStorageOps*  hal_get_storage(void);
const SigmaNetOps*      hal_get_net    (void);
const SigmaTimerOps*    hal_get_timer  (void);
const SigmaSerialOps*   hal_get_serial (void);

/* =========================================================================
 * §8  CONVENIENCE MACROS
 *
 * Allow the kernel core to call hal_display()->put_pixel(x,y,c) etc.
 * ========================================================================= */

#define hal_display()   hal_get_display()
#define hal_input()     hal_get_input()
#define hal_storage()   hal_get_storage()
#define hal_net()       hal_get_net()
#define hal_timer()     hal_get_timer()
#define hal_serial()    hal_get_serial()

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_HAL_CONTRACT_H */
