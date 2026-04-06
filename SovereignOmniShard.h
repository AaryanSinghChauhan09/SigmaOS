/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN OMNI-SHARD (v20.0 - PURE C11 FINALITY)
 * =========================================================================
 * Principle: Zero OOP runtime. Zero vtable overhead. Unified Matrix.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =========================================================================
 */

#ifndef SOVEREIGN_OMNI_SHARD_H
#define SOVEREIGN_OMNI_SHARD_H

#include "libc/SovereignLibC.h"

/* =========================================================================
 * CORE ABSTRACTIONS
 * ========================================================================= */
#define CLASS_DECLARE(name) typedef struct name name##_t; struct name
#define VIRTUAL(ret, name, ...) ret (*name)(__VA_ARGS__)

CLASS_DECLARE(SigmaObject) {
    const char* class_name;
    sigma_u32 object_id;
    VIRTUAL(void, destroy, struct SigmaObject* self);
};

/* =========================================================================
 * DOMAIN: KERNEL & SCHEDULING
 * ========================================================================= */
typedef struct SovereignScheduler {
    const char* type_name;
    sigma_u64   ctx_switches;
    sigma_u64   deadline_misses;
} SovereignScheduler;

void SovereignScheduler_init(SovereignScheduler* s);

/* =========================================================================
 * DOMAIN: OMNI-CLI & DISTRO ABSORPTION CORE
 * ========================================================================= */
typedef struct {
    char legacy_command[64];
    char legacy_distro_origin[64];
    char target_sigma_shard[64];
    sigma_bool was_destructive;
} OmniCLI_AbsorptionRule_t;

static const OmniCLI_AbsorptionRule_t g_omnicli_absorption_table[] = {
    {"apt install", "Debian/Ubuntu", "SovereignAPT_Install", SIGMA_FALSE},
    {"pacman -S",   "Arch Linux",    "SovereignALPM_Sync",   SIGMA_FALSE},
    {"dnf update",  "Fedora/RHEL",   "SovereignDNF_Update",  SIGMA_FALSE},
    {"EOF",         "",              "",                     SIGMA_FALSE}
};

void sigma_omnicli_absorb_command(const char* legacy_input);
void sigma_omnicli_dispatch(const char* distro, const char* command);

/* =========================================================================
 * FREEBSD ABSORPTION (ZENITH SUPREME)
 * ========================================================================= */
void SovereignZFS_Init(void);
void SovereignJail_Init(void);
void SovereignDTrace_Init(void);
void SovereignPF_Init(void);
void SovereignKqueue_Init(void);
void SovereignCapsicum_Init(void);
void SovereignGEOM_Init(void);

/* =========================================================================
 * GLOBAL LINUX DISTRO ABSORPTION (SHARD REGISTRY)
 * ========================================================================= */

// --- Phase 27: Base Parity ---
void SovereignAlpine_Init(void);
void SovereignAVX_Init(void);
void SovereignLisket_Init(void);
void SovereignSlackInit_Init(void);
void SovereignBusybox_Init(void);
void SovereignMusl_Init(void);
void SovereignUbuntuCore_Init(void);
void SovereignBazzite_Init(void);
void SovereignAsahi_Init(void);
void SovereignNixCore_Init(void);
void SovereignAtomicFS_Init(void);

// --- Phase 28: Apex Sovereignty ---
void SovereignIgnition_Init(void);
void SovereignGamescope_Init(void);
void SovereignProcd_Init(void);
void SovereignAshmem_Init(void);
void SovereignSELinux_Init(void);
void SovereignALPM_Init(void);
void SovereignZorin_Init(void);
void SovereignMageia_Init(void);
void SovereignKnoppix_Init(void);
void SovereignBodhi_Init(void);
void SovereignDeepin_Init(void);

// --- Phase 29: Lattice Expansion ---
void SovereignPurple_Init(void);
void SovereignRocky_Init(void);
void SovereignAlma_Init(void);
void SovereignEndeavour_Init(void);
void SovereignGaruda_Init(void);
void SovereignIce_Init(void);
void SovereignLite_Init(void);
void SovereignLXQt_Init(void);
void SovereignPantheon_Init(void);
void SovereignAnonsurf_Init(void);
void SovereignNobara_Init(void);

// --- Phase 30: Apex Resilience ---
void SovereignPure_Init(void);
void SovereignLibre_Init(void);
void SovereignDragora_Init(void);
void SovereignPkgtool_Init(void);
void SovereignAntiX_Init(void);
void SovereignTiny_Init(void);
void SovereignQubes_Init(void);
void SovereignWhonix_Init(void);
void SovereignPCC_Init(void);
void SovereignYaST_Init(void);
void SovereignOBS_Init(void);

// --- Phase 31: Apex Management ---
void SovereignRescue_Init(void);
void SovereignClone_Init(void);
void SovereignProxmox_Init(void);
void SovereignTrueNAS_Init(void);
void SovereignOMV_Init(void);
void SovereignCosmic_Init(void);
void SovereignMXTools_Init(void);
void SovereignPamac_Init(void);
void SovereignClear_Init(void);
void SovereignQiana_Init(void);

#endif /* SOVEREIGN_OMNI_SHARD_H */
