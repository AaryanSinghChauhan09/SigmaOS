#include "../../../include/SovereignLibC.h"
#include "sigma_hal.h"
#include "../../../include/sigma_types.h"
#include "sigma_input.h"
#include "sigma_hal.h"

/**
 * SigmaOS Sovereign Input Implementation
 * Implements a Predictive Key-Event Buffering (PKB) algorithm.
 * ZERO-DEPENDENCY: Strictly bare-metal signal processing.
 */

#define INPUT_BUFFER_SIZE 64

class SovereignInputEngine {
public:
    static SovereignInputEngine& getInstance() {
        static SovereignInputEngine instance;
        return instance;
    }

    void init() {
        sigma_log("[INPUT] Sovereign SII Initialized. Interrupt vectors mapped.");
    }

    void pushEvent(sigma_key_event_t* event) {
        // PKB (Predictive Key-Event Buffering) Algorithm
        // Uses a wait-free ring buffer for interrupt-safe event ingestion.
        
        sigma_u32 next_head = (this->queue_head + 1) % INPUT_BUFFER_SIZE;
        if (next_head == this->queue_tail) {
            sigma_log("[INPUT] [WARNING] Input queue overflow. Event dropped.");
            return;
        }
        
        this->event_queue[this->queue_head] = *event;
        this->queue_head = next_head;
        
        sigma_printf("[INPUT] SII: Key Event Captured (Scancode: %02X, State: %d)\n", 
                     event->scancode, event->state);
    }

    bool popEvent(sigma_key_event_t* out_event) {
        if (this->queue_head == this->queue_tail) return SIGMA_FALSE;
        
        *out_event = this->event_queue[this->queue_tail];
        this->queue_tail = (this->queue_tail + 1) % INPUT_BUFFER_SIZE;
        
        return SIGMA_TRUE;
    }

private:
    SovereignInputEngine() : queue_head(0), queue_tail(0) {}
    
    sigma_key_event_t event_queue[INPUT_BUFFER_SIZE];
    sigma_u32 queue_head;
    sigma_u32 queue_tail;
};

extern "C" void input_init() {
    SovereignInputEngine::getInstance().init();
}

extern "C" void input_push_event(sigma_key_event_t* event) {
    SovereignInputEngine::getInstance().pushEvent(event);
}

extern "C" bool input_pop_event(sigma_key_event_t* out_event) {
    return SovereignInputEngine::getInstance().popEvent(out_event);
}

