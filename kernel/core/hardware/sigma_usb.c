/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: USB CORE SUBSYSTEM (UHCI/EHCI/xHCI Stub)
 * =============================================================================
 * Inspired by: Linux kernel drivers/usb/core/usb.c
 *              FreeBSD sys/dev/usb/usb_core.c
 * =============================================================================
 * Manages USB buses, device states, and endpoint configurations.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

#define USB_MAX_DEVICES 32
#define USB_MAX_ENDPOINTS 8

#define USB_SPEED_UNKNOWN 0
#define USB_SPEED_LOW     1
#define USB_SPEED_FULL    2
#define USB_SPEED_HIGH    3
#define USB_SPEED_SUPER   4

#define USB_STATE_NOTATTACHED 0
#define USB_STATE_ATTACHED    1
#define USB_STATE_POWERED     2
#define USB_STATE_DEFAULT     3
#define USB_STATE_ADDRESS     4
#define USB_STATE_CONFIGURED  5

typedef struct {
    sigma_u8  address;
    sigma_u8  speed;
    sigma_u8  state;
    sigma_u16 vendor_id;
    sigma_u16 product_id;
    sigma_u8  class_code;
    char      manufacturer[32];
    char      product[32];
    sigma_bool active;
} sigma_usb_device_t;

static sigma_usb_device_t usb_devices[USB_MAX_DEVICES];
static sigma_u8 next_usb_addr = 1;

void usb_core_init(void) {
    sigma_memset(usb_devices, 0, sizeof(usb_devices));
    sigma_printf("[usb] USB Core Subsystem initialized\n");
}

/* Simulates a device plugging into a port */
int usb_device_attach(sigma_u8 speed, sigma_u16 vid, sigma_u16 pid, const char* mfg, const char* prod) {
    for (sigma_u32 i = 0; i < USB_MAX_DEVICES; i++) {
        if (!usb_devices[i].active) {
            sigma_usb_device_t* dev = &usb_devices[i];
            dev->address = next_usb_addr++;
            dev->speed   = speed;
            dev->state   = USB_STATE_CONFIGURED;
            dev->vendor_id  = vid;
            dev->product_id = pid;
            dev->active  = SIGMA_TRUE;
            
            sigma_u32 j = 0;
            while (j < 31 && mfg[j]) { dev->manufacturer[j] = mfg[j]; j++; }
            dev->manufacturer[j] = '\0';
            
            j = 0;
            while (j < 31 && prod[j]) { dev->product[j] = prod[j]; j++; }
            dev->product[j] = '\0';
            
            sigma_printf("[usb] Device Attached: Addr %u, %s %s (VID: 0x%04X, PID: 0x%04X)\n",
                         dev->address, dev->manufacturer, dev->product, vid, pid);
            return (int)i;
        }
    }
    sigma_printf("[usb] ERR: USB device table full\n");
    return -1;
}

void usb_device_detach(sigma_u8 address) {
    for (sigma_u32 i = 0; i < USB_MAX_DEVICES; i++) {
        if (usb_devices[i].active && usb_devices[i].address == address) {
            sigma_printf("[usb] Device Detached: Addr %u (%s %s)\n",
                         address, usb_devices[i].manufacturer, usb_devices[i].product);
            usb_devices[i].active = SIGMA_FALSE;
            usb_devices[i].state = USB_STATE_NOTATTACHED;
            return;
        }
    }
}

void usb_dump_devices(void) {
    sigma_printf("\n--- Σ USB DEVICES ---\n");
    for (sigma_u32 i = 0; i < USB_MAX_DEVICES; i++) {
        if (usb_devices[i].active) {
            const char* spd = "UNK";
            switch (usb_devices[i].speed) {
                case USB_SPEED_LOW:   spd = "1.5M"; break;
                case USB_SPEED_FULL:  spd = "12M "; break;
                case USB_SPEED_HIGH:  spd = "480M"; break;
                case USB_SPEED_SUPER: spd = "5G  "; break;
            }
            
            sigma_printf("| Addr: %02u | Speed: %s | ID %04x:%04x | %s %s\n",
                         usb_devices[i].address, spd,
                         usb_devices[i].vendor_id, usb_devices[i].product_id,
                         usb_devices[i].manufacturer, usb_devices[i].product);
        }
    }
    sigma_printf("---------------------\n");
}
