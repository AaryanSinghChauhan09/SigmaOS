/*
 * =========================================================================
 * Σ SIGMAOS: LATTICE-BASED MAC IMPLEMENTATION
 * =========================================================================
 */

#include "lattice_mac.h"

void sigma_mac_init(const void* policy_binary, uint32_t size) {
    // Stub: Parse binary policy rule table
    (void)policy_binary;
    (void)size;
}

bool sigma_mac_check_access(const sigma_mac_label_t* subject, const sigma_mac_label_t* object, bool is_write) {
    if (!subject || !object) return false;

    // Check compartments: subject must have all compartments of the object
    if ((subject->compartments & object->compartments) != object->compartments) {
        return false; // Lattice compartments not satisfied
    }

    if (is_write) {
        // No Write Down (Bell-LaPadula / Biba strict)
        // Subject level must be <= Object level to write 
        // (to prevent leaking high-level data to low-level objects)
        return subject->level <= object->level;
    } else {
        // No Read Up
        // Subject level must be >= Object level to read
        return subject->level >= object->level;
    }
}

void sigma_mac_assign_label(uint64_t shard_id, const sigma_mac_label_t* label) {
    // Stub: Register label in kernel shard manager table
    (void)shard_id;
    (void)label;
}
