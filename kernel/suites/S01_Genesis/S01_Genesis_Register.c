/* S SIGMAOS: S01_Genesis Registry */
#include "sigma_base.h"
#include "SovereignLatticeRegistry.h"

extern void SovereignSyscall_Init(void);

void S01_Genesis_Register(void) {
    SovereignSyscall_Init();
    SovereignRegistry_Register("S01_Genesis", 0, NULL);
    sigma_printf("S [S01_Genesis]: Syscall Dispatcher integrated.\n");
}
