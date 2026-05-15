/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN USB SUBSYSTEM (S-USB)
 * =========================================================================
 * Mission: USB 3.x/3.1/4.0 host controller orchestration at bare metal.
 * Competitor parity: Linux xhci-hcd / Windows USBHUB / macOS IOUSBFamily.
 * ZERO-DEPENDENCY: Direct xHCI register manipulation, no kernel bloat.
 * =========================================================================
 */

#ifndef SIGMA_USB_H
#define SIGMA_USB_H

#include "include/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- USB Device Classes (USB-IF) --- */
#define SIGMA_USB_CLASS_HID      0x03u   /* Human Interface Device    */
#define SIGMA_USB_CLASS_MASS     0x08u   /* Mass Storage              */
#define SIGMA_USB_CLASS_HUB      0x09u   /* Hub                       */
#define SIGMA_USB_CLASS_CDC      0x0Au   /* CDC (Serial/Ethernet)     */
#define SIGMA_USB_CLASS_VENDOR   0xFFu   /* Vendor-specific           */

/* --- USB Speed Tiers --- */
#define SIGMA_USB_SPEED_LOW      0u   /* 1.5 Mbps  */
#define SIGMA_USB_SPEED_FULL     1u   /* 12 Mbps   */
#define SIGMA_USB_SPEED_HIGH     2u   /* 480 Mbps  */
#define SIGMA_USB_SPEED_SUPER    3u   /* 5 Gbps    */
#define SIGMA_USB_SPEED_SUPER_P  4u   /* 20 Gbps   */

#define SIGMA_USB_MAX_DEVICES    127u

typedef struct {
    sigma_u8  address;
    sigma_u8  class_code;
    sigma_u8  speed;
    sigma_u16 vendor_id;
    sigma_u16 product_id;
    char      description[48];
} sigma_usb_device_t;

typedef struct {
    sigma_usb_device_t devices[SIGMA_USB_MAX_DEVICES];
    sigma_u32 device_count;
    sigma_u32 controller_mmio_base;
} sigma_usb_state_t;

/* --- USB Primitives --- */
void usb_init(sigma_u32 xhci_mmio_base);
void usb_enumerate_bus(void);
sigma_u32 usb_get_device_count(void);
const sigma_usb_device_t* usb_get_device(sigma_u32 idx);
void usb_transfer(sigma_u8 addr, sigma_u8 endpoint, const void* data, sigma_u32 len);
void usb_hotplug_notify(sigma_u8 addr, sigma_u32 attached);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_USB_H */
