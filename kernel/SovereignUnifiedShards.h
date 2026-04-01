/*
 * =============================================================================
 * Σ SIGMAOS: SOVEREIGN UNIFIED SHARD MANIFEST (v3.0)
 * =============================================================================
 * Single source of truth for all kernel sub-shards.
 * Add/remove a shard here = Add/remove it from the entire OS build.
 * All shards: Pure C11. No stdlib. No OOP. No external libs.
 * =============================================================================
 */

#ifndef SOVEREIGN_UNIFIED_SHARDS_H
#define SOVEREIGN_UNIFIED_SHARDS_H

/* Foundation types required by all shards */
#include "sigma_kernel_types.h"

/* ---- DOMAIN: DATA SCIENCE & ALGORITHMS ---- */
#include "shards/SovereignDS.c"
#include "shards/SovereignDSA.c"
#include "shards/SovereignCS.c"

/* ---- DOMAIN: AI, ML & BIO ---- */
#include "shards/SovereignAI.c"
#include "shards/SovereignTransformer.c"
#include "shards/SovereignBioMetrics.c"

/* ---- DOMAIN: SECURITY & CRYPTO ---- */
#include "shards/SovereignLatticePQC.c"
#include "shards/SovereignEBPF.c"
#include "shards/SovereignKali.c"

/* ---- DOMAIN: FINANCE ---- */
#include "shards/SovereignHFT.c"

/* ---- DOMAIN: AUTONOMOUS AGENTS ---- */
#include "shards/SovereignCowork.c"
#include "shards/SovereignComputeOracle.c"
#include "shards/SovereignMacroClaw.c"

/* ---- DOMAIN: OS USP EQUIVALENTS ---- */
#include "shards/SovereignQubes.c"
#include "shards/SovereignTimeMachine.c"
#include "shards/SovereignTailRouter.c"
#include "shards/SovereignPlan9.c"
#include "shards/SovereignHolyC.c"

/* ---- DOMAIN: LINUX KERNEL EQUIVALENTS ---- */
#include "shards/SovereignCgroups.c"
#include "shards/SovereignOOMKiller.c"

#endif /* SOVEREIGN_UNIFIED_SHARDS_H */
