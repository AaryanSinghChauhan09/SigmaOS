/*
 * =============================================================================
 * Σ SIGMAOS: DECLARATIVE FEATURE FLAG SYSTEM (Kconfig-style)
 * =============================================================================
 * Defines all compile-time feature toggles for the Sovereign Lattice.
 *
 * Usage in any .c/.cpp file:
 *   #include "sigma/sigma_features.h"
 *
 * Build system sets these via -DSIGMA_FEATURE_xxx on the compiler command line
 * (generated from sigma_features.json by the build orchestrator).
 *
 * Default values are provided here as a baseline configuration.
 * =============================================================================
 */

#ifndef SIGMA_FEATURES_H
#define SIGMA_FEATURES_H

/* =========================================================================
 * §1  ARCHITECTURE SELECTION
 * ========================================================================= */

#if !defined(SIGMA_ARCH_X86_64) && !defined(SIGMA_ARCH_AARCH64) && !defined(SIGMA_ARCH_RISCV64)
#define SIGMA_ARCH_X86_64               /* Default: x86_64 */
#endif

/* =========================================================================
 * §2  DRIVER SELECTION (mutually exclusive within each subsystem)
 * ========================================================================= */

/* Display driver: exactly one must be active */
#if !defined(SIGMA_DRIVER_VGA) && !defined(SIGMA_DRIVER_FRAMEBUFFER)
#define SIGMA_DRIVER_VGA                /* Default: legacy VGA */
#endif

/* Storage driver: pick one */
#if !defined(SIGMA_DRIVER_ATA) && !defined(SIGMA_DRIVER_AHCI) && !defined(SIGMA_DRIVER_NVME)
#define SIGMA_DRIVER_ATA                /* Default: legacy ATA */
#endif

/* Network driver: pick one */
#if !defined(SIGMA_DRIVER_E1000) && !defined(SIGMA_DRIVER_VIRTIO_NET)
#define SIGMA_DRIVER_E1000              /* Default: Intel E1000 (QEMU) */
#endif

/* =========================================================================
 * §3  KERNEL SUBSYSTEM FEATURE FLAGS
 * ========================================================================= */

/* Networking stack */
#ifndef SIGMA_FEATURE_NETWORK
#define SIGMA_FEATURE_NETWORK       1
#endif

/* Graphical shell / Zenith compositor */
#ifndef SIGMA_FEATURE_GUI
#define SIGMA_FEATURE_GUI           1
#endif

/* AI/ML neural pipeline (NPU offload) */
#ifndef SIGMA_FEATURE_AI
#define SIGMA_FEATURE_AI            0   /* Heavy — opt-in */
#endif

/* Web3 decentralized state persistence */
#ifndef SIGMA_FEATURE_WEB3
#define SIGMA_FEATURE_WEB3          0   /* Experimental — opt-in */
#endif

/* Virtualization / Hypervisor layer */
#ifndef SIGMA_FEATURE_HYPERVISOR
#define SIGMA_FEATURE_HYPERVISOR    0   /* opt-in */
#endif

/* Zero-Knowledge Proof subsystem */
#ifndef SIGMA_FEATURE_ZKP
#define SIGMA_FEATURE_ZKP           0   /* opt-in */
#endif

/* Observability / Tracing subsystem */
#ifndef SIGMA_FEATURE_OBSERVABILITY
#define SIGMA_FEATURE_OBSERVABILITY 1
#endif

/* Slab allocator per-module memory pools */
#ifndef SIGMA_FEATURE_SLAB_POOLS
#define SIGMA_FEATURE_SLAB_POOLS    1
#endif

/* =========================================================================
 * §4  MEMORY CONFIGURATION
 * ========================================================================= */

#ifndef SIGMA_MAX_SLAB_POOLS
#define SIGMA_MAX_SLAB_POOLS        64      /* Max independent memory pools */
#endif

#ifndef SIGMA_DEFAULT_POOL_PAGES
#define SIGMA_DEFAULT_POOL_PAGES    16      /* 64KB default per pool */
#endif

#ifndef SIGMA_HEAP_LIMIT_MB
#define SIGMA_HEAP_LIMIT_MB         256     /* Kernel heap ceiling */
#endif

/* =========================================================================
 * §5  BUILD METADATA
 * ========================================================================= */

#ifndef SIGMA_VERSION_MAJOR
#define SIGMA_VERSION_MAJOR         12
#endif
#ifndef SIGMA_VERSION_MINOR
#define SIGMA_VERSION_MINOR         5
#endif
#ifndef SIGMA_VERSION_PATCH
#define SIGMA_VERSION_PATCH         0
#endif

#define SIGMA_VERSION_STRING        "12.5.0-SOVEREIGN"
#define SIGMA_BUILD_CHANNEL         "rolling"

/* =========================================================================
 * §6  CONVENIENCE HELPERS
 * ========================================================================= */

#define SIGMA_ENABLED(flag)   (SIGMA_FEATURE_##flag)

#endif /* SIGMA_FEATURES_H */
