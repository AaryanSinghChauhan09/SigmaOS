/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

/*
 * SigmaOS Extended Kernel Command Library - Advanced Linux Commands
 * =================================================================
 * Advanced commands from all Linux distributions including:
 * - System administration and debugging
 * - Advanced networking and security
 * - Storage and filesystem management
 * - Process and resource management
 * - Hardware and device management
 * - Container and virtualization commands
 * - Package building and development
 */

#ifndef SIGMA_EXTENDED_COMMANDS_H
#define SIGMA_EXTENDED_COMMANDS_H

#include "../include/sigma_command_library.h"

// Extended command categories
typedef enum {
    SIGMA_CMD_DEBUG = SIGMA_CMD_CATEGORY_COUNT,
    SIGMA_CMD_CONTAINER,
    SIGMA_CMD_VIRTUALIZATION,
    SIGMA_CMD_STORAGE_ADVANCED,
    SIGMA_CMD_NETWORK_ADVANCED,
    SIGMA_CMD_HARDWARE,
    SIGMA_CMD_KERNEL,
    SIGMA_CMD_BOOT,
    SIGMA_CMD_RECOVERY,
    SIGMA_CMD_MONITORING,
    SIGMA_CMD_LOGGING,
    SIGMA_CMD_BACKUP_ADVANCED,
    SIGMA_CMD_DEVEL,
    SIGMA_CMD_EXTENDED_COUNT
} SigmaExtendedCategory;

// Initialize extended commands
void sigma_load_extended_commands(void);

// Advanced distro-specific commands
void sigma_load_ubuntu_advanced(void);
void sigma_load_fedora_advanced(void);
void sigma_load_arch_advanced(void);
void sigma_load_debian_advanced(void);
void sigma_load_gentoo_advanced(void);
void sigma_load_opensuse_advanced(void);
void sigma_load_centos_advanced(void);
void sigma_load_alpine_advanced(void);
void sigma_load_void_advanced(void);
void sigma_load_nixos_advanced(void);

// Advanced SigmaOS custom commands
void sigma_load_advanced_automation(void);
void sigma_load_advanced_monitoring(void);
void sigma_load_advanced_security(void);
void sigma_load_container_commands(void);
void sigma_load_virtualization_commands(void);
void sigma_load_development_commands(void);
void sigma_load_system_admin_commands(void);
void sigma_load_network_admin_commands(void);
void sigma_load_storage_admin_commands(void);
void sigma_load_hardware_commands(void);

#endif // SIGMA_EXTENDED_COMMANDS_H

