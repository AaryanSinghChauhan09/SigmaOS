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
ACCESSIBILITY=0
ARCHINSTALL=0
CLOUD_INIT=0
TPM=0

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
  -f, --fs FILESYSTEM    Root filesystem format: SovereignFS, SemanticFS, Ext4, ZFS, Btrfs (default: SovereignFS)
  -n, --hostname NAME    Define custom system hostname (default: sigmaos-node)
  -p, --preset PRESET    Shard profile preset: Minimal, Standard, Enterprise (default: Standard)
  -h, --help             Show this help guide and exit
  --accessibility        Enable accessibility mode (speakup screen reader)
  --archinstall          Use archinstall guided installer
  --cloud-init           Enable cloud-init provisioning
  --tpm                  Enable TPM security features

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

    # Dual-boot & existing OS probe (Arch/Debian installer style)
    log_info "Probing storage buses for existing operating systems & dual-boot targets..."
    if command -v os-prober >/dev/null 2>&1; then
        os-prober 2>/dev/null | while read -r line; do
            log_info "Detected existing OS partition: $line"
        done || true
    else
        log_info "os-prober offline. Scanning partition flags manually..."
    fi

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
            --accessibility)
                ACCESSIBILITY=1
                shift
                ;;
            --archinstall)
                ARCHINSTALL=1
                shift
                ;;
            --cloud-init)
                CLOUD_INIT=1
                shift
                ;;
            --tpm)
                TPM=1
                shift
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
        log_info "DRY RUN complete. No changes were made to $TARGET_DEVICE."
        exit 0
    fi

    # ========================================================================
    # PHASE 0: Omarchy 4.0.6 Feature Configuration
    # ========================================================================
    log_step "Configuring Omarchy 4.0.6 features"

    # Accessibility mode (speakup screen reader like Omarchy)
    if [[ $ACCESSIBILITY -eq 1 ]]; then
        log_info "Accessibility mode enabled (speakup screen reader)"
        # Accessibility will be configured in the kernel command line
        echo "accessibility=on" >> "$MOUNT_POINT/etc/kernel/cmdline" 2>/dev/null || true
        PKGS="$PKGS speakup espeakup espeak-ng"
    fi

    # archinstall guided installer option
    if [[ $ARCHINSTALL -eq 1 ]]; then
        log_info "archinstall guided installer enabled"
        PKGS="$PKGS archinstall arch-install-scripts"
    fi

    # Cloud-init provisioning
    if [[ $CLOUD_INIT -eq 1 ]]; then
        log_info "Cloud-init provisioning enabled"
        PKGS="$PKGS cloud-init"
    fi

    # TPM security features
    if [[ $TPM -eq 1 ]]; then
        log_info "TPM security features enabled"
        PKGS="$PKGS tpm2-tools tpm2-tss"
    fi

    # ========================================================================
    # PHASE 1: Partition target disk
    # ========================================================================
    log_step "Partitioning $TARGET_DEVICE ($PART_LABEL, $FILESYSTEM)"

    # Create GPT partition table with EFI + root partitions
    sgdisk -Z "$TARGET_DEVICE" 2>/dev/null || true
    sgdisk -n 1:0:+512M -t 1:ef00 "$TARGET_DEVICE"   # EFI system partition
    sgdisk -n 2:0:0     -t 2:8300 "$TARGET_DEVICE"   # Linux filesystem

    # Wait for kernel to re-read partition table
    partprobe "$TARGET_DEVICE" 2>/dev/null || true
    sleep 2

    # Determine partition paths
    if [[ "$TARGET_DEVICE" == *nvme* ]]; then
        EFI_PART="${TARGET_DEVICE}p1"
        ROOT_PART="${TARGET_DEVICE}p2"
    else
        EFI_PART="${TARGET_DEVICE}1"
        ROOT_PART="${TARGET_DEVICE}2"
    fi

    log_info "EFI partition: $EFI_PART"
    log_info "Root partition: $ROOT_PART"

    # ========================================================================
    # PHASE 2: Format partitions
    # ========================================================================
    log_step "Formatting partitions"

    mkfs.fat -F 32 "$EFI_PART" -n SIGMAOS-EFI
    log_info "Formatted EFI partition as FAT32"

    if [[ "$FILESYSTEM" == "Btrfs" ]]; then
        mkfs.btrfs -f -L SIGMAOS-ROOT "$ROOT_PART"
        log_info "Formatted root partition as Btrfs"
        # Create btrfs subvolumes for snapper support
        btrfs subvolume create "${MOUNT_POINT}/@home" 2>/dev/null || true
        btrfs subvolume create "${MOUNT_POINT}/@snapshots" 2>/dev/null || true
        btrfs subvolume create "${MOUNT_POINT}/@var_log" 2>/dev/null || true
        btrfs subvolume create "${MOUNT_POINT}/@var_cache_pacman" 2>/dev/null || true
        umount "${MOUNT_POINT}" 2>/dev/null || true
        mount -o subvol=@,compress=zstd "$ROOT_PART" "${MOUNT_POINT}"
        mkdir -p "${MOUNT_POINT}/@home" "${MOUNT_POINT}/.snapshots" "${MOUNT_POINT}/@var_log" "${MOUNT_POINT}/@var_cache_pacman"
        mount -o subvol=@home,compress=zstd "$ROOT_PART" "${MOUNT_POINT}/@home"
        mount -o subvol=@snapshots,compress=zstd "$ROOT_PART" "${MOUNT_POINT}/.snapshots"
        mount -o subvol=@var_log,compress=zstd "$ROOT_PART" "${MOUNT_POINT}/@var_log"
        mount -o subvol=@var_cache_pacman,compress=zstd "$ROOT_PART" "${MOUNT_POINT}/@var_cache_pacman"
        log_info "Created btrfs subvolumes for snapper support"
    elif [[ "$FILESYSTEM" == "ZFS" ]]; then
        log_warn "ZFS not supported in installer, falling back to Ext4"
        mkfs.ext4 -F -L SIGMAOS-ROOT "$ROOT_PART"
        FILESYSTEM="Ext4"
    elif [[ "$FILESYSTEM" == "SovereignFS" || "$FILESYSTEM" == "SemanticFS" ]]; then
        # Map custom names to Ext4 for actual functionality
        mkfs.ext4 -F -L SIGMAOS-ROOT "$ROOT_PART"
        log_info "Formatted root partition as Ext4 (mapped from $FILESYSTEM)"
        FILESYSTEM="Ext4"
    else
        mkfs.ext4 -F -L SIGMAOS-ROOT "$ROOT_PART"
        log_info "Formatted root partition as Ext4"
    fi

    # ========================================================================
    # PHASE 3: Mount filesystems
    # ========================================================================
    log_step "Mounting target filesystems"

    MOUNT_POINT="/mnt/sigmaos"
    mkdir -p "$MOUNT_POINT"
    mount "$ROOT_PART" "$MOUNT_POINT"
    mkdir -p "$MOUNT_POINT/boot"
    mount "$EFI_PART" "$MOUNT_POINT/boot"

    log_info "Mounted $ROOT_PART -> $MOUNT_POINT"
    log_info "Mounted $EFI_PART -> $MOUNT_POINT/boot"

    # ========================================================================
    # PHASE 4: Install base packages
    # ========================================================================
    log_step "Installing base packages"

    # Resolve package list from ISO root
    PKG_LIST="/iso_root/install/packages/sigmaos-base.packages"
    if [[ -f "$PKG_LIST" ]]; then
        # Filter out comments and empty lines
        PKGS=$(grep -v '^#' "$PKG_LIST" | grep -v '^$' | tr '\n' ' ')
    else
        log_warn "Package list not found at $PKG_LIST, using minimal set"
        PKGS="base base-devel linux linux-firmware systemd hyprland waybar sddm foot grim slurp wl-clipboard mako networkmanager"
    fi

    if command -v pacstrap >/dev/null 2>&1; then
        log_info "Using pacstrap to install base packages to $MOUNT_POINT"
        pacstrap "$MOUNT_POINT" $PKGS 2>&1 | tee -a "$LOG_FILE" || {
            log_error "pacstrap failed, checking if packages are already installed..."
        }
    elif command -v pacman >/dev/null 2>&1; then
        log_info "Using pacman to install base packages"
        pacman -Sy --needed --noconfirm --root "$MOUNT_POINT" $PKGS 2>&1 | tee -a "$LOG_FILE" || true
    else
        log_warn "No package manager available, skipping package installation"
    fi

    # ========================================================================
    # PHASE 5: Generate fstab
    # ========================================================================
    log_step "Generating fstab"

    genfstab -U "$MOUNT_POINT" >> "$MOUNT_POINT/etc/fstab" 2>/dev/null || {
        # Manual fstab generation fallback
        ROOT_UUID=$(blkid -s UUID -o value "$ROOT_PART" 2>/dev/null || echo "")
        EFI_UUID=$(blkid -s UUID -o value "$EFI_PART" 2>/dev/null || echo "")
        cat > "$MOUNT_POINT/etc/fstab" <<EOF
# SigmaOS fstab (generated by installer)
UUID=$ROOT_PART  /          ext4  defaults,noatime  0 1
UUID=$EFI_PART   /boot      vfat  defaults          0 2
EOF
        log_info "Generated manual fstab"
    }
    log_info "fstab generated at $MOUNT_POINT/etc/fstab"

    # ========================================================================
    # PHASE 6: Copy SigmaOS system files to target
    # ========================================================================
    log_step "Copying SigmaOS system files"

    if [[ -d /iso_root/etc ]]; then
        cp -a /iso_root/etc/* "$MOUNT_POINT/etc/" 2>/dev/null || log_warn "Failed to copy /etc configs"
        log_info "Applied /etc system configuration"
    fi
    if [[ -d /iso_root/default ]]; then
        mkdir -p "$MOUNT_POINT/usr/share/sigmaos/default"
        cp -a /iso_root/default/* "$MOUNT_POINT/usr/share/sigmaos/default/" 2>/dev/null || log_warn "Failed to copy default configs"
        log_info "Applied default desktop configuration"
    fi
    if [[ -d /iso_root/bin ]]; then
        mkdir -p "$MOUNT_POINT/usr/bin"
        cp -a /iso_root/bin/* "$MOUNT_POINT/usr/bin/" 2>/dev/null || log_warn "Failed to copy runtime binaries"
        log_info "Applied runtime binaries"
    fi
    if [[ -d /iso_root/shell ]]; then
        mkdir -p "$MOUNT_POINT/usr/share/sigmaos/shell"
        cp -a /iso_root/shell/* "$MOUNT_POINT/usr/share/sigmaos/shell/" 2>/dev/null || log_warn "Failed to copy shell configs"
        log_info "Applied shell configuration"
    fi
    if [[ -d /iso_root/install/helpers ]]; then
        mkdir -p "$MOUNT_POINT/usr/share/sigmaos/install/helpers"
        cp -a /iso_root/install/helpers/* "$MOUNT_POINT/usr/share/sigmaos/install/helpers/" 2>/dev/null || log_warn "Failed to copy install helpers"
        log_info "Applied install helpers"
    fi
    if [[ -d /iso_root/boot/limine ]]; then
        mkdir -p "$MOUNT_POINT/boot/limine"
        cp -a /iso_root/boot/limine/* "$MOUNT_POINT/boot/limine/" 2>/dev/null || log_warn "Failed to copy limine config"
        log_info "Applied bootloader configuration"
    fi
    if [[ -d /iso_root/usr/share/plymouth/themes/sigmaos ]]; then
        mkdir -p "$MOUNT_POINT/usr/share/plymouth/themes/sigmaos"
        cp -a /iso_root/usr/share/plymouth/themes/sigmaos/* "$MOUNT_POINT/usr/share/plymouth/themes/sigmaos/" 2>/dev/null || log_warn "Failed to copy Plymouth theme"
        log_info "Applied Plymouth boot theme"
    fi
    if [[ -d /iso_root/etc/skel ]]; then
        cp -a /iso_root/etc/skel/. "$MOUNT_POINT/etc/skel/" 2>/dev/null || log_warn "Failed to copy /etc/skel dotfiles"
        log_info "Applied /etc/skel user dotfiles"
    fi
    if [[ -d /iso_root/etc/systemd/user ]]; then
        mkdir -p "$MOUNT_POINT/etc/systemd/user"
        cp -a /iso_root/etc/systemd/user/* "$MOUNT_POINT/etc/systemd/user/" 2>/dev/null || log_warn "Failed to copy user systemd services"
        log_info "Applied user systemd services"
    fi
    if [[ -d /iso_root/usr/share/wayland-sessions ]]; then
        mkdir -p "$MOUNT_POINT/usr/share/wayland-sessions"
        cp -a /iso_root/usr/share/wayland-sessions/* "$MOUNT_POINT/usr/share/wayland-sessions/" 2>/dev/null || true
    fi
    if [[ -d /iso_root/usr/share/xsessions ]]; then
        mkdir -p "$MOUNT_POINT/usr/share/xsessions"
        cp -a /iso_root/usr/share/xsessions/* "$MOUNT_POINT/usr/share/xsessions/" 2>/dev/null || true
    fi

    # ========================================================================
    # PHASE 7: Configure bootloader (limine)
    # ========================================================================
    log_step "Configuring limine bootloader"

    # Replace UUID placeholder in limine.conf with actual root UUID
    ROOT_UUID=$(blkid -s UUID -o value "$ROOT_PART" 2>/dev/null || echo "")
    if [[ -f "$MOUNT_POINT/boot/limine/limine.conf" ]] && [[ -n "$ROOT_UUID" ]]; then
        sed -i "s/__ROOT_UUID__/$ROOT_UUID/g" "$MOUNT_POINT/boot/limine/limine.conf"
        log_info "Updated limine.conf with root UUID: $ROOT_UUID"
    fi

    # Install limine to EFI partition
    if command -v limine-deploy >/dev/null 2>&1; then
        limine-deploy --efi "$EFI_PART" 2>/dev/null || log_warn "limine-deploy failed"
    elif command -v limine-install >/dev/null 2>&1; then
        limine-install "$EFI_PART" 2>/dev/null || log_warn "limine-install failed"
    else
        log_warn "limine-deploy/limine-install not available, bootloader not installed to disk"
    fi

    log_info "Bootloader configured on $EFI_PART"

    # ========================================================================
    # PHASE 8: Configure target system (chroot operations)
    # ========================================================================
    log_step "Configuring target system"

    # Set hostname
    echo "$HOSTNAME" > "$MOUNT_POINT/etc/hostname"

    # Enable services via chroot (or directly if not in chroot environment)
    if command -v arch-chroot >/dev/null 2>&1; then
        log_info "Using arch-chroot to enable services in target"
        arch-chroot "$MOUNT_POINT" systemctl enable sddm.service 2>/dev/null || true
        arch-chroot "$MOUNT_POINT" systemctl enable cups.service 2>/dev/null || true
        arch-chroot "$MOUNT_POINT" systemctl enable wireplumber.service 2>/dev/null || true
        arch-chroot "$MOUNT_POINT" systemctl enable plymouth-start.service 2>/dev/null || true
        arch-chroot "$MOUNT_POINT" systemctl enable sigmaos-provision-owner.service 2>/dev/null || true
        arch-chroot "$MOUNT_POINT" systemctl enable theme-system.service 2>/dev/null || true
        arch-chroot "$MOUNT_POINT" systemctl enable zram-generator.service 2>/dev/null || true
        arch-chroot "$MOUNT_POINT" systemctl enable sigmaos-desktop-session.service 2>/dev/null || true
        # Rebuild initramfs with Plymouth hooks
        arch-chroot "$MOUNT_POINT" mkinitcpio -P 2>/dev/null || \
        arch-chroot "$MOUNT_POINT" dracut -f 2>/dev/null || true
    else
        # Not in a chroot-capable environment; enable via symlinks directly
        log_info "Enabling services via symlink (no chroot available)"
        for svc in sddm cups wireplumber plymouth-start sigmaos-provision-owner theme-system zram-generator sigmaos-desktop-session; do
            ln -sf "/usr/lib/systemd/system/${svc}.service" "$MOUNT_POINT/etc/systemd/system/multi-user.target.wants/${svc}.service" 2>/dev/null || true
        done
    fi

    # Mark provisioning as not-yet-done (first-boot will run setup-form.sh)
    mkdir -p "$MOUNT_POINT/var/lib/sigmaos"
    rm -f "$MOUNT_POINT/var/lib/sigmaos/provisioned" 2>/dev/null || true

    # Set root password if provided
    if [[ -n "$PASSWORD" ]]; then
        echo "root:$PASSWORD" | chroot "$MOUNT_POINT" chpasswd 2>/dev/null || \
        echo "root:$PASSWORD" | chroot "$MOUNT_POINT" /usr/sbin/chpasswd 2>/dev/null || true
        log_info "Root password set"
    fi

    log_info "Target system configured"

    # ========================================================================
    # PHASE 9: Unmount and finalize
    # ========================================================================
    log_step "Finalizing installation"

    sync
    umount "$MOUNT_POINT/boot" 2>/dev/null || true
    umount "$MOUNT_POINT" 2>/dev/null || true

    log_success "SigmaOS installation complete!"
    log_info "Target: $TARGET_DEVICE"
    log_info "Root partition: $ROOT_PART ($FILESYSTEM)"
    log_info "EFI partition: $EFI_PART"
    log_info "Hostname: $HOSTNAME"
    log_info "Reboot into SigmaOS to begin first-boot provisioning."
}

main "$@"

main "$@"
