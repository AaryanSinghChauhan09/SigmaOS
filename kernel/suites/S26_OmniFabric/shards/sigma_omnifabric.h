/*
 * =========================================================================
 * S SIGMAOS: OMNIFABRIC (Suite S26)
 * =========================================================================
 * Shard: OmniFabric Core (Universal Data Plane)
 * Parity: NVIDIA NVLink / AMD Infinity Fabric / Ultra Ethernet
 * Design: Low-latency, RDMA-ready, unified memory fabric.
 * =========================================================================
 */

#ifndef SOVEREIGN_OMNIFABRIC_H
#define SOVEREIGN_OMNIFABRIC_H

#include "include/SovereignCommon.h"

typedef struct {
    sigma_u32 fabric_id;
    sigma_u64 bandwidth_gbps;
    sigma_u32 latency_ns;
    sigma_bool rdma_enabled;
} omnifabric_node_t;

/* Public API */
void        sigma_omnifabric_init(void);

/* Fabric orchestration */
sigma_err_t sigma_fabric_link(sigma_u32 node_a, sigma_u32 node_b);
sigma_err_t sigma_fabric_broadcast(const void* data, sigma_sz_t size);

/* RDMA / Unified Memory */
void*       sigma_fabric_map_remote(sigma_u32 node_id, sigma_u64 remote_addr, sigma_sz_t size);

#endif /* SOVEREIGN_OMNIFABRIC_H */
