#include "driver_api.h"

static sigma_u32 printer_init(void) { sigma_printf("[Unified Driver] Printer init\n"); return 0; }
static sigma_u32 printer_read(void *b, sigma_u32 l) { (void)b; (void)l; return 0; }
static sigma_u32 printer_write(const void *b, sigma_u32 l) { (void)b; (void)l; return 0; }
static sigma_u32 printer_shutdown(void) { return 0; }

static const struct driver_ops printer_ops = { printer_init, printer_read, printer_write, printer_shutdown };

void register_printer_driver(void) { driver_register(DEV_PRINTER, &printer_ops); }
