#ifndef SIGMA_LATTICE_MAC_H
#define SIGMA_LATTICE_MAC_H

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

/*
 * =========================================================================
 * Σ SIGMAOS: LATTICE-BASED MAC (Mandatory Access Control)
 * =========================================================================
 * Deterministic policy evaluation without probabilistic caching or races.
 * =========================================================================
 */

// Lattice clearance levels
#define MAC_LEVEL_UNCLASSIFIED 0
#define MAC_LEVEL_CONFIDENTIAL 1
#define MAC_LEVEL_SECRET       2
#define MAC_LEVEL_TOP_SECRET   3

// Compartment bitmasks
#define MAC_COMP_NET    (1 << 0)
#define MAC_COMP_FS     (1 << 1)
#define MAC_COMP_GPU    (1 << 2)
#define MAC_COMP_SYS    (1 << 3)

typedef struct {
    uint8_t level;
    uint32_t compartments;
} sigma_mac_label_t;

/**
 * Initialize the MAC subsystem and load the compiled policy binary.
 */
void sigma_mac_init(const void* policy_binary, uint32_t size);

/**
 * Evaluate read down / write up (Bell-LaPadula) or Biba integrity properties.
 * Returns true if subject can access object.
 */
bool sigma_mac_check_access(const sigma_mac_label_t* subject, const sigma_mac_label_t* object, bool is_write);

/**
 * Assign a MAC label to a newly spawned shard.
 */
void sigma_mac_assign_label(uint64_t shard_id, const sigma_mac_label_t* label);

#ifdef __cplusplus
}
#endif

#endif // SIGMA_LATTICE_MAC_H
