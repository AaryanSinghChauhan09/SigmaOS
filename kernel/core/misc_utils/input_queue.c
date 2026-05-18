#include "sigma_kernel_types.h"
/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: LOCK-FREE INPUT QUEUE (v1.0)
 * =============================================================================
 * Principles: Zero-Latency Interrupt Handling & Atomic Synchronization.
 * =============================================================================
 */
#include "sigma_kernel_types.h"

#define KBD_QUEUE_SIZE 256

static char kbd_queue[KBD_QUEUE_SIZE];
static sigma_u32  kbd_head = 0;
static sigma_u32  kbd_tail = 0;

/* Atomic push (Interrupt Context) */
void kbd_queue_push(char c) {
    sigma_u32 next = (kbd_head + 1) % KBD_QUEUE_SIZE;
    if (next != kbd_tail) {
        kbd_queue[kbd_head] = c;
        kbd_head = next;
    }
}

/* Atomic pop (Kernel/Shell Context) */
char kbd_queue_pop() {
    if (kbd_head == kbd_tail) return 0;
    char c = kbd_queue[kbd_tail];
    kbd_tail = (kbd_tail + 1) % KBD_QUEUE_SIZE;
    return c;
}
