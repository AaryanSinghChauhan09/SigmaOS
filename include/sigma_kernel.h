/*
 * =========================================================================
 * Σ SIGMAOS: SIGMA KERNEL AGGREGATOR (v2.0 — PURE C11)
 * =========================================================================
 * Mission: Unified entry point for all sovereign shard headers.
 * Design: C11 / Zero-Dependency / Industrial Aggregation.
 *
 * Changelog v2.0:
 *   + SovereignInitSystem    — PID-1 service supervision (OpenRC/runit/s6)
 *   + SovereignEnvManager    — POSIX environment variables (getenv/setenv)
 *   + SovereignUserManager   — Multi-user UID/GID management (passwd/shadow)
 *   + SovereignDmesg         — Kernel ring buffer / printk / dmesg
 *   + SovereignCLI           — Unified sigma-* CLI dispatcher (25 commands)
 *   + SovereignOmniCLI       — Legacy distro command absorption table
 * =========================================================================
 */

#ifndef SIGMA_KERNEL_H
#define SIGMA_KERNEL_H

/* ── 1. Foundation: LibC & Type Shards ─────────────────────────────────── */
#include "sigma_types.h"
#include "SovereignLibC.h"
#include "SigmaOOP.h"
#include "sigma_libc.h"

/* ── 2. Core Architectural Shards ──────────────────────────────────────── */
#include "SovereignOSBasicsZenith.h"
#include "SovereignSyncZenith.h"
#include "SovereignCoreUtils.h"
#include "SovereignHardwareIOZenith.h"

/* ── 3. System Services (new in v2.0) ───────────────────────────────────── */
#include "SovereignDmesg.h"          /* Kernel ring buffer / printk        */
#include "SovereignEnvManager.h"     /* POSIX environment variables        */
#include "SovereignUserManager.h"    /* Multi-user UID/GID / passwd shadow */
#include "SovereignInitSystem.h"     /* PID-1 service supervisor           */
#include "SovereignCLI.h"            /* Unified sigma-* CLI dispatcher     */
#include "SovereignOmniCLI_DistroAbsorber.h" /* Legacy distro absorber    */

/* ── 4. Flagship Feature Shards ─────────────────────────────────────────── */
#include "SovereignOmniShard.h"
#include "SovereignHyprlandZenith.h"
#include "SovereignInterferenceGuard.h"
#include "SovereignExcelZenith.h"
#include "SovereignPowerBIZenith.h"
#include "SovereignTableauZenith.h"
#include "SovereignPythonZenith.h"
#include "SovereignRZenith.h"

/* Competitor Absorption Shards (Phase 42) */
#include "SovereignJail.h"
#include "SovereignZFS.h"
#include "SovereignMediaCodec.h"
#include "SovereignWineCompat.h"
#include "SovereignDTrace.h"
#include "SovereignBrowserCloud.h"
#include "SovereignVirtualBox.h"
#include "SovereignBandicam.h"

// Master Aggregator Initialization
/*
 * =========================================================================
 * Σ SIGMAOS: SIGMA KERNEL AGGREGATOR (v2.0 — PURE C11)
 * =========================================================================
 * Mission: Unified entry point for all sovereign shard headers.
 * Design: C11 / Zero-Dependency / Industrial Aggregation.
 *
 * Changelog v2.0:
 *   + SovereignInitSystem    — PID-1 service supervision (OpenRC/runit/s6)
 *   + SovereignEnvManager    — POSIX environment variables (getenv/setenv)
 *   + SovereignUserManager   — Multi-user UID/GID management (passwd/shadow)
 *   + SovereignDmesg         — Kernel ring buffer / printk / dmesg
 *   + SovereignCLI           — Unified sigma-* CLI dispatcher (25 commands)
 *   + SovereignOmniCLI       — Legacy distro command absorption table
 * =========================================================================
 */

#ifndef SIGMA_KERNEL_H
#define SIGMA_KERNEL_H

/* ── 1. Foundation: LibC & Type Shards ─────────────────────────────────── */
#include "sigma_types.h"
#include "SovereignLibC.h"
#include "SigmaOOP.h"
#include "sigma_libc.h"

/* ── 2. Core Architectural Shards ──────────────────────────────────────── */
#include "SovereignOSBasicsZenith.h"
#include "SovereignSyncZenith.h"
#include "SovereignCoreUtils.h"
#include "SovereignHardwareIOZenith.h"

/* ── 3. System Services (new in v2.0) ───────────────────────────────────── */
#include "SovereignDmesg.h"          /* Kernel ring buffer / printk        */
#include "SovereignEnvManager.h"     /* POSIX environment variables        */
#include "SovereignUserManager.h"    /* Multi-user UID/GID / passwd shadow */
#include "SovereignInitSystem.h"     /* PID-1 service supervisor           */
#include "SovereignCLI.h"            /* Unified sigma-* CLI dispatcher     */
#include "SovereignOmniCLI_DistroAbsorber.h" /* Legacy distro absorber    */

/* ── 4. Flagship Feature Shards ─────────────────────────────────────────── */
#include "SovereignOmniShard.h"
#include "SovereignHyprlandZenith.h"
#include "SovereignInterferenceGuard.h"
#include "SovereignExcelZenith.h"
#include "SovereignPowerBIZenith.h"
#include "SovereignTableauZenith.h"
#include "SovereignPythonZenith.h"
#include "SovereignRZenith.h"

/* Competitor Absorption Shards (Phase 42) */
#include "SovereignJail.h"
#include "SovereignZFS.h"
#include "SovereignMediaCodec.h"
#include "SovereignWineCompat.h"
#include "SovereignDTrace.h"
#include "SovereignBrowserCloud.h"
#include "SovereignVirtualBox.h"
#include "SovereignBandicam.h"

/* Windows Parity Shards (Phase 43) */
#include "SovereignDefender.h"
#include "SovereignActiveDirectory.h"

/* Android/macOS Parity Shards (Phase 44) */
#include "SovereignAndroidBinder.h"
#include "SovereignDarwinXNU.h"

/* Linux/SerenityOS Parity Shards (Phase 45) */
#include "SovereignLinuxIoUring.h"
#include "SovereignSerenityGUI.h"

/* Embedded/RTOS & Haiku Parity Shards (Phase 46) */
#include "SovereignFreeRTOS.h"
#include "SovereignHaiku.h"

// Master Aggregator Initialization
static inline void SovereignMaster_InitAll(void) {
    SovereignOmniCLI_Init();

    /* Phase 42 Shards */
    SovereignJail_Init();
    SovereignZFS_Init();
    SovereignMediaCodec_Init();
    SovereignWineCompat_Init();
    SovereignDTrace_Init();
    SovereignBrowserCloud_Init();
    SovereignVirtualBox_Init();
    SovereignBandicam_Init();

    /* Phase 43 Shards */
    SovereignDefender_Init();
    SovereignActiveDirectory_Init();

    /* Phase 44 Shards */
    SovereignAndroidBinder_Init();
    SovereignDarwinXNU_Init();

    /* Phase 45 Shards */
    SovereignLinuxIoUring_Init();
    SovereignSerenityGUI_Init();

    /* Phase 46 Shards */
    SovereignFreeRTOS_Init();
    SovereignHaiku_Init();
}

#endif /* SIGMA_KERNEL_H */
