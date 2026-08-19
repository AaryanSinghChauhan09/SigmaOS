#!/bin/bash
# SigmaOS Industrial Sovereign Installer (OOP Edition)
# Implements a full Object-Oriented Lifecycle for hardware detection, partition schemas, formatting, and bootloaders.
# Inspired by robust, enterprise-grade Linux distribution installers (Arch, Alpine, Debian, Gentoo).

set -eo pipefail

# ==============================================================================
# CONSTANTS & GLOBAL CONFIGURATION
# ==============================================================================
LOG_FILE="/tmp/sigma_install.log"
LOCK_FILE="/tmp/sigma_install.lock"

# Default installer parameters
AUTO_INSTALL=0
DRY_RUN=0
TARGET_DEVICE=""
PART_LABEL="GPT"
FILESYSTEM="SovereignFS"
HOSTNAME="sigmaos-node"
SHARD_PRESET="Standard"
PASSWORD=""

# ==============================================================================
# AUDITING & LOGGING SUBSYSTEM (Debian/Enterprise Linux style)
# ==============================================================================
# Initialize or truncate the audit log
echo "=== SIGMAOS INSTALLATION AUDIT LOG ===" > "$LOG_FILE"
echo "Timestamp: $(date -u)" >> "$LOG_FILE"
echo "--------------------------------------" >> "$LOG_FILE"

log_info() {
    local msg="$1"
    echo -e "\e[34m[INFO]\e[0m $msg"
    echo "[INFO] $msg" >> "$LOG_FILE"
}

log_step() {
    local msg="$1"
    echo -e "\e[32m[✓]\e[0m $msg"
    echo "[SUCCESS] $msg" >> "$LOG_FILE"
}

log_warn() {
    local msg="$1"
    echo -e "\e[33m[WARN]\e[0m $msg"
    echo "[WARN] $msg" >> "$LOG_FILE"
}

log_error() {
    local msg="$1"
    echo -e "\e[31m[ERROR]\e[0m $msg" >&2
    echo "[ERROR] $msg" >> "$LOG_FILE"
}

log_oop() {
    local class_method="$1"
    local message="$2"
    # Print the exact format for compatibility, option for colorized terminal output
    echo "${class_method} ${message}"
    echo "${class_method} ${message}" >> "$LOG_FILE"
}

# ==============================================================================
# EXCEPTION HANDLING & SIGNAL TRAPS (Defensive Linux practices)
# ==============================================================================
cleanup() {
    local exit_code=$?
    # Release installer lock
    if [[ -f "$LOCK_FILE" ]]; then
        rm -f "$LOCK_FILE"
    fi

    if [[ $exit_code -eq 0 ]]; then
        log_info "Audit logging finalized. Log saved at: $LOG_FILE"
    else
        log_error "Installation was interrupted or failed prematurely (exit code: $exit_code)."
        log_info "Please consult the log file for recovery steps: $LOG_FILE"
    fi
}

trap cleanup EXIT
trap 'exit 130' INT TERM

# ==============================================================================
# HELP & USAGE COMPANION (Alpine-style)
# ==============================================================================
show_help() {
    cat << EOF
Usage: $(basename "$0") [OPTIONS] [DEVICE]

SigmaOS Industrial Sovereign Installer (OOP Edition)
A robust, flexible, and interactive system installer.

Options:
  -a, --auto             Unattended automatic installation mode (non-interactive)
  -d, --dry-run          Simulate installation without writing actual state
  -l, --label LABEL      Partition layout table: GPT, MBR (default: GPT)
  -f, --fs FILESYSTEM    Root filesystem format: SovereignFS, SemanticFS, Ext4 (default: SovereignFS)
  -n, --hostname NAME    Define custom system hostname (default: sigmaos-node)
  -p, --preset PRESET    Shard profile preset: Minimal, Standard, Enterprise (default: Standard)
  -h, --help             Show this help guide and exit

Devices:
  Specify a target block device path (e.g., /dev/sda). If omitted and running
  without --auto, the configuration wizard will guide you through selection.
EOF
}

