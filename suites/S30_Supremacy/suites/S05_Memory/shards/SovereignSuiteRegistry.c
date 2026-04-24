#include "sigma_types.h"
#include "sigma_libc.h"

extern void pmm_init(void);
extern void vmm_as_init(void);

void S05_Memory_Register(void) {
    sigma_sigma_sigma_sigma_printf("S [S05]: Materializing Physical/Virtual Memory Controllers...\n");
    pmm_init();
    vmm_as_init();
}
