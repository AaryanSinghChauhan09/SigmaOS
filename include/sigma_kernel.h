/* SPDX-License-Identifier: MIT */
/*
 * =========================================================================
 * Σ SIGMAOS: MASTER KERNEL HEADER UMBRELLA (S-KERNEL)
 * =========================================================================
 * Umbrella header exposing all SigmaOS C/C++ kernel subsystem headers
 * for cross-distro Linux (Debian, Arch, Fedora, Gentoo, Alpine, Void)
 * and BSD (FreeBSD, OpenBSD, NetBSD, DragonFly) package builds.
 * =========================================================================
 */

#ifndef SIGMA_KERNEL_H
#define SIGMA_KERNEL_H

#include "./sigma_kernel_types.h"
#include "./sigma_error_codes.h"
#include "./sigma_driver_codes.h"
#include "./sigma_audit.h"
#include "./sigma_profiles.h"
#include "./sigma_pqc.h"
#include "./sigma_abi.h"
#include "./sigma_kmod.h"
#include "./sigma_vfs.h"
#include "./sigma_net.h"
#include "./sigma_sched.h"

#ifdef __cplusplus
extern "C" {
#endif

#define SIGMA_KERNEL_VERSION_CODE 0x010000
#define SIGMA_KERNEL_RELEASE      "1.0.0-sigma-sovereign"

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_KERNEL_H */
