#include "libc/SovereignLibC.h"
#include "core/sigma_types.h"
#include "libc/sigma_libc.h"
extern void sigma_vfs_init(void);
void S06_Storage_Register(void) {
    sigma_sigma_printf("S [S06]: Materializing Storage VFS Layer...\n");
    sigma_vfs_init();
}
