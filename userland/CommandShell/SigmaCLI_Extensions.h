#ifndef SIGMA_CLI_EXTENSIONS_H
#define SIGMA_CLI_EXTENSIONS_H

#include "suites/S01_Genesis/shards/sigma_types.h"

// SigmaOS Command Shell - Extended Utility Set
// The second wave of native commands, built for text streams, networking, and disk management.

// 1. Stream Processing & Data Mutation (awk/sed/jq equivalent)
// sigastream inherently understands JSON, YAML, and raw memory structs, not just raw text.
int cmd_sigastream(int argc, char** argv);

// 2. High-Speed Network Fetching (curl/wget equivalent)
// Hooks directly into the custom TCP/IP stack and zero-trust proxy layer.
int cmd_sigafetch(int argc, char** argv);

// 3. Disk & Volume Management (fdisk/parted/lsblk equivalent)
// Handles ZFS mounting, RAID configs, partitions, and on-the-fly encryption keys.
int cmd_sigadisk(int argc, char** argv);

// 4. Zero-Trust Permission Management (chmod/chown/setfacl equivalent)
// Integrates directly with the biometric enclave and capability ACLs instead of simple octal UNIX permissions.
int cmd_sigaguard(int argc, char** argv);

// 5. Hardware Diagnostics API (lshw/dmidecode equivalent)
// Queries S04_HAL for raw ACPI, battery, and PCI-E lane statuses.
int cmd_sigahardware(int argc, char** argv);

#endif // SIGMA_CLI_EXTENSIONS_H

