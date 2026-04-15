/* S SIGMAOS: S20_GlobalVFS Registry */
#include "sigma_base.h"
#include "SovereignLatticeRegistry.h"

extern void vfs_init(void);

void S20_GlobalVFS_Register(void) {
    vfs_init();
    SovereignRegistry_Register("S20_GlobalVFS", 0, NULL);
    sigma_printf("S [S20_GlobalVFS]: Unified VFS Node Interface integrated.\n");
}
