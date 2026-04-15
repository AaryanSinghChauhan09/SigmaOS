/* S SIGMAOS: S04_HAL Registry */
#include "sigma_base.h"
#include "SovereignLatticeRegistry.h"

extern void hal_init(void);
extern void acpi_init(void);
extern void rom_boot_init(void);

void S04_HAL_Register(void) {
    hal_init();
    acpi_init();
    rom_boot_init();
    SovereignRegistry_Register("S04_HAL", 0, NULL);
    sigma_printf("S [S04_HAL]: ROM-able Kernel & ACPI integrated.\n");
}
