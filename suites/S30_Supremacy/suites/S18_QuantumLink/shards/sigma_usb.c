#include "../../../../../include/libc/SovereignLibC.h"
/*
 * =========================================================================
 * S SIGMAOS kernel/suites/S18_QuantumLink/shards/sigma_usb.c
 * =========================================================================
 */

#include "../../../../../include/drivers/sigma_usb.h"
#include "../../../../../include/libc/sigma_libc.h"

static sigma_usb_dev_t s_devices[USB_MAX_DEVICES];
static usb_u32         s_dev_count  = 0;
static usb_u32         s_next_id    = 1;
static usb_u32         s_next_addr  = 1;   /* USB addresses 1-127     */

static usb_u32         s_gadget_class = 0;
static usb_bool        s_gadget_active = USB_FALSE;

static const char *speed_str[] = {"LS/1.5M","FS/12M","HS/480M",
                                   "SS/5G","SS+/10G","USB4/40G"};
static const char *class_str[] = {
    "Audio","CDC","HID","?","?","?","?","Printer","MSC","Hub"
};

static sigma_usb_dev_t *find_dev(usb_u32 id) {
    for (usb_u32 i = 0; i < s_dev_count; i++)
        if (s_devices[i].dev_id == id) return &s_devices[i];
    return (sigma_usb_dev_t*)0;
}

/* -- Init ------------------------------------------------------------------ */
void sigma_usb_init(void) {
    sigma_sigma_memset(s_devices, 0, sizeof(s_devices));
    sigma_sigma_printf("S [USB] Sovereign USB subsystem initialized\n");
    sigma_sigma_printf("S [USB] xHCI host | USB4/TB4 | Gadget mode | ADB parity\n");
}

/* -- Hot-plug enumeration -------------------------------------------------- */
usb_i32 sigma_usb_enumerate(usb_u32 port) {
    if (s_dev_count >= USB_MAX_DEVICES) return USB_ERR;
    if (s_next_addr > 127) return USB_ERR;

    sigma_usb_dev_t *d = &s_devices[s_dev_count++];
    sigma_sigma_memset(d, 0, sizeof(*d));
    d->dev_id     = s_next_id++;
    d->address    = (usb_u8)s_next_addr++;
    d->speed      = USB_SPEED_SS;   /* default SuperSpeed 5Gbps        */
    d->configured = USB_TRUE;
    d->suspended  = USB_FALSE;
    d->parent_hub = 0;

    /* Simulate different device classes per port */
    switch (port % 4) {
        case 0: d->dev_class=USB_CLASS_MSC;   d->vendor_id=0x090C;
                d->product_id=0x1000;
                sigma_strncpy(d->name,"USB Flash Drive",USB_NAME_LEN-1); break;
        case 1: d->dev_class=USB_CLASS_HID;   d->vendor_id=0x046D;
                d->product_id=0xC52B;
                sigma_strncpy(d->name,"Logitech Keyboard",USB_NAME_LEN-1); break;
        case 2: d->dev_class=USB_CLASS_AUDIO; d->vendor_id=0x041E;
                d->product_id=0x30D3;
                sigma_strncpy(d->name,"USB DAC",USB_NAME_LEN-1); break;
        default:d->dev_class=USB_CLASS_VIDEO; d->vendor_id=0x046D;
                d->product_id=0x0825;
                sigma_strncpy(d->name,"USB Webcam",USB_NAME_LEN-1); break;
    }

    /* Default endpoints */
    d->eps[0] = (sigma_usb_ep_t){0x00, USB_XFER_CONTROL, 64, 0};
    d->eps[1] = (sigma_usb_ep_t){0x81, USB_XFER_BULK,    512, 0};
    d->eps[2] = (sigma_usb_ep_t){0x01, USB_XFER_BULK,    512, 0};
    d->ep_count = 3;

    sigma_sigma_printf("S [USB] ENUMERATE port=%u: %s [%04x:%04x] addr=%u %s\n",
                 port, d->name, d->vendor_id, d->product_id,
                 d->address, speed_str[d->speed]);
    return (usb_i32)d->dev_id;
}

void sigma_usb_disconnect(usb_u32 dev_id) {
    for (usb_u32 i = 0; i < s_dev_count; i++) {
        if (s_devices[i].dev_id == dev_id) {
            sigma_sigma_printf("S [USB] DISCONNECT: %s (id=%u addr=%u)\n",
                         s_devices[i].name, dev_id, s_devices[i].address);
            for (usb_u32 j = i; j < s_dev_count-1; j++)
                s_devices[j] = s_devices[j+1];
            s_dev_count--;
            return;
        }
    }
}

