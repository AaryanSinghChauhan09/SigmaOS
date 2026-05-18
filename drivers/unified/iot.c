#include "driver_api.h"

static sigma_u32 iot_init(void) { sigma_printf("[Unified Driver] IoT init\n"); return 0; }
static sigma_u32 iot_read(void *b, sigma_u32 l) { (void)b; (void)l; return 0; }
static sigma_u32 iot_write(const void *b, sigma_u32 l) { (void)b; (void)l; return 0; }
static sigma_u32 iot_shutdown(void) { return 0; }

static const struct driver_ops iot_ops = { iot_init, iot_read, iot_write, iot_shutdown };

void register_iot_driver(void) { driver_register(DEV_IOT, &iot_ops); }
