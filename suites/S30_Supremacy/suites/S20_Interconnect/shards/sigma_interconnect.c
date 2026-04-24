/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN INTERCONNECT (Suite S20)
 * =========================================================================
 */

#include "sigma_interconnect.h"
#include "sigma_libc.h"

static interconnect_bus_t s_buses[MAX_BUSES];
static sigma_u32          s_bus_count = 0;

/* -- Initialization ----------------------------------------------------- */
void sigma_interconnect_init(void) {
    sigma_sigma_memset(s_buses, 0, sizeof(s_buses));
    sigma_sigma_printf("S [IC] Sovereign Interconnect Subsystem initialized\n");
    sigma_sigma_printf("S [IC] Fabric Support: PCIe Gen5 | CXL 3.0 | Thunderbolt 4\n");

    /* Register primary buses */
    interconnect_bus_t* pci = &s_buses[s_bus_count++];
    pci->type = BUS_TYPE_PCI;
    pci->id = 0;
    
    interconnect_bus_t* cxl = &s_buses[s_bus_count++];
    cxl->type = BUS_TYPE_CXL;
    cxl->id = 1;

    sigma_interconnect_probe_all();
}

/* -- Discovery ---------------------------------------------------------- */
void sigma_interconnect_probe_all(void) {
    sigma_sigma_printf("S [IC] Probing all interconnect fabrics...\n");

    /* Simulated PCI Enumeration */
    interconnect_dev_t gpu = { 0x10DE, 0x2204, 0, 1, 0, {0xD0000000}, "NVIDIA GeForce RTLS (Sovereign Ed.)" };
    sigma_interconnect_add_device(0, gpu);

    interconnect_dev_t nvme = { 0x144D, 0xA801, 0, 2, 0, {0xE0001000}, "Samsung 990 PRO Sovereign Shard" };
    sigma_interconnect_add_device(0, nvme);

    /* Simulated CXL Enumeration (Sovereign Memory Expansion) */
    interconnect_dev_t cxl_mem = { 0x8086, 0x09A5, 1, 4, 0, {0xF0000000}, "Intel CXL Pooled Memory Fabric" };
    sigma_interconnect_add_device(1, cxl_mem);
}

void sigma_interconnect_add_device(sigma_u8 bus_id, interconnect_dev_t dev) {
    if (bus_id >= s_bus_count) return;
    interconnect_bus_t* b = &s_buses[bus_id];
    if (b->dev_count >= MAX_DEVICES_PER_BUS) return;

    b->devices[b->dev_count++] = dev;
    sigma_sigma_printf("S [IC] Attached %s to Bus %d at %02x:%02x.%d\n", 
                 dev.name, bus_id, dev.bus, dev.slot, dev.func);
}

/* -- Resource Management ------------------------------------------------ */
sigma_u32 sigma_interconnect_get_bar(sigma_u8 bus, sigma_u8 slot, sigma_u8 bar_idx) {
    if (bus >= s_bus_count || bar_idx >= 6) return 0;
    interconnect_bus_t* b = &s_buses[bus];
    for (sigma_u32 i = 0; i < b->dev_count; i++) {
        if (b->devices[i].slot == slot) return b->devices[i].bar[bar_idx];
    }
    return 0;
}

/* -- Statistics ---------------------------------------------------------- */
void sigma_interconnect_stats(void) {
    sigma_sigma_printf("\nS INTERCONNECT LATTICE\n");
    sigma_sigma_printf("%-8s %-4s %-20s %-12s\n", "TYPE", "BUS", "DEVICE", "LOCATION");
    for (sigma_u32 i = 0; i < s_bus_count; i++) {
        interconnect_bus_t* b = &s_buses[i];
        const char* type_str = "PCI";
        if (b->type == BUS_TYPE_CXL) type_str = "CXL";
        
        for (sigma_u32 j = 0; j < b->dev_count; j++) {
            interconnect_dev_t* d = &b->devices[j];
            sigma_sigma_printf("%-8s %-4u %-20s %02x:%02x.%d\n", 
                         type_str, b->id, d->name, d->bus, d->slot, d->func);
        }
    }
}
