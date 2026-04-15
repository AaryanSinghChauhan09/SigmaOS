#include "sigma_types.h"
#include "suites/S01_Genesis/shards/sigma_libc.h"

extern void syscall_dispatcher_init(void);

void S01_Genesis_Register(void) {
    sigma_printf("S [S01]: Materializing Genesis Syscall Dispatcher...\n");
    syscall_dispatcher_init();
}