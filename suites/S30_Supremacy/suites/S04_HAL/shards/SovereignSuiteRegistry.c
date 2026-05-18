#include "libc/SovereignLibC.h"
#include "core/sigma_types.h"
#include "libc/sigma_libc.h"
extern void console_init(void);
extern void paging_init(void);
void S04_HAL_Register(void) {
    sigma_sigma_printf("S [S04]: Materializing HAL Console and Paging Matrix...\n");
    console_init();
    paging_init();
}
