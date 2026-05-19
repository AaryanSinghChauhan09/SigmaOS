/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: HIGH-RESOLUTION TIMERS (TIMER WHEEL)
 * =============================================================================
 * Inspired by: Linux kernel kernel/time/timer.c
 *              FreeBSD sys/kern/kern_timeout.c
 * =============================================================================
 * Implements a hashed timer wheel for deferred execution and scheduling delays.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

#define TIMER_WHEEL_SLOTS  256
#define TIMER_MAX_TIMERS   128

typedef void (*timer_callback_t)(void* arg);

typedef struct {
    sigma_u32          expires;   /* Absolute tick count */
    timer_callback_t   callback;
    void*              arg;
    sigma_bool         active;
} sigma_timer_t;

typedef struct {
    sigma_timer_t timers[TIMER_MAX_TIMERS];
    sigma_u64     current_tick;
    sigma_u32     active_count;
} sigma_timer_wheel_t;

static sigma_timer_wheel_t wheel;

void timer_wheel_init(void) {
    sigma_memset(&wheel, 0, sizeof(wheel));
    wheel.current_tick = 0;
    sigma_printf("[timer] High-Resolution Timer Wheel initialized (slots=%d)\n", TIMER_WHEEL_SLOTS);
}

int timer_add(sigma_u32 delay_ticks, timer_callback_t cb, void* arg) {
    for (sigma_u32 i = 0; i < TIMER_MAX_TIMERS; i++) {
        if (!wheel.timers[i].active) {
            wheel.timers[i].expires  = (sigma_u32)wheel.current_tick + delay_ticks;
            wheel.timers[i].callback = cb;
            wheel.timers[i].arg      = arg;
            wheel.timers[i].active   = SIGMA_TRUE;
            wheel.active_count++;
            sigma_printf("[timer] Added timer id %u: fires at tick %u (delay %u)\n",
                         i, wheel.timers[i].expires, delay_ticks);
            return (int)i;
        }
    }
    sigma_printf("[timer] ERR: Timer wheel full\n");
    return -1;
}

void timer_cancel(int timer_id) {
    if (timer_id >= 0 && timer_id < TIMER_MAX_TIMERS && wheel.timers[timer_id].active) {
        wheel.timers[timer_id].active = SIGMA_FALSE;
        wheel.active_count--;
        sigma_printf("[timer] Canceled timer id %u\n", timer_id);
    }
}

void timer_tick(void) {
    wheel.current_tick++;
    
    for (sigma_u32 i = 0; i < TIMER_MAX_TIMERS; i++) {
        if (wheel.timers[i].active && wheel.timers[i].expires <= wheel.current_tick) {
            sigma_printf("[timer] Tick %llu: Firing timer id %u\n", wheel.current_tick, i);
            /* Execute callback */
            if (wheel.timers[i].callback) {
                wheel.timers[i].callback(wheel.timers[i].arg);
            }
            wheel.timers[i].active = SIGMA_FALSE;
            wheel.active_count--;
        }
    }
}

void timer_status(void) {
    sigma_printf("\n--- Σ TIMER WHEEL STATUS ---\n");
    sigma_printf("| Current Tick : %llu\n", wheel.current_tick);
    sigma_printf("| Active Timers: %u / %u\n", wheel.active_count, TIMER_MAX_TIMERS);
    sigma_printf("----------------------------\n");
}
