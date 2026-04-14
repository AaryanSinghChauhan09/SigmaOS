// SigmaOS Sovereign USB Subsystem Shard
// Absorbs LibUSB (Linux), WinUSB (Windows), IOKit (macOS) paradigms.
// Modular, hot-plug aware, zero-dependency C11 shard.

#include <sigma_types.h>


#define SIGMA_USB_MAX_DEVICES 128

typedef enum {
    SIGMA_USB_CLASS_HID      = 0x03, // Keyboard, Mouse, Gamepad
    SIGMA_USB_CLASS_AUDIO    = 0x01, // USB DAC, Headsets
    SIGMA_USB_CLASS_STORAGE  = 0x08, // Flash drives, HDDs
    SIGMA_USB_CLASS_VIDEO    = 0x0E, // Webcams
    SIGMA_USB_CLASS_HUB      = 0x09, // USB Hubs
} SigmaUSBClass;

typedef struct {
    uint8_t       device_addr;
    uint16_t      vendor_id;
    uint16_t      product_id;
    SigmaUSBClass device_class;
    bool          is_connected;
} SigmaUSBDevice;

static SigmaUSBDevice usb_device_table[SIGMA_USB_MAX_DEVICES];

// Initialize USB host controller (XHCI 3.x / EHCI 2.0)
void usb_init_host_controller(void);

// Enumerate and probe all connected USB devices at boot
uint8_t usb_enumerate_bus(void);

// Handle hot-plug event — called by the hardware interrupt handler
void usb_hotplug_event(uint8_t device_addr, bool is_connected);

// Transfer raw data to/from a USB endpoint (bulk/interrupt/isoch)
uint32_t usb_transfer(uint8_t device_addr, uint8_t endpoint, void* buffer, uint32_t length);



