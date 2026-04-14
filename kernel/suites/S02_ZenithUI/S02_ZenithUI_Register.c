/* S SIGMAOS: S02_ZenithUI Registry */
#include "sigma_base.h"
#include "SovereignRegistry.h"

extern void display_server_init(void);

void S02_ZenithUI_Register(void) {
    display_server_init();
    SovereignRegistry_Register("S02_ZenithUI", 0, NULL);
    sigma_printf("S [S02_ZenithUI]: SigmaDisplay Server integrated.\n");
}
