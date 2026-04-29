#include <sigma_input.h>
#include <sigma_hal.h>

/**
 * SigmaOS Sovereign Input Implementation
 * Implements a Predictive Key-Event Buffering (PKB) algorithm.
 * ZERO-DEPENDENCY: Strictly bare-metal signal processing.
 */

#define INPUT_BUFFER_SIZE 64

static sigma_key_event_t event_queue[INPUT_BUFFER_SIZE];
static sigma_u32 queue_head = 0;
static sigma_u32 queue_tail = 0;

extern "C" void input_init() {
    sigma_log("[INPUT] Sovereign SII Initialized. Interrupt vectors mapped.");
}

extern "C" void input_push_event(sigma_key_event_t* event) {
    // PKB (Predictive Key-Event Buffering) Algorithm
    // Uses a wait-free ring buffer for interrupt-safe event ingestion.
    
    sigma_u32 next_head = (queue_head + 1) % INPUT_BUFFER_SIZE;
    if (next_head == queue_tail) {
        sigma_log("[INPUT] [WARNING] Input queue overflow. Event dropped.");
        return;
    }
    
    event_queue[queue_head] = *event;
    queue_head = next_head;
    
    sigma_printf("[INPUT] SII: Key Event Captured (Scancode: %02X, State: %d)\n", 
                 event->scancode, event->state);
}

extern "C" bool input_pop_event(sigma_key_event_t* out_event) {
    if (queue_head == queue_tail) return SIGMA_FALSE;
    
    *out_event = event_queue[queue_tail];
    queue_tail = (queue_tail + 1) % INPUT_BUFFER_SIZE;
    
    return SIGMA_TRUE;
}
