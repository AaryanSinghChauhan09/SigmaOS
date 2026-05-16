#ifndef SOVEREIGN_UNIVERSAL_DISTRO_SUITE_H
#define SOVEREIGN_UNIVERSAL_DISTRO_SUITE_H

#include "../../../../../include/SigmaC11.h"

/**
 * @file SovereignUniversalDistroSuite.h
 * @brief Phase 58: Universal Distro Amalgamation.
 */

/* Master Suite Functionalities */
void sigma_nixos_pure_init(void);
void sigma_qubes_isolation_init(void);
void sigma_kali_forensics_init(void);
void sigma_alpine_lean_init(void);
void sigma_gentoo_opt_init(void);

/* Lifecycle Management */
void sigma_universal_suite_init(void);
void SovereignUniversalDistroSuite_Register(void);

#endif // SOVEREIGN_UNIVERSAL_DISTRO_SUITE_H
