/**
 * =========================================================================
 * Σ SIGMAOS: PCI/PCIe HAL PUBLIC HEADER
 * =========================================================================
 */
#pragma once

#include "../include/sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

#define SIGMA_PCI_MAX_DEVICES   512u
#define SIGMA_MSIX_MAX_VECTORS  32u

typedef struct {
    sigma_u8  bus;
    sigma_u8  slot;
    sigma_u8  func;
    sigma_u16 vendor_id;
    sigma_u16 device_id;
    sigma_u8  class_code;
    sigma_u8  subclass;
} sigma_pci_device_t;

/** Scan all PCI/PCIe buses and populate the internal device table. */
sigma_status sigma_pci_scan_bus(void);

/** Get the scanned device list. */
const sigma_pci_device_t* sigma_pci_get_devices(sigma_u32* count_out);

/**
 * Enable MSI-X for a device.
 * @num_vectors  : how many independent vectors to allocate (≤ HW maximum)
 * @dest_apic_id : which APIC to route all vectors to (NUMA-aware caller picks)
 * @base_vector_out : filled with the base IRQ vector allocated
 */
sigma_status sigma_pci_enable_msix(sigma_u8  bus,
                                    sigma_u8  slot,
                                    sigma_u8  func,
                                    sigma_u32 num_vectors,
                                    sigma_u8  dest_apic_id,
                                    sigma_u8* base_vector_out);

#ifdef __cplusplus
}
#endif
