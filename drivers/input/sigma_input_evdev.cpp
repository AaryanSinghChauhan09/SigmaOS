/*
 * Σ SigmaOS — sigma_input_evdev: Sovereign Event Device Layer
 * Zero-Dependency: No Linux evdev or udev.
 * Absorbs: The generalized input event struct pattern from Linux.
 * Implements: Abstracted input handling for keyboards, mice, touchpads.
 */

typedef unsigned int u32;
typedef unsigned long long u64;

extern "C" void sigma_vga_printf(const char* fmt, ...);

#define SIGMA_EV_SYN       0x00
#define SIGMA_EV_KEY       0x01
#define SIGMA_EV_REL       0x02
#define SIGMA_EV_ABS       0x03

struct SigmaInputEvent {
    u64 time_sec;
    u64 time_usec;
    u32 type;
    u32 code;
    int value;
};

#define MAX_EVENTS 1024
static SigmaInputEvent event_queue[MAX_EVENTS];
static u32 queue_head = 0;
static u32 queue_tail = 0;

extern "C" void sigma_evdev_emit(u32 type, u32 code, int value) {
    u32 next_head = (queue_head + 1) % MAX_EVENTS;
    if (next_head == queue_tail) {
        sigma_vga_printf("[EVDEV] Event queue overflow!\n");
        return;
    }
    
    event_queue[queue_head].type = type;
    event_queue[queue_head].code = code;
    event_queue[queue_head].value = value;
    // time would be populated from sovereign timer here
    
    queue_head = next_head;
}

extern "C" int sigma_evdev_read(SigmaInputEvent* out_event) {
    if (queue_head == queue_tail) return 0; // Empty
    
    *out_event = event_queue[queue_tail];
    queue_tail = (queue_tail + 1) % MAX_EVENTS;
    return 1;
}

extern "C" void sigma_evdev_init() {
    sigma_vga_printf("[EVDEV] Sovereign Event Device Layer Initialized.\n");
}
