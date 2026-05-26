/*
 * =============================================================================
 * Σ SIGMAOS: SECURE GOVERNMENT PROFILE
 * =============================================================================
 * Flagship Niche: High-Security Government Desktops.
 * Mandates PQC, strict forensics, and capability-based access control.
 * =============================================================================
 */

#ifndef SIGMA_GOV_PROFILE_H
#define SIGMA_GOV_PROFILE_H

/* Enforce strict PQC signatures on all executables */
#define SIGMA_ENFORCE_PQC_EXEC 1

/* Enable Forensic Mode automatically on Kernel Panic */
#define SIGMA_PANIC_FORENSICS_MODE 1

/* Disable USB Hotplugging to prevent BadUSB attacks */
#define SIGMA_DISABLE_USB_HOTPLUG 1

/* Strict Memory Isolation (disables SHM) */
#define SIGMA_STRICT_MEMORY_ISOLATION 1

#endif /* SIGMA_GOV_PROFILE_H */
