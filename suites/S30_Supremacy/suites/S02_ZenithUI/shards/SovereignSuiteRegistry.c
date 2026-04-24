#include "sigma_types.h"
#include "sigma_libc.h"

extern void display_server_init(void);

void S02_ZenithUI_Register(void) {
    sigma_sigma_sigma_sigma_printf("S [S02]: Materializing ZenithUI Display Server...\n");
    display_server_init();
}