void sigma_usb_device_list(void) {
    sigma_sigma_printf("\nS USB DEVICES (%u)\n", s_dev_count);
    sigma_sigma_printf("%-3s %-5s %-9s %-20s %-12s %s\n",
                 "ID","ADDR","VID:PID","NAME","CLASS","SPEED");
    for (usb_u32 i = 0; i < s_dev_count; i++) {
        sigma_usb_dev_t *d = &s_devices[i];
        usb_u32 cls_idx = (d->dev_class <= 0x09) ? d->dev_class : 9;
        sigma_sigma_printf("  %-3u %-5u %04x:%04x %-20s %-12s %s%s\n",
                     d->dev_id, d->address, d->vendor_id, d->product_id,
                     d->name, class_str[cls_idx], speed_str[d->speed],
                     d->suspended ? " [suspended]" : "");
    }
}

/* -- Transfer functions ---------------------------------------------------- */
usb_i32 sigma_usb_control_msg(usb_u32 dev_id, usb_u8 request_type,
                               usb_u8 request, usb_u16 value, usb_u16 index,
                               void *buf, usb_u16 len) {
    sigma_usb_dev_t *d = find_dev(dev_id);
    if (!d || d->suspended) return USB_ERR;
    sigma_sigma_printf("S [USB] CTRL: dev=%u type=0x%02x req=0x%02x val=%u len=%u\n",
                 dev_id, request_type, request, value, len);
    (void)buf; (void)index;
    return (usb_i32)len;
}

usb_i32 sigma_usb_bulk_transfer(usb_u32 dev_id, usb_u8 ep,
                                 void *buf, usb_u32 len) {
    sigma_usb_dev_t *d = find_dev(dev_id);
    if (!d || d->suspended) return USB_ERR;
    sigma_sigma_printf("S [USB] BULK: dev=%u ep=0x%02x len=%u\n", dev_id, ep, len);
    (void)buf;
    return (usb_i32)len;
}

usb_i32 sigma_usb_interrupt_transfer(usb_u32 dev_id, usb_u8 ep,
                                      void *buf, usb_u32 len) {
    sigma_usb_dev_t *d = find_dev(dev_id);
    if (!d) return USB_ERR;
    sigma_sigma_printf("S [USB] INT: dev=%u ep=0x%02x len=%u\n", dev_id, ep, len);
    (void)buf;
    return (usb_i32)len;
}

/* -- Suspend / resume ------------------------------------------------------ */
usb_i32 sigma_usb_suspend(usb_u32 dev_id) {
    sigma_usb_dev_t *d = find_dev(dev_id);
    if (!d) return USB_ERR;
    d->suspended = USB_TRUE;
    sigma_sigma_printf("S [USB] SUSPEND: %s\n", d->name);
    return USB_OK;
}

usb_i32 sigma_usb_resume(usb_u32 dev_id) {
    sigma_usb_dev_t *d = find_dev(dev_id);
    if (!d) return USB_ERR;
    d->suspended = USB_FALSE;
    sigma_sigma_printf("S [USB] RESUME: %s\n", d->name);
    return USB_OK;
}

/* -- Gadget mode (ADB endpoint-zero) --------------------------------------- */
usb_i32 sigma_usb_gadget_init(sigma_usb_class_t gadget_class) {
    s_gadget_class  = (usb_u32)gadget_class;
    s_gadget_active = USB_TRUE;
    sigma_sigma_printf("S [USB] Gadget mode: class=0x%02x (ADB/CDC parity)\n",
                 gadget_class);
    return USB_OK;
}

usb_i32 sigma_usb_gadget_send(const void *buf, usb_u32 len) {
    if (!s_gadget_active) return USB_ERR;
    sigma_sigma_printf("S [USB] Gadget TX: %u bytes\n", len);
    (void)buf;
    return (usb_i32)len;
}

void sigma_usb_stats(void) {
    sigma_sigma_printf("\nS USB SUBSYSTEM\n");
    sigma_sigma_printf("  Devices: %u   Gadget: %s\n",
                 s_dev_count, s_gadget_active ? "active" : "off");
    sigma_usb_device_list();
}
