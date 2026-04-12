/*
 * =========================================================================
 * Σ SIGMAOS: SIGMA KERNEL AGGREGATOR (v2.0 — PURE C11)
 * =========================================================================
 * Mission: Unified entry point for all sovereign shard headers.
 * Design: C11 / Zero-Dependency / Industrial Aggregation.
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
#include "SovereignRegistry.h"
#include "SovereignModule.h"

/* ── 3. System Services ────────────────────────────────────────────────── */
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

/* Competitor Absorption Shards (Phase 42-46) */
#include "SovereignJail.h"
#include "SovereignZFS.h"
#include "SovereignMediaCodec.h"
#include "SovereignWineCompat.h"
#include "SovereignDTrace.h"
#include "SovereignBrowserCloud.h"
#include "SovereignVirtualBox.h"
#include "SovereignBandicam.h"
#include "SovereignDefender.h"
#include "SovereignActiveDirectory.h"
#include "SovereignAndroidBinder.h"
#include "SovereignDarwinXNU.h"
#include "SovereignLinuxIoUring.h"
#include "SovereignSerenityGUI.h"
#include "SovereignFreeRTOS.h"
#include "SovereignHaiku.h"
#include "SovereignPersonalizer.h"

// Master Aggregator Initialization
static inline void SovereignMaster_InitAll(void) {
    /* 1. Base Registries & Frameworks */
    SovereignRegistry_Init();
    
    /* 2. Seat Essential Core Modules */
    /* 2. Seat Essential Integration Suites */
    extern void SovereignMemory_Register(void);
    extern void SovereignSecurity_Register(void);
    extern void SovereignAppMgmt_Register(void);
    extern void SovereignService_Register(void);
    extern void SovereignIntelligence_Register(void);
    extern void SovereignFrontend_Register(void);
    extern void SovereignEcosystem_Register(void);
    extern void SovereignBackend_Register(void);
    extern void SovereignConfig_Register(void);
    extern void SovereignCLI_Register(void);
    extern void SovereignPrinciple_Register(void);

    /* --- Activation Sequence --- */
    SovereignMemory_Register();
    SovereignSecurity_Register();
    SovereignCrypto_Register();
    SovereignAppManagement_Register();
    SovereignServiceControl_Register();
    SovereignIntelligence_Register();
    SovereignFrontend_Register();
    SovereignEcosystem_Register();
    SovereignBackend_Register();
    SovereignConfig_Register();
    SovereignCLI_Register();
    SovereignPrinciple_Register();

    sigma_printf("\nΣ [SOVEREIGN-MASTER]: Global Mesh Orchestration COMPLETE.\n");

    /* 3. Run Industrial Init */
    sigma_modules_init_all();

    /* 4. Driver/Init Finalization */
    SovereignDriver_InitAll();
    SovereignInit_StartAll();

    SovereignRegistry_Finalize();
}

#endif /* SIGMA_KERNEL_H */
