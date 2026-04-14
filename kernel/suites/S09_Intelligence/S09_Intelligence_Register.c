/* S SIGMAOS: S09_Intelligence Registry */
#include "sigma_base.h"
#include "SovereignRegistry.h"

extern void neural_engine_init(void);

void S09_Intelligence_Register(void) {
    neural_engine_init();
    SovereignRegistry_Register("S09_Intelligence", 0, NULL);
    sigma_printf("S [S09_Intelligence]: Neural Inference Engine integrated.\n");
}
