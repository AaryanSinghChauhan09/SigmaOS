#ifndef SIGMA_KEYBOARD_H
#define SIGMA_KEYBOARD_H

#include <stdint.h>

#define KBD_BUFFER_SIZE 256

// Initialize the keyboard driver (unmask IRQ1)
void sigma_keyboard_init(void);

// Non-blocking read. Returns 0 if buffer is empty.
char sigma_keyboard_read(void);

// Blocking read. Yields until a character is available.
char sigma_keyboard_getchar(void);

// The IRQ1 interrupt handler
void sigma_keyboard_handler(void);

#endif // SIGMA_KEYBOARD_H