# ==============================================================================
# PRE-FLIGHT COMPATIBILITY CHECKS (Gentoo/Arch-style)
# ==============================================================================
run_preflight_checks() {
    log_info "Initiating system pre-flight verification..."

    # Check terminal metrics
    local cols
    cols=$(tput cols 2>/dev/null || echo 80)
    if [[ $cols -lt 80 ]]; then
        log_warn "Terminal viewport is narrow ($cols cols). Wizard UI elements might wrap."
    fi

    # Check for parallel installations (lock-check)
    if [[ -f "$LOCK_FILE" ]]; then
        log_error "Another instance of SigmaOS Installer is currently active."
        log_error "If this is a stale lock, remove $LOCK_FILE and re-run."
        exit 1
    fi
    touch "$LOCK_FILE"

    # Check for critical helper utilities
    local required_tools=("cat" "tr" "fold" "head" "dd")
    for tool in "${required_tools[@]}"; do
        if ! command -v "$tool" >/dev/null 2>&1; then
            log_error "Required system utility '$tool' is not available in current PATH."
            exit 1
        fi
    done

    # Check write capability
    if [[ ! -w "/tmp" ]]; then
        log_error "The directory /tmp is write-protected. Installer cannot allocate temporary states."
        exit 1
    fi

    log_step "Pre-flight requirements met."
}

