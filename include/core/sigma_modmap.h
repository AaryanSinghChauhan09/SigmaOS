/*
 * =========================================================================
 * Σ SIGMAOS: MODULARIZATION MAP
 * =========================================================================
 * This header provides canonical include-path aliases so that shards at any
 * layer can reference inter-layer headers without fragile relative paths.
 *
 * Rule: Always include this file first via `core/sigma_modmap.h`.
 * =========================================================================
 */
#ifndef SIGMA_MODMAP_H
#define SIGMA_MODMAP_H

/* ── L0: Silicon / Boot ────────────────────────────────────────────────── */
#define SIGMA_INC_TYPES      "core/sigma_types.h"
#define SIGMA_INC_HAL        "hal/sigma_hal.h"
#define SIGMA_INC_BOOT       "core/sigma_boot_types.h"

/* ── L1: Kernel Primitives ─────────────────────────────────────────────── */
#define SIGMA_INC_MEM        "sigma_mem.h"
#define SIGMA_INC_IPC        "ipc/sigma_ipc.h"
#define SIGMA_INC_SCHED      "sched/sigma_sched.h"
#define SIGMA_INC_LOG        "sigma_log.h"

/* ── L2: System Services ────────────────────────────────────────────────── */
#define SIGMA_INC_VFS        "vfs.h"
#define SIGMA_INC_NET        "sigma_net.h"
#define SIGMA_INC_MONITOR    "observability/sigma_monitor.h"

/* ── L3: Security ───────────────────────────────────────────────────────── */
#define SIGMA_INC_PQC        "security/sigma_pqc.h"
#define SIGMA_INC_SANDBOX    "security/sigma_sandbox.h"
#define SIGMA_INC_APPARMOR   "security/sigma_apparmor.h"
#define SIGMA_INC_ATTEST     "security/sigma_attestation.h"
#define SIGMA_INC_QKD        "security/SovereignQKD.hpp"

/* ── L4: AI / Automation ────────────────────────────────────────────────── */
#define SIGMA_INC_NEURAL     "ai/sigma_neural.h"
#define SIGMA_INC_WORKFLOW   "ai/sigma_workflow.h"

/* ── L5: Industrial / Ecosystem ─────────────────────────────────────────── */
#define SIGMA_INC_PKG        "sigma_pkg.h"
#define SIGMA_INC_UNIFIEDPKG "sigma_unifiedpkg.h"

/* ── L6: UI / Zenith ────────────────────────────────────────────────────── */
#define SIGMA_INC_DISPLAY    "sigma_displayserver.h"
#define SIGMA_INC_UX         "sigma_ux.h"

#endif /* SIGMA_MODMAP_H */
