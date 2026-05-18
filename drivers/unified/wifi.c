#include "driver_api.h"

static sigma_u32 wifi_init(void) { sigma_printf("[Unified Driver] Wi-Fi init\n"); return 0; }
static sigma_u32 wifi_read(void *b, sigma_u32 l) { (void)b; (void)l; return 0; }
static sigma_u32 wifi_write(const void *b, sigma_u32 l) { (void)b; (void)l; return 0; }
static sigma_u32 wifi_shutdown(void) { sigma_printf("[Unified Driver] Wi-Fi shutdown\n"); return 0; }

static const struct driver_ops wifi_ops = {
    wifi_init,
    wifi_read,
    wifi_write,
    wifi_shutdown
};

void register_wifi_driver(void) {
    driver_register(DEV_WIFI, &wifi_ops);
}
