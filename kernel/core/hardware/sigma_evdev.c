/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: EVDEV INPUT SUBSYSTEM
 * =============================================================================
 * Inspired by: Linux kernel drivers/input/input.c (evdev)
 *              FreeBSD sys/dev/evdev/evdev.c
 * =============================================================================
 * Centralized routing for HID events (keyboard, mouse, touch) with ring buffers.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

#define EV_SYN  0x00
#define EV_KEY  0x01
#define EV_REL  0x02
#define EV_ABS  0x03

#define SYN_REPORT 0

#define EVDEV_MAX_DEVICES 8
#define EVDEV_BUFFER_SIZE 256

typedef struct {
    sigma_u64 time_sec;
    sigma_u64 time_usec;
    sigma_u16 type;
    sigma_u16 code;
    sigma_u32 value;
} input_event_t;

typedef struct {
    char          name[32];
    input_event_t buffer[EVDEV_BUFFER_SIZE];
    sigma_u32     head;
    sigma_u32     tail;
    sigma_bool    active;
} sigma_evdev_t;

static sigma_evdev_t input_devices[EVDEV_MAX_DEVICES];
static sigma_u64 simulated_time_usec = 0;

void evdev_init(void) {
    sigma_memset(input_devices, 0, sizeof(input_devices));
    sigma_printf("[evdev] Input subsystem initialized\n");
}

int evdev_register_device(const char* name) {
    for (sigma_u32 i = 0; i < EVDEV_MAX_DEVICES; i++) {
        if (!input_devices[i].active) {
            sigma_strcpy(input_devices[i].name, name, 32);
            input_devices[i].head = 0;
            input_devices[i].tail = 0;
            input_devices[i].active = SIGMA_TRUE;
            
            sigma_printf("[evdev] Registered input device: /dev/input/event%d (%s)\n", i, name);
            return (int)i;
        }
    }
    sigma_printf("[evdev] ERR: Max input devices reached\n");
    return -1;
}

void evdev_inject_event(sigma_u32 dev_id, sigma_u16 type, sigma_u16 code, sigma_u32 value) {
    if (dev_id >= EVDEV_MAX_DEVICES || !input_devices[dev_id].active) return;
    sigma_evdev_t* dev = &input_devices[dev_id];
    
    sigma_u32 next_head = (dev->head + 1) % EVDEV_BUFFER_SIZE;
    if (next_head == dev->tail) {
        /* Buffer overflow, drop event */
        sigma_printf("[evdev] ERR: Buffer overflow on event%d\n", dev_id);
        return;
    }
    
    simulated_time_usec += 1000; /* Increment 1ms for simulation */
    
    dev->buffer[dev->head].time_sec  = simulated_time_usec / 1000000;
    dev->buffer[dev->head].time_usec = simulated_time_usec % 1000000;
    dev->buffer[dev->head].type      = type;
    dev->buffer[dev->head].code      = code;
    dev->buffer[dev->head].value     = value;
    
    dev->head = next_head;
    
    /* Simulate immediate dispatch */
    if (type == EV_KEY) {
        sigma_printf("[evdev] %s Key %u %s\n", 
                     dev->name, code, value ? "PRESSED" : "RELEASED");
    } else if (type == EV_REL) {
        sigma_printf("[evdev] %s Rel-Axis %u Move: %d\n", 
                     dev->name, code, (int)value);
    }
}

void evdev_inject_sync(sigma_u32 dev_id) {
    evdev_inject_event(dev_id, EV_SYN, SYN_REPORT, 0);
}

int evdev_read_event(sigma_u32 dev_id, input_event_t* out_event) {
    if (dev_id >= EVDEV_MAX_DEVICES || !input_devices[dev_id].active) return -1;
    sigma_evdev_t* dev = &input_devices[dev_id];
    
    if (dev->head == dev->tail) {
        return 0; /* No events available (would block/EAGAIN in real kernel) */
    }
    
    sigma_memcpy(out_event, &dev->buffer[dev->tail], sizeof(input_event_t));
    dev->tail = (dev->tail + 1) % EVDEV_BUFFER_SIZE;
    
    return 1; /* Success */
}
