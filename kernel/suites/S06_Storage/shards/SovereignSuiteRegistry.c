#include "sigma_types.h"
#include "suites/S01_Genesis/shards/sigma_libc.h"
extern void sigma_vfs_init(void);
void S06_Storage_Register(void) {
    sigma_printf("S [S06]: Materializing Storage VFS Layer...\n");
    sigma_vfs_init();
}