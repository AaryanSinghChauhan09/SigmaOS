#include "suites/S01_Genesis/shards/sigma_types.h"
#include "suites/S01_Genesis/shards/sigma_libc.h"
extern void console_init(void);
extern void paging_init(void);
void S04_HAL_Register(void) {
    sigma_printf("S [S04]: Materializing HAL Console and Paging Matrix...\n");
    console_init();
    paging_init();
}