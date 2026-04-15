/* S SIGMAOS: S03_Orchestrator Registry */
#include "sigma_base.h"
#include "suites/S10_Registry/shards/SovereignLatticeRegistry.h"

extern void dag_init(void);

void S03_Orchestrator_Register(void) {
    dag_init();
    SovereignRegistry_Register("S03_Orchestrator", 0, NULL);
    sigma_printf("S [S03_Orchestrator]: SigmaFlow DAG Engine integrated.\n");
}
