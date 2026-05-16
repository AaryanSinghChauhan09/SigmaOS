#include "../../../../../include/libc/SovereignLibC.h"
#include "suites/S01_Genesis/shards/sigma_base.h"

#include "../../../../../include/SovereignDriver.h"
#include "../../../../../include/libc/sigma_libc.h"

sigma_err_t sigma_nvme_probe(SigmaDevice_t* dev) {
    sigma_sigma_printf("S [NVME]: Sovereign Ring-Based Controller initialised at MMIO=%p\n", (void*)dev->mmio_base);
    return SIGMA_OK;
}

void SovereignNVMe_Register(void) {
    sigma_driver_register("nvme-sigma", BUS_PCI, 0x1022, 0x43b9, sigma_nvme_probe, SIGMA_NULL);
}



