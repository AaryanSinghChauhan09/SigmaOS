/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN INTERCONNECT (Suite S20)
 * =========================================================================
 * Shard: Sovereign Interconnect Core
 * Parity: Linux PCI Core / macOS IOKit Plane / CXL 3.0
 * Design: Hierarchical Bus Discovery and Resource Allocation.
 * =========================================================================
 */

#ifndef SOVEREIGN_INTERCONNECT_H
#define SOVEREIGN_INTERCONNECT_H

#include "../../../include/SovereignCommon.h"

#define MAX_BUSES         16
#define MAX_DEVICES_PER_BUS 32

typedef enum {
    BUS_TYPE_PCI,
    BUS_TYPE_USB,
    BUS_TYPE_CXL,
    BUS_TYPE_THUNDERBOLT,
    BUS_TYPE_VIRTUAL
} bus_type_t;

typedef struct {
    sigma_u16 vendor_id;
    sigma_u16 device_id;
    sigma_u8  bus;
    sigma_u8  slot;
    sigma_u8  func;
    sigma_u32 bar[6];
    char      name[64];
} interconnect_dev_t;

typedef struct {
    bus_type_t type;
    sigma_u8   id;
    interconnect_dev_t devices[MAX_DEVICES_PER_BUS];
    sigma_u32  dev_count;
} interconnect_bus_t;

/* Public API */
void        sigma_interconnect_init(void);

/* Discovery */
void        sigma_interconnect_probe_all(void);
void        sigma_interconnect_add_device(sigma_u8 bus_id, interconnect_dev_t dev);

/* Resource Management */
sigma_u32   sigma_interconnect_get_bar(sigma_u8 bus, sigma_u8 slot, sigma_u8 bar_idx);

/* Stats */
void        sigma_interconnect_stats(void);

#endif /* SOVEREIGN_INTERCONNECT_H */
