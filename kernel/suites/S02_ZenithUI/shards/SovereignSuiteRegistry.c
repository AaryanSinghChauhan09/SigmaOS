#include "suites/S01_Genesis/shards/sigma_types.h"
#include "suites/S01_Genesis/shards/sigma_libc.h"

extern void display_server_init(void);

void S02_ZenithUI_Register(void) {
    sigma_printf("S [S02]: Materializing ZenithUI Display Server...\n");
    display_server_init();
}