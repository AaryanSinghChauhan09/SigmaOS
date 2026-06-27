// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_amnesic.h — Amnesic boot mode (Tails OS-inspired)
 *
 * When SIGMA_AMNESIC_MODE=1:
 *   - All persistent filesystems are mounted read-only
 *   - Writable paths get tmpfs overlays (data lives only in RAM)
 *   - Swap is encrypted with a random key (lost on shutdown)
 *   - On shutdown: RAM is scrubbed with zeros (cold-boot attack defense)
 *   - Opt-in persistence via encrypted "Persistent Volume"
 *
 * The result: a stolen device reveals nothing about previous sessions.
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

/* Compile-time gate: only active when SIGMA_AMNESIC_MODE=1 */
#ifdef SIGMA_AMNESIC_MODE

/* Called early in sigma_init — before any userland starts */
void sigma_amnesic_init(void);

/* Called on shutdown — overwrites all RAM before power-off */
void sigma_amnesic_scrub_ram(void);

/* Mount a tmpfs overlay over a persistent path (Tails overlay pattern) */
int  sigma_mount_tmpfs_overlay(const char* path, sigma_size_t size_bytes);

/* Set up encrypted swap with an ephemeral random key */
int  sigma_cryptfs_setup_swap(const char* swap_dev, const sigma_u8 key[32]);

/* Register a path as persistent (survives reboots if user unlocks persistence) */
int  sigma_amnesic_persist_register(const char* path);

/* Unlock opt-in persistence volume with passphrase */
int  sigma_amnesic_persist_unlock(const char* dev_path, const char* passphrase);

/* Status check */
bool sigma_amnesic_is_active(void);

#else  /* !SIGMA_AMNESIC_MODE — compile everything out */

static inline void sigma_amnesic_init(void)            {}
static inline void sigma_amnesic_scrub_ram(void)       {}
static inline bool sigma_amnesic_is_active(void)       { return false; }

#endif /* SIGMA_AMNESIC_MODE */
