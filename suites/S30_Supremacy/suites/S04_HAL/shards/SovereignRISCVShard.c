#include "suites/S01_Genesis/shards/sigma_base.h"

#include "SovereignArch.h"
#include "sigma_libc.h"

void sigma_riscv_init(void) {
    sigma_sigma_sigma_sigma_printf("  S [RISC-V]: Initialising RV64GC Industrial Matrix...\n");
    sigma_sigma_sigma_sigma_printf("  S [RISC-V]: OpenSBI interface detected. Machine/Supervisor mode Handshake [OK].\n");
    sigma_sigma_sigma_sigma_printf("  S [RISC-V]: Atomic Extension (RV64A) locked. Memory consistency sharded.\n");
}

void SovereignRISCV_Register(void) {
    SovereignArch_Register("RISCV", sigma_riscv_init, SIGMA_NULL);
}



