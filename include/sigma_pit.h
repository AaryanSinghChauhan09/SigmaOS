#ifndef SIGMA_PIT_H
#define SIGMA_PIT_H

#include <stdint.h>

// Initialize PIT timer with given frequency (Hz)
void sigma_pit_init(uint32_t frequency_hz);

// Get current monotonic tick count
uint64_t sigma_pit_get_ticks(void);

// Sleep for given milliseconds
void sigma_sleep_ms(uint32_t ms);

// The IRQ0 interrupt handler
void sigma_pit_handler(void);

#endif // SIGMA_PIT_H
