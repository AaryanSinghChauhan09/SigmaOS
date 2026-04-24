#include "sigma_types.h"
#include "sigma_libc.h"

extern void dag_init(void);

void S03_Orchestrator_Register(void) {
    sigma_sigma_printf("S [S03]: Materializing Orchestrator DAG Engine...\n");
    dag_init();
}