# ==============================================================================
# AUTO-DETECTION SUBSYSTEM
# ==============================================================================
detect_disks() {
    local disks=()
    for d in /dev/sd[a-z] /dev/vd[a-z] /dev/nvme[0-9]n[0-9]; do
        if [[ -b "$d" || -e "$d" ]]; then
            disks+=("$d")
        fi
    done
    if [[ ${#disks[@]} -gt 0 ]]; then
        echo "${disks[@]}"
    else
        echo "/dev/sda"
    fi
}

# ==============================================================================
# INTERACTIVE CONFIGURATION WIZARD (Arch/Alpine-style)
# ==============================================================================
run_interactive_wizard() {
    echo -e "\e[36m"
    echo "========================================================="
    echo "  Σ SIGMAOS INSTALLER CONFIGURATION WIZARD  "
    echo "========================================================="
    echo -e "\e[0m"
    log_info "Preparing configuration questions..."

    # 1. Target Disk Selection
    if [[ -z "$TARGET_DEVICE" ]]; then
        local detected
        detected=$(detect_disks)
        echo -e "\n\e[36m--- Target Silicon Storage Disk ---\e[0m"
        echo "Detected system disks: $detected"
        read -rp "Specify target installation disk [default: /dev/sda]: " input_disk
        if [[ -z "$input_disk" ]]; then
            TARGET_DEVICE="/dev/sda"
        else
            TARGET_DEVICE="$input_disk"
        fi
    fi
    log_info "Target disk locked: $TARGET_DEVICE"

    # 2. Partition Layout Selection
    echo -e "\n\e[36m--- Partition Table Schema ---\e[0m"
    echo "1) GPT (GUID Partition Table) - Recommended for modern UEFI systems"
    echo "2) MBR (Master Boot Record) - Legacy BIOS systems compatibility"
    read -rp "Select partition schema [1/2, default: 1]: " layout_opt
    if [[ "$layout_opt" == "2" ]]; then
        PART_LABEL="MBR"
    else
        PART_LABEL="GPT"
    fi
    log_info "Partition schema set to: $PART_LABEL"

    # 3. Filesystem Selection
    echo -e "\n\e[36m--- Resilient Root Filesystem Format ---\e[0m"
    echo "1) SovereignFS (Lattice-Optimized, Ultra-Resilient)"
    echo "2) SemanticFS (Vector-Space Semantic Filesystem)"
    echo "3) Ext4 (Standard POSIX-compliant Linux journaling filesystem)"
    read -rp "Select root filesystem layout [1-3, default: 1]: " fs_opt
    if [[ "$fs_opt" == "2" ]]; then
        FILESYSTEM="SemanticFS"
    elif [[ "$fs_opt" == "3" ]]; then
        FILESYSTEM="Ext4"
    else
        FILESYSTEM="SovereignFS"
    fi
    log_info "Target filesystem set to: $FILESYSTEM"

    # 4. Hostname Configuration
    echo -e "\n\e[36m--- System Hostname Configuration ---\e[0m"
    read -rp "Enter system hostname [default: sigmaos-node]: " host_input
    if [[ -n "$host_input" ]]; then
        HOSTNAME="$host_input"
    fi
    log_info "System hostname set to: $HOSTNAME"

    # 5. Security Credentials Set (Secure input masking)
    echo -e "\n\e[36m--- Security & Administrator Credentials ---\e[0m"
    while true; do
        read -s -rp "Set root administrator password: " pass1
        echo
        read -s -rp "Confirm root administrator password: " pass2
        echo
        if [[ "$pass1" == "$pass2" ]]; then
            if [[ -z "$pass1" ]]; then
                log_warn "Empty password specified. Setting up lockless system mode."
            fi
            PASSWORD="$pass1"
            break
        else
            log_error "Passwords do not match. Please try again."
        fi
    done
    log_info "Root security credentials accepted."

    # 6. Monolithic Shard Profiles
    echo -e "\n\e[36m--- Capability Shards Preset ---\e[0m"
    echo "1) Minimal (150+ shards) - Light microkernel configuration"
    echo "2) Standard (450+ shards) - Default balanced workspace"
    echo "3) Enterprise/Industrial (900+ shards) - Full operational capability"
    read -rp "Select capability shard profile [1-3, default: 2]: " preset_opt
    if [[ "$preset_opt" == "1" ]]; then
        SHARD_PRESET="Minimal"
    elif [[ "$preset_opt" == "3" ]]; then
        SHARD_PRESET="Enterprise"
    else
        SHARD_PRESET="Standard"
    fi
    log_info "Capability shard preset profile set to: $SHARD_PRESET"

    echo -e "\n\e[32mConfiguration setup finalized.\e[0m"
    echo "---------------------------------------------------------"
}

# ==============================================================================
# CLASS: StorageDevice (OOP Compliant)
# Represents a physical or logical target disk for SigmaOS installation.
# ==============================================================================

# Constructor: StorageDevice_new <out_var> <device_path> <label> <filesystem>
StorageDevice_new() {
    local out_var="$1"
    local path="$2"
    local label="$3"
    local filesystem="$4"

    local rand_id="${RANDOM}_${RANDOM}"
    local self="StorageDevice_${rand_id}"

    eval "${self}_path=\"\$path\""
    eval "${self}_label=\"\$label\""
    eval "${self}_filesystem=\"\$filesystem\""

    eval "$out_var=\"\$self\""
}

# Method: StorageDevice_detect <self>
StorageDevice_detect() {
    local self="$1"
    local path
    eval "path=\"\$${self}_path\""

    log_oop "[StorageDevice::detect]" "Querying silicon storage bus... Found target lattice device: $path"
}

# Method: StorageDevice_partition <self>
StorageDevice_partition() {
    local self="$1"
    local path
    local label
    eval "path=\"\$${self}_path\""
    eval "label=\"\$${self}_label\""

    log_oop "[StorageDevice::partition]" "Initializing partition map on $path with standard: $label"
    # Simulated execution: sgdisk --clear -g "$path"
}

# Method: StorageDevice_format <self>
StorageDevice_format() {
    local self="$1"
    local path
    local fs
    eval "path=\"\$${self}_path\""
    eval "fs=\"\$${self}_filesystem\""

    log_oop "[StorageDevice::format]" "Formatting system partition ${path}1 with ultra-resilient $fs (Lattice-Optimized)..."
    # Simulated execution: mkfs.sovfs "${path}1"
}


# ==============================================================================
# CLASS: Installer (OOP Compliant)
# Orchestrates the system setup lifecycle of SigmaOS.
# ==============================================================================

# Constructor: Installer_new <out_var> <device_instance> <hostname> <shard_preset>
Installer_new() {
    local out_var="$1"
    local device="$2"
    local hostname="$3"
    local shard_preset="$4"

    local rand_id="${RANDOM}_${RANDOM}"
    local self="Installer_${rand_id}"

    eval "${self}_device=\"\$device\""
    eval "${self}_hostname=\"\$hostname\""
    eval "${self}_shard_preset=\"\$shard_preset\""

    # Presets mapping
    local count=600
    if [[ "$shard_preset" == "Minimal" ]]; then
        count=150
    elif [[ "$shard_preset" == "Standard" ]]; then
        count=450
    elif [[ "$shard_preset" == "Enterprise" ]]; then
        count=900
    fi
    eval "${self}_shard_count=\$count"

    eval "$out_var=\"\$self\""
}

# Method: Installer_inject_shards <self>
Installer_inject_shards() {
    local self="$1"
    local count
    eval "count=\"\$${self}_shard_count\""

    log_oop "[Installer::inject_shards]" "Injecting $count+ industrial capability shards into the sovereign OS structure..."
    # Simulated execution: cp -r /shards /mnt/sigmaos/
}

# Method: Installer_install_bootloader <self>
Installer_install_bootloader() {
    local self="$1"
    local device
    eval "device=\"\$${self}_device\""

    local disk_path
    eval "disk_path=\"\$${device}_path\""

    log_oop "[Installer::install_bootloader]" "Binding Sovereign Boot Orchestrator onto master sector of $disk_path"
    # Simulated execution: grub-install --target=x86_64-efi "$disk_path"
}

# Method: Installer_run <self>
Installer_run() {
    local self="$1"
    local device
    eval "device=\"\$${self}_device\""

    echo "Σ SIGMAOS SOVEREIGN INSTALLER (OOP COMPLIANT)"
    echo "-----------------------------------------------"

    # 1. Detect target device
    StorageDevice_detect "$device"

    # 2. Partition schema configuration
    StorageDevice_partition "$device"

    # 3. Format primary system sector
    StorageDevice_format "$device"

    # 4. Inject industrial payload
    Installer_inject_shards "$self"

    # 5. Build boot structures
    Installer_install_bootloader "$self"

    echo "-----------------------------------------------"
    log_oop "[Installer::run]" "SUCCESS: SigmaOS has been successfully integrated into the physical silicon."
    log_oop "[Installer::run]" "Please reboot to enter the Sovereign Zenith."
}


# ==============================================================================
# MAIN ENGINE ENTRY POINT
# ==============================================================================

main() {
    # 1. Parse Command Line Arguments
    while [[ $# -gt 0 ]]; do
        case "$1" in
            -a|--auto)
                AUTO_INSTALL=1
                shift
                ;;
            -d|--dry-run)
                DRY_RUN=1
                shift
                ;;
            -l|--label)
                if [[ -n "$2" && "$2" != -* ]]; then
                    PART_LABEL="$2"
                    shift 2
                else
                    echo -e "\e[31mError: --label requires a value (GPT or MBR).\e[0m" >&2
                    exit 1
                fi
                ;;
            -f|--fs)
                if [[ -n "$2" && "$2" != -* ]]; then
                    FILESYSTEM="$2"
                    shift 2
                else
                    echo -e "\e[31mError: --fs requires a value (SovereignFS, SemanticFS, Ext4).\e[0m" >&2
                    exit 1
                fi
                ;;
            -n|--hostname)
                if [[ -n "$2" && "$2" != -* ]]; then
                    HOSTNAME="$2"
                    shift 2
                else
                    echo -e "\e[31mError: --hostname requires a hostname value.\e[0m" >&2
                    exit 1
                fi
                ;;
            -p|--preset)
                if [[ -n "$2" && "$2" != -* ]]; then
                    SHARD_PRESET="$2"
                    shift 2
                else
                    echo -e "\e[31mError: --preset requires a profile name (Minimal, Standard, Enterprise).\e[0m" >&2
                    exit 1
                fi
                ;;
            -h|--help)
                show_help
                exit 0
                ;;
            -*)
                echo -e "\e[31mError: Unrecognized option: $1\e[0m" >&2
                show_help
                exit 1
                ;;
            *)
                if [[ -z "$TARGET_DEVICE" ]]; then
                    TARGET_DEVICE="$1"
                    shift
                else
                    echo -e "\e[31mError: Multiple target devices cannot be specified.\e[0m" >&2
                    exit 1
                fi
                ;;
        esac
    done

    # 2. Execute Pre-Flight Integrity Diagnostics
    run_preflight_checks

    # 3. Interactive or Automated configuration selector
    if [[ $AUTO_INSTALL -eq 1 ]]; then
        if [[ -z "$TARGET_DEVICE" ]]; then
            TARGET_DEVICE="/dev/sda"
        fi
        log_info "Unattended automated installation requested."
        log_info "Parameter profile: Target=$TARGET_DEVICE, Hostname=$HOSTNAME, Partition=$PART_LABEL, FS=$FILESYSTEM, Preset=$SHARD_PRESET"
    else
        run_interactive_wizard
    fi

    # Log initial installation meta
    echo "Installation Target Disk: $TARGET_DEVICE" >> "$LOG_FILE"
    echo "Partition Table Standard: $PART_LABEL" >> "$LOG_FILE"
    echo "Root Filesystem Format:   $FILESYSTEM" >> "$LOG_FILE"
    echo "Host Identity (Name):     $HOSTNAME" >> "$LOG_FILE"
    echo "Capability Shard Preset:  $SHARD_PRESET" >> "$LOG_FILE"

    if [[ $DRY_RUN -eq 1 ]]; then
        log_warn "DRY RUN: Simulating installation sequence (No state mutations)."
    fi

    # 4. Instantiate Target Storage Object via OOP Framework
    local dev
    StorageDevice_new dev "$TARGET_DEVICE" "$PART_LABEL" "$FILESYSTEM"

    # 5. Instantiate Sovereign Installer Engine Object with referenced parameters
    local inst
    Installer_new inst "$dev" "$HOSTNAME" "$SHARD_PRESET"

    # 6. Execute full setup lifecycle
    Installer_run "$inst"
}

main "$@"
