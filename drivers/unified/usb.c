#include "driver_api.h"

static sigma_u32 usb_init(void) { sigma_printf("[Unified Driver] USB init\n"); return 0; }
static sigma_u32 usb_read(void *b, sigma_u32 l) { (void)b; (void)l; return 0; }
static sigma_u32 usb_write(const void *b, sigma_u32 l) { (void)b; (void)l; return 0; }
static sigma_u32 usb_shutdown(void) { return 0; }

static const struct driver_ops usb_ops = { usb_init, usb_read, usb_write, usb_shutdown };

void register_usb_driver(void) { driver_register(DEV_USB, &usb_ops); }
