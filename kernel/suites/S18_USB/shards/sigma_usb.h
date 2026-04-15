/*
 * =========================================================================
 * S SIGMAOS kernel/suites/S18_USB/shards/sigma_usb.h
 * =========================================================================
 * Sovereign USB Subsystem — gap-closes:
 *   Linux  : USB core, xHCI/EHCI host controller, usb-storage, usbhid
 *            gadget framework, libusb, USB Audio class (UAC)
 *   Windows: WinUSB, KMDF, USB 4.0 / Thunderbolt 4
 *   macOS  : IOUSBHostFamily, IOUSBLib, USBDriverKit
 *   Android: USB OTG, adb (Android Debug Bridge), USB HAL
 * =========================================================================
 */

#ifndef SIGMA_USB_H
#define SIGMA_USB_H

typedef unsigned int  usb_u32;
typedef unsigned short usb_u16;
typedef unsigned char  usb_u8;
typedef signed   int   usb_i32;
typedef unsigned char  usb_bool;
#define USB_TRUE  ((usb_bool)1)
#define USB_FALSE ((usb_bool)0)
#define USB_OK    ((usb_i32) 0)
#define USB_ERR   ((usb_i32)-1)

/* ── USB speed ───────────────────────────────────────────────────────────── */
typedef enum {
    USB_SPEED_LS  = 0,  /* Low speed  1.5 Mb/s (USB 1.0)              */
    USB_SPEED_FS  = 1,  /* Full speed  12 Mb/s (USB 1.1)              */
    USB_SPEED_HS  = 2,  /* High speed 480 Mb/s (USB 2.0)              */
    USB_SPEED_SS  = 3,  /* SuperSpeed   5 Gb/s (USB 3.0)              */
    USB_SPEED_SS_P= 4,  /* SS+         10 Gb/s (USB 3.1 Gen2)         */
    USB_SPEED_USB4= 5   /* USB4/TB4    40 Gb/s                        */
} sigma_usb_speed_t;

/* ── Device class ────────────────────────────────────────────────────────── */
typedef enum {
    USB_CLASS_HID     = 0x03,  /* Keyboard, mouse, gamepad            */
    USB_CLASS_CDC     = 0x02,  /* Serial, network                     */
    USB_CLASS_MSC     = 0x08,  /* Mass storage (flash drives)         */
    USB_CLASS_HUB     = 0x09,
    USB_CLASS_AUDIO   = 0x01,  /* USB Audio Class (UAC)               */
    USB_CLASS_VIDEO   = 0x0E,  /* USB Video Class (UVC — webcams)     */
    USB_CLASS_PRINTER = 0x07,
    USB_CLASS_VENDOR  = 0xFF   /* vendor-specific (ADB, etc.)         */
} sigma_usb_class_t;

/* ── Transfer types ──────────────────────────────────────────────────────── */
typedef enum {
    USB_XFER_CONTROL     = 0,
    USB_XFER_ISOCHRONOUS = 1,
    USB_XFER_BULK        = 2,
    USB_XFER_INTERRUPT   = 3
} sigma_usb_xfer_t;

#define USB_MAX_DEVICES  64
#define USB_MAX_ENDPOINTS 16
#define USB_NAME_LEN      48

/* ── Endpoint descriptor ─────────────────────────────────────────────────── */
typedef struct {
    usb_u8           addr;      /* EP address (MSB = direction)        */
    sigma_usb_xfer_t xfer_type;
    usb_u16          max_packet;
    usb_u8           interval;  /* polling interval for INT/ISO        */
} sigma_usb_ep_t;

/* ── USB device descriptor ───────────────────────────────────────────────── */
typedef struct {
    usb_u32           dev_id;
    usb_u16           vendor_id;
    usb_u16           product_id;
    char              name[USB_NAME_LEN];
    sigma_usb_speed_t speed;
    sigma_usb_class_t dev_class;
    usb_u8            address;     /* 1-127 assigned by host           */
    usb_bool          configured;
    usb_bool          suspended;
    sigma_usb_ep_t    eps[USB_MAX_ENDPOINTS];
    usb_u32           ep_count;
    usb_u32           parent_hub;  /* 0 = root hub                    */
} sigma_usb_dev_t;

/* ── URB — USB Request Block (Linux urb parity) ──────────────────────────── */
typedef struct {
    usb_u32           dev_id;
    usb_u8            ep_addr;
    sigma_usb_xfer_t  type;
    void             *buf;
    usb_u32           buf_len;
    usb_u32           actual_len;
    usb_i32           status;
    /* callback: void (*complete)(struct urb*); -- simplified         */
} sigma_urb_t;

/* ── Public API ─────────────────────────────────────────────────────────── */
void    sigma_usb_init(void);

/* Host controller */
usb_i32 sigma_usb_enumerate(usb_u32 port);  /* detect & configure device */
void    sigma_usb_disconnect(usb_u32 dev_id);
void    sigma_usb_device_list(void);

/* Transfer */
usb_i32 sigma_usb_control_msg(usb_u32 dev_id, usb_u8 request_type,
                               usb_u8 request, usb_u16 value, usb_u16 index,
                               void *buf, usb_u16 len);
usb_i32 sigma_usb_bulk_transfer(usb_u32 dev_id, usb_u8 ep,
                                 void *buf, usb_u32 len);
usb_i32 sigma_usb_interrupt_transfer(usb_u32 dev_id, usb_u8 ep,
                                      void *buf, usb_u32 len);

/* Suspend / resume (selective suspend = USB runtime PM) */
usb_i32 sigma_usb_suspend(usb_u32 dev_id);
usb_i32 sigma_usb_resume(usb_u32 dev_id);

/* Gadget mode (device-side, Android ADB parity) */
usb_i32 sigma_usb_gadget_init(sigma_usb_class_t gadget_class);
usb_i32 sigma_usb_gadget_send(const void *buf, usb_u32 len);

void    sigma_usb_stats(void);

#endif /* SIGMA_USB_H */
