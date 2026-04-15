/* S SIGMAOS: S13_Sentience Registry */
#include "sigma_base.h"
#include "suites/S10_Registry/shards/SovereignLatticeRegistry.h"

extern void forensic_logger_init(void);
extern void self_healer_init(void);

void S13_Sentience_Register(void) {
    forensic_logger_init();
    self_healer_init();
    SovereignRegistry_Register("S13_Sentience", 0, NULL);
    sigma_printf("S [S13_Sentience]: Self-Healing Sentinel integrated.\n");
}
