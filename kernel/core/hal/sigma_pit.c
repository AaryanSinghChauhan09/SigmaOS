#include "sigma_pit.h"
#include "sigma_kernel_types.h"
#include "sigma_mlfq.h" // Scheduler

static inline uint8_t inb_pit(uint16_t port) {
    uint8_t ret;
    __asm__ volatile ( "inb %1, %0" : "=a"(ret) : "Nd"(port) );
    return ret;
}

static inline void outb_pit(uint16_t port, uint8_t val) {
    __asm__ volatile ( "outb %0, %1" : : "a"(val), "Nd"(port) );
}

static volatile uint64_t pit_ticks = 0;
static uint32_t pit_freq = 1000; // Default 1000Hz (1ms tick)

void sigma_pit_init(uint32_t frequency_hz) {
    pit_freq = frequency_hz;
    uint32_t divisor = 1193180 / frequency_hz;
    
    // Command register 0x43: Channel 0, access mode lobyte/hibyte, mode 2 (rate generator), 16-bit binary
    outb_pit(0x43, 0x36);
    
    // Channel 0 data port 0x40
    outb_pit(0x40, (uint8_t)(divisor & 0xFF));
    outb_pit(0x40, (uint8_t)((divisor >> 8) & 0xFF));
    
    pit_ticks = 0;
}

void sigma_pit_handler(void) {
    pit_ticks++;
    
    // Notify the scheduler that a tick occurred
    // If the scheduler isn't initialized yet, this should gracefully do nothing
    sigma_sched_tick();
}

uint64_t sigma_pit_get_ticks(void) {
    return pit_ticks;
}

void sigma_sleep_ms(uint32_t ms) {
    uint64_t target = pit_ticks + ms;
    // Basic busy-wait for now. Real OS might yield.
    while (pit_ticks < target) {
        __asm__ volatile ("hlt");
    }
}
